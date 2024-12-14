//! The module that holds types to embed nodes of a tree into the plane.

use std::fmt::{self};

use syntree::{node::Event, Flavor, Node, Tree};

use crate::{Embedding, LayouterError, Result};

use super::node::{EmbeddingHelperData, InternalNode};

///
/// The Embedder type provides a single (accessible) method `embed` to arrange nodes of a tree into
/// the plane.
/// It is an internal type used by the public API [crate::Layouter].
///
pub(crate) struct Embedder<T, F>
where
    T: Copy,
    F: Flavor,
{
    _1: std::marker::PhantomData<T>,
    _2: std::marker::PhantomData<F>,
}

impl<T, F> Embedder<T, F>
where
    T: Copy,
    F: Flavor,
{
    ///
    /// This method creates an embedding of the nodes of the given tree in the plane.
    ///
    /// # Panics
    ///
    /// The method should not panic. If you encounter a panic this should be originated from
    /// bugs in coding. Please report such panics.
    ///
    pub(crate) fn embed(
        tree: &Tree<T, F>,
        stringify: impl Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result,
        emphasize: impl Fn(&T) -> bool,
    ) -> Result<Embedding> {
        // Insert all tree items with their indices
        // After this step each item has following properties set:
        // 'y_order', 'x_extent', 'text', 'is_emphasized', 'ord'
        let mut items = Self::create_initial_embedding_data(tree, &stringify, &emphasize)?;
        debug_assert_eq!(items.0.len(), items.1.len());

        // Set widths (x_extent_children, x_extent_of_children) on each InternalNode structure
        // After this step each item has following properties set:
        // 'y_order', 'x_extent', 'text', 'is_emphasized', 'ord', 'x_extent_children',
        // 'x_extent_of_children', 'parent'
        Self::apply_children_x_extents(tree, &mut items);

        // Finally set the property 'x_center' from leafs to root
        // After this step each item has all necessary properties set
        Self::apply_x_center(&mut items)?;

        // Transfer result
        Ok(Self::transfer_result(items))
    }

    /// Embeds the nodes of the given tree into the plane. The source code is used to display the
    /// text of the nodes, if they are tokens.
    pub(crate) fn embed_with_source(tree: &Tree<T, F>, source: &str) -> Result<Embedding>
    where
        T: Copy,
        F: Flavor,
    {
        // Insert all tree items with their indices
        // After this step each item has following properties set:
        // 'y_order', 'x_extent', 'text', 'is_emphasized', 'ord'
        let mut items = Self::create_initial_embedding_data_with_source(tree, source)?;
        debug_assert_eq!(items.0.len(), items.1.len());

        // Set widths (x_extent_children, x_extent_of_children) on each InternalNode structure
        // After this step each item has following properties set:
        // 'y_order', 'x_extent', 'text', 'is_emphasized', 'ord', 'x_extent_children',
        // 'x_extent_of_children', 'parent'
        Self::apply_children_x_extents(tree, &mut items);

        // Finally set the property 'x_center' from leafs to root
        // After this step each item has all necessary properties set
        Self::apply_x_center(&mut items)?;

        // Transfer result
        Ok(Self::transfer_result(items))
    }

    pub(crate) fn embed_with_source_and_display(
        tree: &Tree<T, F>,
        source: &str,
    ) -> Result<Embedding>
    where
        T: Copy + fmt::Display,
        F: Flavor,
    {
        // Insert all tree items with their indices
        // After this step each item has following properties set:
        // 'y_order', 'x_extent', 'text', 'is_emphasized', 'ord'
        let mut items = Self::create_initial_embedding_data_with_source_and_display(tree, source)?;
        debug_assert_eq!(items.0.len(), items.1.len());

        // Set widths (x_extent_children, x_extent_of_children) on each InternalNode structure
        // After this step each item has following properties set:
        // 'y_order', 'x_extent', 'text', 'is_emphasized', 'ord', 'x_extent_children',
        // 'x_extent_of_children', 'parent'
        Self::apply_children_x_extents(tree, &mut items);

        // Finally set the property 'x_center' from leafs to root
        // After this step each item has all necessary properties set
        Self::apply_x_center(&mut items)?;

        // Transfer result
        Ok(Self::transfer_result(items))
    }

    fn create_from_node(
        ord: usize,
        depth: usize,
        node: Node<T, F>,
        items: &EmbeddingHelperData<F>,
        stringify: &impl Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result,
        emphasize: &impl Fn(&T) -> bool,
    ) -> InternalNode<F> {
        // Wrapper to help evaluate forwarded Display implementation.
        struct Wrapper<'a, F, T>(&'a F, &'a T);

        impl<F, T> fmt::Display for Wrapper<'_, F, T>
        where
            F: Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result,
        {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                (self.0)(self.1, f)
            }
        }

        let text = Wrapper(stringify, &node.value()).to_string();

        let y_order = depth;
        let x_center = 0;
        let x_extent = text.len() + 1;
        let x_extent_of_children = x_extent;
        let x_extent_children = x_extent;
        let is_emphasized = emphasize(&node.value());
        let parent = node
            .parent()
            .and_then(|p| items.get_by_node_id(&p.id()).map(|n| n.ord));
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

    fn create_from_node_with_source(
        ord: usize,
        depth: usize,
        node: Node<T, F>,
        items: &EmbeddingHelperData<F>,
        source: &str,
    ) -> InternalNode<F> {
        let text = source[node.range()].to_string();

        let y_order = depth;
        let x_center = 0;
        let x_extent = text.len() + 1;
        let x_extent_of_children = x_extent;
        let x_extent_children = x_extent;
        let parent = node
            .parent()
            .and_then(|p| items.get_by_node_id(&p.id()).map(|n| n.ord));
        let node_id = node.id();

        InternalNode {
            y_order,
            x_center,
            x_extent,
            x_extent_of_children,
            x_extent_children,
            text,
            is_emphasized: false,
            parent,
            ord,
            node_id,
        }
    }

    fn create_from_node_with_source_and_diplay(
        ord: usize,
        depth: usize,
        node: Node<T, F>,
        items: &EmbeddingHelperData<F>,
        source: &str,
    ) -> InternalNode<F>
    where
        T: fmt::Display,
    {
        let text = if node.has_children() {
            node.value().to_string()
        } else {
            source[node.range()].to_string()
        };

        let y_order = depth;
        let x_center = 0;
        let x_extent = text.len() + 1;
        let x_extent_of_children = x_extent;
        let x_extent_children = x_extent;
        let parent = node
            .parent()
            .and_then(|p| items.get_by_node_id(&p.id()).map(|n| n.ord));
        let node_id = node.id();

        InternalNode {
            y_order,
            x_center,
            x_extent,
            x_extent_of_children,
            x_extent_children,
            text,
            is_emphasized: false,
            parent,
            ord,
            node_id,
        }
    }

    fn create_initial_embedding_data(
        tree: &Tree<T, F>,
        stringify: &impl Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result,
        emphasize: &impl Fn(&T) -> bool,
    ) -> Result<EmbeddingHelperData<F>> {
        let mut items = EmbeddingHelperData::with_capacity(tree.len());
        if tree.children().count() > 1 {
            return Err(LayouterError::from_description(
                "Currently we support only one root",
            ));
        }

        tree.walk()
            .with_depths()
            .enumerate()
            .for_each(|(ord, (depth, node))| {
                let new_item =
                    Self::create_from_node(ord, depth as usize, node, &items, stringify, emphasize);
                items.insert(ord, new_item);
            });

        Ok(items)
    }

    fn create_initial_embedding_data_with_source(
        tree: &Tree<T, F>,
        source: &str,
    ) -> Result<EmbeddingHelperData<F>> {
        let mut items = EmbeddingHelperData::with_capacity(tree.len());
        if tree.children().count() > 1 {
            return Err(LayouterError::from_description(
                "Currently we support only one root",
            ));
        }

        tree.walk()
            .with_depths()
            .enumerate()
            .for_each(|(ord, (depth, node))| {
                let new_item =
                    Self::create_from_node_with_source(ord, depth as usize, node, &items, source);
                items.insert(ord, new_item);
            });

        Ok(items)
    }

    fn create_initial_embedding_data_with_source_and_display(
        tree: &Tree<T, F>,
        source: &str,
    ) -> Result<EmbeddingHelperData<F>>
    where
        T: fmt::Display,
    {
        let mut items = EmbeddingHelperData::with_capacity(tree.len());
        if tree.children().count() > 1 {
            return Err(LayouterError::from_description(
                "Currently we support only one root",
            ));
        }

        tree.walk()
            .with_depths()
            .enumerate()
            .for_each(|(ord, (depth, node))| {
                let new_item = Self::create_from_node_with_source_and_diplay(
                    ord,
                    depth as usize,
                    node,
                    &items,
                    source,
                );
                items.insert(ord, new_item);
            });

        Ok(items)
    }

    fn apply_children_x_extents(tree: &Tree<T, F>, items: &mut EmbeddingHelperData<F>) {
        tree.walk_events().for_each(|(event, node)| {
            if let Event::Up = event {
                let x_extent_of_children = node.children().fold(0, |acc, child| {
                    if let Some(internal_child) = items.get_by_node_id(&child.id()) {
                        acc + internal_child.x_extent_children
                    } else {
                        acc
                    }
                });
                if let Some(internal_node) = items.get_mut_by_node_id(&node.id()) {
                    internal_node.x_extent_of_children = x_extent_of_children;
                    internal_node.x_extent_children =
                        std::cmp::max(internal_node.x_extent, x_extent_of_children);
                }
            }
        });
    }

    fn x_center_layer(layer: usize, items: &mut EmbeddingHelperData<F>) -> Result<()> {
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
            .map(|ord| {
                Ok(items
                    .get_by_ord(*ord)
                    .ok_or(LayouterError::from_description("Expecting existing node"))?
                    .parent)
            })
            .collect::<Result<Vec<Option<usize>>>>()?;

        for p in parents_in_layer {
            let nodes_in_layer_per_parent = node_ids_in_layer
                .iter()
                .filter_map(|ord| {
                    if let Some(node) = items.get_by_ord(*ord) {
                        if node.parent == p {
                            Some(*ord)
                        } else {
                            None
                        }
                    } else {
                        debug_assert!(false, "Expecting existing node");
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
                        // This really should not happen
                        return Err(LayouterError::from_description("Some item expected here!"));
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

        Ok(())
    }

    fn apply_x_center(items: &mut EmbeddingHelperData<F>) -> Result<()> {
        let height = items
            .0
            .iter()
            .max_by(|x, y| x.y_order.cmp(&y.y_order))
            .map(|i| i.y_order)
            .unwrap_or_default();
        for l in 0..height + 1 {
            Self::x_center_layer(l, items)?;
        }
        Ok(())
    }

    /// Transforming the internal `EmbeddingHelperMap` to the external representation `Embedding`.
    /// The `items` parameter is hereby consumed.
    fn transfer_result(items: EmbeddingHelperData<F>) -> Embedding {
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
