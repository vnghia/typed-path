use crate::{
    unix::{CURRENT_DIR, PARENT_DIR, SEPARATOR_STR},
    Component,
};

/// Byte slice version of [`std::path::Component`] that represents a Unix-specific component
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnixComponent<'a> {
    RootDir,
    CurDir,
    ParentDir,
    Normal(&'a [u8]),
}

impl Component for UnixComponent<'_> {
    /// Extracts the underlying [`[u8]`] slice
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_path::UnixPath;
    ///
    /// let path = UnixPath::new(b"/tmp/./foo/../bar.txt");
    /// let components: Vec<_> = path.components().map(|comp| comp.as_str()).collect();
    /// assert_eq!(&components, &[b"/", b"tmp", b".", b"foo", b".", b"bar.txt"]);
    /// ```
    fn as_bytes(&self) -> &[u8] {
        match self {
            Self::RootDir => SEPARATOR_STR,
            Self::CurDir => CURRENT_DIR,
            Self::ParentDir => PARENT_DIR,
            Self::Normal(path) => path,
        }
    }

    /// Size of component in bytes
    fn len(&self) -> usize {
        self.as_bytes().len()
    }

    /// Returns true only when the component is a normal path, but the path is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}