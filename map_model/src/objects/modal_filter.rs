use serde::{Deserialize, Serialize};

use geom::Distance;

/// The type of a modal filter. Most of these don't have semantics yet; the variation is just for
/// visual representation
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FilterType {
    NoEntry,
    WalkCycleOnly,
    BusGate,
    SchoolStreet,
}

/// A filter placed somewhere along a road
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct RoadFilter {
    pub dist: Distance,
    pub filter_type: FilterType,
    // TODO Need to remember original filter per road and compare
    pub user_modified: bool,
}

impl RoadFilter {
    pub fn new_by_user(dist: Distance, filter_type: FilterType) -> Self {
        Self {
            dist,
            filter_type,
            user_modified: true,
        }
    }
}
