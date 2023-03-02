//! The module with the `Drawer` trait.
use crate::{EmbeddedNode, Result};

///
/// By implementing this trait anyone can provide his own drawer, for instance one that draws onto
/// a bitmap, if he don't want to use the `SvgDrawer` used by the crate by default.
///
pub trait Drawer {
    fn draw(&self, file_name: &std::path::Path, embedding: &[EmbeddedNode]) -> Result<()>;
}
