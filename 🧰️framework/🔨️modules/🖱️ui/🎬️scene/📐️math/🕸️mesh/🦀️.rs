//! 🕸️ Paged mesh ownership: the exclusive authority, write tokens, leases, native resource custody and the instance and component source records drawn from them.
//!
//! Carved out of the former `semio-framework-ui-scene` math module by ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`.

#[path = "../♻️retirement/🦀️.rs"]
mod mesh3d_retirement;
use mesh3d_retirement::{IneligibleGroups, NumericSeen};
use semio_framework_value::{
    retained_clone::{RetainedCloneGrant, RetainedCloneProgress},
    retirement::controlled::ControlledRetirement,
    RetirementDemand,
};
use semio_framework_geometry::{Mat4, Vec3};
pub use semio_framework_mesh_component_references::ComponentReferenceTable;
use semio_framework_ui_scene_algebra::{mat4_from_quat_m, mat4_scale_vec_m, mat4_translation_m, vec3_from_array_m, Mat4Math};

//#region Mesh

#[path = "📐️schema-faults-and-lease-types/🦀️.rs"]
mod schema_faults_and_lease_types;
pub use schema_faults_and_lease_types::{
    Mesh3dComponentAdmissionStep, Mesh3dFault, Mesh3dField, Mesh3dLease, Mesh3dOwnershipStep,
    Mesh3dSchema, Mesh3dWriteToken, MESH3D_AUTHORITY_CAPACITY, MESH3D_AUTHORITY_PAGE_CAPACITY,
    MESH3D_OWNER_BYTE_CAPACITY, MESH3D_OWNER_PAGE_CAPACITY, MESH3D_PAGE_BYTES,
};

#[path = "🧱️mesh-owner/🦀️.rs"]
mod mesh_owner;
use mesh_owner::{
    component_lengths, controlled_demands, mesh3d_component_number, Mesh3dLayout, Mesh3dOwner,
    Mesh3dPage, Mesh3dSlotState,
};

#[path = "🏛️mesh-authority-and-api/🦀️.rs"]
mod mesh_authority_and_api;
#[cfg(test)]
use mesh_authority_and_api::Mesh3dAuthority;
use mesh_authority_and_api::mesh3d_authority;
pub use mesh_authority_and_api::{
    mesh3d_abort, mesh3d_abort_demands, mesh3d_abort_step, mesh3d_allocate_capacity_byte_demand,
    mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_capacity_byte_demand, mesh3d_begin_close,
    mesh3d_close_demands, mesh3d_close_step, mesh3d_component_admission_step,
    mesh3d_component_capacity_byte_demand, mesh3d_move_component_references, mesh3d_read_write_u32,
    mesh3d_read_write_vec3, mesh3d_seal, mesh3d_terminal_is_empty, mesh3d_update_vec3,
    mesh3d_write_edge, mesh3d_write_u32, mesh3d_write_vec2, mesh3d_write_vec3, mesh3d_write_vec4,
};

#[path = "🔖️mesh-lease-and-cursors/🦀️.rs"]
mod mesh_lease_and_cursors;
pub use mesh_lease_and_cursors::{Mesh3dItem, Mesh3dItemCursor, Mesh3dPageCursor};

#[path = "🌈️scene-instance-materials/🦀️.rs"]
mod scene_instance_materials;
pub use scene_instance_materials::{
    ComponentSource3d, Instance3d, SceneColorSource3d, SceneInstanceMaterial3d, SceneSectionRole3d,
};

#[path = "../📦️resource/🦀️.rs"]
mod mesh3d_resource;
pub use mesh3d_resource::{Mesh3dResourceIdentity, Mesh3dResourceLoan, Mesh3dResourceOwner, Mesh3dResourceReceiving, Mesh3dReceivingStep, MESH3D_RESOURCE_COMPONENT_COUNT_CAPACITY};

//#endregion Mesh

#[cfg(any(test, feature = "testkit"))]
#[path = "../../🧪️tests/🧰️mesh-testkit/🦀️.rs"]
pub mod testkit;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️mesh-unit/🦀️.rs"]
mod tests;
