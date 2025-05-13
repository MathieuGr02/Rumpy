use std::path::Path;

pub(crate) trait FromFile {
    fn from_file<P: AsRef<Path>>(path: P) -> Self;
}

pub(crate) trait ToFile {
    fn to_file<P: AsRef<Path>>(self, path: P);
}