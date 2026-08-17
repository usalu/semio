//! 🛂️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §1 + §3). `describe::describe` —
//! build-time only, never called at runtime by the OS. Builds a real `manifest::PackageDescriptor`
//! (packet E1's type, already landed) from the SAME `Plugin`/`PluginManifest` bundle
//! `plugin_runtime::plugin_manifest()` already exposes.
//!
//! ⚠️ Scope note (reported honestly): `activation_events`/`capability_requests`/`extension_points`/
//! `execution`/`quotas` are emitted EMPTY/default — `PluginBuilder` (`🏗️builder/🦀️component.rs`,
//! this packet's own owned path) does not yet have `.activation(..)`/`.extension_point(..)`/
//! `.requests(..)`/`.quota(..)`/`.execution(..)` methods for a plugin author to populate them with
//! (design-abi.md §3 assigns that builder wiring to this same packet; not reached in this wave's
//! remaining budget — flagged in the report, not silently skipped). `hashes` are empty strings:
//! `wasm_sha256`/`core_wasm_sha256` can only be computed AFTER this component is built (by the
//! external `semio-framework-plugin-describe` bin crate, packet E1), never by `describe()` itself
//! running inside the not-yet-hashed wasm.

use semio_framework::{ContributionSet, ExecutionMode, PackageDescriptor, PackageHashes, PackageRole};

pub fn describe_plugin() -> Vec<u8> {
    let manifest = crate::plugin_runtime::plugin_manifest();
    let descriptor = PackageDescriptor {
        descriptor_version: 1,
        role: PackageRole::Plugin,
        manifest,
        activation_events: Vec::new(),
        capability_requests: Vec::new(),
        extension_points: Vec::new(),
        execution: ExecutionMode::default(),
        quotas: semio_framework::kernel::QuotaSchema::default(),
        contributions: ContributionSet::default(),
        assets: Vec::new(),
        hashes: PackageHashes { wasm_sha256: String::new(), core_wasm_sha256: String::new(), descriptor_sha256: String::new() },
    };
    store::pack_rt::encode_wire_value(&dsl::to_dsl_value(&descriptor).unwrap_or(dsl::DslValue::Null))
}
