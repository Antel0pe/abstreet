var sourcesIndex = JSON.parse('{\
"abstio":["",[],["abst_data.rs","abst_paths.rs","download.rs","http.rs","io.rs","io_native.rs","lib.rs"]],\
"abstutil":["",[],["cli.rs","clone.rs","collections.rs","lib.rs","logger.rs","priority_queue.rs","process.rs","serde.rs","time.rs","utils.rs"]],\
"cli":["",[],["augment_scenario.rs","clip_osm.rs","generate_houses.rs","geojson_to_osmosis.rs","import_grid2demand.rs","import_scenario.rs","main.rs","one_step_import.rs"]],\
"collisions":["",[],["lib.rs"]],\
"convert_osm":["",[],["elevation.rs","extract.rs","gtfs.rs","lib.rs","parking.rs"]],\
"fifteen_min":["",[],["bus.rs","find_amenities.rs","find_home.rs","isochrone.rs","lib.rs","viewer.rs"]],\
"game":["",[["challenges",[],["cutscene.rs","mod.rs","prebake.rs"]],["common",[],["mod.rs","route_sketcher.rs","select.rs","share.rs","warp.rs"]],["debug",[],["blocked_by.rs","blockfinder.rs","floodfill.rs","mod.rs","objects.rs","path_counter.rs","polygons.rs","routes.rs","select_roads.rs","uber_turns.rs"]],["devtools",[],["collisions.rs","compare_counts.rs","destinations.rs","kml.rs","mod.rs","polygon.rs","scenario.rs","story.rs"]],["edit",[["traffic_signals",[],["edits.rs","gmns.rs","mod.rs","offsets.rs","picker.rs","preview.rs"]]],["crosswalks.rs","mod.rs","multiple_roads.rs","roads.rs","routes.rs","stop_signs.rs","validate.rs","zones.rs"]],["info",[],["building.rs","debug.rs","intersection.rs","lane.rs","mod.rs","parking_lot.rs","person.rs","transit.rs","trip.rs"]],["layer",[],["elevation.rs","favorites.rs","map.rs","mod.rs","pandemic.rs","parking.rs","population.rs","problems.rs","problems_diff.rs","traffic.rs","transit.rs"]],["pregame",[],["mod.rs","proposals.rs"]],["sandbox",[["dashboards",[],["commuter.rs","generic_trip_table.rs","misc.rs","mod.rs","mode_shift.rs","parking_overhead.rs","risks.rs","selector.rs","traffic_signals.rs","travel_times.rs","trip_problems.rs","trip_table.rs"]],["gameplay",[["freeform",[],["area_spawner.rs","importers.rs","mod.rs","spawner.rs"]]],["actdev.rs","commute.rs","fix_traffic_signals.rs","mod.rs","play_scenario.rs","tutorial.rs"]]],["minimap.rs","misc_tools.rs","mod.rs","speed.rs","time_warp.rs"]],["ungap",[["trip",[],["mod.rs","results.rs"]]],["bike_network.rs","explore.rs","layers.rs","mod.rs","predict.rs","quick_sketch.rs"]]],["app.rs","lib.rs"]],\
"geom":["",[],["angle.rs","bounds.rs","circle.rs","conversions.rs","distance.rs","duration.rs","find_closest.rs","gps.rs","lib.rs","line.rs","percent.rs","polygon.rs","polyline.rs","pt.rs","ring.rs","speed.rs","stats.rs","tessellation.rs","time.rs"]],\
"headless":["",[],["main.rs"]],\
"importer":["",[["soundcast",[],["mod.rs","popdat.rs","trips.rs"]]],["basemap.rs","berlin.rs","configuration.rs","lib.rs","map_config.rs","pick_geofabrik.rs","seattle.rs","uk.rs","utils.rs"]],\
"kml":["",[],["lib.rs"]],\
"ltn":["",[["components",[],["about.rs","appwide_panel.rs","layers.rs","left_panel.rs","mod.rs"]],["edit",[],["filters.rs","freehand_filters.rs","mod.rs","one_ways.rs","shortcuts.rs"]],["filters",[],["auto.rs","existing.rs","mod.rs"]],["impact",[],["mod.rs","ui.rs"]],["save",[],["mod.rs","perma.rs","share.rs"]]],["app.rs","colors.rs","crossings.rs","customize_boundary.rs","design_ltn.rs","draw_cells.rs","export.rs","lib.rs","neighbourhood.rs","partition.rs","pick_area.rs","route_planner.rs","select_boundary.rs","shortcuts.rs"]],\
"map_editor":["",[],["app.rs","camera.rs","edit.rs","lib.rs","load.rs","model.rs"]],\
"map_gui":["",[["render",[],["agents.rs","area.rs","bike.rs","building.rs","car.rs","intersection.rs","lane.rs","map.rs","mod.rs","parking_lot.rs","pedestrian.rs","road.rs","traffic_signal.rs","transit_stop.rs","turn.rs"]],["tools",[],["camera.rs","city_picker.rs","colors.rs","command.rs","compare_counts.rs","draw_overlapping_paths.rs","heatmap.rs","icons.rs","importer.rs","labels.rs","minimap.rs","mod.rs","navigate.rs","polygon.rs","title_screen.rs","trip_files.rs","turn_explorer.rs","ui.rs","updater.rs","waypoints.rs"]]],["colors.rs","lib.rs","load.rs","options.rs","simple_app.rs"]],\
"map_model":["",[["connectivity",[],["mod.rs","walking.rs"]],["edits",[],["compat.rs","mod.rs","perma.rs"]],["make",[["traffic_signals",[],["lagging_green.rs","mod.rs"]]],["bridges.rs","buildings.rs","mod.rs","parking_lots.rs","transit.rs","turns.rs","walking_turns.rs"]],["objects",[],["area.rs","block.rs","building.rs","intersection.rs","lane.rs","mod.rs","movement.rs","parking_lot.rs","road.rs","stop_signs.rs","traffic_signals.rs","transit.rs","turn.rs","zone.rs"]],["pathfind",[],["engine.rs","mod.rs","node_map.rs","pathfinder.rs","uber_turns.rs","v1.rs","v2.rs","vehicles.rs","walking.rs"]]],["city.rs","lib.rs","map.rs","traversable.rs"]],\
"osm_viewer":["",[],["lib.rs","viewer.rs"]],\
"parking_mapper":["",[],["main.rs","mapper.rs"]],\
"piggyback":["",[],["lib.rs"]],\
"popdat":["",[],["activities.rs","distribute_people.rs","import_census.rs","lib.rs","make_person.rs","od.rs"]],\
"raw_map":["",[],["lib.rs","types.rs"]],\
"santa":["",[],["after_level.rs","animation.rs","before_level.rs","buildings.rs","controls.rs","game.rs","levels.rs","lib.rs","meters.rs","music.rs","player.rs","session.rs","title.rs","vehicles.rs"]],\
"sim":["",[["make",[],["load.rs","mod.rs","spawner.rs"]],["mechanics",[],["car.rs","driving.rs","intersection.rs","mod.rs","parking.rs","queue.rs","walking.rs"]],["pandemic",[],["mod.rs","model.rs"]],["sim",[],["mod.rs","queries.rs","scenario.rs"]]],["analytics.rs","events.rs","lib.rs","prebake.rs","recorder.rs","render.rs","router.rs","scheduler.rs","transit.rs","trips.rs"]],\
"synthpop":["",[["make",[],["activity_model.rs","generator.rs","mod.rs"]]],["borders.rs","counts.rs","endpoint.rs","external.rs","lib.rs","modifier.rs","scenario.rs"]],\
"tests":["",[],["main.rs"]],\
"traffic_seitan":["",[],["main.rs"]],\
"traffic_signal_data":["",[],["lib.rs"]],\
"updater":["",[],["main.rs"]],\
"widgetry":["",[["geom",[],["geom_batch_stack.rs","mod.rs"]],["mapspace",[],["mod.rs","unzoomed.rs","world.rs"]],["style",[],["button_style.rs","mod.rs"]],["tools",[],["choose_something.rs","colors.rs","lasso.rs","load.rs","mod.rs","popup.rs","prompt_input.rs","screenshot.rs","url.rs","warper.rs"]],["widgets",[],["autocomplete.rs","button.rs","compare_times.rs","containers.rs","drag_drop.rs","dropdown.rs","fan_chart.rs","filler.rs","image.rs","just_draw.rs","line_plot.rs","menu.rs","mod.rs","panel.rs","persistent_split.rs","plots.rs","scatter_plot.rs","slider.rs","spinner.rs","stash.rs","table.rs","tabs.rs","text_box.rs","toggle.rs"]]],["app_state.rs","assets.rs","backend_glow.rs","backend_glow_native.rs","canvas.rs","color.rs","drawing.rs","event.rs","event_ctx.rs","input.rs","lib.rs","runner.rs","screen_geom.rs","svg.rs","text.rs"]],\
"widgetry_demo":["",[],["lib.rs"]]\
}');
createSourceSidebar();
