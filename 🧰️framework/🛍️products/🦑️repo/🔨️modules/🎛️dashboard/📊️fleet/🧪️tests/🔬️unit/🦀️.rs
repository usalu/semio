use super::model::{group_table, overview_ids, Freshness, HostName, Policy, Range, Request, SeriesKind, Snapshot, StoreState};
use super::store::{fold_columns, metric_label, metric_supported, metric_unit, origins_directory, read_snapshot, read_snapshot_with, thin, Bucket, Reader};
use std::path::{Path, PathBuf};

const AT: u64 = 1_800_000_305_000;

fn here() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🧪️tests/🧫️fixtures") }

fn store() -> PathBuf { here().join("🏪️store/origins") }

fn metrics_golden() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../../../../🔨️modules/🏃️process/🏗️build/📈️metrics/🧪️tests/🧫️fixtures/🏅️golden") }

fn request(range: Range, columns: u16, origin: Option<&str>) -> Request { Request { range, columns, origin: origin.map(str::to_string) } }

fn scratch(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("fleet-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).expect("scratch");
    path
}

fn golden_check(name: &str, snapshot: &Snapshot) {
    let path = here().join("🏅️golden").join(name);
    let text = serde_json::to_string_pretty(snapshot).expect("snapshot json") + "\n";
    if std::env::var("FLEET_GOLDEN_WRITE").as_deref() == Ok("1") { std::fs::write(&path, &text).expect("write golden"); }
    let expected: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("{name} missing; rerun with FLEET_GOLDEN_WRITE=1"))).expect("golden json");
    assert_eq!(serde_json::to_value(snapshot).expect("value"), expected, "{name} drifted");
}

#[test]
fn ranges_and_policy_come_from_the_schema() {
    let ids: Vec<&str> = Range::ALL.iter().map(|range| range.id()).collect();
    assert_eq!(ids, ["5m", "1h", "24h", "7d", "90d"]);
    assert_eq!(Range::Days90.span_ms(), 90 * 86_400_000);
    assert!(Range::ALL.windows(2).all(|pair| pair[0].span_ms() < pair[1].span_ms() && pair[0].refresh_ms() <= pair[1].refresh_ms() && pair[0].refresh_ms() >= 1000));
    assert_eq!((Range::Hour1.label("en"), Range::Hour1.label("de"), Range::Hour1.label("fr")), ("1 h", "1 Std.", "1h"));
    assert_eq!((Range::Days90.coarser(), Range::Minutes5.finer(), Range::Minutes5.coarser()), (Range::Days90, Range::Minutes5, Range::Hour1));
    let policy = Policy::get();
    assert!(policy.minimum_columns < policy.overview_columns && policy.overview_columns < policy.maximum_columns && policy.heartbeat_ms >= 1000);
}

