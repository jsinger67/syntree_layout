//! The visualize module provides the `Visualize` trait.

use std::fmt;

/// The `Visualize` trait abstracts the visual presentation of the node's data.
/// It can be implemented by the Tree<T, ...>'s node type T when custom visualization is desired.
/// Only mandatory to implement is the `visualize` method.
pub trait Visualize {
    /// Writes the string representation of the nodes data.
    fn visualize(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    /// Writes the string representation of the nodes data with the help of the given source.
    /// Is default implemented by calling `visualize`.
    fn visualize_with_source(&self, f: &mut fmt::Formatter<'_>, _source: &str) -> fmt::Result {
        self.visualize(f)
    }

    /// When this method returns true the drawer can emphasize the node's string representation
    /// in an implementation dependent way, i.e. it can print it bold.
    fn emphasize(&self) -> bool {
        false
    }
}
