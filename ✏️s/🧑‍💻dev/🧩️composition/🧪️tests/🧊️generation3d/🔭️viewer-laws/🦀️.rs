/// 🔭️ Viewer laws: the preview-eval and viewer law wrappers, each mounting its unit test file.

#[path = ".."]
mod preview_eval_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_preview_eval::*;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod viewer_generation3d_eval_chain_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_viewer::*;
use semio_s_artifact_procedural_3d_preview_eval as preview_eval;
use semio_s_artifact_procedural_3d_viewer_commands::export_document;
use semio_s_artifact_procedural_3d_viewer_commands::flow_eval_release;
use semio_s_artifact_procedural_3d_viewer_commands::flow_eval_resolve;
use semio_s_artifact_procedural_3d_viewer_commands::flow_eval_tick;
use semio_s_artifact_procedural_3d_viewer_commands::flow_tessellate_cancel_resolve;
use semio_s_artifact_procedural_3d_viewer_commands::flow_tessellate_resolve;
use semio_s_artifact_procedural_3d_viewer_commands::set_active_example;
use semio_s_artifact_procedural_3d_viewer_commands::set_camera;
use semio_s_artifact_procedural_3d_viewer_commands::set_contributions;
use semio_s_artifact_procedural_3d_viewer_commands::set_lod_mode;
use semio_s_artifact_procedural_3d_viewer_commands::set_show_mode;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_azimuth;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_elevation;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_intensity;
use semio_s_artifact_procedural_3d_viewer_commands::toggle_sun;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_viewer_modes::view;
use semio_s_artifact_procedural_3d_viewer_modes::view::windows::preview;
use semio_s_artifact_procedural_3d_viewer_presence::Generation3dViewPresence;
use semio_s_artifact_procedural_3d_viewer_presence::Generation3dViewPresenceMutation;
use semio_s_artifact_procedural_3d_viewer_transient::Generation3dViewTransient;
use semio_s_artifact_procedural_3d_viewer_transient::Generation3dViewTransientMutation;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_model::GENERATION3D_DIALECT;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_SCHEMA;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️eval-chain/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod viewer_domain {
use super::*;
use semio_s_artifact_procedural_3d_viewer::*;
use semio_s_artifact_procedural_3d_preview_eval as preview_eval;
use semio_s_artifact_procedural_3d_viewer_commands::export_document;
use semio_s_artifact_procedural_3d_viewer_commands::flow_eval_release;
use semio_s_artifact_procedural_3d_viewer_commands::flow_eval_resolve;
use semio_s_artifact_procedural_3d_viewer_commands::flow_eval_tick;
use semio_s_artifact_procedural_3d_viewer_commands::flow_tessellate_cancel_resolve;
use semio_s_artifact_procedural_3d_viewer_commands::flow_tessellate_resolve;
use semio_s_artifact_procedural_3d_viewer_commands::set_active_example;
use semio_s_artifact_procedural_3d_viewer_commands::set_camera;
use semio_s_artifact_procedural_3d_viewer_commands::set_contributions;
use semio_s_artifact_procedural_3d_viewer_commands::set_lod_mode;
use semio_s_artifact_procedural_3d_viewer_commands::set_show_mode;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_azimuth;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_elevation;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_intensity;
use semio_s_artifact_procedural_3d_viewer_commands::toggle_sun;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_viewer_modes::view;
use semio_s_artifact_procedural_3d_viewer_modes::view::windows::preview;
use semio_s_artifact_procedural_3d_viewer_presence::Generation3dViewPresence;
use semio_s_artifact_procedural_3d_viewer_presence::Generation3dViewPresenceMutation;
use semio_s_artifact_procedural_3d_viewer_transient::Generation3dViewTransient;
use semio_s_artifact_procedural_3d_viewer_transient::Generation3dViewTransientMutation;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_model::GENERATION3D_DIALECT;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_SCHEMA;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod viewer_laws;
}

#[path = ".."]
mod viewer_generation3d_status_contract_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_viewer::*;
use semio_s_artifact_procedural_3d_preview_eval as preview_eval;
use semio_s_artifact_procedural_3d_viewer_commands::export_document;
use semio_s_artifact_procedural_3d_viewer_commands::flow_eval_release;
use semio_s_artifact_procedural_3d_viewer_commands::flow_eval_resolve;
use semio_s_artifact_procedural_3d_viewer_commands::flow_eval_tick;
use semio_s_artifact_procedural_3d_viewer_commands::flow_tessellate_cancel_resolve;
use semio_s_artifact_procedural_3d_viewer_commands::flow_tessellate_resolve;
use semio_s_artifact_procedural_3d_viewer_commands::set_active_example;
use semio_s_artifact_procedural_3d_viewer_commands::set_camera;
use semio_s_artifact_procedural_3d_viewer_commands::set_contributions;
use semio_s_artifact_procedural_3d_viewer_commands::set_lod_mode;
use semio_s_artifact_procedural_3d_viewer_commands::set_show_mode;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_azimuth;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_elevation;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_intensity;
use semio_s_artifact_procedural_3d_viewer_commands::toggle_sun;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_viewer_modes::view;
use semio_s_artifact_procedural_3d_viewer_modes::view::windows::preview;
use semio_s_artifact_procedural_3d_viewer_presence::Generation3dViewPresence;
use semio_s_artifact_procedural_3d_viewer_presence::Generation3dViewPresenceMutation;
use semio_s_artifact_procedural_3d_viewer_transient::Generation3dViewTransient;
use semio_s_artifact_procedural_3d_viewer_transient::Generation3dViewTransientMutation;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_model::GENERATION3D_DIALECT;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_SCHEMA;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️status-contract/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod viewer_generation3d_modes_view_windows_preview_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_viewer_modes::view::windows::preview::*;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_preview_eval::self;
use semio_s_artifact_procedural_3d_preview_eval::PreviewChannelItem;
use semio_s_artifact_procedural_3d_preview_eval::PreviewInlineGeometry;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod viewer_generation3d_commands_set_sun_azimuth_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_azimuth::*;
use semio_s_artifact_procedural_3d_viewer_config as config;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🧭️set-sun-azimuth/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod viewer_generation3d_commands_set_sun_elevation_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_elevation::*;
use semio_s_artifact_procedural_3d_viewer_config as config;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🌄️set-sun-elevation/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod viewer_generation3d_commands_set_sun_intensity_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_intensity::*;
use semio_s_artifact_procedural_3d_viewer_config as config;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🔆️set-sun-intensity/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod viewer_generation3d_commands_set_camera_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_viewer_commands::set_camera::*;
use semio_s_artifact_procedural_3d_viewer_config as config;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewCamera;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/📷️set-camera/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod viewer_generation3d_commands_set_active_example_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_viewer_commands::set_active_example::*;
use semio_s_artifact_procedural_3d_model::is_generation3d_example_id;
use semio_s_artifact_procedural_3d_viewer_config as config;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🎨️set-active-example/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod viewer_generation3d_commands_set_show_mode_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_viewer_commands::set_show_mode::*;
use semio_s_artifact_procedural_3d_viewer_config as config;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/👁️set-show-mode/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod viewer_generation3d_commands_set_lod_mode_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_viewer_commands::set_lod_mode::*;
use semio_s_artifact_procedural_3d_viewer_config as config;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🔬️set-lod-mode/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod viewer_generation3d_commands_toggle_sun_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_viewer_commands::toggle_sun::*;
use semio_s_artifact_procedural_3d_viewer_config as config;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🌞️toggle-sun/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}
