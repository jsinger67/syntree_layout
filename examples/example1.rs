use syntree::Builder;
use syntree_layout::{Layouter, Result, Visualize};

#[derive(Debug)]
struct MyNodeData(i32);

// You need to implement id_tree_layout::Visualize for your nodes data type.
// This way you provide basic formatting information.
impl Visualize for MyNodeData {
    fn visualize(&self) -> std::string::String {
        // We simply convert the i32 value to string here.
        self.0.to_string()
    }
    fn emphasize(&self) -> bool {
        // This simply emphasizes only the leaf nodes.
        // It only works for this example.
        self.0 > 1
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
        .with_file_path(std::path::Path::new("examples/example1.svg"))
        .embed_with_visualize()?
        .write()
}
