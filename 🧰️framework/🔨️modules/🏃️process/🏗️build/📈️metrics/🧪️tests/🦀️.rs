//! 🧪️ The Rust reader reads the golden ring files written by the TypeScript writer into the series checked in as `🧫️fixtures/🏅️golden/series.tsv`, which the TypeScript reader and the Python oracle reproduce too.

use semio_framework_process_metrics::{read_series, MetricsError, Point, Series};
use std::{
    fs::{self, OpenOptions},
    io::{Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

fn golden() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🧪️tests/🧫️fixtures/🏅️golden")
}

fn queries() -> Vec<(String, u64, u64)> {
    fs::read_to_string(golden().join("queries.tsv"))
        .expect("queries")
        .lines()
        .map(|row| {
            let cells: Vec<&str> = row.split('\t').collect();
            (cells[0].to_owned(), cells[1].parse().expect("from"), cells[2].parse().expect("to"))
        })
        .collect()
}

fn expected() -> Vec<(String, Vec<(String, Point)>)> {
    let mut out: Vec<(String, Vec<(String, Point)>)> = Vec::new();
    for line in fs::read_to_string(golden().join("series.tsv")).expect("series").lines() {
        if let Some(name) = line.strip_prefix("# ") {
            out.push((name.to_owned(), Vec::new()));
            continue;
        }
        let cells: Vec<&str> = line.split('\t').collect();
        let point = Point {
            start: cells[1].parse().expect("start"),
            width: cells[2].parse().expect("width"),
            tier: cells[3].parse().expect("tier"),
            count: cells[4].parse().expect("count"),
            min: cells[5].parse().expect("min"),
            max: cells[6].parse().expect("max"),
            sum: cells[7].parse().expect("sum"),
            last: cells[8].parse().expect("last"),
            partial: cells[9] == "1",
        };
        out.last_mut().expect("block").1.push((cells[0].to_owned(), point));
    }
    out
}

fn flatten(series: &Series) -> Vec<(String, Point)> {
    series.metrics.iter().flat_map(|(id, points)| points.iter().map(move |point| (id.clone(), point.clone()))).collect()
}

fn copy_golden(name: &str) -> PathBuf {
    let target = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("metrics-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&target);
    fs::create_dir_all(&target).expect("target");
    for tier in 0..3 {
        fs::copy(golden().join(format!("t{tier}.ring")), target.join(format!("t{tier}.ring"))).expect("copy");
    }
    target
}

#[test]
fn every_golden_query_equals_the_checked_in_series() {
    let expected = expected();
    let queries = queries();
    assert_eq!(expected.len(), queries.len());
    assert!(expected.iter().map(|(_, rows)| rows.len()).sum::<usize>() > 40);
    for ((name, from, to), (expected_name, rows)) in queries.iter().zip(&expected) {
        assert_eq!(name, expected_name);
        let series = read_series(&golden(), *from, *to, None).expect("series");
        assert_eq!(&flatten(&series), rows, "query {name}");
    }
}

#[test]
fn a_merged_series_is_ordered_never_overlaps_and_gets_finer_towards_now() {
    let series = read_series(&golden(), 0, u64::MAX / 2, None).expect("series");
    let mut tiers = std::collections::BTreeSet::new();
    for (id, points) in &series.metrics {
        for pair in points.windows(2) {
            assert!(pair[0].start + pair[0].width <= pair[1].start, "{id} overlaps");
            assert!(pair[0].tier >= pair[1].tier, "{id} is finer in the past");
        }
        tiers.extend(points.iter().map(|point| point.tier));
    }
    assert_eq!(tiers.len(), 3);
    assert!(series.metrics.iter().flat_map(|(_, points)| points).any(|point| point.partial));
}

#[test]
fn ids_narrow_the_metrics_and_unknown_ids_and_empty_ranges_are_refused_in_both_languages() {
    let ids = vec!["ready.stage".to_owned()];
    let all = read_series(&golden(), 0, u64::MAX / 2, None).expect("all");
    let narrow = read_series(&golden(), 0, u64::MAX / 2, Some(&ids)).expect("narrow");
    assert_eq!(narrow.metrics.len(), 1);
    assert_eq!(narrow.metrics[0], *all.metrics.iter().find(|(id, _)| id == "ready.stage").expect("stage"));
    let unknown = read_series(&golden(), 0, 10, Some(&["x.y".to_owned()])).expect_err("unknown");
    assert!(matches!(unknown, MetricsError::Unknown(_)));
    assert!(unknown.to_string().contains(" / ") && unknown.to_string().contains("Kennungen"));
    let reversed = read_series(&golden(), 10, 10, None).expect_err("empty");
    assert!(reversed.to_string().contains("leer"));
    let absent = read_series(Path::new("/nonexistent-metrics-directory"), 0, 10, None).expect("absent");
    assert!(absent.metrics.is_empty());
}

#[test]
fn a_torn_row_reads_as_absent() {
    let directory = copy_golden("torn");
    let whole = read_series(&directory, 0, u64::MAX / 2, None).expect("whole");
    let newest = whole.metrics.iter().flat_map(|(_, points)| points).filter(|point| point.tier == 0).map(|point| point.start).max().expect("newest");
    let bytes = fs::read(directory.join("t0.ring")).expect("t0");
    let data_offset = u32::from_le_bytes(bytes[28..32].try_into().expect("offset")) as u64;
    let buckets = u32::from_le_bytes(bytes[12..16].try_into().expect("buckets")) as u64;
    let row_bytes = u32::from_le_bytes(bytes[24..28].try_into().expect("row")) as u64;
    let slot = data_offset + (newest / 1000 % buckets) * row_bytes;
    let mut file = OpenOptions::new().write(true).open(directory.join("t0.ring")).expect("open");
    file.seek(SeekFrom::Start(slot + row_bytes - 8)).expect("seek");
    file.write_all(&[0xab; 8]).expect("tear");
    let torn = read_series(&directory, 0, u64::MAX / 2, None).expect("torn");
    assert!(!flatten(&torn).iter().any(|(_, point)| point.tier == 0 && point.start == newest));
    assert!(flatten(&torn).iter().any(|(_, point)| point.tier == 0));
    file.seek(SeekFrom::Start(40)).expect("seek head");
    file.write_all(&[0; 8]).expect("clear head");
    assert_eq!(flatten(&read_series(&directory, 0, u64::MAX / 2, None).expect("scan")), flatten(&torn));
    let _ = fs::remove_dir_all(&directory);
}
