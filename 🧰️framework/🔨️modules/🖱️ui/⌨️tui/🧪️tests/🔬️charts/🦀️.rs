use semio_ui_tui_core::cell::{Cell, CellBuffer};
use semio_ui_tui_core::geometry::{Rect, Size};
use semio_ui_tui_core::theme::{GlyphSet, Role, Theme};
use semio_ui_tui_charts::chart_scale::{chart_labels, columns, domain, dot_row, eighths, grid_text, level_of, tier_columns, Aggregate, Bucket, ChartLabels, Level, Locale, Polarity, Range, Stats, XAxis};
use semio_ui_tui_charts::gauge::{filled_eighths, gauge_grid, gauge_state, paint_gauge, GaugeSpec};
use semio_ui_tui_charts::line_chart::{line_chart_grid, paint_line_chart, ChartSeries, Dash, LineChartSpec, Marks};
use semio_ui_tui_charts::sparkline::{paint_sparkline, sparkline_grid, SparklineSpec};
use semio_ui_tui_charts::stat::{format_value, paint_stat, stat_grid, StatSpec, Unit};
use serde_json::Value;
use ui_styling::appearance::AppearanceName;

const FIXTURE: &str = include_str!("../🧫️fixtures/📈️charts/🔣️.json");

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the charts fixture is valid JSON")
}

fn num(value: &Value) -> f64 {
    value.as_f64().unwrap_or(f64::NAN)
}

fn stats(value: &Value) -> Option<Stats> {
    let parts = value.as_array()?;
    Some(Stats { min: num(&parts[0]), max: num(&parts[1]), mean: num(&parts[2]), last: num(&parts[3]) })
}

fn buckets(value: &Value) -> Vec<Bucket> {
    value.as_array().unwrap().iter().map(|row| Bucket { start: row[0].as_u64().unwrap(), end: row[1].as_u64().unwrap(), stats: stats(&row[2]) }).collect()
}

fn axis(value: &Value) -> XAxis {
    if value == "index" { XAxis::Index } else { XAxis::Time }
}

fn aggregate(value: &Value) -> Aggregate {
    match value.as_str().unwrap() {
        "last" => Aggregate::Last,
        "min" => Aggregate::Min,
        "max" => Aggregate::Max,
        _ => Aggregate::Mean,
    }
}

fn unit(value: &Value) -> Unit {
    match value.as_str().unwrap() {
        "count" => Unit::Count,
        "bytes" => Unit::Bytes,
        "percent" => Unit::Percent,
        "rate" => Unit::Rate,
        "byte_rate" => Unit::ByteRate,
        _ => Unit::DurationMs,
    }
}

fn locale(value: &Value) -> &'static ChartLabels {
    chart_labels(if value == "de" { Locale::De } else { Locale::En })
}

fn framed(rows: &[String]) -> String {
    rows.iter().map(|row| format!("|{row}|")).collect::<Vec<_>>().join("\n")
}

fn check(name: &str, rows: Vec<String>) {
    let got = framed(&rows);
    let want = SNAPSHOTS.iter().find(|(key, _)| *key == name).map_or("", |(_, text)| *text);
    if std::env::var_os("M1_PRINT_SNAPSHOTS").is_some() {
        println!("[SNAP {name}]\n{got}\n[END]");
        return;
    }
    assert_eq!(got, want, "snapshot {name}");
}

fn close(a: f64, b: f64) -> bool {
    (a.is_nan() && b.is_nan()) || (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0)
}

fn en() -> &'static ChartLabels {
    chart_labels(Locale::En)
}

fn b(start: u64, end: u64, v: f64) -> Bucket {
    Bucket::new(start, end, Stats::point(v))
}

fn secs(values: &[Option<f64>]) -> Vec<Bucket> {
    values.iter().enumerate().map(|(i, v)| v.map_or_else(|| Bucket::gap(i as u64 * 1000, i as u64 * 1000 + 1000), |v| b(i as u64 * 1000, i as u64 * 1000 + 1000, v))).collect()
}

fn tiered() -> Vec<Bucket> {
    let mut out = vec![b(0, 60_000, 2.0), b(60_000, 120_000, 3.0), Bucket::gap(120_000, 180_000)];
    out.extend([5.0, 7.0, 1.0, 4.0].iter().enumerate().map(|(i, v)| b(180_000 + i as u64 * 1000, 181_000 + i as u64 * 1000, *v)));
    out
}

