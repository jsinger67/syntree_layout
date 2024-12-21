//! The module with the crate's default drawer.

use crate::{Drawer, EmbeddedNode, LayouterError, Result};
use std::path::Path;
use xml_writer::XmlWriter;

use std::fs::File;

const X_MARGIN: f32 = 10.0;
const Y_MARGIN: f32 = 25.0;
const Y_FACTOR: f32 = 3.5;
const FONT_X_SIZE: f32 = 10.0;
const FONT_Y_SIZE: f32 = 10.0;

///
/// The `SvgDrawer` type provides the transformation of the embedding information into the Svg
/// format.
///
#[derive(Debug, Default)]
pub struct SvgDrawer;

impl SvgDrawer {
    /// Method to create a fresh instance of the `SvgDrawer` type.
    pub const fn new() -> Self {
        Self
    }

    fn scale_y(y: usize) -> f32 {
        y as f32 * FONT_Y_SIZE * Y_FACTOR + Y_MARGIN
    }

    fn scale_x(x: usize) -> f32 {
        x as f32 * FONT_X_SIZE + X_MARGIN
    }

    fn measure_string(str: &str) -> f32 {
        str.len() as f32 * FONT_X_SIZE
    }
}

///
/// The concrete implementation of the `Drawer` trait for `SvgDrawer`.
///
impl Drawer for SvgDrawer {
    ///
    /// The concrete implementation of the `Drawer::draw` trait method.
    /// The realization is as it is - with no way to configure for instance the font used.
    /// This decision was mode for the sake of simplicity.
    ///
    /// Anyway it should be easy to provide ones own Drawer implementation that fits the concrete
    /// use case better.
    /// When using the Layouter API you can set the Drawer instance by calling the `with_drawer`
    /// method.
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
    fn draw(&self, file_name: &Path, embedding: &[EmbeddedNode]) -> Result<()> {
        let file = File::create(file_name).map_err(LayouterError::from_io_error)?;
        let xml = XmlWriter::new(file);

        fn build_xml(mut xml: XmlWriter<File>, embedding: &[EmbeddedNode]) -> std::io::Result<()> {
            xml.dtd("UTF-8")?;
            xml.begin_elem("svg")?;
            xml.attr("xmlns", "http://www.w3.org/2000/svg")?;
            xml.attr("version", "1.1")?;
            xml.attr("lang", "en")?;

            const STRING_FONT: &str = "font-family: 'Courier'; font-style: normal";
            const EMPHASIZE_FONT: &str =
                "font-family: 'Courier'; font-weight: bold; font-style: normal";

            let tree_depth =
                embedding
                    .iter()
                    .fold(0, |acc, e| if e.y_order > acc { e.y_order } else { acc });
            let tree_width = embedding.iter().fold(0, |acc, e| {
                if e.x_extent_children > acc {
                    e.x_extent_children
                } else {
                    acc
                }
            });

            let img_width = SvgDrawer::scale_x(tree_width);
            let img_height = SvgDrawer::scale_y(tree_depth + 1);

            xml.attr("width", format!("{}", img_width).as_str())?;
            xml.attr("height", format!("{}", img_height).as_str())?;

            // Draw on a white rectangle to be visible also on black backgrounds.
            xml.begin_elem("rect")?;
            xml.attr("x", "0")?;
            xml.attr("y", "0")?;
            xml.attr("width", format!("{}", img_width).as_str())?;
            xml.attr("height", format!("{}", img_height).as_str())?;
            xml.attr("fill", "white")?;
            xml.end_elem()?;

            for data in embedding {
                let font = if data.is_emphasized {
                    EMPHASIZE_FONT
                } else {
                    STRING_FONT
                };
                let szx = SvgDrawer::measure_string(&data.text);
                let x = SvgDrawer::scale_x(data.x_center) - szx / 2.0;
                let y = SvgDrawer::scale_y(data.y_order);
                xml.begin_elem("text")?;
                xml.attr("x", format!("{}", x).as_str())?;
                xml.attr("y", format!("{}", y).as_str())?;
                xml.attr("style", font)?;
                xml.text(data.text.as_str())?;
                xml.end_elem()?;

                if let Some(parent_index) = data.parent {
                    let parent_data = embedding.iter().find(|e| e.ord == parent_index).unwrap();

                    // Draw a line from the nodes parent down to this node
                    xml.begin_elem("line")?;
                    xml.attr(
                        "x1",
                        format!("{}", (SvgDrawer::scale_x(parent_data.x_center))).as_str(),
                    )?;
                    xml.attr(
                        "y1",
                        format!(
                            "{}",
                            (SvgDrawer::scale_y(parent_data.y_order) + FONT_Y_SIZE)
                        )
                        .as_str(),
                    )?;
                    xml.attr(
                        "x2",
                        format!("{}", (SvgDrawer::scale_x(data.x_center))).as_str(),
                    )?;
                    xml.attr("y2", format!("{}", (y - FONT_Y_SIZE - 3.0)).as_str())?;
                    xml.attr("stroke", "black")?;
                    xml.end_elem()?;
                }
            }

            xml.end_elem()?;
            xml.close()?;
            xml.flush()?;

            Ok(())
        }

        build_xml(xml, embedding).map_err(LayouterError::from_io_error)
    }
}
