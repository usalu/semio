//! 🧪️ Original membership laws of the load-center and plant-loop operations, kept in a dedicated test crate so their domain crates stay inside the source boundary.
#![allow(clippy::too_many_arguments)]
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path="."]
mod add_electrical_load_center_pv {
    #[allow(unused_imports)]
    use semio_s_artifact_energy_model_mut_load_center::add_electrical_load_center_pv::*;
    #[path="../../🔌️load-center/☄️add-pv/🧪️native/🦀️.rs"]
    mod native;
}

#[path="."]
mod remove_electrical_load_center_pv {
    #[allow(unused_imports)]
    use semio_s_artifact_energy_model_mut_load_center::remove_electrical_load_center_pv::*;
    #[path="../../🔌️load-center/🌘️remove-pv/🧪️native/🦀️.rs"]
    mod native;
}

#[path="."]
mod add_electrical_load_center_battery {
    #[allow(unused_imports)]
    use semio_s_artifact_energy_model_mut_load_center::add_electrical_load_center_battery::*;
    #[path="../../🔌️load-center/🔋️add-battery/🧪️native/🦀️.rs"]
    mod native;
}

#[path="."]
mod remove_electrical_load_center_battery {
    #[allow(unused_imports)]
    use semio_s_artifact_energy_model_mut_load_center::remove_electrical_load_center_battery::*;
    #[path="../../🔌️load-center/🪝️remove-battery/🧪️native/🦀️.rs"]
    mod native;
}

#[path="."]
mod add_plant_loop_equipment {
    #[allow(unused_imports)]
    use semio_s_artifact_energy_model_mut_plant_loop::add_plant_loop_equipment::*;
    #[path="../../🏭️plant-loop/🔩️add-equipment/🧪️native/🦀️.rs"]
    mod native;
}

#[path="."]
mod remove_plant_loop_equipment {
    #[allow(unused_imports)]
    use semio_s_artifact_energy_model_mut_plant_loop::remove_plant_loop_equipment::*;
    #[path="../../🏭️plant-loop/⚙️remove-equipment/🧪️native/🦀️.rs"]
    mod native;
}