fn spark(buckets: &[Bucket], width: usize, rows: usize) -> Vec<String> {
    grid_text(&sparkline_grid(&SparklineSpec { buckets, axis: XAxis::Time, range: Range::Auto, aggregate: Aggregate::Mean, band: false, role: Role::Accent }, width, rows, GlyphSet::Unicode))
}

fn plain(series: &[ChartSeries], axis: XAxis, marks: Marks, width: usize, height: usize) -> Vec<String> {
    grid_text(&line_chart_grid(&LineChartSpec { series, axis, range: Range::Auto, unit: Unit::Count, marks, area: false, legend: false }, width, height, en(), GlyphSet::Unicode))
}

fn one<'a>(buckets: &'a [Bucket], name: &'a str) -> ChartSeries<'a> {
    ChartSeries { name, buckets, role: Role::Accent, dash: Dash::Solid, aggregate: Aggregate::Mean, band: false }
}

//#region 🧪️ Oracle fixture
#[test]
fn scaling_math_matches_the_python_oracle_fixture() {
    let fixture = fixture();
    for case in fixture["eighths"].as_array().unwrap() {
        assert_eq!(eighths(num(&case["v"]), num(&case["lo"]), num(&case["hi"]), case["levels"].as_u64().unwrap() as usize) as u64, case["expect"].as_u64().unwrap(), "{case}");
    }
    for case in fixture["dot_row"].as_array().unwrap() {
        assert_eq!(dot_row(num(&case["v"]), num(&case["lo"]), num(&case["hi"]), case["dots"].as_u64().unwrap() as usize) as u64, case["expect"].as_u64().unwrap(), "{case}");
    }
    for case in fixture["filled_eighths"].as_array().unwrap() {
        assert_eq!(filled_eighths(num(&case["fraction"]), case["cells"].as_u64().unwrap() as usize) as u64, case["expect"].as_u64().unwrap(), "{case}");
    }
    for case in fixture["level"].as_array().unwrap() {
        let polarity = if case["polarity"] == "high" { Polarity::HighIsBad } else { Polarity::LowIsBad };
        let expect = match case["expect"].as_str().unwrap() { "ok" => Level::Ok, "warn" => Level::Warn, _ => Level::Critical };
        assert_eq!(level_of(num(&case["fraction"]), num(&case["warn"]), num(&case["critical"]), polarity), expect, "{case}");
    }
}

#[test]
fn column_resampling_and_tier_columns_match_the_python_oracle_fixture() {
    for case in fixture()["columns"].as_array().unwrap() {
        let (series, width, axis) = (buckets(&case["buckets"]), case["width"].as_u64().unwrap() as usize, axis(&case["axis"]));
        let dom = (case["domain"][0].as_u64().unwrap(), case["domain"][1].as_u64().unwrap());
        let got = columns(&series, axis, dom, width);
        let expect = case["expect"].as_array().unwrap();
        assert_eq!(got.len(), expect.len(), "{}", case["name"]);
        for (column, (got, expect)) in got.iter().zip(expect).enumerate() {
            match (got, stats(expect)) {
                (None, None) => {}
                (Some(got), Some(want)) => assert!(close(got.min, want.min) && close(got.max, want.max) && close(got.mean, want.mean) && close(got.last, want.last), "{} column {column}: {got:?} vs {want:?}", case["name"]),
                (got, want) => panic!("{} column {column}: {got:?} vs {want:?}", case["name"]),
            }
        }
        let boundaries: Vec<(u64, u64)> = tier_columns(&series, axis, dom, width).into_iter().map(|(c, i)| (c as u64, i as u64)).collect();
        let want: Vec<(u64, u64)> = case["tier_columns"].as_array().unwrap().iter().map(|pair| (pair[0].as_u64().unwrap(), pair[1].as_u64().unwrap())).collect();
        assert_eq!(boundaries, want, "{} tier columns", case["name"]);
    }
    for case in fixture()["domain"].as_array().unwrap() {
        let series = buckets(&case["buckets"]);
        let got = domain(&[&series], axis(&case["axis"])).unwrap();
        assert_eq!([got.0, got.1], [case["expect"][0].as_u64().unwrap(), case["expect"][1].as_u64().unwrap()]);
    }
}

