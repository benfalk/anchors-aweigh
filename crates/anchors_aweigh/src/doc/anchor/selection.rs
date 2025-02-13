//TODO: this doesn't belong here, should be where ever
//      the file is matched with an anchor.

use super::Strategy;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub path: PathBuf,
    pub strategy: Strategy,
    pub data: String,
}
