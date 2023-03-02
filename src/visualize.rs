//! The visualize module provides the `Visualize` trait.

/// The `Visualize` trait abstracts the visual presentation of the node's data.
/// It can be implemented by the Tree<T, ...>'s node type T when custom visualization is desired.
/// Only mandatory to implement is the `visualize` method.
pub trait Visualize {
    /// Returns the string representation of the nodes data.
    fn visualize(&self) -> String;

    /// When this method returns true the drawer can emphasize the node's string representation
    /// in an implementation dependent way, i.e. it can print it bold.
    fn emphasize(&self) -> bool {
        false
    }
}
