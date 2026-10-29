//! 🧮️ Energy model mutation domain `model-simulation`: `add-output-variable`, `remove-output-variable`, `replace-airflow-network`


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;


#[path = "."]
pub mod remove_output_variable {
    #[path = "../📉️remove-output-variable/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../📉️remove-output-variable/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../📉️remove-output-variable/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📉️remove-output-variable/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod add_output_variable {
    #[path = "../📊️add-output-variable/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../📊️add-output-variable/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../📊️add-output-variable/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📊️add-output-variable/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod replace_airflow_network {
    #[path = "../🫧️replace-airflow-network/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🫧️replace-airflow-network/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🫧️replace-airflow-network/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🫧️replace-airflow-network/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use remove_output_variable::{remove_output_variable, RemoveOutputVariable};
pub use add_output_variable::{add_output_variable, AddOutputVariable};
pub use replace_airflow_network::{replace_airflow_network, ReplaceAirflowNetwork};
