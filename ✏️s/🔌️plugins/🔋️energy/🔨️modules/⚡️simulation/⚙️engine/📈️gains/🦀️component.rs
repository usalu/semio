//! 🔥️ Internal gains: people, lighting, equipment, process, data center decomposition.

use serde::{Deserialize, Serialize};

// #region 🔖️GainDecomposition
/// 📊️ Internal gain split into sensible, radiant, latent, and return-air fractions [W].
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GainDecomposition {
    pub total_w: f64,
    pub sensible_w: f64,
    pub radiant_w: f64,
    pub latent_w: f64,
    pub convective_w: f64,
    pub return_air_w: f64,
}

impl GainDecomposition {
    pub async fn add(&self, other: &Self) -> Self {
        Self {
            total_w: self.total_w + other.total_w,
            sensible_w: self.sensible_w + other.sensible_w,
            radiant_w: self.radiant_w + other.radiant_w,
            latent_w: self.latent_w + other.latent_w,
            convective_w: self.convective_w + other.convective_w,
            return_air_w: self.return_air_w + other.return_air_w,
        }
    }
}
// #endregion 🔖️GainDecomposition

// #region 🔖️People
/// 👤️ Metabolic rate presets [W/person] per ASHRAE 55 activity levels.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActivityLevel {
    SeatedQuiet,
    OfficeWork,
    StandingLight,
    Walking,
    HeavyWork,
}

impl ActivityLevel {
    pub async fn metabolic_w_per_person(self) -> f64 {
        match self {
            Self::SeatedQuiet => 70.0,
            Self::OfficeWork => 100.0,
            Self::StandingLight => 120.0,
            Self::Walking => 160.0,
            Self::HeavyWork => 250.0,
        }
    }

    pub async fn sensible_fraction(self) -> f64 {
        match self {
            Self::SeatedQuiet | Self::OfficeWork => 0.58,
            Self::StandingLight => 0.55,
            Self::Walking | Self::HeavyWork => 0.50,
        }
    }

    pub async fn latent_fraction(self) -> f64 {
        1.0 - self.sensible_fraction()
    }
}

/// 👤️ People gain [W] from count, activity, and radiant fraction.
pub async fn compute_people_gain_w(count: f64, activity: ActivityLevel, schedule_factor: f64, radiant_fraction: f64) -> GainDecomposition {
    let total = count * activity.metabolic_w_per_person() * schedule_factor.clamp(0.0, 1.0);
    let sensible = total * activity.sensible_fraction();
    let latent = total * activity.latent_fraction();
    let radiant = sensible * radiant_fraction.clamp(0.0, 1.0);
    let convective = sensible - radiant;
    GainDecomposition { total_w: total, sensible_w: sensible, radiant_w: radiant, latent_w: latent, convective_w: convective, return_air_w: 0.0 }
}
// #endregion 🔖️People

// #region 🔖️Lighting
/// 💡️ Lighting gain [W] from power density and fractions.
pub async fn compute_lighting_gain_w(watts_per_area: f64, floor_area_m2: f64, schedule_factor: f64, radiant_fraction: f64, return_air_fraction: f64) -> GainDecomposition {
    let total = watts_per_area * floor_area_m2 * schedule_factor.clamp(0.0, 1.0);
    let radiant = total * radiant_fraction.clamp(0.0, 1.0);
    let return_air = total * return_air_fraction.clamp(0.0, 1.0);
    let convective = total - radiant - return_air;
    GainDecomposition { total_w: total, sensible_w: total, radiant_w: radiant, latent_w: 0.0, convective_w: convective.max(0.0), return_air_w: return_air }
}
// #endregion 🔖️Lighting

// #region 🔖️Equipment
/// 🔌️ Electric equipment gain [W].
pub async fn compute_equipment_gain_w(watts_per_area: f64, floor_area_m2: f64, schedule_factor: f64, radiant_fraction: f64, latent_fraction: f64) -> GainDecomposition {
    let total = watts_per_area * floor_area_m2 * schedule_factor.clamp(0.0, 1.0);
    let latent = total * latent_fraction.clamp(0.0, 1.0);
    let sensible = total - latent;
    let radiant = sensible * radiant_fraction.clamp(0.0, 1.0);
    let convective = sensible - radiant;
    GainDecomposition { total_w: total, sensible_w: sensible, radiant_w: radiant, latent_w: latent, convective_w: convective, return_air_w: 0.0 }
}
// #endregion 🔖️Equipment

// #region 🔖️Process
/// 🏭️ Process load gain [W] with configurable split.
pub async fn compute_process_gain_w(design_load_w: f64, schedule_factor: f64, sensible_fraction: f64, latent_fraction: f64, radiant_fraction: f64) -> GainDecomposition {
    let total = design_load_w * schedule_factor.clamp(0.0, 1.0);
    let latent = total * latent_fraction.clamp(0.0, 1.0);
    let sensible = total * sensible_fraction.clamp(0.0, 1.0);
    let radiant = sensible * radiant_fraction.clamp(0.0, 1.0);
    let convective = sensible - radiant;
    GainDecomposition { total_w: total, sensible_w: sensible, radiant_w: radiant, latent_w: latent, convective_w: convective, return_air_w: 0.0 }
}
// #endregion 🔖️Process

// #region 🔖️DataCenter
/// 🖥️ Data center IT load [W] with air-side heat capture fraction.
pub async fn compute_datacenter_gain_w(it_load_w: f64, schedule_factor: f64, air_cooled_fraction: f64, supply_return_delta_t_k: f64) -> GainDecomposition {
    let total = it_load_w * schedule_factor.clamp(0.0, 1.0);
    let air_frac = air_cooled_fraction.clamp(0.0, 1.0);
    let air_w = total * air_frac;
    let liquid_w = total - air_w;
    let return_air = if supply_return_delta_t_k > 0.1 { air_w * 0.9 } else { air_w * 0.5 };
    GainDecomposition { total_w: total, sensible_w: total, radiant_w: liquid_w * 0.1, latent_w: 0.0, convective_w: air_w - return_air, return_air_w: return_air }
}
// #endregion 🔖️DataCenter

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn people_gain_scales_with_count() {
        let g1 = compute_people_gain_w(1.0, ActivityLevel::OfficeWork, 1.0, 0.3);
        let g10 = compute_people_gain_w(10.0, ActivityLevel::OfficeWork, 1.0, 0.3);
        assert!((g10.total_w - 10.0 * g1.total_w).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn lighting_return_air_reduces_convective() {
        let g = compute_lighting_gain_w(10.0, 100.0, 1.0, 0.2, 0.5);
        assert!((g.total_w - 1000.0).abs() < 1e-6);
        assert!((g.return_air_w - 500.0).abs() < 1e-6);
        assert!((g.radiant_w - 200.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn equipment_latent_reduces_sensible() {
        let g = compute_equipment_gain_w(5.0, 200.0, 1.0, 0.5, 0.1);
        assert!((g.latent_w - 100.0).abs() < 1e-6);
        assert!((g.sensible_w - 900.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn datacenter_total_matches_it_load() {
        let g = compute_datacenter_gain_w(50_000.0, 0.8, 0.7, 12.0);
        assert!((g.total_w - 40_000.0).abs() < 1e-6);
    }
}
