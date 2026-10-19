//! 🥣️ semio wgpu cross-crate laws: document tree reconcile, conformance corpus, theme token parity, text renderer and prepared engine.

#![cfg(test)]

use semio_framework_ui_draw_contract::DrawList;
use semio_framework_ui_theme::GlassStyle;
use ui_scene::math::Instance3d;
use ui_scene::math::Mat4;
use ui_scene::math::Mesh3dField;
use ui_scene::math::Mesh3dSchema;
use semio_framework_ui_prepared::PreparedRenderInput;
use semio_framework_ui_prepared::PreparedRenderJob;
use ui_scene::math::ProceduralGrid3d;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Rgba;
use ui_scene::math::SceneDraw3d;
use ui_scene::math::SceneMaterialDraw3d;
use ui_scene::math::SceneMaterialKind3d;
use ui_scene::math::ScenePass3d;
use semio_framework_ui_wgpu_hosting::cursor;
use semio_framework_ui_draw_contract as draw_types;
use ui_scene::math as kernel_3d_scene;
use ui_scene::math::mesh3d_allocate_capacity_byte_demand;
use ui_scene::math::mesh3d_allocate_step;
use ui_scene::math::mesh3d_begin;
use ui_scene::math::mesh3d_begin_capacity_byte_demand;
use ui_scene::math::mesh3d_begin_close;
use ui_scene::math::mesh3d_close_demands;
use ui_scene::math::mesh3d_close_step;
use ui_scene::math::mesh3d_seal;
use ui_scene::math::mesh3d_terminal_is_empty;
use ui_scene::math::mesh3d_write_edge;
use ui_scene::math::mesh3d_write_u32;
use ui_scene::math::mesh3d_write_vec3;
use semio_framework_ui_wgpu_paint as paint;
use semio_framework_ui_theme as theme;

#[path = "../../../../🧪️tests/🔬️targets-wgpu-text-renderer/🦀️.rs"]
mod text_renderer_tests;

#[path = "../../../../🧪️tests/🔬️targets-wgpu-theme-token-parity/🦀️.rs"]
mod theme_token_tests;

#[path = "../../../../🧪️tests/🌳️document-tree-reconcile/🦀️.rs"]
mod document_tree_tests;

#[path = "../../../../🧪️tests/🧪️conformance-corpus/🦀️.rs"]
mod conformance_corpus_tests;

#[path = "../../../../🧪️tests/🔬️targets-wgpu-prepared-engine-unit/🦀️.rs"]
mod prepared_engine_tests;

