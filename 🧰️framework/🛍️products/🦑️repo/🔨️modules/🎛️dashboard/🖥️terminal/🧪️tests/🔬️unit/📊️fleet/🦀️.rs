/// 📊️ The Fleet window: render-to-text snapshots of the overview and a drill-down from the fixture rings of the synthetic fleet (a live, a stale and a missing origin), the keys, the requests it sends the daemon and the symbols that carry every state without colour.
mod fleet_window {
    use super::*;
    use dashboard_fleet::{Freshness, Range, Request, Snapshot, StoreState};
    use dashboard_terminal::testing::RowAction;

    fn golden(name: &str) -> Snapshot { serde_json::from_str(match name {
        "overview-5m.json" => semio_framework_test_fixtures::fixture_text!("../../../../../📊️fleet/🧪️tests/🧫️fixtures/🏅️golden/overview-5m.json"),
        "overview-1h.json" => semio_framework_test_fixtures::fixture_text!("../../../../../📊️fleet/🧪️tests/🧫️fixtures/🏅️golden/overview-1h.json"),
        "overview-90d.json" => semio_framework_test_fixtures::fixture_text!("../../../../../📊️fleet/🧪️tests/🧫️fixtures/🏅️golden/overview-90d.json"),
        _ => semio_framework_test_fixtures::fixture_text!("../../../../../📊️fleet/🧪️tests/🧫️fixtures/🏅️golden/detail-g1-m01-1h.json"),
    }).expect("golden snapshot") }

    fn texts_directory() -> std::path::PathBuf { std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../🧪️tests/🧫️fixtures/📊️fleet") }

    fn check_text(name: &str, actual: &str) {
        let path = texts_directory().join(name);
        if std::env::var("FLEET_TEXT_WRITE").as_deref() == Ok("1") { std::fs::write(&path, actual).expect("write snapshot text"); }
        let expected = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("{name} missing; rerun with FLEET_TEXT_WRITE=1"));
        assert_eq!(actual, expected, "{name} drifted");
    }

    fn open(language: &str) -> (Dashboard, Tui, usize) {
        let (mut dashboard, mut tui) = dashboard_with(Preferences { language: Some(language.into()), ..Default::default() });
        dashboard.perform(&mut tui, Action::ShowFleet);
        dashboard.frame(&mut tui);
        let index = dashboard.windows.iter().position(|window| matches!(window.body, Body::Fleet(_))).expect("a Fleet window");
        (dashboard, tui, index)
    }

    fn feed(dashboard: &mut Dashboard, tui: &mut Tui, snapshot: Snapshot) {
        dashboard.apply(tui, Message::Control(ServerMsg::Fleet { snapshot: Box::new(snapshot) }));
        dashboard.frame(tui);
    }

    fn rows(dashboard: &Dashboard, index: usize) -> Vec<String> { dashboard.windows[index].pane.rows.iter().map(|(text, _)| text.trim_end().to_string()).collect() }

    fn watches(dashboard: &Dashboard) -> Vec<Request> { dashboard.link.outbox.iter().filter_map(|message| if let ClientMsg::FleetWatch { request } = message { Some(request.clone()) } else { None }).collect() }

    fn plain(ansi: &str) -> String {
        let mut text = String::new();
        let mut chars = ansi.chars().peekable();
        while let Some(character) = chars.next() {
            if character == '\u{1b}' { if chars.peek() == Some(&'[') { chars.next(); for next in chars.by_ref() { if next.is_ascii_alphabetic() { break; } } } continue; }
            text.push(character);
        }
        text
    }

    #[test]
    fn the_overview_snapshot_text_is_pinned_in_english_and_german() {
        for (language, name) in [("en", "overview-en.txt"), ("de", "overview-de.txt")] {
            let (mut dashboard, mut tui, index) = open(language);
            feed(&mut dashboard, &mut tui, golden("overview-5m.json"));
            check_text(name, &(rows(&dashboard, index).join("\n") + "\n"));
        }
    }

