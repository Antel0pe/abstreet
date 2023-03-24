mod cells;
pub mod colors;

use geom::{ArrowCap, Circle, Distance, PolyLine};
use map_model::{AmenityType, CommonEndpoint, ExtraPOIType, Map, RestrictionType, Road};
use widgetry::mapspace::DrawCustomUnzoomedShapes;
use widgetry::{Color, Drawable, EventCtx, GeomBatch, GfxCtx, Line, RewriteColor, Text};

pub use cells::RenderCells;

pub fn render_poi_icons(ctx: &EventCtx, map: &Map) -> Drawable {
    let mut batch = GeomBatch::new();
    let school = GeomBatch::load_svg(ctx, "system/assets/map/school.svg")
        .scale(0.2)
        .color(RewriteColor::ChangeAll(Color::WHITE));

    for b in map.all_buildings() {
        if b.amenities.iter().any(|a| {
            let at = AmenityType::categorize(&a.amenity_type);
            at == Some(AmenityType::School) || at == Some(AmenityType::University)
        }) {
            batch.append(school.clone().centered_on(b.polygon.polylabel()));
        }
    }

    let tfl =
        GeomBatch::load_svg(ctx, "system/assets/map/tfl_underground.svg").scale_to_fit_width(20.0);
    let national_rail =
        GeomBatch::load_svg(ctx, "system/assets/map/national_rail.svg").scale_to_fit_width(20.0);

    // TODO Toggle3Zoomed could be nicer; these're not terribly visible from afar
    for extra in map.all_extra_pois() {
        let (name, icon) = match extra.kind {
            ExtraPOIType::LondonUndergroundStation(ref name) => (name, &tfl),
            ExtraPOIType::NationalRailStation(ref name) => (name, &national_rail),
        };
        batch.append(icon.clone().centered_on(extra.pt));
        batch.append(
            Text::from(Line(name).fg(Color::WHITE))
                .bg(Color::hex("#0019A8"))
                .render_autocropped(ctx)
                .scale_to_fit_height(10.0)
                .centered_on(extra.pt.offset(0.0, icon.get_bounds().height())),
        );
    }

    ctx.upload(batch)
}

pub fn render_bus_routes(ctx: &EventCtx, map: &Map) -> Drawable {
    let mut batch = GeomBatch::new();
    for r in map.all_roads() {
        if map.get_bus_routes_on_road(r.id).is_empty() {
            continue;
        }
        // Draw dashed outlines surrounding the road
        let width = r.get_width();
        for pl in [
            r.center_pts.shift_left(width * 0.7),
            r.center_pts.shift_right(width * 0.7),
        ]
        .into_iter()
        .flatten()
        {
            batch.extend(
                *colors::BUS_ROUTE,
                pl.exact_dashed_polygons(
                    Distance::meters(2.0),
                    Distance::meters(5.0),
                    Distance::meters(2.0),
                ),
            );
        }
    }
    ctx.upload(batch)
}

pub fn render_turn_restrictions(ctx: &EventCtx, map: &Map) -> Drawable {
    let mut batch = GeomBatch::new();
    for r1 in map.all_roads() {
        // TODO Also interpret lane-level? Maybe just check all the generated turns and see what's
        // allowed / banned in practice?
        for (restriction, r2) in &r1.turn_restrictions {
            // TODO "Invert" OnlyAllowTurns so we can just draw banned things
            if *restriction == RestrictionType::BanTurns {
                batch.append(draw_restriction(map, r1, map.get_r(*r2)));
            }
        }
        for (_, r2) in &r1.complicated_turn_restrictions {
            // TODO Show the 'via'? Or just draw the entire shape?
            batch.append(draw_restriction(map, r1, map.get_r(*r2)));
        }
    }
    ctx.upload(batch)
}

fn draw_restriction(map: &Map, r1: &Road, r2: &Road) -> GeomBatch {
    let mut batch = GeomBatch::new();
    let i = match r1.common_endpoint(r2) {
        CommonEndpoint::One(i) => i,
        // This is probably rare, just pick one side arbitrarily
        CommonEndpoint::Both => r1.src_i,
        CommonEndpoint::None => unreachable!(),
    };
    let (pt1, road_angle) = r1
        .center_pts
        .must_dist_along((if r1.src_i == i { 0.2 } else { 0.8 }) * r1.center_pts.length());
    let pt2 = map.get_i(i).polygon.center();
    let pt3 = r2
        .center_pts
        .must_dist_along((if r2.src_i == i { 0.2 } else { 0.8 }) * r2.center_pts.length())
        .0;
    if let Ok(pl) = PolyLine::new(vec![pt1, pt2, pt3]) {
        let border_thickness = Distance::meters(1.0);

        // TODO The arrow cap is covered up sometimes, and the line has inconsistent thickness.
        // Just use angles
        batch.push(
            Color::BLACK,
            pl.make_arrow(Distance::meters(5.0), ArrowCap::Triangle),
        );
        let radius = r1.get_width() / 2.0;
        // Shrink it to fit inside an icon
        batch = batch
            .autocrop()
            .scale_to_fit_square(0.8 * 2.0 * radius.inner_meters())
            .centered_on(pt1);

        // Circle background
        batch.unshift(Color::WHITE, Circle::new(pt1, radius).to_polygon());
        if let Ok(outline) = Circle::new(pt1, radius).to_outline(border_thickness) {
            batch.push(Color::RED, outline);
        }

        // The Slash of Prohibition
        // TODO Should it be oriented relative to the road or not? If not, just 135 and 315 degrees
        if let Ok(pl) = PolyLine::new(vec![
            pt1.project_away(radius, road_angle.rotate_degs(45.0)),
            pt1.project_away(radius, road_angle.opposite().rotate_degs(45.0)),
        ]) {
            batch.push(Color::RED, pl.make_polygons(border_thickness));
        }
    }
    batch
}

/// Depending on the canvas zoom level, draws one of 2 things.
// TODO Rethink filter styles and do something better than this.
pub struct Toggle3Zoomed {
    draw_zoomed: Drawable,
    unzoomed: DrawCustomUnzoomedShapes,
}

impl Toggle3Zoomed {
    pub fn new(draw_zoomed: Drawable, unzoomed: DrawCustomUnzoomedShapes) -> Self {
        Self {
            draw_zoomed,
            unzoomed,
        }
    }

    pub fn empty(ctx: &EventCtx) -> Self {
        Self::new(Drawable::empty(ctx), DrawCustomUnzoomedShapes::empty())
    }

    pub fn draw(&self, g: &mut GfxCtx) {
        if !self.unzoomed.maybe_draw(g) {
            self.draw_zoomed.draw(g);
        }
    }
}
