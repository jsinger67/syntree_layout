//! The module with the `Drawer` trait.
use crate::{EmbeddedNode, Result};

///
/// By implementing this trait anyone can provide his own drawer, for instance one that draws onto
/// a bitmap, if he don't want to use the `SvgDrawer` used by the crate by default.
///
pub trait Drawer {
    fn draw(&self, file_name: &std::path::Path, embedding: &[EmbeddedNode]) -> Result<()>;
}

struct DummyDrawer;

impl Drawer for DummyDrawer {
    #[inline]
    fn draw(&self, _: &std::path::Path, _: &[EmbeddedNode]) -> Result<()> {
        Ok(())
    }
}

// Test to assert that drawer is object safe, even though this is *for now*
// ensured in the bound used in `Layouter::new`.
const _: &dyn Drawer = &DummyDrawer;
