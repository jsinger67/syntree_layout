use std::fmt;

use syntree::Builder;
use syntree_layout::{Layouter, Result, Visualize};

#[derive(Debug)]
struct MyNodeData(i32);

// You need to implement syntree_layout::Visualize for your nodes data type if you want your own
// node representation.
impl Visualize for MyNodeData {
    fn visualize(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id({})", self.0)
    }

    fn emphasize(&self) -> bool {
        // This simply emphasizes only the leaf nodes.
        // It only works for this example.
        self.0 > 1
    }
}

impl fmt::Display for MyNodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() -> Result<()> {
    //      0
    //     / \
    //    1   2
    //   / \
    //  3   4
    let mut tree = Builder::new();

    tree.open(MyNodeData(0)).unwrap();
    tree.open(MyNodeData(1)).unwrap();
    tree.token(MyNodeData(3), 1).unwrap();
    tree.token(MyNodeData(4), 1).unwrap();
    tree.close().unwrap();
    tree.token(MyNodeData(2), 1).unwrap();
    tree.close().unwrap();

    let tree = tree.build().unwrap();
    Layouter::new(&tree)
        .with_file_path("examples/example1_vis.svg")
        .embed_with_visualize()?
        .write()?;

    Layouter::new(&tree)
        .with_file_path("examples/example1_deb.svg")
        .embed_with_debug()?
        .write()?;

    Layouter::new(&tree)
        .with_file_path("examples/example1_dis.svg")
        .embed()?
        .write()
}