    #[test]
    fn a_drill_down_shows_one_chart_per_metric_of_every_group_with_a_tier_ruler() {
        let (mut dashboard, mut tui, index) = open("en");
        feed(&mut dashboard, &mut tui, golden("overview-5m.json"));
        press(&mut dashboard, &mut tui, &[key('2')]);
        let overview = golden("overview-1h.json");
        feed(&mut dashboard, &mut tui, overview);
        dashboard.windows[index].pane.selected = dashboard.windows[index].pane.rows.iter().position(|(_, action)| *action == RowAction::FleetOrigin("g1-m01".into())).expect("the g1-m01 row");
        press(&mut dashboard, &mut tui, &[named(Key::Enter)]);
        let Body::Fleet(view) = &dashboard.windows[index].body else { panic!("fleet body") };
        assert_eq!(view.drill.as_deref(), Some("g1-m01"));
        feed(&mut dashboard, &mut tui, golden("detail-g1-m01-1h.json"));
        let lines = rows(&dashboard, index);
        check_text("detail-en.txt", &(lines.join("\n") + "\n"));
        let text = lines.join("\n");
        for heading in ["Host", "Build", "Cache", "Execution", "Graph", "Readiness"] { assert!(text.contains(&format!("\u{25be} {heading}")), "{heading}"); }
        assert!(text.contains("CPU busy share") && text.contains("Units compiled") && text.contains("/min") && text.contains("no samples in this range"));
        assert!(lines.iter().any(|line| line.contains('\u{252c}')), "the tier ruler marks the tiers");
        press(&mut dashboard, &mut tui, &[named(Key::Esc)]);
        let Body::Fleet(view) = &dashboard.windows[index].body else { panic!("fleet body after Esc") };
        assert!(view.drill.is_none(), "Esc leaves the drill-down first");
        press(&mut dashboard, &mut tui, &[named(Key::Esc)]);
        assert!(matches!(dashboard.windows[index].body, Body::Overview), "a second Esc returns to the task overview");
    }

    #[test]
    fn every_state_has_a_symbol_and_a_word_and_stale_rows_are_dimmed() {
        let (mut dashboard, mut tui, index) = open("en");
        feed(&mut dashboard, &mut tui, golden("overview-5m.json"));
        let rows: Vec<(String, RowAction)> = dashboard.windows[index].pane.rows.clone();
        let origin = |id: &str| rows.iter().position(|(_, action)| *action == RowAction::FleetOrigin(id.into())).unwrap_or_else(|| panic!("{id} row"));
        let list = match &tui.scene.node(dashboard.windows[index].list).content { semio_ui_tui_frame::scene::NodeContent::Widget(WidgetState::List(list)) => list, _ => panic!("list") };
        let (live, stale, missing) = (origin("g1-m01"), origin("g1-m02"), origin("g1-m03"));
        let status = |position: usize| list.statuses.get(position - list.offset).copied().flatten();
        assert_eq!((status(live), status(stale), status(missing)), (Some(Status::Success), Some(Status::Waiting), Some(Status::Fault)));
        let dimmed = |position: usize| list.dimmed.get(position - list.offset).copied().unwrap_or(false);
        assert_eq!((dimmed(live), dimmed(stale), dimmed(missing)), (false, true, true));
        assert!(rows[stale].0.contains("stale") && rows[missing].0.contains("missing") && rows[live].0.contains("ready"));
        assert!(rows[stale].0.contains('\u{25b2}') && rows[stale].0.contains("dup 1"), "a machine-name warning is a symbol and a word");
        assert!(rows[live].0.contains("ok") && rows[live].0.contains('%'));
        assert!(rows[missing].0.contains("no data"), "a missing origin shows no invented numbers");
    }

