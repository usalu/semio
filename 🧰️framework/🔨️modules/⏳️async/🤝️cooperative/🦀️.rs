//#region 🧪️CooperativeMaintenance
use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture { cases: Vec<Case> }

#[derive(Deserialize)]
struct Case { lane: Lane, weight: u32, deficits: Vec<i64>, selected: Vec<bool> }

#[cfg(test)]
include!("🧪️tests/🔬️standalone/🦀️.rs");
//#endregion 🧪️CooperativeMaintenance
