//! 

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_puzzle_3d_precompute_brush::*;
use semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id;
use semio_s_artifact_puzzle_3d_model::AttractionProps;
use semio_s_artifact_puzzle_3d_model::BrushCompatibleCandidate;
use semio_s_artifact_puzzle_3d_model::BrushHostRules;
use semio_s_artifact_puzzle_3d_model::BrushKindWeights;
use semio_s_artifact_puzzle_3d_model::BrushPlacePayload;
use semio_s_artifact_puzzle_3d_model::BrushPreviewState;
use semio_s_artifact_puzzle_3d_model::CableKindCatalog;
use semio_s_artifact_puzzle_3d_model::EngineSceneSnapshot;
use semio_s_artifact_puzzle_3d_model::EngineSceneObject;
use semio_s_artifact_puzzle_3d_model::KindCatalogBundle;
use semio_s_artifact_puzzle_3d_model::KindCompatEntry;
use semio_s_artifact_puzzle_3d_model::ObjectKind;
use semio_s_artifact_puzzle_3d_model::ObjectKindVortexTemplate;
use semio_s_artifact_puzzle_3d_model::Quat;
use semio_s_artifact_puzzle_3d_model::Vec3;
use semio_s_artifact_puzzle_3d_model::VortexKindCatalog;
use semio_s_artifact_puzzle_3d_model::VortexProps;
use semio_s_artifact_puzzle_3d_precompute_geometry::collision_body_from_buffers;
use semio_s_artifact_puzzle_3d_precompute_geometry::compute_brush_placement_pose;
use semio_s_artifact_puzzle_3d_precompute_geometry::normalize_vec3;
use semio_s_artifact_puzzle_3d_precompute_geometry::pose_isometry;
use semio_s_artifact_puzzle_3d_precompute_geometry::precompute_work;
use semio_s_artifact_puzzle_3d_precompute_geometry::quat_rotate_vec;
use semio_s_artifact_puzzle_3d_precompute_geometry::vec3_add;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionAabb;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionBody;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionPenetrationState;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionStepContext;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionStepResult;
use semio_s_artifact_puzzle_3d_precompute_geometry::Pose3d;
use semio_s_artifact_puzzle_3d_model::BrushSuggestionsRunCounter;
use semio_s_artifact_puzzle_3d_model::BrushSuggestionsRunReason;
use semio_s_artifact_puzzle_3d_model::BrushSuggestionsRunStage;
use semio_s_artifact_puzzle_3d_model::SceneConfig;
use semio_s_plugin_puzzle_jobs::JobTurn;
use semio_framework_job::InteractiveJob;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_job::StepContext;
use semio_framework_plugin::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin::ToolRunJobPort;
use semio_framework_tool_run::ToolRunCounter;
use semio_framework_tool_run::ToolRunIdentity;
use semio_framework_tool_run::ToolRunProgress;
use semio_framework_tool_run::ToolRunState;
use semio_framework_tool_run::ToolRunStepArg;
use semio_framework_tool_run::ToolRunStepKind;
use semio_framework_tool_run::ToolRunStepRing;
use semio_framework_tool_run::ToolRunTickWriter;
use semio_framework_tool_run::ToolRunTraceSubject;
use semio_framework_tool_run::ToolRunVerdict;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;


use semio_s_artifact_puzzle_3d_precompute::Puzzle3dCollision;
use semio_s_artifact_puzzle_3d_precompute_geometry::precompute_work_clock;
use semio_s_artifact_puzzle_3d_precompute_geometry::precompute_work_done;
use semio_s_artifact_puzzle_3d_model::ObjectKindRepresentation;

include!("🍓️placement-compatibility-and-fill-laws/🦀️.rs");

include!("🥦️suggestions-run-laws/🦀️.rs");