#[test]
fn requests_are_validated_and_clamped() {
    assert!(request(Range::Hour1, 40, Some("g1-m01")).validate().is_ok());
    for bad in [request(Range::Hour1, 7, None), request(Range::Hour1, 241, None), request(Range::Hour1, 40, Some("")), request(Range::Hour1, 40, Some("../x")), request(Range::Hour1, 40, Some("a/b")), request(Range::Hour1, 40, Some(".hidden"))] {
        assert!(bad.validate().is_err(), "{bad:?}");
    }
    assert_eq!(request(Range::Hour1, 3, None).clamped().columns, 8);
    assert_eq!(request(Range::Hour1, 900, None).clamped().columns, 240);
    let json = serde_json::to_string(&request(Range::Days7, 24, Some("g1-m01"))).unwrap();
    assert_eq!(json, r#"{"range":"7d","columns":24,"origin":"g1-m01"}"#);
    assert!(serde_json::from_str::<Request>(r#"{"range":"7d","columns":24,"extra":1}"#).is_err());
}

#[test]
fn every_declared_metric_is_in_the_catalogue_with_both_languages() {
    let mut ids: Vec<String> = overview_ids().into_values().collect();
    for (_, label, metrics) in group_table() {
        assert!(label.contains_key("en") && label.contains_key("de"));
        ids.extend(metrics);
    }
    for id in ids {
        assert!(metric_unit(&id).is_some(), "{id} is not in the metrics catalogue");
        assert!(metric_label(&id, "en").is_some() && metric_label(&id, "de").is_some(), "{id} lacks a label");
    }
    assert_eq!(metric_label("host.cpu.busy", "en"), Some("CPU busy share"));
}

#[test]
fn the_store_root_comes_from_the_configuration_and_never_from_a_constant() {
    let workspace = scratch("root");
    assert_eq!(origins_directory(&workspace, Some("/srv/store"), None), Some(PathBuf::from("/srv/store/v1/origins")));
    assert_eq!(origins_directory(&workspace, Some(""), None), None);
    std::fs::create_dir_all(workspace.join(".cache/f")).unwrap();
    std::fs::write(workspace.join(".cache/f/machine.json"), r#"{"version":1,"originId":"g1-server","storeRoot":"/Volumes/semio-store"}"#).unwrap();
    assert_eq!(origins_directory(&workspace, None, None), Some(PathBuf::from("/Volumes/semio-store/v1/origins")));
    let moved = scratch("moved");
    std::fs::create_dir_all(moved.join("f")).unwrap();
    std::fs::write(moved.join("f/machine.json"), r#"{"storeRoot":"S:\\"}"#).unwrap();
    assert_eq!(origins_directory(&workspace, None, Some(&moved)), Some(PathBuf::from("S:\\").join("v1").join("origins")));
    assert_eq!(origins_directory(&scratch("empty"), None, None), None);
}

#[test]
fn a_missing_store_is_unavailable_not_empty() {
    let none = read_snapshot(None, &request(Range::Minutes5, 16, None), AT);
    assert_eq!((none.store, none.rows.len()), (StoreState::Unavailable, 0));
    let absent = read_snapshot(Some(Path::new("/definitely/not/a/store")), &request(Range::Minutes5, 16, None), AT);
    assert_eq!(absent.store, StoreState::Unavailable);
}

#[test]
fn the_synthetic_fleet_shows_live_stale_and_missing_origins() {
    let snapshot = read_snapshot(Some(&store()), &request(Range::Minutes5, 16, None), AT);
    assert_eq!(snapshot.store, StoreState::Available);
    let origins: Vec<(&str, Freshness, HostName)> = snapshot.rows.iter().map(|row| (row.origin.as_str(), row.freshness, row.host_name)).collect();
    assert_eq!(origins, [("g1-m01", Freshness::Live, HostName::Ok), ("g1-m02", Freshness::Stale, HostName::Warning), ("g1-m03", Freshness::Missing, HostName::Ok)]);
    let live = snapshot.row("g1-m01").unwrap();
    assert!(live.cpu.is_some() && live.memory.is_some() && live.disk.is_some() && live.units_per_minute.is_some());
    assert!(live.local_hit > live.local_miss && live.cpu_series.len() > 4 && live.units_series.len() > 4);
    assert!(live.cpu_series.iter().all(|point| point[2] <= point[4] && point[4] <= point[3]));
    assert!(snapshot.rows.iter().all(|row| row.offload_in.is_none()), "offloads into the fleet are a fleet fact, never a per-origin one");
    let totals = snapshot.fleet.as_ref().expect("fleet totals from the fleet rings");
    assert!(totals.offload_in.is_some() && totals.queue_running.is_some() && totals.remote_entries.is_some() && totals.remote_bytes.is_some());
    let missing = snapshot.row("g1-m03").unwrap();
    assert!(missing.ready.is_none() && missing.cpu.is_none() && missing.cpu_series.is_empty() && missing.metrics_ms.is_none());
    let stale = snapshot.row("g1-m02").unwrap();
    assert!(stale.cpu.is_some() && stale.metrics_ms.unwrap() < AT - 100_000);
    assert_eq!(stale.duplicates, 1);
}

#[test]
fn no_address_and_no_os_host_name_travels() {
    let text = serde_json::to_string(&read_snapshot(Some(&store()), &request(Range::Hour1, 24, Some("g1-m01")), AT)).unwrap();
    for word in ["address", "hostLabel", "\"host\":", "192.168", ".lan", ".fritz"] { assert!(!text.contains(word), "{word} leaked into the snapshot"); }
}

#[test]
fn goldens_pin_the_overview_of_every_range_and_a_drill_down() {
    for (name, range, columns) in [("overview-5m.json", Range::Minutes5, 16), ("overview-1h.json", Range::Hour1, 16), ("overview-90d.json", Range::Days90, 16)] {
        golden_check(name, &read_snapshot(Some(&store()), &request(range, columns, None), AT));
    }
    let detail = read_snapshot(Some(&store()), &request(Range::Hour1, 24, Some("g1-m01")), AT);
    assert_eq!(detail.rows.len(), 1);
    golden_check("detail-g1-m01-1h.json", &detail);
    let groups: Vec<&str> = detail.detail.as_ref().unwrap().groups.iter().map(|group| group.id.as_str()).collect();
    assert_eq!(groups, ["host", "build", "cache", "exec", "graph", "ready"]);
    let build = &detail.detail.as_ref().unwrap().groups[1];
    assert!(build.series.iter().any(|series| series.id == "build.units.compiled" && series.kind == SeriesKind::Counter));
    assert!(detail.detail.as_ref().unwrap().groups[4].series.is_empty());
    let unknown = read_snapshot(Some(&store()), &request(Range::Hour1, 24, Some("g1-m09")), AT);
    assert!(unknown.rows.is_empty() && unknown.detail.is_none());
}

#[test]
fn the_overview_equals_the_m2_golden_series_through_the_same_reader() {
    let origins = scratch("m2");
    let metrics = origins.join("g1-gold/metrics");
    std::fs::create_dir_all(&metrics).unwrap();
    for tier in 0..3 { std::fs::copy(metrics_golden().join(format!("t{tier}.ring")), metrics.join(format!("t{tier}.ring"))).unwrap(); }
    let snapshot = read_snapshot(Some(&origins), &request(Range::Minutes5, 16, None), 1_800_000_125_000);
    let row = &snapshot.rows[0];
    let series = std::fs::read_to_string(metrics_golden().join("series.tsv")).unwrap();
    let block = series.split("# ").find(|block| block.starts_with("all\n")).unwrap();
    let cpu_last: f64 = block.lines().filter(|line| line.starts_with("host.cpu.busy\t")).next_back().unwrap().split('\t').nth(8).unwrap().parse().unwrap();
    assert_eq!(row.cpu, Some(cpu_last));
    assert!(row.units_per_minute.is_some_and(|rate| rate > 0.0));
    assert_eq!(row.memory, None);
    assert_eq!(row.freshness, Freshness::Stale);
}

#[test]
fn a_ready_origin_without_rings_falls_back_to_its_memory_record() {
    let origins = scratch("fallback");
    std::fs::create_dir_all(origins.join("g1-m05")).unwrap();
    std::fs::write(origins.join("g1-m05/ready.json"), r#"{"state":"ready","ready":true,"stage":null,"stageIndex":0,"stageCount":5,"updatedMs":1800000300000,"refusal":{"code":"disk-collecting","en":"x","de":"y"}}"#).unwrap();
    std::fs::write(origins.join("g1-m05/memory.json"), r#"{"host":{"totalMegabytes":16000,"availableMegabytes":4000}}"#).unwrap();
    std::fs::write(origins.join("stray.json"), "{}").unwrap();
    let snapshot = read_snapshot(Some(&origins), &request(Range::Hour1, 16, None), AT);
    assert_eq!(snapshot.rows.len(), 1);
    let row = &snapshot.rows[0];
    assert_eq!((row.freshness, row.memory, row.cpu, row.host_name), (Freshness::Live, Some(75.0), None, HostName::Unknown));
    assert_eq!(row.ready.as_ref().unwrap().refusal.as_deref(), Some("disk-collecting"));
    assert_eq!(row.ready.as_ref().unwrap().beat_ms, Policy::get().default_beat_ms);
}

#[test]
fn a_torn_or_foreign_file_never_fails_the_snapshot() {
    let origins = scratch("torn");
    std::fs::create_dir_all(origins.join("g1-m06/metrics")).unwrap();
    std::fs::write(origins.join("g1-m06/ready.json"), "{not json").unwrap();
    std::fs::write(origins.join("g1-m06/metrics/t0.ring"), b"garbage").unwrap();
    let snapshot = read_snapshot(Some(&origins), &request(Range::Hour1, 16, None), AT);
    assert_eq!((snapshot.rows.len(), snapshot.rows[0].freshness), (1, Freshness::Missing));
}

fn bucket(start: u64, width: u64, tier: u16, value: f64) -> Bucket { Bucket { start, width, tier, min: value, max: value, mean: value, last: value } }

#[test]
fn folding_keeps_min_max_weighted_mean_and_last_and_leaves_gaps_out() {
    let rows = [Bucket { start: 0, width: 1000, tier: 0, min: 1.0, max: 9.0, mean: 4.0, last: 5.0 }, Bucket { start: 1000, width: 1000, tier: 0, min: 0.0, max: 2.0, mean: 2.0, last: 2.0 }, bucket(5000, 1000, 0, 7.0)];
    let columns = fold_columns(&rows, 0, 6000, 3);
    assert_eq!(columns.len(), 2);
    assert_eq!(columns[0], [0.0, 2000.0, 0.0, 9.0, 3.0, 2.0]);
    assert_eq!(columns[1], [4000.0, 2000.0, 7.0, 7.0, 7.0, 7.0]);
}

#[test]
fn thinning_keeps_tier_boundaries_and_the_newest_buckets() {
    let mut rows: Vec<Bucket> = (0..40).map(|index| bucket(index * 60_000, 60_000, 2, index as f64)).collect();
    rows.extend((0..90).map(|index| bucket(2_400_000 + index * 10_000, 10_000, 1, 100.0 + index as f64)));
    rows.extend((0..30).map(|index| bucket(3_300_000 + index * 1000, 1000, 0, 200.0 + index as f64)));
    let points = thin(&rows, 24);
    assert!(points.len() < rows.len() && points.len() >= 3 * Policy::get().minimum_run_columns.min(6));
    let widths: Vec<u64> = points.iter().map(|point| point[1] as u64).collect();
    let boundaries = widths.windows(2).filter(|pair| pair[0] != pair[1]).count();
    assert_eq!(boundaries, 2, "{widths:?}");
    assert_eq!(points.last().unwrap()[5], 229.0);
    assert!(points.windows(2).all(|pair| pair[0][0] + pair[0][1] <= pair[1][0] + 1.0));
    let few = thin(&rows[..4], 24);
    assert_eq!(few.len(), 4);
}

#[test]
fn snapshots_round_trip_and_compare_by_content() {
    let first = read_snapshot(Some(&store()), &request(Range::Hour1, 16, None), AT);
    let second = read_snapshot(Some(&store()), &request(Range::Hour1, 16, None), AT + 1);
    assert_ne!(first.at_ms, second.at_ms);
    assert!(first.same_content(&second) || first.to_ms != second.to_ms);
    let text = serde_json::to_string(&first).unwrap();
    assert!(text.contains("\"atMs\"") && text.contains("\"cpuSeries\""));
    assert_eq!(serde_json::from_str::<Snapshot>(&text).unwrap(), first);
}

#[test]
fn a_metric_a_platform_cannot_measure_is_not_supported_there() {
    for id in ["host.cpu.load", "host.swap.used", "host.swap.rate", "host.files.open", "host.memory.compressed", "host.images.attached", "warden.headroom", "process.repo.cpu"] {
        assert!(!metric_supported(id, "win32") && !metric_supported(id, "windows"), "{id} is n/a on Windows");
    }
    assert!(metric_supported("host.cpu.load", "linux") && metric_supported("host.cpu.load", "macos"));
    assert!(!metric_supported("host.memory.compressed", "linux") && metric_supported("host.memory.compressed", "darwin"));
    assert!(metric_supported("host.cpu.busy", "windows") && metric_supported("no.such.metric", "windows"));
}

fn copy_origin(from: &str, to_origins: &Path, to: &str, platform: &str) {
    let source = store().join(from);
    let target = to_origins.join(to);
    std::fs::create_dir_all(target.join("metrics")).unwrap();
    for file in std::fs::read_dir(source.join("metrics")).unwrap().flatten() { std::fs::copy(file.path(), target.join("metrics").join(file.file_name())).unwrap(); }
    let ready = std::fs::read_to_string(source.join("ready.json")).unwrap().replace("\"macos\"", &format!("\"{platform}\""));
    std::fs::write(target.join("ready.json"), ready).unwrap();
}

#[test]
fn a_metric_the_platform_cannot_measure_is_an_unsupported_series_not_a_gap() {
    let origins = scratch("unsupported");
    copy_origin("g1-m01", &origins, "g1-m01", "macos");
    copy_origin("g1-m01", &origins, "g1-m05", "windows");
    let detail = |origin: &str| read_snapshot(Some(&origins), &request(Range::Hour1, 40, Some(origin)), AT).detail.expect("a drill-down");
    let unsupported = |origin: &str| -> Vec<String> { detail(origin).groups.iter().flat_map(|group| group.series.iter()).filter(|series| series.unsupported).map(|series| series.id.clone()).collect() };
    assert_eq!(unsupported("g1-m05"), ["host.cpu.load", "host.swap.used"], "the host metrics of the group that Windows has no source for");
    assert!(unsupported("g1-m01").is_empty(), "a Mac measures all of them");
    let windows = detail("g1-m05");
    let load = windows.groups.iter().flat_map(|group| group.series.iter()).find(|series| series.id == "host.cpu.load").unwrap();
    assert!(load.points.is_empty(), "no points are invented for an unsupported metric");
    assert!(windows.groups.iter().flat_map(|group| group.series.iter()).any(|series| series.id == "host.cpu.busy" && !series.unsupported && !series.points.is_empty()), "supported metrics keep their data");
    let mac_load = detail("g1-m01").groups.iter().flat_map(|group| group.series.iter()).any(|series| series.id == "host.cpu.load" && !series.points.is_empty());
    assert!(mac_load);
    let json = serde_json::to_value(load).unwrap();
    assert_eq!(json["unsupported"], true);
    assert!(serde_json::to_value(detail("g1-m01").groups[0].series[0].clone()).unwrap().get("unsupported").is_none(), "the flag travels only when set");
    let overview = read_snapshot(Some(&origins), &request(Range::Hour1, 16, None), AT);
    assert!(overview.rows.iter().all(|row| row.unsupported.is_empty()), "every overview metric is measurable on both platforms");
    assert_eq!(overview.row("g1-m05").and_then(|row| row.cpu), overview.row("g1-m01").and_then(|row| row.cpu));
}

#[test]
fn an_overview_metric_unsupported_on_the_platform_is_listed_and_never_read() {
    let origins = scratch("unsupported-overview");
    copy_origin("g1-m01", &origins, "g1-m01", "linux");
    let mut catalogue_probe = overview_ids();
    catalogue_probe.retain(|_, id| !metric_supported(id, "linux"));
    assert!(catalogue_probe.is_empty(), "the overview of today reads only metrics every platform has: {catalogue_probe:?}");
    let snapshot = read_snapshot(Some(&origins), &request(Range::Hour1, 16, None), AT);
    assert!(snapshot.rows[0].unsupported.is_empty() && snapshot.rows[0].cpu.is_some());
}

#[test]
fn a_remembered_row_is_reused_until_a_file_behind_it_changes_or_it_gets_old() {
    let origins = scratch("remember");
    copy_origin("g1-m01", &origins, "g1-m01", "macos");
    let ready = origins.join("g1-m01/ready.json");
    let platform = |snapshot: &Snapshot| snapshot.rows[0].ready.as_ref().and_then(|ready| ready.platform.clone());
    let mut reader = Reader::default();
    let ask = request(Range::Hour1, 16, None);
    let first = read_snapshot_with(&mut reader, Some(&origins), &ask, AT);
    assert_eq!(platform(&first), Some("macos".into()));
    let (stamp, original) = (std::fs::metadata(&ready).unwrap().modified().unwrap(), std::fs::read_to_string(&ready).unwrap());
    std::fs::write(&ready, original.replace("macos", "linux")).unwrap();
    std::fs::File::options().write(true).open(&ready).unwrap().set_modified(stamp).unwrap();
    let again = read_snapshot_with(&mut reader, Some(&origins), &ask, AT + 1000);
    assert_eq!(platform(&again), Some("macos".into()), "nothing changed by modification time and size: the remembered row answers");
    assert_eq!(again.rows[0].cpu_series, first.rows[0].cpu_series);
    assert_eq!(platform(&read_snapshot(Some(&origins), &ask, AT + 1000)), Some("linux".into()), "a reader that remembers nothing sees the file");
    let old = read_snapshot_with(&mut reader, Some(&origins), &ask, AT + Policy::get().remember_ms + 1);
    assert_eq!(platform(&old), Some("linux".into()), "a row older than rememberMs is read again");
    std::fs::write(&ready, original).unwrap();
    assert_eq!(platform(&read_snapshot_with(&mut reader, Some(&origins), &ask, AT + Policy::get().remember_ms + 2)), Some("macos".into()), "a changed file is read at once");
    let later = read_snapshot_with(&mut reader, Some(&origins), &ask, AT + 40 * 3_600_000);
    assert_eq!(later.rows[0].freshness, Freshness::Stale, "freshness is judged again at every read, also for a remembered row");
    let drill = read_snapshot_with(&mut reader, Some(&origins), &request(Range::Hour1, 40, Some("g1-m01")), AT);
    assert!(drill.rows[0].cpu_series.len() > first.rows[0].cpu_series.len(), "another column budget is another window of the row");
    std::fs::remove_dir_all(origins.join("g1-m01")).unwrap();
    assert!(read_snapshot_with(&mut reader, Some(&origins), &ask, AT).rows.is_empty(), "a vanished origin is gone, not remembered");
}

fn cpu_ms() -> f64 {
    let output = std::process::Command::new("ps").args(["-o", "time=", "-p", &std::process::id().to_string()]).output().expect("ps");
    String::from_utf8_lossy(&output.stdout).trim().split(':').fold(0.0, |total, part| total * 60.0 + part.parse::<f64>().unwrap_or(0.0)) * 1000.0
}

/// 📏️ Measures what the dashboard data path costs the daemon with the REAL shared store: per range, one overview read and one drill-down read, how long it takes (wall time and process CPU over many reads), how many bytes the snapshot has on the wire (JSON) and what that is per second at the refresh time of the range. Not part of the suite: `FLEET_LIVE_OUT=<directory> cargo test live_overhead -- --ignored --nocapture`.
#[test]
#[ignore = "reads the real shared store of this machine"]
fn live_overhead() {
    let mut workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !workspace.join(".cache/f/machine.json").is_file() { assert!(workspace.pop(), "no machine.json above the crate"); }
    let origins = origins_directory(&workspace, std::env::var("SEMIO_FLEET_STORE_ROOT").ok().as_deref(), None).expect("a configured store");
    let now = || std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
    let origin = std::env::var("FLEET_LIVE_ORIGIN").unwrap_or_else(|_| "g1-m15".into());
    let mut text = String::from("| range | request | origins | wire bytes | CPU ms/refresh, nothing remembered | CPU ms/refresh, remembered (steady) | wall ms/refresh (steady) | refresh | daemon CPU % of one core (steady) | wire bytes/s |\n|---|---|---|---|---|---|---|---|---|---|\n");
    for range in Range::ALL {
        for (label, columns, drill) in [("overview", 16u16, None), ("drill-down", 120, Some(origin.as_str()))] {
            let request = request(range, columns, drill);
            let _ = read_snapshot(Some(&origins), &request, now());
            let cold_reads = 10;
            let before = cpu_ms();
            for _ in 0..cold_reads { let _ = read_snapshot(Some(&origins), &request, now()); }
            let cold = (cpu_ms() - before) / f64::from(cold_reads);
            let refresh = range.refresh_ms() as f64;
            let pause = std::time::Duration::from_millis(range.refresh_ms().min(5000));
            let reads: u32 = if drill.is_some() { 6 } else { match range { Range::Minutes5 => 12, Range::Hour1 => 6, _ => 3 } };
            let mut reader = Reader::default();
            let _ = read_snapshot_with(&mut reader, Some(&origins), &request, now());
            let (mut cpu, mut wall, mut bytes, mut rows) = (0.0, 0.0, 0, 0);
            for _ in 0..reads {
                std::thread::sleep(pause);
                let (before, started) = (cpu_ms(), std::time::Instant::now());
                let snapshot = read_snapshot_with(&mut reader, Some(&origins), &request, now());
                wall += started.elapsed().as_secs_f64() * 1000.0;
                cpu += cpu_ms() - before;
                bytes = serde_json::to_vec(&snapshot).unwrap().len() + 1;
                rows = snapshot.rows.len();
            }
            let (cpu, wall) = (cpu / f64::from(reads), wall / f64::from(reads));
            text += &format!("| {} | {label} | {rows} | {bytes} | {cold:.1} | {cpu:.1} | {wall:.1} | {} s | {:.2} | {:.0} |\n", range.id(), refresh / 1000.0, cpu / refresh * 100.0, bytes as f64 / refresh * 1000.0);
        }
    }
    let names: Vec<String> = overview_ids().into_values().filter(|id| !id.starts_with("exec.queue") && id != "exec.offload.in").collect();
    let dirs: Vec<PathBuf> = std::fs::read_dir(&origins).unwrap().flatten().map(|entry| entry.path().join("metrics")).filter(|path| path.is_dir()).collect();
    let to = now() / 1000 * 1000;
    let mut phase = |label: &str, body: &dyn Fn()| { body(); let (before, started) = (cpu_ms(), std::time::Instant::now()); for _ in 0..20 { body(); } text += &format!("\nphase {label}: wall {:.1} ms, CPU {:.1} ms per refresh ({} origins)", started.elapsed().as_secs_f64() * 50.0, (cpu_ms() - before) / 20.0, dirs.len()); };
    phase("raw reader 5 min, 10 overview ids", &|| for dir in &dirs { let _ = semio_framework_process_metrics::read_series(dir, to - 300_000, to, Some(&names)); });
    phase("raw reader 5 min, all ids", &|| for dir in &dirs { let _ = semio_framework_process_metrics::read_series(dir, to - 300_000, to, None); });
    phase("raw reader 1 h, 10 overview ids", &|| for dir in &dirs { let _ = semio_framework_process_metrics::read_series(dir, to - 3_600_000, to, Some(&names)); });
    match std::env::var("FLEET_LIVE_OUT") { Ok(directory) => { std::fs::create_dir_all(&directory).unwrap(); std::fs::write(PathBuf::from(directory).join("overhead.md"), &text).unwrap(); } Err(_) => {} }
    println!("{text}");
}
