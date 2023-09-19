fn main() {
    let tree = SparseVoxelOctree::new(2);
    dbg!(tree.get(0, 0, 0));
    dbg!(tree.get(1, 0, 0));
    dbg!(tree.get(0, 1, 0));
    dbg!(tree.get(1, 1, 0));
    dbg!(tree.get(0, 0, 1));
    dbg!(tree.get(1, 0, 1));
    dbg!(tree.get(0, 1, 1));
    dbg!(tree.get(1, 1, 1));
}

#[derive(Debug)]
struct SparseVoxelOctree {
    root: Node,
    max_depth: u32,
}

#[derive(Debug)]
enum Node {
    Branch { children: [Box<Node>; 8] },
    Leaf(Option<Voxel>),
}

#[derive(Debug, Clone, Copy)]
struct Voxel {
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
                children: [
                    Box::new(Node::new(depth - 1)),
                    Box::new(Node::new(depth - 1)),
                    Box::new(Node::new(depth - 1)),
                    Box::new(Node::new(depth - 1)),
                    Box::new(Node::new(depth - 1)),
                    Box::new(Node::new(depth - 1)),
                    Box::new(Node::new(depth - 1)),
                    Box::new(Node::new(depth - 1)),
                ],
            }
        }
    }

    fn get(&self, x: u32, y: u32, z: u32, size: u32) -> Option<&Voxel> {
        match self {
            Node::Leaf(voxel) => voxel.as_ref(),
            Node::Branch { children } => {
                let size = size / 2;

                match (x < size, y < size, z < size) {
                    (true, true, true) => children[0].get(x, y, z, size), // 000
                    (false, true, true) => children[1].get(x - size, y, z, size), // 100
                    (true, false, true) => children[2].get(x, y - size, z, size), // 010
                    (false, false, true) => children[3].get(x - size, y - size, z, size), // 110
                    (true, true, false) => children[4].get(x, y, z - size, size), // 001
                    (false, true, false) => children[5].get(x - size, y, z - size, size), // 101
                    (true, false, false) => children[6].get(x, y - size, z - size, size), // 011
                    (false, false, false) => children[7].get(x - size, y - size, z - size, size), // 111
                }
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
                let size = size / 2;

                match (x < size, y < size, z < size) {
                    (true, true, true) => children[0].insert(x, y, z, node, size, depth - 1), // 000
                    (false, true, true) => {
                        children[1].insert(x - size, y, z, node, size, depth - 1)
                    } // 100
                    (true, false, true) => {
                        children[2].insert(x, y - size, z, node, size, depth - 1)
                    } // 010
                    (false, false, true) => {
                        children[3].insert(x - size, y - size, z, node, size, depth - 1)
                    } // 110
                    (true, true, false) => {
                        children[4].insert(x, y, z - size, node, size, depth - 1)
                    } // 001
                    (false, true, false) => {
                        children[5].insert(x - size, y, z - size, node, size, depth - 1)
                    } // 101
                    (true, false, false) => {
                        children[6].insert(x, y - size, z - size, node, size, depth - 1)
                    } // 011
                    (false, false, false) => {
                        children[7].insert(x - size, y - size, z - size, node, size, depth - 1)
                    } // 111
                };
            }
            Node::Leaf(voxel) => {
                *self = Node::Branch {
                    children: [
                        Box::new(Node::Leaf(*voxel)),
                        Box::new(Node::Leaf(*voxel)),
                        Box::new(Node::Leaf(*voxel)),
                        Box::new(Node::Leaf(*voxel)),
                        Box::new(Node::Leaf(*voxel)),
                        Box::new(Node::Leaf(*voxel)),
                        Box::new(Node::Leaf(*voxel)),
                        Box::new(Node::Leaf(*voxel)),
                    ],
                };

                self.insert(x, y, z, node, size / 2, depth - 1)
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
