//! 🎛️ Plant and equipment dispatch strategies.

use serde::{Deserialize, Serialize};

// #region 🔖️Dispatch
/// 🎛️ Equipment dispatch scheme.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EquipmentPriority {
    pub equipment_id: u32,
    pub priority: u32,
    pub min_runtime_hours: f64,
    pub capacity_w: f64,
}

/// 🎛️ Dispatch request for plant equipment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DispatchRequest {
    pub total_load_w: f64,
    pub available_capacity_w: f64,
    pub outdoor_temp_c: f64,
}

/// 🎛️ Dispatch result per equipment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum DispatchFault {
    ResultBacking,
    UnorderedPriority,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum DispatchStage {
    Reserve,
    ValidateOrder,
    AccumulateCapacity,
    Emit,
    Complete,
}

impl DispatchBuilder {
    pub(crate) fn new(request: DispatchRequest) -> Self {
        let remaining_w = request.total_load_w;
        Self { request, stage: DispatchStage::Reserve, cursor: 0, previous_priority: None, total_capacity_w: 0.0, remaining_w, results: Vec::new(), fault: None }
    }

    pub(crate) fn is_complete(&self) -> bool {
        self.stage == DispatchStage::Complete
    }

    pub(crate) fn fault(&self) -> Option<DispatchFault> {
        self.fault
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

    pub(crate) fn close_step(&mut self, maximum_items: usize) -> bool {
        if maximum_items == 0 {
            return false;
        }
        self.results.pop().is_none()
    }
}
// #endregion 🔖️Dispatcher

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_fills_first_unit() {
        let d = Dispatcher::new(
            DispatchScheme::Sequential,
            vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 5000.0 }, EquipmentPriority { equipment_id: 2, priority: 2, min_runtime_hours: 0.0, capacity_w: 5000.0 }],
        );
        let results = d.dispatch(&DispatchRequest { total_load_w: 7000.0, available_capacity_w: 10000.0, outdoor_temp_c: 20.0 });
        assert!((results[0].load_w - 5000.0).abs() < 1e-6);
        assert!((results[1].load_w - 2000.0).abs() < 1e-6);
    }

    #[test]
    fn uniform_splits_proportionally_to_capacity() {
        let d = Dispatcher::new(
            DispatchScheme::Uniform,
            vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 3000.0 }, EquipmentPriority { equipment_id: 2, priority: 2, min_runtime_hours: 0.0, capacity_w: 1000.0 }],
        );
        let results = d.dispatch(&DispatchRequest { total_load_w: 2000.0, available_capacity_w: 4000.0, outdoor_temp_c: 20.0 });
        assert_eq!(results.len(), 2);
        let plr = 2000.0 / 4000.0;
        assert!((results[0].load_w - 3000.0 * plr).abs() < 1e-6);
        assert!((results[1].load_w - 1000.0 * plr).abs() < 1e-6);
        assert!((results[0].part_load_ratio - plr).abs() < 1e-9);
    }

    #[test]
    fn uniform_with_no_capacity_returns_empty() {
        let d = Dispatcher::new(DispatchScheme::Uniform, vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 0.0 }]);
        let results = d.dispatch(&DispatchRequest { total_load_w: 1000.0, available_capacity_w: 1000.0, outdoor_temp_c: 20.0 });
        assert!(results.is_empty());
    }

    #[test]
    fn optimal_delegates_to_uniform() {
        let equip = vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 2000.0 }];
        let request = DispatchRequest { total_load_w: 1000.0, available_capacity_w: 2000.0, outdoor_temp_c: 20.0 };
        let optimal = Dispatcher::new(DispatchScheme::Optimal, equip.clone()).dispatch(&request);
        let uniform = Dispatcher::new(DispatchScheme::Uniform, equip).dispatch(&request);
        assert_eq!(optimal, uniform);
    }

    #[test]
    fn unhandled_scheme_falls_back_to_sequential() {
        let equip = vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 500.0 }];
        let request = DispatchRequest { total_load_w: 300.0, available_capacity_w: 500.0, outdoor_temp_c: 20.0 };
        let results = Dispatcher::new(DispatchScheme::ThermalStorage, equip).dispatch(&request);
        assert!((results[0].load_w - 300.0).abs() < 1e-9);
    }
}
