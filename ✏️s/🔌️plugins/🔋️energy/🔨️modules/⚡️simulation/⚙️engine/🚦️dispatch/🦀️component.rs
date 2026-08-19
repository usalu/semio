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
    pub async fn new(scheme: DispatchScheme, equipment: Vec<EquipmentPriority>) -> Self {
        Self { scheme, equipment }
    }

    /// 🎛️ Distribute load across equipment per dispatch scheme.
    pub async fn dispatch(&self, request: &DispatchRequest) -> Vec<DispatchResult> {
        let mut sorted = self.equipment.clone();
        sorted.sort_by_key(|e| e.priority);

        match self.scheme {
            DispatchScheme::Sequential => self.dispatch_sequential(&sorted, request),
            DispatchScheme::Uniform | DispatchScheme::UniformPartLoadRatio => self.dispatch_uniform(&sorted, request),
            DispatchScheme::Optimal => self.dispatch_optimal(&sorted, request),
            _ => self.dispatch_sequential(&sorted, request),
        }
    }

    async fn dispatch_sequential(&self, equipment: &[EquipmentPriority], request: &DispatchRequest) -> Vec<DispatchResult> {
        let mut remaining = request.total_load_w;
        let mut results = Vec::new();
        for eq in equipment {
            let load = remaining.min(eq.capacity_w).max(0.0);
            let plr = if eq.capacity_w > 0.0 { load / eq.capacity_w } else { 0.0 };
            results.push(DispatchResult { equipment_id: eq.equipment_id, load_w: load, part_load_ratio: plr, runtime_fraction: if load > 0.0 { 1.0 } else { 0.0 } });
            remaining -= load;
        }
        results
    }

    async fn dispatch_uniform(&self, equipment: &[EquipmentPriority], request: &DispatchRequest) -> Vec<DispatchResult> {
        let active: Vec<_> = equipment.iter().filter(|e| e.capacity_w > 0.0).collect();
        if active.is_empty() {
            return Vec::new();
        }
        let total_cap: f64 = active.iter().map(|e| e.capacity_w).sum();
        let plr = (request.total_load_w / total_cap).clamp(0.0, 1.0);
        active.iter().map(|eq| DispatchResult { equipment_id: eq.equipment_id, load_w: eq.capacity_w * plr, part_load_ratio: plr, runtime_fraction: if plr > 0.01 { 1.0 } else { 0.0 } }).collect()
    }

    async fn dispatch_optimal(&self, equipment: &[EquipmentPriority], request: &DispatchRequest) -> Vec<DispatchResult> {
        self.dispatch_uniform(equipment, request)
    }
}
// #endregion 🔖️Dispatcher

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sequential_fills_first_unit() {
        let d = Dispatcher::new(
            DispatchScheme::Sequential,
            vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 5000.0 }, EquipmentPriority { equipment_id: 2, priority: 2, min_runtime_hours: 0.0, capacity_w: 5000.0 }],
        );
        let results = d.dispatch(&DispatchRequest { total_load_w: 7000.0, available_capacity_w: 10000.0, outdoor_temp_c: 20.0 });
        assert!((results[0].load_w - 5000.0).abs() < 1e-6);
        assert!((results[1].load_w - 2000.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn uniform_splits_proportionally_to_capacity() {
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

    #[semio_framework_async_macros::async_test]
    async fn uniform_with_no_capacity_returns_empty() {
        let d = Dispatcher::new(DispatchScheme::Uniform, vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 0.0 }]);
        let results = d.dispatch(&DispatchRequest { total_load_w: 1000.0, available_capacity_w: 1000.0, outdoor_temp_c: 20.0 });
        assert!(results.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn optimal_delegates_to_uniform() {
        let equip = vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 2000.0 }];
        let request = DispatchRequest { total_load_w: 1000.0, available_capacity_w: 2000.0, outdoor_temp_c: 20.0 };
        let optimal = Dispatcher::new(DispatchScheme::Optimal, equip.clone()).dispatch(&request);
        let uniform = Dispatcher::new(DispatchScheme::Uniform, equip).dispatch(&request);
        assert_eq!(optimal, uniform);
    }

    #[semio_framework_async_macros::async_test]
    async fn unhandled_scheme_falls_back_to_sequential() {
        let equip = vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 500.0 }];
        let request = DispatchRequest { total_load_w: 300.0, available_capacity_w: 500.0, outdoor_temp_c: 20.0 };
        let results = Dispatcher::new(DispatchScheme::ThermalStorage, equip).dispatch(&request);
        assert!((results[0].load_w - 300.0).abs() < 1e-9);
    }
}
