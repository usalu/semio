//! 🎛️ Plant and equipment dispatch strategies.

use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

// #region 🔖️Dispatch
/// 🎛️ Equipment dispatch scheme.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum DispatchScheme {
    Sequential,
    Uniform,
    Optimal,
    UniformPartLoadRatio,
    LoadRange,
    OutdoorTemperature,
    ThermalStorage,
}

/// 🎛️ Equipment priority entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct EquipmentPriority {
    pub equipment_id: u32,
    pub priority: u32,
    pub min_runtime_hours: f64,
    pub capacity_w: f64,
}

/// 🎛️ Dispatch request for plant equipment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct DispatchRequest {
    pub total_load_w: f64,
    pub available_capacity_w: f64,
    pub outdoor_temp_c: f64,
}

/// 🎛️ Dispatch result per equipment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct DispatchResult {
    pub equipment_id: u32,
    pub load_w: f64,
    pub part_load_ratio: f64,
    pub runtime_fraction: f64,
}
// #endregion 🔖️Dispatch

// #region 🔖️Dispatcher
/// 🎛️ Plant equipment dispatcher.
pub struct Dispatcher {
    pub scheme: DispatchScheme,
    pub equipment: Vec<EquipmentPriority>,
}

impl Dispatcher {
    pub fn new(scheme: DispatchScheme, equipment: Vec<EquipmentPriority>) -> Self {
        Self { scheme, equipment }
    }

    /// 🎛️ Distribute load across equipment per dispatch scheme.
    #[cfg(test)]
    pub(crate) fn dispatch(&self, request: &DispatchRequest) -> Vec<DispatchResult> {
        let mut builder = DispatchBuilder::new(request.clone());
        while !builder.is_complete() {
            builder.step(self);
        }
        builder.finish().expect("test dispatch backing")
    }
}

/// 🎛️ Stable one-equipment-at-a-time dispatch cursor over pre-admitted input order.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
#[cfg(test)]
pub(crate) struct DispatchBuilder {
    request: DispatchRequest,
    stage: DispatchStage,
    cursor: usize,
    previous_priority: Option<u32>,
    total_capacity_w: f64,
    remaining_w: f64,
    results: Vec<DispatchResult>,
    fault: Option<DispatchFault>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
#[cfg(test)]
pub(crate) enum DispatchFault {
    ResultBacking,
    UnorderedPriority,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
#[cfg(test)]
pub(crate) enum DispatchStage {
    Reserve,
    ValidateOrder,
    AccumulateCapacity,
    Emit,
    Complete,
}

#[cfg(test)]
impl DispatchBuilder {
    pub(crate) fn new(request: DispatchRequest) -> Self {
        let remaining_w = request.total_load_w;
        Self { request, stage: DispatchStage::Reserve, cursor: 0, previous_priority: None, total_capacity_w: 0.0, remaining_w, results: Vec::new(), fault: None }
    }

    pub(crate) fn is_complete(&self) -> bool {
        self.stage == DispatchStage::Complete
    }

    pub(crate) fn step(&mut self, dispatcher: &Dispatcher) {
        match self.stage {
            DispatchStage::Reserve => {
                if self.results.try_reserve_exact(dispatcher.equipment.len()).is_err() {
                    self.fault = Some(DispatchFault::ResultBacking);
                    self.stage = DispatchStage::Complete;
                } else {
                    self.stage = DispatchStage::ValidateOrder;
                }
            }
            DispatchStage::ValidateOrder => {
                if let Some(equipment) = dispatcher.equipment.get(self.cursor) {
                    if self.previous_priority.is_some_and(|priority| priority > equipment.priority) {
                        self.fault = Some(DispatchFault::UnorderedPriority);
                        self.stage = DispatchStage::Complete;
                        return;
                    }
                    self.previous_priority = Some(equipment.priority);
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = if matches!(dispatcher.scheme, DispatchScheme::Uniform | DispatchScheme::UniformPartLoadRatio | DispatchScheme::Optimal) { DispatchStage::AccumulateCapacity } else { DispatchStage::Emit };
                }
            }
            DispatchStage::AccumulateCapacity => {
                if let Some(equipment) = dispatcher.equipment.get(self.cursor) {
                    if equipment.capacity_w > 0.0 {
                        self.total_capacity_w += equipment.capacity_w;
                    }
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = DispatchStage::Emit;
                }
            }
            DispatchStage::Emit => {
                let Some(equipment) = dispatcher.equipment.get(self.cursor) else {
                    self.stage = DispatchStage::Complete;
                    return;
                };
                let uniform = matches!(dispatcher.scheme, DispatchScheme::Uniform | DispatchScheme::UniformPartLoadRatio | DispatchScheme::Optimal);
                let part_load_ratio = if uniform {
                    if self.total_capacity_w > 0.0 {
                        (self.request.total_load_w / self.total_capacity_w).clamp(0.0, 1.0)
                    } else {
                        0.0
                    }
                } else if equipment.capacity_w > 0.0 {
                    self.remaining_w.min(equipment.capacity_w).max(0.0) / equipment.capacity_w
                } else {
                    0.0
                };
                let load_w = if uniform { equipment.capacity_w * part_load_ratio } else { self.remaining_w.min(equipment.capacity_w).max(0.0) };
                self.results.push(DispatchResult { equipment_id: equipment.equipment_id, load_w, part_load_ratio, runtime_fraction: if load_w > 0.0 { 1.0 } else { 0.0 } });
                self.remaining_w -= load_w;
                self.cursor += 1;
            }
            DispatchStage::Complete => {}
        }
    }

    #[cfg(test)]
    pub(crate) fn finish(self) -> Result<Vec<DispatchResult>, DispatchFault> {
        self.fault.map_or(Ok(self.results), Err)
    }

}
// #endregion 🔖️Dispatcher

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
