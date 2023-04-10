use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use abstutil::{deserialize_btreemap, serialize_btreemap};
use geom::{Angle, Distance, Speed};
use map_model::{
    CrossingType, DiagonalFilter, EditRoad, FilterType, IntersectionID, Map, RoadFilter, RoadID,
    RoutingParams, TurnID,
};
use widgetry::mapspace::{DrawCustomUnzoomedShapes, PerZoom};
use widgetry::{EventCtx, GeomBatch, RewriteColor};

use crate::render;

/// Stored in App per-map state. Before making any changes, call `before_edit`.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Edits {
    // We use serialize_btreemap so that save::perma can detect and transform IDs
    #[serde(
        serialize_with = "serialize_btreemap",
        deserialize_with = "deserialize_btreemap"
    )]
    pub roads: BTreeMap<RoadID, RoadFilter>,
    #[serde(
        serialize_with = "serialize_btreemap",
        deserialize_with = "deserialize_btreemap"
    )]
    pub intersections: BTreeMap<IntersectionID, DiagonalFilter>,
    /// For roads with modified directions or speed limits, what's their current state?
    // TODO Misnomer; this includes speed limit changes now too. Not worth a backwards incompatible
    // change right now.
    #[serde(
        serialize_with = "serialize_btreemap",
        deserialize_with = "deserialize_btreemap"
    )]
    pub one_ways: BTreeMap<RoadID, EditRoad>,
    /// For roads with modified speeds, what's their current state?
    #[serde(
        serialize_with = "serialize_btreemap",
        deserialize_with = "deserialize_btreemap"
    )]
    pub speed_limits: BTreeMap<RoadID, Speed>,
    /// One road may have multiple crossings. They're sorted by increasing distance.
    #[serde(
        serialize_with = "serialize_btreemap",
        deserialize_with = "deserialize_btreemap"
    )]
    pub crossings: BTreeMap<RoadID, Vec<Crossing>>,

    /// Edit history is preserved recursively
    #[serde(skip_serializing, skip_deserializing)]
    pub previous_version: Box<Option<Edits>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Crossing {
    pub kind: CrossingType,
    pub dist: Distance,
    pub user_modified: bool,
}

/// This logically changes every time an edit occurs. MapName isn't captured here.
#[derive(Default, PartialEq)]
pub struct ChangeKey {
    roads: BTreeMap<RoadID, RoadFilter>,
    intersections: BTreeMap<IntersectionID, DiagonalFilter>,
    one_ways: BTreeMap<RoadID, EditRoad>,
    crossings: BTreeMap<RoadID, Vec<Crossing>>,
}

impl Edits {
    /// Modify RoutingParams to respect these modal filters
    pub fn update_routing_params(&self, params: &mut RoutingParams) {
        params.avoid_roads.extend(self.roads.keys().cloned());
        for filter in self.intersections.values() {
            params
                .avoid_movements_between
                .extend(filter.avoid_movements_between_roads());
        }
    }

    pub fn allows_turn(&self, t: TurnID) -> bool {
        if let Some(filter) = self.intersections.get(&t.parent) {
            return filter.allows_turn(t.src.road, t.dst.road);
        }
        true
    }

    /// Draw all modal filters
    pub fn draw(&self, ctx: &EventCtx, map: &Map) -> render::Toggle3Zoomed {
        let mut batch = GeomBatch::new();
        let mut low_zoom = DrawCustomUnzoomedShapes::builder();

        let mut icons = BTreeMap::new();
        for ft in [
            FilterType::NoEntry,
            FilterType::WalkCycleOnly,
            FilterType::BusGate,
            FilterType::SchoolStreet,
        ] {
            icons.insert(ft, GeomBatch::load_svg(ctx, render::filter_svg_path(ft)));
        }

        for (r, filter) in &self.roads {
            let icon = &icons[&filter.filter_type];
            let rewrite_color = if filter.user_modified {
                RewriteColor::NoOp
            } else {
                RewriteColor::ChangeAlpha(0.7)
            };

            let road = map.get_r(*r);
            if let Ok((pt, road_angle)) = road.center_pts.dist_along(filter.dist) {
                let angle = if filter.filter_type == FilterType::NoEntry {
                    road_angle.rotate_degs(90.0)
                } else {
                    Angle::ZERO
                };

                batch.append(
                    icon.clone()
                        .scale_to_fit_width(road.get_width().inner_meters())
                        .centered_on(pt)
                        .rotate(angle)
                        .color(rewrite_color),
                );

                // TODO Memory intensive
                let icon = icon.clone();
                // TODO They can shrink a bit past their map size
                low_zoom.add_custom(Box::new(move |batch, thickness| {
                    batch.append(
                        icon.clone()
                            .scale_to_fit_width(30.0 * thickness)
                            .centered_on(pt)
                            .rotate(angle)
                            .color(rewrite_color),
                    );
                }));
            }
        }

        for (_, filter) in &self.intersections {
            let icon = &icons[&filter.filter_type];
            let rewrite_color = if filter.user_modified {
                RewriteColor::NoOp
            } else {
                RewriteColor::ChangeAlpha(0.7)
            };

            let line = filter.geometry(map);
            let angle = if filter.filter_type == FilterType::NoEntry {
                line.angle()
            } else {
                Angle::ZERO
            };
            let pt = line.middle().unwrap();

            batch.append(
                icon.clone()
                    .scale_to_fit_width(line.length().inner_meters())
                    .centered_on(pt)
                    .rotate(angle)
                    .color(rewrite_color),
            );

            let icon = icon.clone();
            low_zoom.add_custom(Box::new(move |batch, thickness| {
                // TODO Why is this magic value different than the one above?
                batch.append(
                    icon.clone()
                        .scale(0.4 * thickness)
                        .centered_on(pt)
                        .rotate(angle)
                        .color(rewrite_color),
                );
            }));
        }

        let min_zoom_for_detail = 5.0;
        let step_size = 0.1;
        // TODO Ideally we get rid of Toggle3Zoomed and make DrawCustomUnzoomedShapes handle this
        // medium-zoom case.
        render::Toggle3Zoomed::new(
            batch.build(ctx),
            low_zoom.build(PerZoom::new(min_zoom_for_detail, step_size)),
        )
    }

    pub fn get_change_key(&self) -> ChangeKey {
        ChangeKey {
            roads: self.roads.clone(),
            intersections: self.intersections.clone(),
            one_ways: self.one_ways.clone(),
            crossings: self.crossings.clone(),
        }
    }
}
