use serde::{Deserialize, Serialize};

/// The type of a modal filter. Most of these don't have semantics yet; the variation is just for
/// visual representation
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FilterType {
    NoEntry,
    WalkCycleOnly,
    BusGate,
    SchoolStreet,
}
