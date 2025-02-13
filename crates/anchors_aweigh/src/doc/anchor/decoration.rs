use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Decoration {
    /// Leave the source alone
    #[default]
    None,

    /// Remove repeating leading whitespace from the
    /// start of all source lines,  great for nested
    /// functions you want too bring focus to
    LeftShift,
}