#[test]
fn value_formatting_matches_the_python_oracle_fixture() {
    for case in fixture()["format"].as_array().unwrap() {
        assert_eq!(format_value(num(&case["v"]), unit(&case["unit"]), locale(&case["locale"])), case["expect"].as_str().unwrap(), "{case}");
    }
}

#[test]
fn sparklines_match_the_independent_python_renderer() {
    for case in fixture()["spark"].as_array().unwrap() {
        let series = buckets(&case["buckets"]);
        let range = case["range"].as_array().map_or(Range::Auto, |r| Range::Fixed(num(&r[0]), num(&r[1])));
        let spec = SparklineSpec { buckets: &series, axis: axis(&case["axis"]), range, aggregate: aggregate(&case["aggregate"]), band: case["band"].as_bool().unwrap(), role: Role::Accent };
        let got = grid_text(&sparkline_grid(&spec, case["width"].as_u64().unwrap() as usize, case["rows"].as_u64().unwrap() as usize, GlyphSet::Unicode));
        let want: Vec<String> = case["expect"].as_array().unwrap().iter().map(|row| row.as_str().unwrap().to_string()).collect();
        assert_eq!(got, want, "sparkline {}", case["name"]);
    }
}
//#endregion

//#region 📈️ Sparkline
fn spec_of<'a>(buckets: &'a [Bucket], axis: XAxis, band: bool) -> SparklineSpec<'a> {
    SparklineSpec { buckets, axis, range: Range::Auto, aggregate: Aggregate::Mean, band, role: Role::Accent }
}

#[test]
fn sparkline_snapshots_cover_empty_gaps_one_point_flat_spikes_band_and_narrow_widths() {
    let rows = |buckets: &[Bucket], width: usize, band_rows: usize, band: bool| grid_text(&sparkline_grid(&spec_of(buckets, XAxis::Time, band), width, band_rows, GlyphSet::Unicode));
    check("spark empty", rows(&[], 8, 1, false));
    check("spark gaps", rows(&secs(&[Some(1.0), Some(2.0), None, None, Some(8.0), Some(4.0), None, Some(5.0)]), 8, 1, false));
    check("spark one point", rows(&secs(&[Some(42.0)]), 6, 1, false));
    check("spark flat", rows(&secs(&[Some(3.0); 8]), 8, 1, false));
    let mut spiky = vec![Some(1.0); 12];
    spiky[3] = Some(100.0);
    spiky[8] = Some(100.0);
    check("spark spikes", rows(&secs(&spiky), 12, 1, false));
    let banded: Vec<Bucket> = (0..8).map(|i| Bucket::new(i * 1000, i * 1000 + 1000, Stats { min: i as f64, max: i as f64 + 6.0, mean: i as f64 + 3.0, last: i as f64 + 2.0 })).collect();
    check("spark band", rows(&banded, 8, 3, true));
    check("spark band off", rows(&banded, 8, 3, false));
    check("spark narrow", rows(&secs(&(0..8).map(|i| Some(f64::from(i))).collect::<Vec<_>>()), 3, 1, false));
    check("spark zero width", rows(&secs(&[Some(1.0)]), 0, 1, false));
}

#[test]
fn sparkline_tiers_follow_time_or_bucket_index() {
    let series = tiered();
    check("spark tiers time", grid_text(&sparkline_grid(&spec_of(&series, XAxis::Time, false), 23, 1, GlyphSet::Unicode)));
    check("spark tiers index", grid_text(&sparkline_grid(&spec_of(&series, XAxis::Index, false), 23, 1, GlyphSet::Unicode)));
}

#[test]
fn sparkline_has_an_ascii_repertoire_with_a_distinct_gap_glyph() {
    let series = secs(&[Some(1.0), Some(8.0), None, Some(4.0)]);
    check("spark ascii", grid_text(&sparkline_grid(&spec_of(&series, XAxis::Time, false), 4, 1, GlyphSet::Ascii)));
}

