
use super::*;

#[test]
fn time_series_average() {
    let mut ts = TimeSeries { key: "t".into(), timestamps_hours: Vec::new(), values: Vec::new(), unit: crate::units::Unit::Celsius, admitted_samples: 0 };
    ts.push(0.0, 10.0);
    ts.push(1.0, 20.0);
    assert!((ts.average() - 15.0).abs() < 1e-9);
}

#[test]
fn time_series_sum_and_min_max() {
    let mut ts = TimeSeries { key: "t".into(), timestamps_hours: Vec::new(), values: Vec::new(), unit: crate::units::Unit::Watts, admitted_samples: 0 };
    ts.push(0.0, 5.0);
    ts.push(1.0, -3.0);
    ts.push(2.0, 8.0);
    assert!((ts.sum() - 10.0).abs() < 1e-9);
    let (min, max) = ts.min_max();
    assert!((min - -3.0).abs() < 1e-9);
    assert!((max - 8.0).abs() < 1e-9);
}

#[test]
fn time_series_average_of_empty_is_zero() {
    let ts = TimeSeries { key: "t".into(), timestamps_hours: Vec::new(), values: Vec::new(), unit: crate::units::Unit::Watts, admitted_samples: 0 };
    assert_eq!(ts.average(), 0.0);
}

#[test]
fn registry_matches_exact_and_wildcard() {
    let mut reg = OutputRegistry::default();
    reg.register(OutputVariable { key: "Zone1 Temp".into(), unit: crate::units::Unit::Celsius, frequency: ReportingFrequency::Hourly, aggregation: Aggregation::Average });
    reg.register(OutputVariable { key: "Zone2 Temp".into(), unit: crate::units::Unit::Celsius, frequency: ReportingFrequency::Hourly, aggregation: Aggregation::Average });
    assert_eq!(reg.matches_wildcard("Zone1 Temp").len(), 1);
    assert_eq!(reg.matches_wildcard("Zone*").len(), 2);
    assert_eq!(reg.matches_wildcard("Nope").len(), 0);
}

#[test]
fn store_record_get_and_csv() {
    let mut store = TimeSeriesTable::default();
    store.record("Zone1 Temp", 0.0, 21.0, crate::units::Unit::Celsius);
    store.record("Zone1 Temp", 1.0, 22.0, crate::units::Unit::Celsius);
    let series = store.get("Zone1 Temp").unwrap();
    assert_eq!(series.values.len(), 2);
    let csv = store.to_csv("Zone1 Temp").unwrap();
    assert!(csv.starts_with("hours,value\n"));
    assert!(csv.contains("21"));
    assert!(store.to_csv("Missing").is_none());
    assert!(store.get("Missing").is_none());
}

#[test]
fn admitted_append_never_grows_and_rejects_maximum_plus_one() {
    let mut unadmitted = TimeSeries { key: "unadmitted".into(), timestamps_hours: Vec::new(), values: Vec::new(), unit: crate::units::Unit::Celsius, admitted_samples: 1 };
    assert_eq!(unadmitted.append_admitted(0.0, 20.0), Err(TimeSeriesAppendError::BackingNotAdmitted));
    let mut mismatched_timestamps = Vec::new();
    let mut mismatched_values = Vec::new();
    mismatched_timestamps.try_reserve_exact(2).expect("mismatched timestamp admission");
    mismatched_values.try_reserve_exact(2).expect("mismatched value admission");
    mismatched_timestamps.push(0.0);
    let mut mismatched = TimeSeries { key: "mismatched".into(), timestamps_hours: mismatched_timestamps, values: mismatched_values, unit: crate::units::Unit::Celsius, admitted_samples: 2 };
    assert_eq!(mismatched.append_admitted(1.0, 21.0), Err(TimeSeriesAppendError::LengthMismatch));
    let mut timestamps = Vec::new();
    let mut values = Vec::new();
    timestamps.try_reserve_exact(2).expect("timestamp admission");
    values.try_reserve_exact(2).expect("value admission");
    let mut series = TimeSeries { key: "fixed".into(), timestamps_hours: timestamps, values, unit: crate::units::Unit::Celsius, admitted_samples: 2 };
    let timestamp_capacity = series.timestamps_hours.capacity();
    let value_capacity = series.values.capacity();
    assert_eq!(series.append_admitted(0.0, 20.0), Ok(()));
    assert_eq!(series.append_admitted(1.0, 21.0), Ok(()));
    let before = series.clone();
    assert_eq!(series.append_admitted(2.0, 22.0), Err(TimeSeriesAppendError::Full));
    assert_eq!(series, before);
    assert_eq!(series.timestamps_hours.capacity(), timestamp_capacity);
    assert_eq!(series.values.capacity(), value_capacity);
}
