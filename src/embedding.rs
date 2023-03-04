//! The module with the data structures used in the **Public API**.

use syntree::pointer::Width;

use crate::internal::node::InternalNode;

///
/// The Embedding is the interface to drawers that need the embedding for the purpose
/// to transform it to their own output format.
/// It is accessible at the [Layouter][crate::Layouter::embedding] after calling an embed method
///
pub type Embedding = Vec<EmbeddedNode>;

///
/// The [EmbeddedNode] is the embedding information for one single tree node.
/// It is used only in a collection type `Embedding`.
///
#[derive(Debug, Clone, Default)]
pub struct EmbeddedNode {
    /// The nodes level, root has level 0. Can be used to calculate an y coordinate for the node
    pub y_order: usize,
    /// The logical x coordinate of the node's center
    pub x_center: usize,
    /// The x-extent of the nodes text representation in logical coordinate units
    pub x_extent: usize,
    /// The maximum extent over the nodes text representation and the sum of all children's x-extent
    pub x_extent_children: usize,
    /// The text representation of the nodes data - created e.g. by the `Visualize` trait's
    /// implementation, by the node type's Display or Debug implementation or by custom methods
    pub text: String,
    /// The *emphasize* property obtained from the `Visualize` trait or via a custom method
    pub is_emphasized: bool,
    /// The parent's `ord`, if there is one
    pub parent: Option<usize>,
    /// A unique number reflecting the topological post-ordering of the nodes in the tree
    pub ord: usize,
}

///
/// Conversion form internal to external (i.e. public) representation of the embedding structure.
///
impl<P: Width> From<InternalNode<P>> for EmbeddedNode {
    fn from(e: InternalNode<P>) -> Self {
        Self {
            y_order: e.y_order,
            x_center: e.x_center,
            x_extent: e.x_extent,
            x_extent_children: e.x_extent_children,
            text: e.text,
            is_emphasized: e.is_emphasized,
            parent: e.parent,
            ord: e.ord,
        }
    }
}
