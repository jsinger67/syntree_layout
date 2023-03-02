//! The module that holds types to embed nodes of a tree into the plane.

use syntree::{index::Index, pointer::Width, Node, Tree};

use crate::{
    internal::node::{EmbeddingHelperData, InternalNode},
    visualize::Visualize,
    Embedding, LayouterError, Result,
};

///
/// The Embedder type provides a single public method `embed` to arrange nodes of a tree into the
/// plane.
///
pub struct VisualizeEmbedder<T, I, W>
where
    T: Visualize,
    I: Index,
    W: Width,
{
    _1: std::marker::PhantomData<T>,
    _2: std::marker::PhantomData<I>,
    _3: std::marker::PhantomData<W>,
}

impl<T, I, W> VisualizeEmbedder<T, I, W>
where
    T: Visualize,
    I: Index,
    W: Width,
{
    ///
    /// This method creates an embedding of the nodes of the given tree in the plane.
    ///
    /// # Panics
    ///
    /// The method should not panic. If you encounter a panic this should be originated from
    /// bugs in coding. Please report such panics.
    ///
    /// # Complexity
    ///
    /// The algorithm is of time complexity class O(n).
    ///
    pub fn embed(tree: &Tree<T, I, W>) -> Result<Embedding> {
        // Insert all tree items with their indices
        // After this step each item has following properties set:
        // 'y_order', 'x_extent', 'text', 'is_emphasized', 'ord'
        let mut items = Self::create_initial_embedding_data(tree)?;
        debug_assert_eq!(items.0.len(), items.1.len());

        // Set widths (x_extent_children, x_extent_of_children) on each InternalNode structure
        // After this step each item has following properties set:
        // 'y_order', 'x_extent', 'text', 'is_emphasized', 'ord', 'x_extent_children',
        // 'x_extent_of_children', 'parent'
        Self::apply_children_x_extents(tree, &mut items);

        // Finally set the property 'x_center' from leafs to root
        // After this step each item has all necessary properties set
        Self::apply_x_center(&mut items);

        // Transfer result
        Ok(Self::transfer_result(items))
    }

    fn create_from_node(
        ord: usize,
        depth: usize,
        node: Node<T, I, W>,
        items: &EmbeddingHelperData<W>,
    ) -> InternalNode<W> {
        let text = node.value().visualize();
        let y_order = depth;
        let x_center = 0;
        let x_extent = text.len() + 1;
        let x_extent_of_children = 0;
        let x_extent_children = 0;
        let is_emphasized = node.value().emphasize();
        let parent = node
            .parent()
            .map(|p| items.get_by_node_id(&p.id()).map(|n| n.ord))
            .flatten();
        let node_id = node.id();

        InternalNode {
            y_order,
            x_center,
            x_extent,
            x_extent_of_children,
            x_extent_children,
            text,
            is_emphasized,
            parent,
            ord,
            node_id,
        }
    }

    fn create_initial_embedding_data(tree: &Tree<T, I, W>) -> Result<EmbeddingHelperData<W>> {
        let mut items = EmbeddingHelperData::with_capacity(tree.len());
        if tree.children().count() > 1 {
            return Err(LayouterError::from_description(
                "Currently we support only one root".to_string(),
            ));
        }

        tree.walk()
            .with_depths()
            .enumerate()
            .for_each(|(ord, (depth, node))| {
                let new_item = Self::create_from_node(ord, depth, node, &items);
                items.insert(ord, new_item);
            });

        Ok(items)
    }

    fn apply_children_x_extents(tree: &Tree<T, I, W>, items: &mut EmbeddingHelperData<W>) {
        tree.walk().enumerate().for_each(|(ord, node)| {
            let x_extent_children = node.children().fold(0, |acc, child| {
                if let Some(internal_child) = items.get_by_node_id(&child.id()) {
                    acc + internal_child.x_extent_children
                } else {
                    acc
                }
            });
            if let Some(internal_node) = items.get_mut_by_ord(ord) {
                internal_node.x_extent_children = x_extent_children;
            }
        });
    }

    fn x_center_layer(layer: usize, items: &mut EmbeddingHelperData<W>) {
        let node_ids_in_layer =
            items
                .0
                .iter()
                .enumerate()
                .fold(Vec::new(), |mut acc, (ord, item)| {
                    if item.y_order == layer {
                        acc.push(ord)
                    }
                    acc
                });

        let parents_in_layer = node_ids_in_layer
            .iter()
            .map(|ord| items.get_by_ord(*ord).unwrap().parent)
            .collect::<Vec<Option<usize>>>();

        for p in parents_in_layer {
            let nodes_in_layer_per_parent = node_ids_in_layer
                .iter()
                .filter_map(|ord| {
                    if items.get_by_ord(*ord).unwrap().parent == p {
                        Some(*ord)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();

            let mut moving_x_center = {
                if let Some(parent_ord) = p {
                    if let Some(placed_parent_item) = items.get_by_ord(parent_ord) {
                        // We start half way left from the parents x center
                        placed_parent_item.x_center - placed_parent_item.x_extent_of_children / 2
                    } else {
                        // This really should not happen, because the parent_node_id was
                        // previously retrieved from the tree itself. And the tree is not
                        // touched at all.
                        panic!("Some item expected here!")
                    }
                } else {
                    // `None` means we are in layer 0
                    debug_assert_eq!(layer, 0);
                    // and we should have only one root
                    debug_assert_eq!(node_ids_in_layer.len(), 1);
                    // We start all the way left
                    0
                }
            };
            for ord in nodes_in_layer_per_parent {
                if let Some(placed_item) = items.get_mut_by_ord(ord) {
                    placed_item.x_center = moving_x_center + placed_item.x_extent_children / 2;
                    moving_x_center += placed_item.x_extent_children;
                }
            }
        }
    }

    fn apply_x_center(items: &mut EmbeddingHelperData<W>) {
        let height = items
            .0
            .iter()
            .max_by(|x, y| x.y_order.cmp(&y.y_order))
            .map(|i| i.y_order)
            .unwrap_or_default();
        for l in 0..height + 1 {
            Self::x_center_layer(l, items);
        }
    }

    /// Transforming the internal `EmbeddingHelperMap` to the external representation `Embedding`.
    /// The `items` parameter is hereby consumed.
    fn transfer_result(items: EmbeddingHelperData<W>) -> Embedding {
        let len = items.0.len();
        items
            .0
            .into_iter()
            .fold(Embedding::with_capacity(len), |mut acc, e| {
                acc.push(e.into());
                acc
            })
    }
}