    #[test]
    fn the_fleet_total_counts_ready_origins_and_sums_the_rates() {
        let (mut dashboard, mut tui, index) = open("en");
        feed(&mut dashboard, &mut tui, golden("overview-5m.json"));
        let lines = rows(&dashboard, index);
        let total = lines.iter().find(|line| line.starts_with("\u{3a3} fleet")).expect("a fleet total row");
        assert!(total.contains("1/3 ready"), "{total}");
        let snapshot = golden("overview-5m.json");
        let live = snapshot.row("g1-m01").unwrap();
        let rate = format!("{}", live.units_per_minute.unwrap());
        assert!(total.contains(&rate), "{total} should carry the live rate {rate}");
    }

    #[test]
    fn digits_and_arrows_zoom_and_every_change_tells_the_daemon_exactly_once() {
        let (mut dashboard, mut tui, index) = open("en");
        dashboard.link.online_for_test = true;
        dashboard.frame(&mut tui);
        assert_eq!(watches(&dashboard).last().map(|request| (request.range, request.columns, request.origin.clone())), Some((Range::Minutes5, 16, None)));
        press(&mut dashboard, &mut tui, &[key('3')]);
        assert_eq!(watches(&dashboard).last().map(|request| request.range), Some(Range::Hours24));
        press(&mut dashboard, &mut tui, &[named(Key::Right), named(Key::Right), named(Key::Right)]);
        assert_eq!(watches(&dashboard).last().map(|request| request.range), Some(Range::Days90), "the arrow stops at the coarsest range");
        press(&mut dashboard, &mut tui, &[named(Key::Left)]);
        assert_eq!(watches(&dashboard).last().map(|request| request.range), Some(Range::Days7));
        let count = watches(&dashboard).len();
        dashboard.frame(&mut tui);
        dashboard.frame(&mut tui);
        assert_eq!(watches(&dashboard).len(), count, "an unchanged request is not sent again");
        for digit in ['1', '2', '3', '4', '5'] {
            press(&mut dashboard, &mut tui, &[key(digit)]);
            let Body::Fleet(view) = &dashboard.windows[index].body else { panic!() };
            assert_eq!(view.range, Range::ALL[usize::from(digit as u8 - b'1')]);
        }
        dashboard.windows[index].pane.selected = 0;
        press(&mut dashboard, &mut tui, &[named(Key::Esc)]);
        assert!(dashboard.link.outbox.iter().any(|message| matches!(message, ClientMsg::FleetUnwatch {})), "leaving the Fleet window ends the watch");
    }

    #[test]
    fn a_snapshot_of_another_range_is_not_shown_and_the_loading_hint_is() {
        let (mut dashboard, mut tui, index) = open("en");
        assert!(rows(&dashboard, index).iter().any(|line| line.contains("Waiting for the fleet snapshot")));
        dashboard.link.online_for_test = true;
        dashboard.frame(&mut tui);
        press(&mut dashboard, &mut tui, &[key('2')]);
        feed(&mut dashboard, &mut tui, golden("overview-5m.json"));
        assert!(dashboard.fleet.snapshot.is_none(), "an answer to the old request is dropped");
        feed(&mut dashboard, &mut tui, golden("overview-1h.json"));
        assert!(rows(&dashboard, index).iter().any(|line| line.contains("g1-m01")));
    }

    #[test]
    fn s_cycles_the_order_and_c_the_columns() {
        let (mut dashboard, mut tui, index) = open("en");
        feed(&mut dashboard, &mut tui, golden("overview-5m.json"));
        let order = |dashboard: &Dashboard| -> Vec<String> { dashboard.windows[index].pane.rows.iter().filter_map(|(_, action)| if let RowAction::FleetOrigin(origin) = action { Some(origin.clone()) } else { None }).collect() };
        assert_eq!(order(&dashboard), ["g1-m03", "g1-m02", "g1-m01"], "problems first: missing, stale, then healthy");
        press(&mut dashboard, &mut tui, &[key('s')]);
        assert!(rows(&dashboard, index)[0].contains("sort CPU"));
        assert_eq!(order(&dashboard)[0], "g1-m01", "values descending, an origin without a value last");
        press(&mut dashboard, &mut tui, &[key('c')]);
        let header = rows(&dashboard, index)[2].clone();
        assert!(header.contains("CPU trend") && !header.contains("Units/min"), "{header}");
        for _ in 0..3 { press(&mut dashboard, &mut tui, &[key('c')]); }
        assert!(rows(&dashboard, index)[0].contains("columns all"));
    }

