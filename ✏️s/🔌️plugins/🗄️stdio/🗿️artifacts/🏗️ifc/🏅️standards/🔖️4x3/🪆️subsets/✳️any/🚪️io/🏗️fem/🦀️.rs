//! 🏗️ Native ADD2 structural and BIM domain exchange: FEM and BIM snapshots compose with the retained IFC4x3 graph under bounded, atomic projection.
#[cfg(test)]
#[global_allocator]
static IFC_ORIGINAL_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

use pack::snapshot::ArtifactPack;
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_diagnostic::{Diagnostic, Severity, TextSpan};
use semio_plugin::io::{AnalyzeSource, ComposeError, ComposeSource, ComposerEntry, Composition, Confidence};
use semio_plugin::ArtifactComposer;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::{geometry, snapshot::{IfcSnapshot, IfcValue}};
use semio_os_kernel::io::{io_mechanism::{self, ArchiveChildren, Deserializer, IoEntry, IoRunControl, Serializer}, io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult}};
use pack::snapshot::{ArtifactNativeSnapshot, NativeSnapshotEncoding, NativeSnapshotInput};
use std::sync::OnceLock;
use semio_s_artifact_stdio_ifc4x3_exchange::{base_io_entries, bim, checkpoint, composition_error, error, graph, loss, original_pack, projection, ExchangeError, ExchangeProgress, GLTF_DIALECT};
use semio_s_artifact_stdio_ifc4x3_schema as catalog;
use semio_s_artifact_stdio_ifc4x3_step as io;
mod validation {
    pub use semio_s_artifact_stdio_ifc4x3_express::metadata;
}
#[path = "../native_fem.rs"]
pub mod fem;
#[path = "../native_fem_owned.rs"]
mod fem_owned;
#[path = "../native_fem_build.rs"]
mod fem_build;
#[cfg(test)]
#[path = "../fem/nodal/native-support.rs"]
mod nodal_support;
#[cfg(test)]
#[path = "../fem/self-weight/native-exchange-tests.rs"]
mod self_weight_tests;
#[cfg(test)]
#[path = "../fem/nodal/native-export-tests.rs"]
mod nodal_export_tests;
#[cfg(test)]
#[path = "../fem/nodal/native-tests.rs"]
mod fem_nodal_tests;
#[cfg(test)]
#[path = "../🧪️tests/native_fem_owned.rs"]
mod fem_owned_tests;
#[path = "../native_domains.rs"]
pub mod domains;
#[cfg(test)]
#[path = "../🧪️tests/native_bim.rs"]
mod bim_tests;

/// 📡 Controlled host routes of every carrier and domain exchange.
pub fn io_entries() -> &'static [IoEntry] {
    static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            let mut entries = base_io_entries();
            entries.extend(domains::io_entries());
            entries
        })
        .as_slice()
}
