
    use super::*;

    #[test]
    fn end_use_scales_with_schedule() {
        let load = EndUseLoad { name: "Lighting".into(), rated_power_w: 1000.0, schedule_factor: 0.5, power_factor: 0.95 };
        assert!((load.power_w() - 500.0).abs() < 1e-6);
    }

    #[test]
    fn pv_zero_at_night() {
        let pv = PvSystem { dc_capacity_w: 10_000.0, module_efficiency: 0.2, area_m2: 50.0, inverter_efficiency: 0.96, temperature_coefficient: -0.004, tilt_deg: 30.0, azimuth_deg: 180.0 };
        assert!(pv.simulate(0.0, 20.0).abs() < 1e-6);
    }

    #[test]
    fn wind_cubic_below_rated() {
        let turbine = WindTurbine { rated_power_w: 20_000.0, cut_in_m_s: 3.0, rated_speed_m_s: 12.0, cut_out_m_s: 25.0, hub_height_m: 30.0, rotor_diameter_m: 12.0 };
        let low = turbine.simulate(5.0, 1.2);
        let high = turbine.simulate(8.0, 1.2);
        assert!(high > low);
        assert!(turbine.simulate(2.0, 1.2).abs() < 1e-6);
    }

    #[test]
    fn battery_soc_bounds() {
        let battery = Battery { capacity_kwh: 10.0, max_charge_w: 5000.0, max_discharge_w: 5000.0, round_trip_efficiency: 0.92, min_soc: 0.1, max_soc: 0.95, state_of_charge: 0.5 };
        let (charge_w, soc_after) = battery.simulate(3000.0, 3600.0);
        assert!(charge_w > 0.0);
        assert!(soc_after > 0.5);
        let (_, soc_dis) = battery.simulate(-8000.0, 3600.0);
        assert!(soc_dis >= battery.min_soc);
    }

    #[test]
    fn grid_balance_import_when_load_exceeds_supply() {
        let xf = Transformer { rated_kva: 100.0, no_load_loss_w: 50.0, load_loss_w: 800.0, impedance_fraction: 0.04 };
        let balance = grid_balance(50_000.0, 10_000.0, 0.0, 0.0, 0.0, &xf);
        assert!(balance.net_import_w > 0.0);
        assert!(balance.net_export_w.abs() < 1e-6);
    }

    #[test]
    fn generator_respects_minimum_load() {
        let gen = Generator { rated_power_w: 100_000.0, fuel_lhv_j_per_kg: 42e6, electrical_efficiency: 0.35, min_load_fraction: 0.3 };
        let (out, fuel) = gen.simulate(5000.0, true);
        assert!(out >= 30_000.0);
        assert!(fuel > 0.0);
    }