    #[test]
    fn an_unavailable_store_and_an_empty_store_say_so() {
        let (mut dashboard, mut tui, index) = open("de");
        let mut snapshot = golden("overview-5m.json");
        snapshot.store = StoreState::Unavailable;
        snapshot.rows.clear();
        feed(&mut dashboard, &mut tui, snapshot.clone());
        assert!(rows(&dashboard, index).iter().any(|line| line.contains("nicht lesbar")));
        snapshot.store = StoreState::Available;
        feed(&mut dashboard, &mut tui, snapshot);
        assert!(rows(&dashboard, index).iter().any(|line| line.contains("keine Herkunftsordner")));
    }

    #[test]
    fn the_window_is_reachable_by_the_prefix_key_and_drawn_on_screen() {
        let (mut dashboard, mut tui) = view();
        press(&mut dashboard, &mut tui, &[ctrl('b'), key('W')]);
        let index = dashboard.windows.iter().position(|window| matches!(window.body, Body::Fleet(_))).expect("the prefix key opens the Fleet window");
        feed(&mut dashboard, &mut tui, golden("overview-5m.json"));
        let screen = plain(&tui.render_full().0);
        for expected in ["Fleet", "g1-m01", "g1-m03", "missing", "stale"] { assert!(screen.contains(expected), "{expected} is on the screen"); }
        let _ = index;
        assert!(dashboard.text().action("show-fleet").contains("fleet"));
    }

    #[test]
    fn a_plain_ascii_terminal_gets_ascii_symbols_for_every_chart_glyph() {
        let (mut dashboard, mut tui, index) = open("en");
        dashboard.apply_capabilities(&mut tui, false);
        feed(&mut dashboard, &mut tui, golden("overview-5m.json"));
        let live = rows(&dashboard, index).into_iter().find(|line| line.contains("g1-m01")).unwrap();
        assert!(live.chars().all(|character| character.is_ascii() || character == '\u{3a3}' || character == '\u{b7}'), "{live}");
        assert!(rows(&dashboard, index).iter().any(|line| line.contains("g1-m02") && line.contains('*')), "the ascii warning symbol");
    }

    #[test]
    fn a_metric_the_platform_cannot_measure_is_n_a_with_a_symbol_and_never_no_data_or_stale() {
        for (language, word, glyphs_ascii) in [("en", "n/a", false), ("de", "n. v.", false), ("en", "n/a", true)] {
            let (mut dashboard, mut tui, index) = open(language);
            if glyphs_ascii { dashboard.apply_capabilities(&mut tui, false); }
            let mut snapshot = golden("overview-5m.json");
            let row = snapshot.rows.iter_mut().find(|row| row.origin == "g1-m01").unwrap();
            row.unsupported = vec!["host.cpu.busy".into()];
            row.cpu = None;
            row.cpu_series.clear();
            feed(&mut dashboard, &mut tui, snapshot);
            let live = rows(&dashboard, index).into_iter().find(|line| line.starts_with("g1-m01")).unwrap();
            let mark = if glyphs_ascii { '~' } else { '\u{2298}' };
            assert!(live.contains(&format!("{mark} {word}")), "{live}");
            assert_eq!(live.matches(&format!("{mark} {word}")).count(), 2, "the CPU gauge and the CPU trend are n/a: {live}");
            assert!(!live.contains("stale") && !live.contains("missing"), "n/a is not stale: {live}");
            let gap = rows(&dashboard, index).into_iter().find(|line| line.starts_with("g1-m03")).unwrap();
            assert!(!gap.contains(&format!("{mark} {word}")) && gap.contains(if language == "en" { "no data" } else { "keine Daten" }), "an origin without data yet says no data, not n/a: {gap}");
            assert!(live.contains("ok") && live.contains('%'), "the other gauges of the row keep their values: {live}");
        }
    }