#[test]
fn a_fixed_range_clamps_and_the_aggregate_chooses_the_statistic() {
    let series: Vec<Bucket> = (0..4).map(|i| Bucket::new(i * 1000, i * 1000 + 1000, Stats { min: 0.0, max: 100.0, mean: 10.0 * i as f64, last: 5.0 })).collect();
    let draw = |range: Range, aggregate: Aggregate| grid_text(&sparkline_grid(&SparklineSpec { buckets: &series, axis: XAxis::Time, range, aggregate, band: false, role: Role::Accent }, 4, 1, GlyphSet::Unicode));
    assert_eq!(draw(Range::Fixed(0.0, 20.0), Aggregate::Mean), ["\u{2581}\u{2585}\u{2588}\u{2588}"]);
    assert_eq!(draw(Range::Auto, Aggregate::Max), ["\u{2584}\u{2584}\u{2584}\u{2584}"]);
    assert_eq!(draw(Range::Fixed(3.0, 1.0), Aggregate::Last), ["\u{2584}\u{2584}\u{2584}\u{2584}"].map(|row| row.replace('\u{2584}', "\u{2588}")), "a reversed fixed range is ordered and clamped");
}

#[test]
fn painting_a_sparkline_writes_the_grid_text_and_colours_gaps_apart() {
    let series = secs(&[Some(1.0), None, Some(8.0)]);
    let rect = Rect::new(1, 1, 6, 1);
    let mut buf = CellBuffer::new(Size { width: 8, height: 3 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let theme = Theme::new(AppearanceName::Dark);
    paint_sparkline(&spec_of(&series, XAxis::Time, false), &theme, rect, &mut buf);
    assert_eq!(buf.row_text(1), " \u{2581}\u{2581}\u{00b7}\u{00b7}\u{2588}\u{2588} ");
    assert_eq!(buf.get(2, 1).unwrap().fg, theme.role(Role::Accent));
    assert_eq!(buf.get(3, 1).unwrap().fg, theme.role(Role::MutedForeground));
}
//#endregion

//#region 📉️ LineChart
#[test]
fn line_chart_snapshots_cover_empty_one_point_flat_gaps_spikes_and_a_band() {
    let ramp = secs(&(1..=8).map(|i| Some(f64::from(i))).collect::<Vec<_>>());
    check("line empty", plain(&[], XAxis::Time, Marks::Braille, 20, 6));
    check("line ramp", plain(&[one(&ramp, "a")], XAxis::Time, Marks::Braille, 20, 6));
    check("line one point", plain(&[one(&secs(&[Some(5.0)]), "a")], XAxis::Time, Marks::Braille, 20, 6));
    check("line flat", plain(&[one(&secs(&[Some(5.0); 6]), "a")], XAxis::Time, Marks::Braille, 20, 6));
    let gappy = secs(&[Some(1.0), Some(5.0), None, None, Some(9.0), Some(2.0), None, Some(4.0)]);
    check("line gaps", plain(&[one(&gappy, "a")], XAxis::Time, Marks::Braille, 20, 6));
    let mut spiky = vec![Some(1.0); 8];
    spiky[3] = Some(50.0);
    check("line spikes", plain(&[one(&secs(&spiky), "a")], XAxis::Time, Marks::Braille, 20, 6));
    let band = vec![Bucket::new(0, 1000, Stats { min: 1.0, max: 9.0, mean: 5.0, last: 5.0 }), Bucket::new(1000, 2000, Stats { min: 2.0, max: 4.0, mean: 3.0, last: 3.0 }), Bucket::new(2000, 3000, Stats { min: 6.0, max: 10.0, mean: 8.0, last: 8.0 })];
    let mut banded = one(&band, "a");
    banded.band = true;
    check("line band", plain(&[banded], XAxis::Time, Marks::Braille, 20, 6));
}

#[test]
fn line_chart_snapshots_cover_narrow_widths_and_blocks() {
    let ramp = secs(&(1..=8).map(|i| Some(f64::from(i))).collect::<Vec<_>>());
    check("line narrow", plain(&[one(&ramp, "a")], XAxis::Time, Marks::Braille, 6, 5));
    check("line narrower", plain(&[one(&ramp, "a")], XAxis::Time, Marks::Braille, 3, 3));
    check("line tiny", plain(&[one(&ramp, "a")], XAxis::Time, Marks::Braille, 2, 1));
    let spec = LineChartSpec { series: &[one(&ramp, "a")], axis: XAxis::Time, range: Range::Auto, unit: Unit::Count, marks: Marks::Blocks, area: true, legend: false };
    check("line area blocks", grid_text(&line_chart_grid(&spec, 20, 6, en(), GlyphSet::Unicode)));
}

#[test]
fn non_uniform_tiers_are_proportional_to_time_or_to_the_bucket_index() {
    let series = tiered();
    check("line tiers time", plain(&[one(&series, "a")], XAxis::Time, Marks::Braille, 40, 7));
    check("line tiers index", plain(&[one(&series, "a")], XAxis::Index, Marks::Braille, 40, 7));
    check("line tiers blocks", plain(&[one(&series, "a")], XAxis::Index, Marks::Blocks, 40, 7));
    let spec = LineChartSpec { series: &[one(&series, "a")], axis: XAxis::Index, range: Range::Auto, unit: Unit::Count, marks: Marks::Braille, area: false, legend: true };
    check("line ascii", grid_text(&line_chart_grid(&spec, 40, 7, en(), GlyphSet::Ascii)));
    check("line tiers german", grid_text(&line_chart_grid(&spec, 40, 7, chart_labels(Locale::De), GlyphSet::Unicode)));
}

#[test]
fn several_series_keep_their_style_and_get_a_legend() {
    let first = tiered();
    let second: Vec<Bucket> = first.iter().map(|x| x.stats.map_or(*x, |s| Bucket::new(x.start, x.end, Stats::point(s.mean * 0.5)))).collect();
    let mut dashed = one(&second, "mem");
    dashed.dash = Dash::Dashed;
    dashed.role = Role::Warning;
    let spec = LineChartSpec { series: &[one(&first, "cpu"), dashed], axis: XAxis::Time, range: Range::Auto, unit: Unit::Percent, marks: Marks::Braille, area: false, legend: true };
    check("line two series", grid_text(&line_chart_grid(&spec, 44, 9, en(), GlyphSet::Unicode)));
    let mut dotted = one(&second, "mem");
    dotted.dash = Dash::Dotted;
    let spec = LineChartSpec { series: &[one(&first, "cpu"), dotted], axis: XAxis::Index, range: Range::Fixed(0.0, 10.0), unit: Unit::Count, marks: Marks::Blocks, area: false, legend: true };
    check("line two series blocks", grid_text(&line_chart_grid(&spec, 44, 9, en(), GlyphSet::Unicode)));
}

#[test]
fn painting_a_chart_colours_each_series_with_its_role() {
    let (low, high) = (secs(&[Some(1.0), Some(1.0)]), secs(&[Some(9.0), Some(9.0)]));
    let mut second = one(&high, "b");
    second.role = Role::Danger;
    let spec = LineChartSpec { series: &[one(&low, "a"), second], axis: XAxis::Time, range: Range::Auto, unit: Unit::Count, marks: Marks::Braille, area: false, legend: false };
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 12, 6);
    let mut buf = CellBuffer::new(Size { width: 12, height: 6 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    paint_line_chart(&spec, en(), &theme, rect, &mut buf);
    assert_eq!(buf.get(3, 0).unwrap().fg, theme.role(Role::Danger), "the high series is drawn at the top");
    assert_eq!(buf.get(3, 3).unwrap().fg, theme.role(Role::Accent), "the low series is drawn at the bottom");
}
//#endregion

//#region 🌡️ Gauge
fn gauge(value: f64, width: usize, locale: Locale) -> Vec<String> {
    let spec = GaugeSpec { label: "disk", value: value * 1073741824.0, capacity: 8.0 * 1073741824.0, unit: Unit::Bytes, warn: 0.7, critical: 0.9, polarity: Polarity::HighIsBad };
    grid_text(&gauge_grid(&spec, width, chart_labels(locale), GlyphSet::Unicode))
}

#[test]
fn gauge_snapshots_show_the_level_as_a_word_and_the_thresholds_as_ticks() {
    check("gauge", [gauge(1.0, 52, Locale::En), gauge(6.0, 52, Locale::En), gauge(7.6, 52, Locale::En), gauge(7.6, 52, Locale::De), gauge(12.0, 52, Locale::En), gauge(0.0, 52, Locale::En)].concat());
    check("gauge narrow", [gauge(7.6, 30, Locale::En), gauge(7.6, 24, Locale::En), gauge(7.6, 14, Locale::En), gauge(7.6, 6, Locale::En), gauge(7.6, 1, Locale::En)].concat());
    check("gauge no data", [gauge(f64::NAN, 30, Locale::En), gauge(f64::INFINITY, 30, Locale::De)].concat());
    let none = GaugeSpec { label: "", value: 1.0, capacity: 0.0, unit: Unit::Count, warn: 0.7, critical: 0.9, polarity: Polarity::HighIsBad };
    check("gauge zero capacity", grid_text(&gauge_grid(&none, 20, en(), GlyphSet::Unicode)));
}

#[test]
fn a_low_is_bad_gauge_flags_a_low_value() {
    let spec = |value: f64| GaugeSpec { label: "free", value, capacity: 100.0, unit: Unit::Count, warn: 0.2, critical: 0.1, polarity: Polarity::LowIsBad };
    assert_eq!(gauge_state(&spec(50.0)).unwrap().1, Level::Ok);
    assert_eq!(gauge_state(&spec(15.0)).unwrap().1, Level::Warn);
    assert_eq!(gauge_state(&spec(5.0)).unwrap().1, Level::Critical);
    check("gauge low is bad", [5.0, 15.0, 50.0].iter().flat_map(|value| grid_text(&gauge_grid(&spec(*value), 40, en(), GlyphSet::Ascii))).collect());
}

#[test]
fn painting_a_gauge_colours_the_bar_and_the_state_word_by_level() {
    let spec = GaugeSpec { label: "", value: 95.0, capacity: 100.0, unit: Unit::Count, warn: 0.7, critical: 0.9, polarity: Polarity::HighIsBad };
    let theme = Theme::new(AppearanceName::Dark);
    let mut buf = CellBuffer::new(Size { width: 30, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    paint_gauge(&spec, en(), &theme, Rect::new(0, 0, 30, 1), &mut buf);
    assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::Danger));
    let text = buf.row_text(0);
    let word = text.find("critical").map(|byte| text[..byte].chars().count() as u16).expect("the level is written out");
    assert_eq!(buf.get(word, 0).unwrap().fg, theme.role(Role::Danger));
    assert_ne!(buf.get(word, 0).unwrap().attrs & semio_ui_tui_core::cell::attr::BOLD, 0);
}
//#endregion

//#region 🔢️ StatCell
#[test]
fn stat_cells_format_value_and_unit_in_one_or_two_rows() {
    let stat = |title: &str, value: Option<f64>, unit: Unit, level: Option<Level>, width: usize, height: usize, locale: Locale| grid_text(&stat_grid(&StatSpec { title, value, unit, level }, width, height, chart_labels(locale), GlyphSet::Unicode));
    let gib = 1073741824.0;
    check(
        "stat",
        [
            stat("memory", Some(3.5 * gib), Unit::Bytes, Some(Level::Warn), 24, 1, Locale::En),
            stat("memory", Some(3.5 * gib), Unit::Bytes, None, 10, 1, Locale::En),
            stat("cpu", None, Unit::Percent, None, 12, 2, Locale::En),
            stat("graph reads", Some(1500.0), Unit::Rate, Some(Level::Critical), 16, 2, Locale::En),
            stat("Dauer", Some(93_000.0), Unit::DurationMs, Some(Level::Ok), 18, 1, Locale::De),
            stat("duration", Some(2500.0), Unit::DurationMs, None, 18, 1, Locale::En),
            stat("x", Some(1.0), Unit::Count, None, 0, 1, Locale::En),
        ]
        .concat(),
    );
}

#[test]
fn painting_a_stat_colours_the_value_by_level() {
    let theme = Theme::new(AppearanceName::Dark);
    let mut buf = CellBuffer::new(Size { width: 16, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    paint_stat(&StatSpec { title: "errors", value: Some(3.0), unit: Unit::Count, level: Some(Level::Critical) }, en(), &theme, Rect::new(0, 0, 16, 1), &mut buf);
    assert_eq!(buf.row_text(0), "errors       3 !");
    assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::MutedForeground));
    assert_eq!(buf.get(13, 0).unwrap().fg, theme.role(Role::Danger));
}
//#endregion

const SNAPSHOTS: &[(&str, &str)] = &[
    ("gauge", r####"|disk ██▊░░░░░░░░░░░░╎░░░╎░░ 1.00GiB / 8.00GiB 12% ok|
|disk ███████████████░░░╎░ 6.00GiB / 8.00GiB 75% warn|
|disk ███████████████▎ 7.60GiB / 8.00GiB 95% critical|
|disk ███████████████▎ 7,60GiB / 8,00GiB 95% kritisch|
|disk ███████████████ 12.0GiB / 8.00GiB 150% critical|
|disk ░░░░░░░░░░░░░░░░░░╎░░░░╎░░ 0B / 8.00GiB 0.0% ok|"####),
    ("gauge low is bad", r####"|free -||---------- 5 / 100 5.0% critical|
|free ##-|------------- 15 / 100 15% warn|
|free #########---------- 50 / 100 50% ok|"####),
    ("gauge narrow", r####"|disk ███████████▍ 95% critical|
|disk █████▊ 95% critical|
|disk ████████▌|
|d… ██▉|
|…|"####),
    ("gauge no data", r####"|disk no data                  |
|disk keine Daten              |"####),
    ("gauge zero capacity", r####"|no data             |"####),
    ("line area blocks", r####"|8│             ▁▄▅██|
| │         ▃▃▆▇█████|
| │     ▂▃▆▆█████████|
|1│▁▁▄▅██████████████|
| └──────────────────|
|  -8.0s          now|"####),
    ("line ascii", r####"|- a                                     |
|7|                :     @@@@@@          |
| |                :*****           .....|
| |      -----     :                     |
|1|******          :           .....     |
| +-----------.....+---------------------|
|  -3min 4s        -4.0s              now|"####),
    ("line band", r####"|10│⢄⢄⢄⢄⢄⢄     ⢕⣕⣕⣕⣕⣕|
|  │⢕⢕⢕⢕⢕⢕     ⢽⢕⢕⢕⢕⢕|
|  │⢝⢝⢝⢝⢝⢽⣄⢄⢄⢄⢄⡟     |
| 1│⢕⢕⢕⢕⢕⢕⠙⠙⠙⠙⠙⠁     |
|  └─────────────────|
|   -3.0s         now|"####),
    ("line empty", r####"|                    |
|                    |
|                    |
|no data             |
|                    |
|                    |"####),
    ("line flat", r####"|5│                  |
| │                  |
| │⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉|
|5│                  |
| └──────────────────|
|  -6.0s          now|"####),
    ("line gaps", r####"|9│         ⠉⠉⡇      |
| │           ⣇      |
| │  ⣸⠉⠁      ⢸   ⠐⠒⠒|
|1│⣀⣀⡇        ⠘⠒⠂    |
| └─────╌╌╌╌─────╌───|
|  -8.0s          now|"####),
    ("line narrow", r####"|8│  ⢀⡼|
| │ ⢠⠞ |
|1│⣰⠋  |
| └────|
|   now|"####),
    ("line narrower", r####"|⣠⠴⠚|
|───|
|now|"####),
    ("line one point", r####"|5│                  |
| │                  |
| │⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉|
|5│                  |
| └──────────────────|
|  -1.0s          now|"####),
    ("line ramp", r####"|8│             ⢠⠤⠴⠋⠉|
| │         ⡤⠤⠞⠉⠉    |
| │    ⢀⣀⣠⠖⠒⠃        |
|1│⣀⣀⡴⠒⠚             |
| └──────────────────|
|  -8.0s          now|"####),
    ("line spikes", r####"|50│      ⢸⠉⢹        |
|  │      ⢸ ⢸        |
|  │      ⣸ ⢸        |
| 1│⣀⣀⣀⣀⣀⣀⡇ ⢸⣀⣀⣀⣀⣀⣀⣀⣀|
|  └─────────────────|
|   -8.0s         now|"####),
    ("line tiers blocks", r####"|7│                ┊     ██████          |
| │                ┊▃▃▃▃▃                |
| │                ┊                ▅▅▅▅▅|
| │      ▆▆▆▆▆     ┊                     |
|1│██████          ┊           ▁▁▁▁▁     |
| └───────────╌╌╌╌╌┴─────────────────────|
|  -3min 4s        -4.0s              now|"####),
    ("line tiers german", r####"|━ a                                     |
|7│                ┊     ⡏⠉⠉⠉⠉⢹          |
| │                ⠐⠒⠒⠒⠒⠒⠃    ⢸          |
| │     ⢠⠤⠤⠤⠤⠤     ┊          ⢸     ⡏⠉⠉⠉⠉|
|1│⠒⠒⠒⠒⠒⠚          ┊          ⢸⣀⣀⣀⣀⣀⡇    |
| └───────────╌╌╌╌╌┴─────────────────────|
|  -3Min 4s        -4,0s            jetzt|"####),
    ("line tiers index", r####"|7│                ┊     ⡏⠉⠉⠉⠉⢹          |
| │                ⠠⠤⠤⠤⠤⠤⠇    ⢸          |
| │                ┊          ⢸     ⡤⠤⠤⠤⠤|
| │     ⢰⠒⠒⠒⠒⠒     ┊          ⢸     ⡇    |
|1│⠉⠉⠉⠉⠉⠉          ┊          ⢸⣀⣀⣀⣀⣀⡇    |
| └───────────╌╌╌╌╌┴─────────────────────|
|  -3min 4s        -4.0s              now|"####),
    ("line tiers time", r####"|7│                                      |
| │                                     ⢹|
| │                                     ⢸|
| │            ⣰⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒            ⠈|
|1│⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠁                         |
| └─────────────────────────╌╌╌╌╌╌╌╌╌╌╌╌┴|
|  -3min 4s                           now|"####),
    ("line tiny", r####"|⣠⠞|"####),
    ("line two series", r####"|━ cpu  ╍ mem                                |
|7.0%│                                       |
|    │                                      ⢹|
|    │                                      ⢸|
|    │             ⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠄            ⣘|
|    │⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠼⡁⣀⡀⣀⡀⣀⡀⣀⡀⣀⡀⣀⡀            ⢸|
|0.5%│⠒⠂⠒⠂⠒⠂⠒⠂⠒⠂⠒⠂⠚⠁                         |
|    └──────────────────────────╌╌╌╌╌╌╌╌╌╌╌╌┴|
|     -3min 4s                            now|"####),
    ("line two series blocks", r####"|━ cpu  ┈ mem                                |
|10│                 ┊                       |
|  │                 ┊      ▂▂▂▂▂▂           |
|  │                 ┊▁▁▁▁▁▁                 |
|  │                 ┊      ▁ ▁ ▁       ▄▄▄▄▄|
|  │▂▂▂▂▂▂▇▇▇▇▇▇     ┊▅ ▅ ▅             ▂ ▂ ▂|
| 0│▆ ▆ ▆ █ █ █      ┊            ▆▆▆▆▆▆     |
|  └────────────╌╌╌╌╌┴───────────────────────|
|   -3min 4s         -4.0s                now|"####),
    ("spark ascii", r####"|.@?=|"####),
    ("spark band", r####"|   ░░░▁▃|
|░░▂▄▅▇██|
|▆███████|"####),
    ("spark band off", r####"|     ▁▅█|
|   ▃▆███|
|▁▄██████|"####),
    ("spark empty", r####"|········|"####),
    ("spark flat", r####"|▄▄▄▄▄▄▄▄|"####),
    ("spark gaps", r####"|▁▂··█▄·▅|"####),
    ("spark narrow", r####"|▂▄▇|"####),
    ("spark one point", r####"|▄▄▄▄▄▄|"####),
    ("spark spikes", r####"|▁▁▁█▁▁▁▁█▁▁▁|"####),
    ("spark tiers index", r####"|▂▂▂▂▃▃▃···▆▆▆▆███▁▁▁▅▅▅|"####),
    ("spark tiers time", r####"|▂▂▂▂▂▂▂▃▃▃▃▃▃▃▃·······▅|"####),
    ("spark zero width", r####"||"####),
    ("stat", r####"|memory         3.50GiB ▲|
|m… 3.50GiB|
|cpu         |
|-           |
|graph reads     |
|1.50k/s !       |
|Dauer     1Min 33s|
|duration      2.5s|
||"####),
];
