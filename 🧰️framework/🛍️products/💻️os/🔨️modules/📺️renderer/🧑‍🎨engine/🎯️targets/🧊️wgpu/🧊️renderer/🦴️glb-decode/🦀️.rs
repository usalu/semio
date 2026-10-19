//! 🦴️ GLB decode: JSON lexer, schema structs and cursor, instance plan, math, outline and materialize cursors for renderer mesh assets.
use super::*;

#[path = "🔤️json-lexer-and-schema-types/🦀️.rs"]
mod json_lexer_and_schema_types;
use json_lexer_and_schema_types::{
    GlbAccessorSchema, GlbImageSchema, GlbJsonAtom, GlbJsonToken, GlbJsonTokenCursor,
    GlbMaterialSchema, GlbMeshSchema, GlbNodeSchema, GlbNumericArray, GlbNumericArrayKind,
    GlbPrimitiveSchema, GlbSamplerSchema, GlbSceneSchema, GlbTextureSchema, GlbViewSchema,
    GLB_FACE_ID_ATTRIBUTE, GLB_SCHEMA_ITEM_CAPACITY, GLB_SCHEMA_OUTPUT_BYTES,
    GLB_VERTEX_ID_ATTRIBUTE,
};

#[path = "🗃️schema-output-and-cursor-state/🦀️.rs"]
mod schema_output_and_cursor_state;
use schema_output_and_cursor_state::{GlbSchemaOutput, GlbSchemaSection};
pub use schema_output_and_cursor_state::GlbSchemaCursor;

#[path = "🧮️schema-token-application/🦀️.rs"]
mod schema_token_application;

#[path = "🏁️schema-finishing-and-plan-types/🦀️.rs"]
mod schema_finishing_and_plan_types;
pub use schema_finishing_and_plan_types::GlbMatrix;
use schema_finishing_and_plan_types::{
    GlbInstanceOutput, GlbInstancePlanCursor, GlbNodePlanFrame, GlbPlanMode,
};

#[path = "📐️instance-plan-math-and-outline/🦀️.rs"]
mod instance_plan_math_and_outline;
use instance_plan_math_and_outline::{glb_normalize, glb_transform_normal, glb_transform_point};
pub use instance_plan_math_and_outline::{GlbOutlineAccumulator, GLB_OUTLINE_SCALE};

#[path = "🧵️materialize-cursor-step/🦀️.rs"]
mod materialize_cursor_step;
pub use materialize_cursor_step::{GlbMaterializeCursor, GlbMaterializePhase};

#[path = "📖️materialize-readers-and-component-helpers/🦀️.rs"]
mod materialize_readers_and_component_helpers;
use materialize_readers_and_component_helpers::{
    glb_mag_filter, glb_mesh_fault, glb_min_filter, glb_texture_wrap,
};