    #[test]
    fn a_drill_down_marks_an_unsupported_metric_n_a_with_the_platform() {
        for (language, expected) in [("en", "n/a \u{b7} not measured on windows"), ("de", "n. v. \u{b7} auf windows nicht gemessen")] {
            let (mut dashboard, mut tui, index) = open(language);
            feed(&mut dashboard, &mut tui, golden("overview-5m.json"));
            press(&mut dashboard, &mut tui, &[key('2')]);
            let mut overview = golden("overview-1h.json");
            overview.rows.iter_mut().find(|row| row.origin == "g1-m01").unwrap().ready.as_mut().unwrap().platform = Some("windows".into());
            feed(&mut dashboard, &mut tui, overview);
            dashboard.windows[index].pane.selected = dashboard.windows[index].pane.rows.iter().position(|(_, action)| *action == RowAction::FleetOrigin("g1-m01".into())).expect("the g1-m01 row");
            press(&mut dashboard, &mut tui, &[named(Key::Enter)]);
            let mut detail = golden("detail-g1-m01-1h.json");
            detail.rows.iter_mut().find(|row| row.origin == "g1-m01").unwrap().ready.as_mut().unwrap().platform = Some("windows".into());
            let series = detail.detail.as_mut().unwrap().groups.iter_mut().find(|group| group.id == "host").unwrap().series.iter_mut().find(|series| series.id == "host.cpu.load").unwrap();
            series.unsupported = true;
            series.points.clear();
            feed(&mut dashboard, &mut tui, detail);
            let lines = rows(&dashboard, index);
            let line = lines.iter().find(|line| line.contains(expected)).unwrap_or_else(|| panic!("no n/a line in {lines:?}"));
            assert!(line.starts_with(if language == "en" { "Load average (1 min)" } else { "Last im Mittel (1 Min.)" }) && line.contains('\u{2298}'), "{line}");
            let busy = dashboard_fleet::metric_label("host.cpu.busy", language).unwrap();
            assert!(lines.iter().any(|line| line.starts_with(busy) && !line.contains('\u{2298}')), "supported metrics still draw their chart");
        }
    }

    fn wide_dashboard(language: &str) -> (Dashboard, Tui, usize) {
        let mut tui = Tui::new(Size { width: 200, height: 60 }, Theme::new(AppearanceName::Dark));
        let mut dashboard = Dashboard::new(&mut tui, "workspace".into(), Preferences { language: Some(language.into()), ..Default::default() }, std::env::temp_dir().join("unused-dashboard-preferences"), false);
        dashboard.link = Link::offline();
        dashboard.state = LinkState::Connected;
        dashboard.set_registry(&mut tui, registry());
        dashboard.perform(&mut tui, Action::ShowFleet);
        dashboard.frame(&mut tui);
        let index = dashboard.windows.iter().position(|window| matches!(window.body, Body::Fleet(_))).expect("a Fleet window");
        (dashboard, tui, index)
    }

    fn live_workspace() -> std::path::PathBuf {
        let mut found = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        while !found.join(".cache/f/machine.json").is_file() { assert!(found.pop(), "no machine.json above the crate"); }
        found
    }

