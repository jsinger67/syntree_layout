use std::fmt;

use syntree::{Builder, Tree};
use syntree_layout::{Layouter, Visualize};

#[derive(Debug)]
struct MyNodeData(i32);

impl Visualize for MyNodeData {
    fn visualize(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn empty_tree() {
    let tree: Tree<MyNodeData, _, _> = Builder::new().build().unwrap();
    let layouter = Layouter::new(&tree).embed_with_visualize().unwrap();
    let embedding = layouter.embedding();
    assert!(embedding.is_empty());
}

#[test]
fn tree_with_single_node() {
    let mut tree = Builder::new();
    tree.open(MyNodeData(0)).unwrap();
    tree.close().unwrap();
    let tree = tree.build().unwrap();
    let layouter = Layouter::new(&tree).embed_with_visualize().unwrap();
    let embedding = layouter.embedding();
    assert_eq!(1, embedding.len());

    {
        let e = &embedding[0];
        assert_eq!("0".to_string(), e.text);
        assert_eq!(0, e.y_order);
        assert_eq!(1, e.x_center);
        assert_eq!(2, e.x_extent);
        assert_eq!(2, e.x_extent_children);
    }
}

#[test]
fn more_complex_tree() {
    //      0
    //     / \
    //    1   2
    //   / \
    //  3   4
    let mut tree = Builder::new();

    tree.open(MyNodeData(0)).unwrap();
    tree.open(MyNodeData(1)).unwrap();
    tree.open(MyNodeData(3)).unwrap();
    tree.close().unwrap();
    tree.open(MyNodeData(4)).unwrap();
    tree.close().unwrap();
    tree.close().unwrap();
    tree.open(MyNodeData(2)).unwrap();
    tree.close().unwrap();
    tree.close().unwrap();

    let tree = tree.build().unwrap();

    let mut s = Vec::new();
    syntree::print::print(&mut s, &tree).unwrap();
    let s = String::from_utf8(s).unwrap();
    println!("{s}");

    let layouter = Layouter::new(&tree).embed_with_visualize().unwrap();
    let embedding = layouter.embedding();

    assert!(!embedding.is_empty());
    assert_eq!(5, embedding.len());

    {
        let e = &embedding.iter().find(|e| e.text == "0").unwrap();
        assert_eq!(0, e.ord);
        assert_eq!(0, e.y_order);
        assert_eq!(3, e.x_center);
        assert_eq!(2, e.x_extent);
        assert_eq!(6, e.x_extent_children);
    }
    {
        let e = &embedding.iter().find(|e| e.text == "1").unwrap();
        assert_eq!(1, e.ord);
        assert_eq!(1, e.y_order);
        assert_eq!(2, e.x_center);
        assert_eq!(2, e.x_extent);
        assert_eq!(4, e.x_extent_children);
    }
    {
        let e = &embedding.iter().find(|e| e.text == "2").unwrap();
        assert_eq!(4, e.ord);
        assert_eq!(1, e.y_order);
        assert_eq!(5, e.x_center);
        assert_eq!(2, e.x_extent);
        assert_eq!(2, e.x_extent_children);
    }
    {
        let e = &embedding.iter().find(|e| e.text == "3").unwrap();
        assert_eq!(2, e.ord);
        assert_eq!(2, e.y_order);
        assert_eq!(1, e.x_center);
        assert_eq!(2, e.x_extent);
        assert_eq!(2, e.x_extent_children);
    }
    {
        let e = &embedding.iter().find(|e| e.text == "4").unwrap();
        assert_eq!(3, e.ord);
        assert_eq!(2, e.y_order);
        assert_eq!(3, e.x_center);
        assert_eq!(2, e.x_extent);
        assert_eq!(2, e.x_extent_children);
    }
}
