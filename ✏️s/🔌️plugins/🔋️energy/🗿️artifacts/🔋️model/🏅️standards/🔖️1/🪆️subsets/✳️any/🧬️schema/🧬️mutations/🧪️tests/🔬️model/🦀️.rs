//! 🧪️ The `model` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mut_model as model_mutations;
extern crate semio_s_artifact_energy_model_mut_model_simulation as model_simulation_mutations;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod disconnect_referenced_model {
    pub use crate::model_mutations::disconnect_referenced_model::*;
    #[path = "../../🔋️model/✂️disconnect-referenced-model/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔋️model/✂️disconnect-referenced-model/🧪️tests/✅️disconnects/🦀️.rs"]
    mod tests_disconnects;
}

#[path = "."]
mod unbind_weather_file {
    pub use crate::model_mutations::unbind_weather_file::*;
    #[path = "../../🔋️model/🌤️unbind-weather-file/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔋️model/🌤️unbind-weather-file/🧪️tests/✅️unbinds/🦀️.rs"]
    mod tests_unbinds;
}

#[path = "."]
mod bind_weather_file {
    pub use crate::model_mutations::bind_weather_file::*;
    #[path = "../../🔋️model/🌦️bind-weather-file/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔋️model/🌦️bind-weather-file/🧪️tests/✅️binds/🦀️.rs"]
    mod tests_binds;
}

#[path = "."]
mod rename_model {
    pub use crate::model_mutations::rename_model::*;
    #[path = "../../🔋️model/🏷️rename/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔋️model/🏷️rename/🧪️tests/✅️renames-the-model/🦀️.rs"]
    mod tests_renames_the_model;
}

#[path = "."]
mod remove_output_variable {
    pub use crate::model_simulation_mutations::remove_output_variable::*;
    #[path = "../../🔋️model/📉️remove-output-variable/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔋️model/📉️remove-output-variable/🧪️tests/✅️drops/🦀️.rs"]
    mod tests_drops;
}

#[path = "."]
mod add_output_variable {
    pub use crate::model_simulation_mutations::add_output_variable::*;
    #[path = "../../🔋️model/📊️add-output-variable/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔋️model/📊️add-output-variable/🧪️tests/✅️adds/🦀️.rs"]
    mod tests_adds;
}

#[path = "."]
mod change_model_version {
    pub use crate::model_mutations::change_model_version::*;
    #[path = "../../🔋️model/🔢️change-version/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔋️model/🔢️change-version/🧪️tests/✅️bumps/🦀️.rs"]
    mod tests_bumps;
}

#[path = "."]
mod connect_referenced_model {
    pub use crate::model_mutations::connect_referenced_model::*;
    #[path = "../../🔋️model/🪢️connect-referenced-model/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔋️model/🪢️connect-referenced-model/🧪️tests/✅️connects/🦀️.rs"]
    mod tests_connects;
}

#[path = "."]
mod replace_airflow_network {
    pub use crate::model_simulation_mutations::replace_airflow_network::*;
    #[path = "../../🔋️model/🫧️replace-airflow-network/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔋️model/🫧️replace-airflow-network/🧪️tests/✅️attaches/🦀️.rs"]
    mod tests_attaches;
}
