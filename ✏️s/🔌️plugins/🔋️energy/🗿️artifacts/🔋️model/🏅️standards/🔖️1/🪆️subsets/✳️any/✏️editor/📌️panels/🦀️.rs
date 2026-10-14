//! 📌️ Energy model editor panels: the artifact tree (outliner) and the inspector, both reading the one `"energyModel"`
//! interaction domain.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_s_artifact_energy_model_editor_commands as editor_commands;
extern crate semio_s_artifact_energy_model_editor_config as editor_config;
extern crate semio_s_artifact_energy_model_editor_edit as editor_edit;
extern crate semio_s_artifact_energy_model_editor_interaction as editor_interaction;
extern crate semio_s_plugin_energy_engine_geometry as geometry;
extern crate semio_s_plugin_energy_engine_material as material;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
#[cfg(test)]
extern crate semio_s_plugin_energy_engine_bestest as bestest;
#[cfg(test)]
extern crate semio_s_artifact_energy_model_examples as examples;

#[path = "🗿️artifact/🦀️.rs"]
pub mod artifact;
#[path = "🔍️inspection/🦀️.rs"]
pub mod inspection;
