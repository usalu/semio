// 👥️ HUB WP3: mount hub of WP3 owners and extension: child, composition-emission, composition-publication, publication-mounted, document-media, job, command, peer, retained-command, extension.
// This file is spliced into the plugin crate root by `include!` from the root, so every item below stays a crate-root item;
// its `#[path]` literals resolve against this folder (`🪝️mounts/<hub>/`), hence the `../../` prefix of the plugin folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-PLUGIN-CRATES.md, section 12.5).

//#region 📜️DeclarationFacet
#[path = "../../💡️inference/🦀️.rs"]
pub(crate) mod artifact_inference;
pub use artifact_inference::{ArtifactInferenceGatewayFailure,ArtifactInferenceGatewayRefusal};
#[path="../../🧩️composition/📨️emission/📦️preparation/🦀️.rs"]
pub(crate) mod child_emit_preparation;
#[path = "../../🧵️retained-command/🧬️context/🦀️.rs"]
pub(crate) mod owned_context;
//#endregion 📜️DeclarationFacet
//#region 📬️PublicationFacet
#[path = "../../🪆️child/🦀️.rs"]
pub(crate) mod child_content;
#[path = "../../🗃️document/🦀️.rs"]
pub(crate) mod document_ingress;
#[path = "../../👥️peer/🦀️.rs"]
pub(crate) mod peer_roster;
#[path="../../🧩️composition/📨️emission/🌱️genesis/📄️input/🦀️.rs"]
pub(crate) mod private_child_input;
#[path = "../../🧩️composition/📬️publication/🦀️.rs"]
pub(crate) mod store_publication;
//#endregion 📬️PublicationFacet
//#region 👷️JobsFacet
#[path = "../../🎮️command/📥️admission/🦀️.rs"]
pub(crate) mod command_admission;
#[path = "../../🎮️command/🦀️.rs"]
pub(crate) mod command_history;
#[path="../../🧩️extension/📨️invoke/⚠️failure/🦀️.rs"]
pub(crate) mod extension_invocation_failure;
#[path = "../../🎞️media/🦀️.rs"]
pub(crate) mod media_export;
#[path="../../🎞️media/⚠️turn/🦀️.rs"]
pub(crate) mod media_operation_refusal;
#[path="../../👷️job/🧾️bytes/🦀️.rs"]
pub(crate) mod reserved_job_bytes;
#[path="../../👷️job/📤️outcomes/🦀️.rs"]
pub(crate) mod reserved_job_outcomes;
#[path = "../../👷️job/🏛️reserved/🦀️.rs"]
pub(crate) mod reserved_jobs;
#[path = "../../🧵️retained-command/🦀️.rs"]
pub mod retained_command;
#[path = "../../👷️job/🦀️.rs"]
pub(crate) mod tool_jobs;
#[path = "../../🧵️retained-command/🔄️full-operation/🦀️.rs"]
pub(crate) mod typed_operation;
//#endregion 👷️JobsFacet
//#region ⚛️ReactorFacet
#[path = "../../🛂️describe/🦀️.rs"]
pub mod describe;
#[path="../../💡️inference/🚪️gateway/🦀️.rs"]
pub(crate) mod inference_gateway;
pub use inference_gateway::{cancel_artifact_inference,wire_artifact_infer_cold,ArtifactInferenceGateway,ArtifactInferenceGatewayStep,wire_artifact_infer,original_inference_gateway_demands};
//#endregion ⚛️ReactorFacet
