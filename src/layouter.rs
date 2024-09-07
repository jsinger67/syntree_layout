//! The module with the **Public API**.

use std::fmt::{self, Debug, Display};
use std::path::Path;

use syntree::{index::Index, pointer::Width, Tree};

use crate::{
    internal::embedder::Embedder, Drawer, Embedding, LayouterError, Result, SvgDrawer, Visualize,
};

///
/// The Layouter type provides a simple builder mechanism with a fluent API.
///
pub struct Layouter<'a, T, I, W, D>
where
    T: Copy,
    I: Index,
    W: Width,
    D: ?Sized + Drawer,
{
    tree: &'a Tree<T, I, W>,
    drawer: &'a D,
    file_name: Option<&'a Path>,
    embedding: Embedding,
}

impl<'a, T, I, W> Layouter<'a, T, I, W, SvgDrawer>
where
    T: Copy,
    I: Index,
    W: Width,
{
    ///
    /// Creates a new Layouter with the required tree.
    ///
    /// ```
    /// use std::fmt;
    /// use syntree_layout::{Layouter, Visualize};
    /// use syntree::{Tree, Builder};
    ///
    /// #[derive(Copy, Clone, Debug)]
    /// struct MyNodeData(i32);
    ///
    /// impl Visualize for MyNodeData {
    ///     fn visualize(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
    ///     fn emphasize(&self) -> bool { false }
    /// }
    ///
    ///
    /// let tree: Tree<MyNodeData, _, _> = Builder::new().build().unwrap();
    /// let layouter = Layouter::new(&tree);
    /// ```
    ///
    pub fn new(tree: &'a Tree<T, I, W>) -> Self {
        static DEFAULT_DRAWER: SvgDrawer = SvgDrawer::new();

        Self {
            tree,
            drawer: &DEFAULT_DRAWER,
            file_name: None,
            embedding: Vec::default(),
        }
    }
}

