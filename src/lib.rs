#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn insert() {
        let mut tree = SparseVoxelOctree::new(1);

        tree.insert(0, 0, 0, Node::Leaf(Some(Voxel { index: 9999 })), 3);

        dbg!(tree.get(0, 0, 0));
    }
}

#[derive(Debug)]
pub struct SparseVoxelOctree {
    root: Node,
    max_depth: u32,
}

#[derive(Debug)]
pub enum Node {
    Branch { children: Box<[Node; 8]> },
    Leaf(Option<Voxel>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Voxel {
    index: u32,
}

impl SparseVoxelOctree {
    fn new(depth: u32) -> Self {
        Self {
            root: Node::new(depth),
            max_depth: depth,
        }
    }

    fn get(&self, x: u32, y: u32, z: u32) -> Option<&Voxel> {
        self.root.get(x, y, z, 2_u32.pow(self.max_depth))
    }

    fn insert(&mut self, x: u32, y: u32, z: u32, node: Node, depth: u32) {
        if depth > self.max_depth { self.max_depth = depth; }
        self.root
            .insert(x, y, z, node, 2_u32.pow(self.max_depth), depth);
    }
}

impl Node {
    fn new(depth: u32) -> Self {
        if depth == 0 {
            Node::Leaf(Some(Voxel::new()))
        } else {
            Node::Branch {
                children: Box::new(std::array::from_fn(|_| Node::new(depth - 1))),
            }
        }
    }

    fn iter_get(&self, mut x: u32, mut y: u32, mut z: u32, mut s: u32) -> Option<&Voxel> {
        let mut this = self;

        loop {
            match this {
                Node::Leaf(voxel) => return voxel.as_ref(),
                Node::Branch { children } => {
                    s /= 2;
                    let index = ((x >= s) as usize) << 0 | ((y >= s) as usize) << 1 | ((z >= s) as usize) << 2;

                    this = &children[index];

                    x %= s;
                    y %= s;
                    z %= s;
                }
            }
        }
    }

    fn get(&self, x: u32, y: u32, z: u32, size: u32) -> Option<&Voxel> {
        match self {
            Node::Leaf(voxel) => voxel.as_ref(),
            Node::Branch { children } => {
                let s = size / 2;
                let index = ((x >= s) as usize) << 0 | ((y >= s) as usize) << 1 | ((z >= s) as usize) << 2;

                children[index].get(x % s, y % s, z % s, s)
            }
        }
    }

    fn insert(&mut self, x: u32, y: u32, z: u32, node: Node, size: u32, depth: u32) {
        if depth == 0 {
            *self = node;
            return;
        }

        match self {
            Node::Branch { children } => {
                let s = size / 2;
                let index = ((x >= s) as usize) << 0 | ((y >= s) as usize) << 1 | ((z >= s) as usize) << 2;

                children[index].insert(x % s, y % s, z % s, node, s, depth - 1)
            }
            Node::Leaf(voxel) => {
                let voxel = voxel.clone();
                *self = Node::Branch {
                    children: Box::new(std::array::from_fn(|_| Node::Leaf(voxel))),
                };

                self.insert(x, y, z, node, size, depth);
            }
        }
    }
}

static mut INDEX: u32 = 0;

impl Voxel {
    fn new() -> Self {
        unsafe {
            let index = INDEX;
            INDEX += 1;
            Self { index }
        }
    }
}