    /// 🖨️ Renders the Fleet overview and one drill-down of the REAL shared store to text, exactly as the window shows them (the store root comes from the configuration: `SEMIO_FLEET_STORE_ROOT`, else `storeRoot` of `<state>/f/machine.json`). Not part of the suite: run with `FLEET_LIVE_OUT=<directory> cargo test live_store_render -- --ignored`; `FLEET_LIVE_ORIGIN` names the drill-down origin, `FLEET_LIVE_RANGES` the ranges (`1,2,3`), `FLEET_LIVE_LANGUAGE` the language.
    #[test]
    #[ignore = "reads the real shared store of this machine"]
    fn live_store_render() {
        let workspace = live_workspace();
        let origins = dashboard_fleet::origins_directory(&workspace, std::env::var("SEMIO_FLEET_STORE_ROOT").ok().as_deref(), None).expect("a configured store");
        let language = std::env::var("FLEET_LIVE_LANGUAGE").unwrap_or_else(|_| "en".into());
        let ranges = std::env::var("FLEET_LIVE_RANGES").unwrap_or_else(|_| "1,2,3".into());
        let out = std::env::var("FLEET_LIVE_OUT").ok().map(std::path::PathBuf::from);
        let now = || std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("clock").as_millis() as u64;
        let mut text = String::new();
        for digit in ranges.split(',').filter_map(|digit| digit.trim().chars().next()) {
            let (mut dashboard, mut tui, index) = wide_dashboard(&language);
            press(&mut dashboard, &mut tui, &[key(digit)]);
            let request = dashboard.fleet_request(&tui).expect("the window asks for a snapshot");
            let started = std::time::Instant::now();
            let snapshot = dashboard_fleet::read_snapshot(Some(&origins), &request, now());
            let read = started.elapsed();
            let live: Vec<String> = snapshot.rows.iter().filter(|row| row.freshness == Freshness::Live).map(|row| row.origin.clone()).collect();
            feed(&mut dashboard, &mut tui, snapshot);
            text += &format!("## Overview, range key {digit}, language {language} (read {read:.1?})\n\n```\n{}\n```\n\n", rows(&dashboard, index).join("\n"));
            let wanted: Vec<String> = match std::env::var("FLEET_LIVE_ORIGIN") { Ok(origin) => vec![origin], Err(_) => vec![live.first().cloned().unwrap_or_default(), "g1-m05".into()] };
            for origin in wanted {
                let Some(position) = dashboard.windows[index].pane.rows.iter().position(|(_, action)| *action == RowAction::FleetOrigin(origin.clone())) else { continue };
                dashboard.windows[index].pane.selected = position;
                press(&mut dashboard, &mut tui, &[named(Key::Enter)]);
                let request = dashboard.fleet_request(&tui).expect("the drill-down asks for a snapshot");
                let started = std::time::Instant::now();
                let detail = dashboard_fleet::read_snapshot(Some(&origins), &request, now());
                let read = started.elapsed();
                feed(&mut dashboard, &mut tui, detail);
                text += &format!("## Drill-down {origin}, range key {digit}, language {language} (read {read:.1?})\n\n```\n{}\n```\n\n", rows(&dashboard, index).join("\n"));
                press(&mut dashboard, &mut tui, &[named(Key::Esc)]);
                let back = dashboard.fleet_request(&tui).expect("back in the overview");
                feed(&mut dashboard, &mut tui, dashboard_fleet::read_snapshot(Some(&origins), &back, now()));
            }
        }
        match out { Some(directory) => { std::fs::create_dir_all(&directory).expect("output directory"); std::fs::write(directory.join(format!("live-render-{language}.md")), &text).expect("write the render"); } None => println!("{text}") }
    }

    #[test]
    fn the_snapshot_goldens_are_fresh_enough_to_show_the_three_kinds_of_origin() {
        let snapshot = golden("overview-90d.json");
        let kinds: Vec<Freshness> = snapshot.rows.iter().map(|row| row.freshness).collect();
        assert_eq!(kinds, [Freshness::Live, Freshness::Stale, Freshness::Missing]);
    }
}