impl<'a, T, I, W, D> Layouter<'a, T, I, W, D>
where
    T: Copy,
    I: Index,
    W: Width,
    D: ?Sized + Drawer,
{
    ///
    /// Sets the path of the output file on the layouter.
    ///
    /// ```
    /// use std::fmt;
    /// use syntree_layout::{Layouter, Visualize};
    /// use syntree::{Tree, Builder};
    ///
    /// #[derive(Copy, Clone, Debug)]
    /// struct MyNodeData(i32);
    ///
    /// impl Visualize for MyNodeData {
    ///     fn visualize(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
    ///     fn emphasize(&self) -> bool { false }
    /// }
    ///
    ///
    /// let tree: Tree<MyNodeData, _, _> = Builder::new().build().unwrap();
    /// let layouter = Layouter::new(&tree)
    ///     .with_file_path("target/tmp/test.svg");
    /// ```
    ///
    pub fn with_file_path<P>(self, path: &'a P) -> Self
    where
        P: ?Sized + AsRef<Path>,
    {
        Self {
            tree: self.tree,
            file_name: Some(path.as_ref()),
            drawer: self.drawer,
            embedding: self.embedding,
        }
    }

    ///
    /// Sets a different drawer when you don't want to use the default svg-drawer.
    /// If this method is not called the crate's own svg-drawer is used.
    ///
    /// ```
    /// use std::fmt;
    /// use std::path::Path;
    /// use syntree_layout::{Drawer, Layouter, EmbeddedNode, Result, Visualize};
    /// use syntree::{Tree, Builder};
    ///
    /// struct NilDrawer;
    /// impl Drawer for NilDrawer {
    ///     fn draw(&self, _file_name: &Path, _embedding: &[EmbeddedNode]) -> Result<()> {
    ///         Ok(())
    ///     }
    /// }
    /// #[derive(Copy, Clone, Debug)]
    /// struct MyNodeData(i32);
    ///
    /// impl Visualize for MyNodeData {
    ///     fn visualize(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
    ///     fn emphasize(&self) -> bool { false }
    /// }
    ///
    ///
    /// let tree: Tree<MyNodeData, _, _> = Builder::new().build().unwrap();
    /// let drawer = NilDrawer;
    /// let layouter = Layouter::new(&tree)
    ///     .with_drawer(&drawer)
    ///     .with_file_path("target/tmp/test.svg");
    /// ```
    ///
    pub fn with_drawer<U>(self, drawer: &'a U) -> Layouter<T, I, W, U>
    where
        U: Drawer,
    {
        Layouter {
            tree: self.tree,
            file_name: self.file_name,
            drawer,
            embedding: self.embedding,
        }
    }

    ///
    /// When the layouter instance is fully configured this method invokes the necessary embedding
    /// functionality and uses the drawer which writes the result to the output file in its own
    /// output format.
    ///
    /// ```
    /// use std::fmt;
    /// use syntree_layout::{Layouter, Visualize, Result};
    /// use syntree::{Tree, Builder};
    ///
    /// #[derive(Copy, Clone, Debug)]
    /// struct MyNodeData(i32);
    ///
    /// impl Visualize for MyNodeData {
    ///     fn visualize(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
    ///     fn emphasize(&self) -> bool { false }
    /// }
    ///
    /// fn test() -> Result<()> {
    ///     let tree: Tree<MyNodeData, _, _> = Builder::new().build().unwrap();
    ///     Ok(Layouter::new(&tree)
    ///         .with_file_path("target/tmp/test.svg")
    ///         .embed_with_visualize()?
    ///         .write().expect("Failed writing layout"))
    /// }
    ///
    /// test().expect("Embedding should work");
    /// ```
    ///
    pub fn write(&self) -> Result<()> {
        let Some(file_name) = &self.file_name else {
            return Err(LayouterError::from_description(
                "No output file name given - use Layouter::with_file_path.",
            ));
        };

        self.drawer.draw(file_name, &self.embedding)
    }

    /// Provides access to the embedding data for other uses than drawing, e.g. for tests
    pub fn embedding(&self) -> &Embedding {
        &self.embedding
    }
}

impl<'a, T, I, W, D> Layouter<'a, T, I, W, D>
where
    T: Copy + Visualize,
    I: Index,
    W: Width,
    D: ?Sized + Drawer,
{
    ///
    /// This method creates an embedding of the nodes of the given tree in the plane.
    /// The nodes representation is taken form the [Visualize][crate::Visualize] implementation of
    /// type T.
    ///
    /// # Panics
    ///
    /// The method should not panic. If you encounter a panic this should be originated from
    /// bugs in coding. Please report such panics.
    ///
    pub fn embed_with_visualize(self) -> Result<Self> {
        let embedding = Embedder::embed(
            self.tree,
            |value: &T, f| value.visualize(f),
            |value: &T| value.emphasize(),
        )?;
        Ok(Self {
            tree: self.tree,
            file_name: self.file_name,
            drawer: self.drawer,
            embedding,
        })
    }
}

impl<'a, T, I, W, D> Layouter<'a, T, I, W, D>
where
    T: Copy,
    I: Index,
    W: Width,
    D: ?Sized + Drawer,
{
    ///
    /// This method creates an embedding of the nodes of the given tree in the plane.
    /// The nodes representation is done with the help of the given source string.
    ///
    /// # Panics
    ///
    /// The method should not panic. If you encounter a panic this should be originated from
    /// bugs in coding. Please report such panics.
    ///
    pub fn embed_with_source(self, source: &str) -> Result<Self> {
        let embedding = Embedder::embed_with_source(self.tree, source)?;
        Ok(Self {
            tree: self.tree,
            file_name: self.file_name,
            drawer: self.drawer,
            embedding,
        })
    }
}

impl<'a, T, I, W, D> Layouter<'a, T, I, W, D>
where
    T: Copy + Display,
    I: Index,
    W: Width,
    D: ?Sized + Drawer,
{
    ///
    /// This method creates an embedding of the nodes of the given tree in the plane.
    /// The nodes representation is done with the help of the given source string for tokens and the
    /// implementation of the `Display` trait of the node data for inner nodes.
    ///
    /// # Panics
    ///
    /// The method should not panic. If you encounter a panic this should be originated from
    /// bugs in coding. Please report such panics.
    ///
    pub fn embed_with_source_and_display(self, source: &str) -> Result<Self> {
        let embedding = Embedder::embed_with_source_and_display(self.tree, source)?;
        Ok(Self {
            tree: self.tree,
            file_name: self.file_name,
            drawer: self.drawer,
            embedding,
        })
    }
}

impl<'a, T, I, W, D> Layouter<'a, T, I, W, D>
where
    T: Copy + Debug,
    I: Index,
    W: Width,
    D: ?Sized + Drawer,
{
    ///
    /// This method creates an embedding of the nodes of the given tree in the plane.
    /// The nodes representation is taken form the [Debug] implementation of type T.
    ///
    /// # Panics
    ///
    /// The method should not panic. If you encounter a panic this should be originated from
    /// bugs in coding. Please report such panics.
    ///
    pub fn embed_with_debug(self) -> Result<Self> {
        let embedding =
            Embedder::embed(self.tree, |value: &T, f| value.fmt(f), |_value: &T| false)?;
        Ok(Self {
            tree: self.tree,
            file_name: self.file_name,
            drawer: self.drawer,
            embedding,
        })
    }
}

impl<'a, T, I, W, D> Layouter<'a, T, I, W, D>
where
    T: Copy + Display,
    I: Index,
    W: Width,
    D: ?Sized + Drawer,
{
    ///
    /// This method creates an embedding of the nodes of the given tree in the plane.
    /// The nodes representation is taken form the [Display] implementation of type T.
    ///
    /// # Panics
    ///
    /// The method should not panic. If you encounter a panic this should be originated from
    /// bugs in coding. Please report such panics.
    ///
    pub fn embed(self) -> Result<Self> {
        let embedding =
            Embedder::embed(self.tree, |value: &T, f| value.fmt(f), |_value: &T| false)?;
        Ok(Self {
            tree: self.tree,
            file_name: self.file_name,
            drawer: self.drawer,
            embedding,
        })
    }
}

impl<'a, T, I, W, D> Layouter<'a, T, I, W, D>
where
    T: Copy,
    I: Index,
    W: Width,
    D: Drawer,
{
    ///
    /// This method creates an embedding of the nodes of the given tree in the plane.
    /// The nodes representation is taken form the two given functions
    /// [stringify][Layouter::embed_with] and [emphasize][Layouter::embed_with].
    ///
    /// # Panics
    ///
    /// The method should not panic. If you encounter a panic this should be originated from
    /// bugs in coding. Please report such panics.
    ///
    pub fn embed_with(
        &self,
        stringify: impl Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result,
        emphasize: impl Fn(&T) -> bool,
    ) -> Result<Self> {
        let embedding = Embedder::embed(self.tree, &stringify, &emphasize)?;
        Ok(Self {
            tree: self.tree,
            file_name: self.file_name,
            drawer: self.drawer,
            embedding,
        })
    }
}
