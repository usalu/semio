//! 📊️ Output variable registration and time aggregation.

use crate::model::FixedTable;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

// #region 🔖️Variable
/// 📈️ Reporting frequency for output variables.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum ReportingFrequency {
    Timestep,
    Hourly,
    Daily,
    Monthly,
    RunPeriod,
    Annual,
}

/// 📈️ Aggregation method for reported values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum Aggregation {
    Instantaneous,
    Average,
    Sum,
    Minimum,
    Maximum,
}

/// 📈️ Registered output variable descriptor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct OutputVariable {
    pub key: String,
    pub unit: crate::units::Unit,
    pub frequency: ReportingFrequency,
    pub aggregation: Aggregation,
}

/// 📦️ Variable registry for sparse reporting.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct OutputRegistry {
    pub(crate) variables: Vec<OutputVariable>,
}

impl OutputRegistry {
    #[cfg(test)]
    pub(crate) fn register(&mut self, var: OutputVariable) {
        self.variables.push(var);
    }

    #[cfg(test)]
    pub(crate) fn matches_wildcard(&self, pattern: &str) -> Vec<&OutputVariable> {
        if pattern.contains('*') {
            let prefix = pattern.split('*').next().unwrap_or("");
            self.variables.iter().filter(|v| v.key.starts_with(prefix)).collect()
        } else {
            self.variables.iter().filter(|v| v.key == pattern).collect()
        }
    }
}
// #endregion 🔖️Variable

// #region 🔖️TimeSeries
/// 📈️ Time-series storage for one variable.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct TimeSeries {
    pub(crate) key: String,
    pub(crate) timestamps_hours: Vec<f64>,
    pub(crate) values: Vec<f64>,
    pub(crate) unit: crate::units::Unit,
    pub(crate) admitted_samples: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub(crate) enum TimeSeriesAppendError {
    BackingNotAdmitted,
    LengthMismatch,
    Full,
}

impl TimeSeries {
    pub(crate) fn append_admitted(&mut self, t_hours: f64, value: f64) -> Result<(), TimeSeriesAppendError> {
        if self.timestamps_hours.capacity() < self.admitted_samples || self.values.capacity() < self.admitted_samples {
            return Err(TimeSeriesAppendError::BackingNotAdmitted);
        }
        if self.timestamps_hours.len() != self.values.len() {
            return Err(TimeSeriesAppendError::LengthMismatch);
        }
        if self.values.len() >= self.admitted_samples {
            return Err(TimeSeriesAppendError::Full);
        }
        self.timestamps_hours.push(t_hours);
        self.values.push(value);
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn push(&mut self, t_hours: f64, value: f64) {
        self.timestamps_hours.push(t_hours);
        self.values.push(value);
    }

    #[cfg(test)]
    pub(crate) fn average(&self) -> f64 {
        if self.values.is_empty() {
            return 0.0;
        }
        self.values.iter().sum::<f64>() / self.values.len() as f64
    }

    #[cfg(test)]
    pub(crate) fn sum(&self) -> f64 {
        self.values.iter().sum()
    }

    #[cfg(test)]
    pub(crate) fn min_max(&self) -> (f64, f64) {
        let min = self.values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = self.values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        (min, max)
    }
}

/// 📦️ All time-series output.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct TimeSeriesTable {
    pub(crate) series: FixedTable<String, TimeSeries>,
}

impl TimeSeriesTable {
    #[cfg(test)]
    pub(crate) fn record(&mut self, key: impl Into<String>, t_hours: f64, value: f64, unit: crate::units::Unit) {
        let key = key.into();
        if self.series.capacity() == 0 {
            self.series.admit(64).expect("test time-series backing");
        }
        if let Some(index) = self.series.test_index_of(|candidate| candidate == &key) {
            self.series.get_index_mut(index).expect("test fixed series slot").push(t_hours, value);
            return;
        }
        self.series.insert_stable(key.clone(), TimeSeries { key, timestamps_hours: Vec::new(), values: Vec::new(), unit, admitted_samples: 0 }).expect("test fixed series slot");
        self.series.last_mut().expect("test fixed series slot").1.push(t_hours, value);
    }

    #[cfg(test)]
    pub(crate) fn get(&self, key: &str) -> Option<&TimeSeries> {
        self.series.iter().find(|(candidate, _)| candidate.as_str() == key).map(|(_, value)| value)
    }

    #[cfg(test)]
    pub(crate) fn to_csv(&self, key: &str) -> Option<String> {
        let ts = self.get(key)?;
        let mut out = String::from("hours,value\n");
        for (t, v) in ts.timestamps_hours.iter().zip(ts.values.iter()) {
            out.push_str(&format!("{t},{v}\n"));
        }
        Some(out)
    }
}
// #endregion 🔖️TimeSeries

#[cfg(test)]
mod tests {
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
}
