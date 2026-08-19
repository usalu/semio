//! 📊️ Output variable registration and time aggregation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// #region 🔖️Variable
/// 📈️ Reporting frequency for output variables.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportingFrequency {
    Timestep,
    Hourly,
    Daily,
    Monthly,
    RunPeriod,
    Annual,
}

/// 📈️ Aggregation method for reported values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Aggregation {
    Instantaneous,
    Average,
    Sum,
    Minimum,
    Maximum,
}

/// 📈️ Registered output variable descriptor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputVariable {
    pub key: String,
    pub unit: crate::units::Unit,
    pub frequency: ReportingFrequency,
    pub aggregation: Aggregation,
}

/// 📦️ Variable registry for sparse reporting.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OutputRegistry {
    pub variables: Vec<OutputVariable>,
}

impl OutputRegistry {
    pub async fn register(&mut self, var: OutputVariable) {
        self.variables.push(var);
    }

    pub async fn matches_wildcard(&self, pattern: &str) -> Vec<&OutputVariable> {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeries {
    pub key: String,
    pub timestamps_hours: Vec<f64>,
    pub values: Vec<f64>,
    pub unit: crate::units::Unit,
}

impl TimeSeries {
    pub async fn push(&mut self, t_hours: f64, value: f64) {
        self.timestamps_hours.push(t_hours);
        self.values.push(value);
    }

    pub async fn average(&self) -> f64 {
        if self.values.is_empty() {
            return 0.0;
        }
        self.values.iter().sum::<f64>() / self.values.len() as f64
    }

    pub async fn sum(&self) -> f64 {
        self.values.iter().sum()
    }

    pub async fn min_max(&self) -> (f64, f64) {
        let min = self.values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = self.values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        (min, max)
    }
}

/// 📦️ All time-series output.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesTable {
    pub series: HashMap<String, TimeSeries>,
}

impl TimeSeriesTable {
    pub async fn record(&mut self, key: impl Into<String>, t_hours: f64, value: f64, unit: crate::units::Unit) {
        let key = key.into();
        let entry = self.series.entry(key.clone()).or_insert_with(|| TimeSeries { key, timestamps_hours: Vec::new(), values: Vec::new(), unit });
        entry.push(t_hours, value);
    }

    pub async fn get(&self, key: &str) -> Option<&TimeSeries> {
        self.series.get(key)
    }

    pub async fn to_csv(&self, key: &str) -> Option<String> {
        let ts = self.series.get(key)?;
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
    async fn time_series_average() {
        let mut ts = TimeSeries { key: "t".into(), timestamps_hours: Vec::new(), values: Vec::new(), unit: crate::units::Unit::Celsius };
        ts.push(0.0, 10.0);
        ts.push(1.0, 20.0);
        assert!((ts.average() - 15.0).abs() < 1e-9);
    }

    #[test]
    async fn time_series_sum_and_min_max() {
        let mut ts = TimeSeries { key: "t".into(), timestamps_hours: Vec::new(), values: Vec::new(), unit: crate::units::Unit::Watts };
        ts.push(0.0, 5.0);
        ts.push(1.0, -3.0);
        ts.push(2.0, 8.0);
        assert!((ts.sum() - 10.0).abs() < 1e-9);
        let (min, max) = ts.min_max();
        assert!((min - -3.0).abs() < 1e-9);
        assert!((max - 8.0).abs() < 1e-9);
    }

    #[test]
    async fn time_series_average_of_empty_is_zero() {
        let ts = TimeSeries { key: "t".into(), timestamps_hours: Vec::new(), values: Vec::new(), unit: crate::units::Unit::Watts };
        assert_eq!(ts.average(), 0.0);
    }

    #[test]
    async fn registry_matches_exact_and_wildcard() {
        let mut reg = OutputRegistry::default();
        reg.register(OutputVariable { key: "Zone1 Temp".into(), unit: crate::units::Unit::Celsius, frequency: ReportingFrequency::Hourly, aggregation: Aggregation::Average });
        reg.register(OutputVariable { key: "Zone2 Temp".into(), unit: crate::units::Unit::Celsius, frequency: ReportingFrequency::Hourly, aggregation: Aggregation::Average });
        assert_eq!(reg.matches_wildcard("Zone1 Temp").len(), 1);
        assert_eq!(reg.matches_wildcard("Zone*").len(), 2);
        assert_eq!(reg.matches_wildcard("Nope").len(), 0);
    }

    #[test]
    async fn store_record_get_and_csv() {
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
}
