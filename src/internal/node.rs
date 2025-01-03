use std::collections::HashMap;

use syntree::{Flavor, Pointer};

pub(crate) type NodeId<F> = <F as Flavor>::Pointer;

///
/// The [InternalNode] is the internal embedding information for one single tree node.
///
pub(crate) struct InternalNode<F>
where
    F: Flavor,
{
    /// The nodes level, root has level 0. Can be used to calculate an y coordinate for the node
    pub(crate) y_order: usize,
    /// The logical x coordinate of the node's center
    pub(crate) x_center: usize,
    /// The x-extent of the nodes text representation in logical coordinate units
    pub(crate) x_extent: usize,
    /// Internal value used to sum up the x-extent of all children of the node
    pub(crate) x_extent_of_children: usize,
    /// The maximum extent over the nodes text representation and the sum of all children's x-extent
    pub(crate) x_extent_children: usize,
    /// The text representation of the nodes data - created by the `Visualize` trait's implementation
    pub(crate) text: String,
    /// The *emphasize* property possibly obtained from the `Visualize` trait
    pub(crate) is_emphasized: bool,
    /// The parent's `ord`, if there is one
    pub(crate) parent: Option<usize>,
    /// A unique number reflecting the depth first walk order of the nodes in the tree
    /// It is assumed that parents are inserted before their child nodes
    pub(crate) ord: usize,
    /// Internal node id
    pub(crate) node_id: NodeId<F>,
}

impl<F> Default for InternalNode<F>
where
    F: Flavor,
{
    fn default() -> Self {
        Self {
            y_order: Default::default(),
            x_center: Default::default(),
            x_extent: Default::default(),
            x_extent_of_children: Default::default(),
            x_extent_children: Default::default(),
            text: Default::default(),
            is_emphasized: Default::default(),
            parent: Default::default(),
            ord: Default::default(),
            node_id: F::Pointer::new(0).unwrap(),
        }
    }
}

///
/// Internal helper data
///
pub(crate) struct EmbeddingHelperData<F: Flavor>(
    /// ord => InternalNode
    pub(crate) Vec<InternalNode<F>>,
    /// NodeId => ord
    pub(crate) HashMap<NodeId<F>, usize>,
);

impl<F> EmbeddingHelperData<F>
where
    F: Flavor,
{
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self(
            Vec::with_capacity(capacity),
            HashMap::with_capacity(capacity),
        )
    }

    pub(crate) fn get_by_ord(&self, ord: usize) -> Option<&InternalNode<F>> {
        self.0.get(ord)
    }

    pub(crate) fn get_mut_by_ord(&mut self, ord: usize) -> Option<&mut InternalNode<F>> {
        self.0.get_mut(ord)
    }

    pub(crate) fn get_by_node_id(&self, node_id: &NodeId<F>) -> Option<&InternalNode<F>> {
        self.1.get(node_id).and_then(|n| self.0.get(*n))
    }

    pub(crate) fn get_mut_by_node_id(
        &mut self,
        node_id: &NodeId<F>,
    ) -> Option<&mut InternalNode<F>> {
        self.1.get(node_id).and_then(|n| self.0.get_mut(*n))
    }

    pub(crate) fn insert(&mut self, ord: usize, item: InternalNode<F>) {
        self.1.insert(item.node_id, ord);
        self.0.insert(ord, item);
    }
}
