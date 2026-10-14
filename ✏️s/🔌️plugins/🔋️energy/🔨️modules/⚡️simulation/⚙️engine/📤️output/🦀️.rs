//! 📊️ Output variable registration and time aggregation.

extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_units as units;

use crate::model::FixedTable;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};

// #region 🔖️Variable
/// 📈️ Reporting frequency for output variables.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum ReportingFrequency {
    Timestep,
    Hourly,
    Daily,
    Monthly,
    RunPeriod,
    Annual,
}

/// 📈️ Aggregation method for reported values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum Aggregation {
    Instantaneous,
    Average,
    Sum,
    Minimum,
    Maximum,
}

/// 📈️ Registered output variable descriptor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct OutputVariable {
    pub key: String,
    pub unit: units::Unit,
    pub frequency: ReportingFrequency,
    pub aggregation: Aggregation,
}

/// 📦️ Variable registry for sparse reporting.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct OutputRegistry {
    pub variables: Vec<OutputVariable>,
}

impl OutputRegistry {
    #[cfg(test)]
    pub fn register(&mut self, var: OutputVariable) {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct TimeSeries {
    pub key: String,
    pub timestamps_hours: Vec<f64>,
    pub values: Vec<f64>,
    pub unit: units::Unit,
    pub admitted_samples: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum TimeSeriesAppendError {
    BackingNotAdmitted,
    LengthMismatch,
    Full,
}

impl TimeSeries {
    pub fn append_admitted(&mut self, t_hours: f64, value: f64) -> Result<(), TimeSeriesAppendError> {
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
    pub fn push(&mut self, t_hours: f64, value: f64) {
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
    pub fn sum(&self) -> f64 {
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct TimeSeriesTable {
    pub series: FixedTable<String, TimeSeries>,
}

impl TimeSeriesTable {
    #[cfg(test)]
    pub fn record(&mut self, key: impl Into<String>, t_hours: f64, value: f64, unit: crate::units::Unit) {
        let key = key.into();
        if self.series.capacity() == 0 {
            self.series.admit(64).expect("test time-series backing");
        }
        let found = self.series.iter().position(|(candidate, _)| candidate == &key);
        if let Some(index) = found {
            self.series.get_index_mut(index).expect("test fixed series slot").push(t_hours, value);
            return;
        }
        self.series.insert_stable(key.clone(), TimeSeries { key, timestamps_hours: Vec::new(), values: Vec::new(), unit, admitted_samples: 0 }).expect("test fixed series slot");
        self.series.last_mut().expect("test fixed series slot").1.push(t_hours, value);
    }

    pub fn get(&self, key: &str) -> Option<&TimeSeries> {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
