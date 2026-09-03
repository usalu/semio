//! Stdio plugin glue — zero-app library of well-known file-format artifacts.
//!
//! WIRING ONLY. Every pub mod points at one taxonomy component via #[path].

// 🚫️async: R7 — `async fn` in a public trait warns because auto trait bounds (e.g. `Send`) cannot
// be named on the method. Answered structurally per R3: every former `dyn` seam in this crate is a
// concrete enum or a generic parameter, so `Send` is derived at each call site from the concrete
// type, never from a bound on the trait method's returned future. This crate is guest-reachable, so
// its futures are deliberately `?Send` — do not "fix" this warning by adding `-> impl Future<..> +
// Send` (contradicts R3) or by making a trait method sync (contradicts O1/R1).
#![allow(async_fn_in_trait)]

// 🧮️const-eval: `#[derive(dsl::Mutations)]` validates its whole leaf roster inside the
// `DESCRIPTORS` const — every descriptor's kind, variant, verb and taxonomy path, byte by byte.
// `🧊️gltf` alone declares 120 leaves, so that one const legitimately exceeds rustc's
// `long_running_const_eval` step budget. The lint exists to catch infinite loops in const eval;
// this evaluation terminates (it is a bounded walk over a fixed roster) and the compiler's own
// help text names allowing it as the remedy for a genuinely long evaluation.
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;
// 🚚 Wave MATHEND (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS):
// `graph_core` aliases `semio_framework_graph` so `✳️graph/🧬️schema/🚶️traversal-internals`/
// `🔧️operators-internals`/`➕️normal-internals`/`🔌️ports-internals` — relocated verbatim from
// `🧰️framework/🔨️modules/🧮️math/🕸️graph/{🚶️traversal,🔧️operators,➕️normal,🔌️ports}` (ZERO consumers
// anywhere in the repo, migrated anyway under the "nothing deleted" rule) — compile unedited; those
// files already say `graph_core::…` throughout (their original alias, set by `🧮️math`'s own
// `🦀️.rs`), so aliasing here instead of a repo-wide `semio_framework_graph::` rewrite preserves
// them byte-for-byte.
extern crate semio_framework_graph as graph_core;

//#region MutationWire
/// 📡 Supplies the canonical JSON text/binary wire representation for schema-owned mutation
/// vocabularies over `ToValue`/`FromValue` (`Mutation`'s own supertraits — see
/// `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`) bridged through `pack::json` for the
/// literal text/byte shape (`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs`'s
/// `json_from_dsl_value`/`json_to_dsl_value`, since `DslValue` and `pack::json::Value` are sibling
/// trees with no shared type).
macro_rules! impl_serde_op_codec {
    ($mutation:ty, $what:literal) => {
        impl protocol::OpText for $mutation {
            fn print_op(&self) -> String {
                pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(self)))
            }

            fn parse_op(line: &str) -> Result<Self, store::TextError> {
                let parsed = pack::parse_json(line).map_err(|error| store::TextError::new(error.to_string(), dsl::TextSpan::at(1, 1)))?;
                <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| store::TextError::new(error.to_string(), dsl::TextSpan::at(1, 1)))
            }
        }

        impl protocol::OpBinary for $mutation {
            fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
                Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(self))).into_bytes())
            }

            fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
                let parsed = pack::parse_json_bytes(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: $what, offset: 0, detail: error.to_string() })?;
                <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: $what, offset: 0, detail: error.to_string() })
            }
        }
    };
}
pub(crate) use impl_serde_op_codec;
//#endregion MutationWire

//#region Base64
/// 🔡 Encodes bytes with the padded RFC 4648 standard Base64 alphabet.
pub fn base64_standard(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::with_capacity(bytes.len().div_ceil(3).saturating_mul(4));
    let mut chunks = bytes.chunks_exact(3);
    for chunk in &mut chunks {
        let value = u32::from_be_bytes([0, chunk[0], chunk[1], chunk[2]]);
        output.push(ALPHABET[((value >> 18) & 63) as usize] as char);
        output.push(ALPHABET[((value >> 12) & 63) as usize] as char);
        output.push(ALPHABET[((value >> 6) & 63) as usize] as char);
        output.push(ALPHABET[(value & 63) as usize] as char);
    }
    match chunks.remainder() {
        [first] => {
            output.push(ALPHABET[(first >> 2) as usize] as char);
            output.push(ALPHABET[((first & 3) << 4) as usize] as char);
            output.push_str("==");
        }
        [first, second] => {
            output.push(ALPHABET[(first >> 2) as usize] as char);
            output.push(ALPHABET[(((first & 3) << 4) | (second >> 4)) as usize] as char);
            output.push(ALPHABET[((second & 15) << 2) as usize] as char);
            output.push('=');
        }
        _ => {}
    }
    output
}
//#endregion Base64

//#region SemanticFingerprint
fn hash_hex_bytes(hash: &str) -> Vec<u8> {
    fn nibble(byte: u8) -> u8 {
        match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            _ => unreachable!("framework hash must be lowercase hexadecimal"),
        }
    }

    hash.as_bytes().chunks_exact(2).map(|pair| nibble(pair[0]) << 4 | nibble(pair[1])).collect()
}

/// 🪪️ Computes the stable BLAKE3 identity of a semantic projection.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semantic_fingerprint<T: dsl::ToValue>(projection: &T) -> Result<Vec<u8>, String> {
    let encoded = pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(projection))).into_bytes();
    Ok(hash_hex_bytes(&semio_framework_hash::hash_bytes(&encoded)))
}
//#endregion SemanticFingerprint

//#region Plugin
#[path = "../../🦀️.rs"]
pub mod plugin;
pub use plugin::plugin;
// 🚀 Ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M0 — installs the process-wide plugin
// bundle, anchors the `component-guest`-gated wasm export against link-time dead-code elimination,
// and (E1-describe) adds the `#[cfg(test)] descriptor_is_fresh()` test that byte-compares
// `describe::describe_plugin()` against the committed `🛂️.descriptor.semio` at this crate's owner
// root. Mirrors `✏️s/🔌️plugins/🗒️note`'s own `semio_framework_plugin::plugin_exports!(plugin::plugin)`
// call verbatim — stdio never had this wired up before this packet.
#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, plugin::StdioApps);
//#endregion Plugin

//#region Registry
#[path = "../../📇️registry/🦀️.rs"]
pub mod registry;
//#endregion Registry

//#region Manifest
#[path = "../../🛂️manifest/🦀️.rs"]
pub mod manifest;
//#endregion Manifest

//#region Store
/// 🗄️ Names the document-codec traits every snapshot in this crate implements, so a consumer can
/// actually call the impls: `ArtifactDsl::print_dsl`/`parse_dsl` and `ArtifactPack::encode_pack`/
/// `decode_pack`. The impls were public while the traits were reachable only through the private
/// `store` extern-crate alias above, which made them uncallable from outside — see
/// `🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🔣️oracle.json`, which recorded
/// exactly that gap.
pub use semio_framework_os_kernel::{ArtifactDsl, ArtifactPack};
//#endregion Store

//#region Artifacts
// 🎫️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, S2 mutation-triad
// mounting policy (load-bearing for F1-F6): `POLICY_MUTATION_TRIAD_DIRS`
// (📜️script.ts's `policyMutationTriadCompletenessBreaches`) does NOT require a `🧬️mutations/📄<variant>/
// {🦠️mutation,🔺️diff,↩️inverse}` triad directory per mutation variant — it only checks triad-kind
// completeness for mutation dirs that ALREADY EXIST as subdirectories, and `policyMutationDispatchCoverageBreaches`
// (variant-vs-triad-dir coverage) is a deliberate no-op placeholder. Verified empirically at S2: every
// one of the 31 stdio standards' non-SetSnapshot mutation variants (gif 89a's InsertFrame/RemoveFrame/…,
// svg's InsertElement/…) live entirely inline in the top-level `🧬️mutations/🦀️.rs` — only
// `📄set-snapshot` has its own triad dir, anywhere in this crate. Every top-level facet file
// (`🧬️mutations/🦀️.rs`, `🔺️diff/🦀️.rs`, `📸️snapshot/🦀️.rs`) is already
// mounted below for all 31 standards. CONSEQUENCE for F1-F6 fan-out agents: you can do ALL of your real
// work — new mutation variants, new diff fields, new snapshot fields — by editing your artifact's
// already-mounted top-level facet files, with ZERO edits to this file and ZERO new directories. A
// per-variant triad dir is optional scaffolding (for consistency with set-snapshot), never required for
// correctness; if you want one anyway, it needs a NEW directory, which structurally requires a glue.rs
// mount — queue it in your report's `glue_followup` field for this wave's closer instead of touching
// this file yourself. See `s2-spine-report.md` in this ticket folder for the full writeup.
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod binary {
        #[path = "../../🗿️artifacts/💾️binary/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_raw {
                // 🌳️ Standard root (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P
                // pilot): `standard() -> StandardDeclaration`, mounts subset `any` below.
                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🦀️.rs"]
                mod component;
                pub use component::*;

                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // codecs already lived beside `BinarySnapshot`'s `ArtifactDsl`/`ArtifactPack` impls in
                // `subsets::any::schema::snapshot` (untouched); `empty_binary_snapshot`/
                // `demo_binary_snapshot` moved to `subsets::any::schema`; `BinaryEngine` (zero
                // construction sites repo-wide) deleted outright; the register cluster + `io_registry`
                // moved to `subsets::any::io`; tests moved into `subsets::any::schema::inferences`.
                // `register()` is one of stdio's 10 protected imperative plugin-root calls
                // (`crate::artifacts::binary::engine::register()` in `🗄️stdio/🦀️.rs`, reached
                // via this artifact's own top-level `pub mod engine` shim below) — left callable at
                // this exact path via a pure re-export of `subsets::any::io::register` (itself
                // unchanged).
                pub mod engine {
                    pub use super::subsets::any::io::register;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        // 🪆️ Subset root (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM,
                        // W2-P pilot): `subset() -> SubsetDeclaration`, assembles the schema/io/
                        // viewer/editor/examples children mounted below (and `crate::editor::binary`/
                        // `crate::viewer::binary`, mounted at the plugin's top-level `editor`/`viewer`
                        // modules, not here — see that file's own doc comment).
                        #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod extent {
                                    #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/💡️inferences/📏extent/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_raw::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_raw::engine::*;
        }
        pub mod io {
            pub use super::standards::v_raw::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod txt {
        #[path = "../../🗿️artifacts/📄️txt/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_utf_8 {
                // 🌳️ Standard root (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P
                // pilot): `standard() -> StandardDeclaration`, mounts subset `any` below.
                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🦀️.rs"]
                mod component;
                pub use component::*;

                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // `TxtEngine` (zero construction sites) deleted outright; `empty_txt_snapshot`/
                // `demo_txt_snapshot` moved to `subsets::any::schema`; register cluster + `io_registry`
                // moved to `subsets::any::io`; tests moved beside what they test in `subsets::any::
                // schema`. `register()` is one of stdio's 10 protected imperative plugin-root calls
                // (`crate::artifacts::txt::engine::register()` in `🗄️stdio/🦀️.rs`, reached via
                // this artifact's own top-level `pub mod engine` shim) — left callable at this exact
                // path via a pure re-export of `subsets::any::io::register` (itself unchanged).
                pub mod engine {
                    pub use super::subsets::any::io::register;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        // 🪆️ Subset root (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM,
                        // W2-P pilot): `subset() -> SubsetDeclaration` — see `💾️binary`'s mirrored
                        // mount above for the shared reasoning.
                        #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔨️modules/🧬️mutation-support/🦀️.rs"]
                            pub mod mutation_support;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_utf_8::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_utf_8::engine::*;
        }
        pub mod io {
            pub use super::standards::v_utf_8::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod json {
        #[path = "../../🗿️artifacts/🔣️json/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_rfc8259 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/🔨️modules/🧬️mutation-support/🦀️.rs"]
                            pub mod mutation_support;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/🧬️mutations/✏️set-member/🦀️.rs"]
                                pub mod set_member;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🗑️remove-member/🦀️.rs"]
                                pub mod remove_member;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📥️insert-array-element/🦀️.rs"]
                                pub mod insert_array_element;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🗑️remove-array-element/🦀️.rs"]
                                pub mod remove_array_element;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/🧬️mutations/✏️set-scalar/🦀️.rs"]
                                pub mod set_scalar;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod i_json {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️i-json/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;

                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️i-json/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️i-json/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_rfc8259::subsets::base::schema::*;
        }
        pub mod io {
            pub use super::standards::v_rfc8259::subsets::base::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod xml {
        #[path = "../../🗿️artifacts/📰️xml/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_0 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                        // `XmlEngine` (zero construction sites) deleted outright; its orphaned
                        // `register()`/`register_artifact_schema()`/`register_artifact_inferences()`/
                        // `register_pilot_languages()` (zero callers, superseded by `xml::declaration()`)
                        // deleted outright too; `io_registry` moved to `subsets::base::io`;
                        // `empty_xml_snapshot`/`demo_xml_snapshot` + tests moved to `subsets::base::schema`.
                        // xml has no dedicated codec of its own to move -- the real text codec
                        // (`xml_document_from_text`/`xml_document_to_text`) already lives in
                        // `subsets::base::schema::snapshot`, unmoved. xml is NOT one of stdio's 10
                        // protected imperative plugin-root `engine::register()` calls, so no `engine`
                        // shim remains — no external caller ever reached `xml::…::engine::` (confirmed
                        // repo-wide).
                        #[path = "."]
                        pub mod examples {
                            #[path = "."]
                            pub mod demo {
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/📚️examples/🎬️demo/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🔨️modules/🧬️mutation-support/🦀️.rs"]
                            pub mod mutation_support;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🧬️mutations/✏️set-declaration/🦀️.rs"]
                                pub mod set_declaration;
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🧬️mutations/✏️set-doctype/🦀️.rs"]
                                pub mod set_doctype;
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📥️insert-element/🦀️.rs"]
                                pub mod insert_element;
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🗑️remove-element/🦀️.rs"]
                                pub mod remove_element;
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🧬️mutations/✏️set-attribute/🦀️.rs"]
                                pub mod set_attribute;
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🧬️mutations/✏️set-text/🦀️.rs"]
                                pub mod set_text;
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod valid {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod examples {
                            #[path = "."]
                            pub mod no_doctype {
                                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/📚️examples/🚫️no-doctype/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_0::subsets::base::schema::*;
        }
        pub mod io {
            pub use super::standards::v1_0::subsets::base::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod csv {
        #[path = "../../🗿️artifacts/📊️csv/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_rfc4180 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod examples {
                            #[path = "."]
                            pub mod demo {
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_rfc4180::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v_rfc4180::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod md {
        #[path = "../../🗿️artifacts/📝️md/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_commonmark {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // `MdEngine` (zero construction sites) deleted outright; its orphaned `register()`/
                // `register_artifact_schema()`/`register_artifact_inferences()`/
                // `register_pilot_languages()` (zero callers, superseded by `md::declaration()`)
                // deleted outright too; `parse_markdown_blocks` + the block/inline parser moved to
                // `subsets::any::io::import::deserializers`; `render_markdown_blocks` + the
                // block/inline renderer moved to `subsets::any::io::export::serializers`;
                // `io_registry` moved to `subsets::any::io`; `empty_md_snapshot`/`demo_md_snapshot`
                // + tests moved to `subsets::any::schema`. md is NOT one of stdio's 10 protected
                // imperative plugin-root `engine::register()` calls, so no `engine` shim remains —
                // external callers (🔱️trinity's jack/rewrite, 📜️imperative) were repointed to the
                // new `subsets::any::io::{import::deserializers,export::serializers}` paths.
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_commonmark::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v_commonmark::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod deflate {
        #[path = "../../🗿️artifacts/🗜️deflate/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_rfc1950 {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // pure format algorithms (Adler32/BitIO/Huffman/LZ77) + `zlib_compress`/
                // `zlib_decompress` + `encode_deflate_snapshot`/`decode_deflate_snapshot` +
                // `io_registry` + `register_schema_specs` all moved into `subsets::any::io`
                // (rule 6: deflate's Huffman/LZ77 is the clearest "keep with the codec" case);
                // `empty_deflate_snapshot`/`demo_deflate_snapshot` moved to `subsets::any::schema`;
                // `DeflateEngine` (zero construction sites repo-wide) deleted outright; tests moved
                // into `subsets::any::io` (codec tests) and `subsets::any::schema::inferences`
                // (conformance laws). `deflate` is NOT one of stdio's 10 protected imperative
                // plugin-root calls (it already used the declarative `ArtifactDeclaration` builder)
                // — every call site was repointed directly at the new locations, and no `engine`
                // shim survives here since nothing references it anymore.
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod window {
                                    #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪟window/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_rfc1950::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v_rfc1950::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod zip {
        #[path = "../../🗿️artifacts/🎒️zip/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v2_0 {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // CRC32 + CP437 + extra-fields + EOCD parsing + `decode_zip`/`encode_zip`/
                // `sniff_zip_bytes`/`SniffConfidence`/`ZipError` all moved into `subsets::any::io`
                // (rule 2: this is the codec); `empty_zip_snapshot`/`demo_zip_snapshot` moved to
                // `subsets::any::schema`; `ZipEngine` (zero construction sites repo-wide) deleted
                // outright; tests moved into `subsets::any::io` (codec tests) and
                // `subsets::any::schema::inferences` (conformance laws). `zip` is NOT one of stdio's
                // 10 protected imperative plugin-root calls (it already used the declarative
                // `ArtifactDeclaration` builder) — every call site (incl. `📜️docx`/`📕️xlsx`/`🎞️pptx`'s
                // own zip-wrap export paths and `💬️bcf`'s zip-container sniff) was repointed directly
                // at the new locations, and no `engine` shim survives here since nothing references
                // it anymore.
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod entries {
                                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧬️schema/💡️inferences/🗃entries/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod deflate {
                                            #[path = "."]
                                            pub mod v_rfc1950 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod deflate {
                                            #[path = "."]
                                            pub mod v_rfc1950 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod iso21320 {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v2_0::subsets::base::schema::*;
        }
        pub mod io {
            pub use super::standards::v2_0::subsets::base::io::*;
        }

        /// 📦️ Shared OPC (Open Packaging Conventions) layer — zip-and-XML container plumbing
        /// that `docx`/`xlsx`/`pptx` import cross-artifact (`crate::artifacts::zip::opc::*`).
        #[path = "."]
        pub mod opc {
            #[path = "../../🗿️artifacts/🎒️zip/📦️opc/🦀️.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod step {
        #[path = "../../🗿️artifacts/📐️step/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ap214 {
                // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
                // so every existing `standards::v_ap214::engine::*`/root `engine::*` path still resolves.
                pub mod engine {
                    pub use super::subsets::base::io::*;
                    pub use super::subsets::base::schema::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod cc1 {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc1/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc1/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod cc2 {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc2/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc2/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod cc3 {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc3/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc3/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod cc4 {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc4/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc4/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod cc5 {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc5/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc5/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod cc6 {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ap214::subsets::base::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_ap214::engine::*;
        }
        pub mod io {
            pub use super::standards::v_ap214::subsets::base::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod ifc {
        #[path = "../../🗿️artifacts/🏗️ifc/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v4 {
                // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
                // so every existing `standards::v4::engine::*` path still resolves — including
                // `register()`, deliberately left imperative and callable (ticket instruction: do
                // not touch ifc's registration mechanism), called explicitly from this artifact's
                // own root `engine` shim (which also glob-imports this standard as the default).
                pub mod engine {
                    pub use super::subsets::any::io::*;
                    pub use super::subsets::any::schema::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod v2x3 {
                // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
                // so every existing `standards::v2x3::engine::*` path still resolves — including
                // `register()`, deliberately left imperative and callable (ticket instruction: do
                // not touch ifc's registration mechanism), called explicitly from this artifact's
                // own root `engine` shim below.
                pub mod engine {
                    pub use super::subsets::base::io::*;
                    pub use super::subsets::base::schema::*;
                }
                // 🏗️ Part-21 editing primitives the three model-view-definition subsets
                // (✳️cv20/✳️cobie/✳️sav) genuinely share — an MVD is a conformance filter over one
                // schema, so their vocabularies differ in meaning, never in mechanics. Mounted at
                // the STANDARD level rather than copied into each subset's own mutations module.
                #[path = "."]
                pub mod mvd {
                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🧬️mvd/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod cv20 {
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod sav {
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️sav/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️sav/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod cobie {
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v4::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v4::engine::*;
            /// 📎 Registers BOTH standards' engines (v4 canonical + v2x3 new-this-ticket) -- a
            /// flat glob re-export can't do this (two `register` fns of the same name would
            /// collide), so this local definition shadows the glob-imported v4 one and calls both
            /// explicitly. Same shape as pdf's own shim fix for 1.4/1.7.
            // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
            pub fn register() {
                super::standards::v4::engine::register();
                super::standards::v2x3::engine::register();
            }
        }
        pub mod io {
            pub use super::standards::v4::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod las {
        #[path = "../../🗿️artifacts/☁️las/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_0 {
                // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
                // so every existing `standards::v1_0::engine::*`/root `engine::*` path still resolves.
                pub mod engine {
                    pub use super::subsets::any::io::*;
                    pub use super::subsets::any::schema::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v1_0::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod gltf {
        #[path = "../../🗿️artifacts/🧊️gltf/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v2_0 {
                // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
                // so every existing `standards::v2_0::engine::*`/root `engine::*` path still resolves.
                pub mod engine {
                    pub use super::subsets::any::io::*;
                    pub use super::subsets::any::schema::*;
                    // 🎯 io and schema each define their own inferences/mutations submodule
                    // (io holds binary/text codecs, schema holds the actual domain logic) --
                    // disambiguate the resulting glob collision by explicitly preferring
                    // schema, the richer, load-bearing side.
                    pub use super::subsets::any::schema::inferences;
                    pub use super::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔗️adjacency/🦀️.rs"]
                                pub mod adjacency;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧱️area-volume/🦀️.rs"]
                                pub mod area_volume;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/↔️clearance/🦀️.rs"]
                                pub mod clearance;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚪️compactness/🦀️.rs"]
                                pub mod compactness;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🕳️concavity/🦀️.rs"]
                                pub mod concavity;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌀️curvature/🦀️.rs"]
                                pub mod curvature;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔨️dag-assembly/🦀️.rs"]
                                pub mod dag_assembly;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔨️geometry-core/🦀️.rs"]
                                pub mod geometry_core;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️mass-distribution/🦀️.rs"]
                                pub mod mass_distribution;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭️orientation/🦀️.rs"]
                                pub mod orientation;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📏️proportion/🦀️.rs"]
                                pub mod proportion;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌊️roughness/🦀️.rs"]
                                pub mod roughness;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️size/🦀️.rs"]
                                pub mod size;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪞️symmetry/🦀️.rs"]
                                pub mod symmetry;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/↕️thickness/🦀️.rs"]
                                pub mod thickness;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🕸️topology/🦀️.rs"]
                                pub mod topology;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🎬️bind-default-scene/🦀️.rs"]
                                pub mod bind_default_scene;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🧬️bind-morph-target-attribute/🦀️.rs"]
                                pub mod bind_morph_target_attribute;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔘️bind-node-camera/🦀️.rs"]
                                pub mod bind_node_camera;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔘️bind-node-child/🦀️.rs"]
                                pub mod bind_node_child;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔘️bind-node-mesh/🦀️.rs"]
                                pub mod bind_node_mesh;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔘️bind-node-skin/🦀️.rs"]
                                pub mod bind_node_skin;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔺️bind-primitive-attribute/🦀️.rs"]
                                pub mod bind_primitive_attribute;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔺️bind-primitive-indices/🦀️.rs"]
                                pub mod bind_primitive_indices;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔺️bind-primitive-material/🦀️.rs"]
                                pub mod bind_primitive_material;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🎬️bind-scene-root-node/🦀️.rs"]
                                pub mod bind_scene_root_node;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️📦️change-asset-descriptive-metadata/🦀️.rs"]
                                pub mod change_asset_descriptive_metadata;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️📦️change-asset-extension-data/🦀️.rs"]
                                pub mod change_asset_extension_data;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️📦️change-asset-extra-data/🦀️.rs"]
                                pub mod change_asset_extra_data;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️📦️change-asset-version/🦀️.rs"]
                                pub mod change_asset_version;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️📄️change-document-extension-data/🦀️.rs"]
                                pub mod change_document_extension_data;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️📄️change-document-extra-data/🦀️.rs"]
                                pub mod change_document_extra_data;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️💎️change-material-alpha-mode/🦀️.rs"]
                                pub mod change_material_alpha_mode;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️💎️change-material-double-sided/🦀️.rs"]
                                pub mod change_material_double_sided;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🕸️change-mesh-extension-data/🦀️.rs"]
                                pub mod change_mesh_extension_data;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🕸️change-mesh-extra-data/🦀️.rs"]
                                pub mod change_mesh_extra_data;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🕸️change-mesh-morph-weights/🦀️.rs"]
                                pub mod change_mesh_morph_weights;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🕸️change-mesh-name/🦀️.rs"]
                                pub mod change_mesh_name;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-extension-data/🦀️.rs"]
                                pub mod change_node_extension_data;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-extra-data/🦀️.rs"]
                                pub mod change_node_extra_data;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-morph-weights/🦀️.rs"]
                                pub mod change_node_morph_weights;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-name/🦀️.rs"]
                                pub mod change_node_name;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔺️change-primitive-extension-data/🦀️.rs"]
                                pub mod change_primitive_extension_data;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔺️change-primitive-extra-data/🦀️.rs"]
                                pub mod change_primitive_extra_data;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔺️change-primitive-topology-mode/🦀️.rs"]
                                pub mod change_primitive_topology_mode;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🎬️change-scene-extension-data/🦀️.rs"]
                                pub mod change_scene_extension_data;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🎬️change-scene-extra-data/🦀️.rs"]
                                pub mod change_scene_extra_data;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🎬️change-scene-name/🦀️.rs"]
                                pub mod change_scene_name;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️📐️create-accessor/🦀️.rs"]
                                pub mod create_accessor;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🎞️create-animation/🦀️.rs"]
                                pub mod create_animation;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️💾️create-buffer/🦀️.rs"]
                                pub mod create_buffer;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️👁️create-buffer-view/🦀️.rs"]
                                pub mod create_buffer_view;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🎥️create-camera/🦀️.rs"]
                                pub mod create_camera;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🖼️create-image/🦀️.rs"]
                                pub mod create_image;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️💎️create-material/🦀️.rs"]
                                pub mod create_material;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🕸️create-mesh/🦀️.rs"]
                                pub mod create_mesh;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🧬️create-morph-target/🦀️.rs"]
                                pub mod create_morph_target;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🔘️create-node/🦀️.rs"]
                                pub mod create_node;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🔺️create-primitive/🦀️.rs"]
                                pub mod create_primitive;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🎛️create-sampler/🦀️.rs"]
                                pub mod create_sampler;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🎬️create-scene/🦀️.rs"]
                                pub mod create_scene;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🧥️create-skin/🦀️.rs"]
                                pub mod create_skin;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🎨️create-texture/🦀️.rs"]
                                pub mod create_texture;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📣️🧩️add-used-extension/🦀️.rs"]
                                pub mod add_used_extension;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️📐️delete-accessor/🦀️.rs"]
                                pub mod delete_accessor;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🎞️delete-animation/🦀️.rs"]
                                pub mod delete_animation;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️💾️delete-buffer/🦀️.rs"]
                                pub mod delete_buffer;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️👁️delete-buffer-view/🦀️.rs"]
                                pub mod delete_buffer_view;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🎥️delete-camera/🦀️.rs"]
                                pub mod delete_camera;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🖼️delete-image/🦀️.rs"]
                                pub mod delete_image;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️💎️delete-material/🦀️.rs"]
                                pub mod delete_material;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🕸️delete-mesh/🦀️.rs"]
                                pub mod delete_mesh;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🧬️delete-morph-target/🦀️.rs"]
                                pub mod delete_morph_target;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🔘️delete-node/🦀️.rs"]
                                pub mod delete_node;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🔺️delete-primitive/🦀️.rs"]
                                pub mod delete_primitive;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🎛️delete-sampler/🦀️.rs"]
                                pub mod delete_sampler;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🎬️delete-scene/🦀️.rs"]
                                pub mod delete_scene;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🧥️delete-skin/🦀️.rs"]
                                pub mod delete_skin;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🎨️delete-texture/🦀️.rs"]
                                pub mod delete_texture;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️📐️move-accessor/🦀️.rs"]
                                pub mod move_accessor;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🎞️move-animation/🦀️.rs"]
                                pub mod move_animation;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️💾️move-buffer/🦀️.rs"]
                                pub mod move_buffer;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️👁️move-buffer-view/🦀️.rs"]
                                pub mod move_buffer_view;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🎥️move-camera/🦀️.rs"]
                                pub mod move_camera;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🖼️move-image/🦀️.rs"]
                                pub mod move_image;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️💎️move-material/🦀️.rs"]
                                pub mod move_material;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🕸️move-mesh/🦀️.rs"]
                                pub mod move_mesh;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🧬️move-morph-target/🦀️.rs"]
                                pub mod move_morph_target;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🧬️move-morph-target-attribute/🦀️.rs"]
                                pub mod move_morph_target_attribute;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🔘️move-node/🦀️.rs"]
                                pub mod move_node;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🔘️move-node-child/🦀️.rs"]
                                pub mod move_node_child;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🔺️move-primitive/🦀️.rs"]
                                pub mod move_primitive;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🔺️move-primitive-attribute/🦀️.rs"]
                                pub mod move_primitive_attribute;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🧩️move-required-extension/🦀️.rs"]
                                pub mod move_required_extension;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🎛️move-sampler/🦀️.rs"]
                                pub mod move_sampler;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🎬️move-scene/🦀️.rs"]
                                pub mod move_scene;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🎬️move-scene-root-node/🦀️.rs"]
                                pub mod move_scene_root_node;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🧥️move-skin/🦀️.rs"]
                                pub mod move_skin;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🎨️move-texture/🦀️.rs"]
                                pub mod move_texture;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🧩️move-used-extension/🦀️.rs"]
                                pub mod move_used_extension;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️📐️reorder-accessors/🦀️.rs"]
                                pub mod reorder_accessors;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🎞️reorder-animations/🦀️.rs"]
                                pub mod reorder_animations;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️👁️reorder-buffer-views/🦀️.rs"]
                                pub mod reorder_buffer_views;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️💾️reorder-buffers/🦀️.rs"]
                                pub mod reorder_buffers;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🎥️reorder-cameras/🦀️.rs"]
                                pub mod reorder_cameras;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🖼️reorder-images/🦀️.rs"]
                                pub mod reorder_images;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️💎️reorder-materials/🦀️.rs"]
                                pub mod reorder_materials;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🕸️reorder-meshs/🦀️.rs"]
                                pub mod reorder_meshs;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🧬️reorder-morph-target-attributes/🦀️.rs"]
                                pub mod reorder_morph_target_attributes;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🧬️reorder-morph-targets/🦀️.rs"]
                                pub mod reorder_morph_targets;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🔘️reorder-node-children/🦀️.rs"]
                                pub mod reorder_node_children;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🔘️reorder-nodes/🦀️.rs"]
                                pub mod reorder_nodes;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🔺️reorder-primitive-attributes/🦀️.rs"]
                                pub mod reorder_primitive_attributes;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🔺️reorder-primitives/🦀️.rs"]
                                pub mod reorder_primitives;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🧩️reorder-required-extensions/🦀️.rs"]
                                pub mod reorder_required_extensions;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🎛️reorder-samplers/🦀️.rs"]
                                pub mod reorder_samplers;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🎬️reorder-scene-root-nodes/🦀️.rs"]
                                pub mod reorder_scene_root_nodes;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🎬️reorder-scenes/🦀️.rs"]
                                pub mod reorder_scenes;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🧥️reorder-skins/🦀️.rs"]
                                pub mod reorder_skins;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🎨️reorder-textures/🦀️.rs"]
                                pub mod reorder_textures;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🧩️reorder-used-extensions/🦀️.rs"]
                                pub mod reorder_used_extensions;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👪️🔘️move-node-parent/🦀️.rs"]
                                pub mod move_node_parent;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️🧩️add-required-extension/🦀️.rs"]
                                pub mod add_required_extension;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️🔘️change-node-transform/🦀️.rs"]
                                pub mod change_node_transform;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🎬️unbind-default-scene/🦀️.rs"]
                                pub mod unbind_default_scene;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🧬️unbind-morph-target-attribute/🦀️.rs"]
                                pub mod unbind_morph_target_attribute;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🔘️unbind-node-camera/🦀️.rs"]
                                pub mod unbind_node_camera;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🔘️unbind-node-child/🦀️.rs"]
                                pub mod unbind_node_child;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🔘️unbind-node-mesh/🦀️.rs"]
                                pub mod unbind_node_mesh;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🔘️unbind-node-skin/🦀️.rs"]
                                pub mod unbind_node_skin;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🔺️unbind-primitive-attribute/🦀️.rs"]
                                pub mod unbind_primitive_attribute;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🔺️unbind-primitive-indices/🦀️.rs"]
                                pub mod unbind_primitive_indices;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🔺️unbind-primitive-material/🦀️.rs"]
                                pub mod unbind_primitive_material;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🎬️unbind-scene-root-node/🦀️.rs"]
                                pub mod unbind_scene_root_node;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️🧩️remove-required-extension/🦀️.rs"]
                                pub mod remove_required_extension;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔙️🧩️remove-used-extension/🦀️.rs"]
                                pub mod remove_used_extension;
                            }
                            #[path = "."]
                            pub mod modules {
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/💡️inference-measures/🦀️.rs"]
                                pub mod inference_measures;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧾️measurement-contracts/🦀️.rs"]
                                pub mod measurement_contracts;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🕸️mesh-topology/🦀️.rs"]
                                pub mod mesh_topology;
                                #[path = "."]
                                pub mod mutation_support {
                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧬️mutation-support/🎬️create-scene/🦀️.rs"]
                                    pub mod create_scene;
                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧬️mutation-support/🎞️material-animation/🦀️.rs"]
                                    pub mod material_animation;
                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧬️mutation-support/🧱️structure-geometry/🦀️.rs"]
                                    pub mod structure_geometry;
                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧬️mutation-support/🗂️top-level-collections/🦀️.rs"]
                                    pub mod top_level_collections;
                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧬️mutation-support/📚️top-level/🦀️.rs"]
                                    pub mod top_level;
                                }
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧮️vector-operations/🦀️.rs"]
                                pub mod vector_operations;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v2_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v2_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v2_0::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod metabolism {
                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/📚️examples/🌱️metabolism/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/📚️examples/🌱️metabolism/🧪️tests/🦀️.rs"]
                mod metabolism_tests;
            }
        }
    }
    #[path = "."]
    pub mod obj {
        #[path = "../../🗿️artifacts/🧊️obj/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v3_0 {
                // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
                // so every existing `standards::v3_0::engine::*`/root `engine::*` path still resolves.
                pub mod engine {
                    pub use super::subsets::any::io::*;
                    pub use super::subsets::any::schema::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v3_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v3_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v3_0::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod ply {
        #[path = "../../🗿️artifacts/☁️ply/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_0 {
                // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
                // so every existing `standards::v1_0::engine::*`/root `engine::*` path still resolves.
                pub mod engine {
                    pub use super::subsets::any::io::*;
                    pub use super::subsets::any::schema::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v1_0::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod dxf {
        #[path = "../../🗿️artifacts/🖊️dxf/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_r12 {
                // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
                // so every existing `standards::v_r12::engine::*`/root `engine::*` path still resolves.
                pub mod engine {
                    pub use super::subsets::any::io::*;
                    pub use super::subsets::any::schema::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_r12::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_r12::engine::*;
        }
        pub mod io {
            pub use super::standards::v_r12::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod stl {
        #[path = "../../🗿️artifacts/🟪️stl/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ascii {
                // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
                // so every existing `standards::v_ascii::engine::*`/root `engine::*` path still resolves.
                pub mod engine {
                    pub use super::subsets::any::io::*;
                    pub use super::subsets::any::schema::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ascii::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_ascii::engine::*;
        }
        pub mod io {
            pub use super::standards::v_ascii::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod svg {
        #[path = "../../🗿️artifacts/🎨️svg/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_1 {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::base::io` (codecs/io_registry) and
                // `subsets::base::schema` (document helpers); this stays an inline barrel so every
                // existing `standards::v1_1::engine::*`/root `engine::*` path still resolves.
                pub mod engine {
                    pub use super::subsets::base::io::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🔨️modules/🧬️mutation-support/🦀️.rs"]
                            pub mod mutation_support;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod dimensions {
                                    #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/💡️inferences/📐dimensions/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/✏️set-declaration/🦀️.rs"]
                                pub mod set_declaration;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/✏️set-doctype/🦀️.rs"]
                                pub mod set_doctype;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📥️insert-element/🦀️.rs"]
                                pub mod insert_element;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🗑️remove-element/🦀️.rs"]
                                pub mod remove_element;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/✏️set-element-name/🦀️.rs"]
                                pub mod set_element_name;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/✏️set-attribute/🦀️.rs"]
                                pub mod set_attribute;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/✏️set-text/🦀️.rs"]
                                pub mod set_text;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/✏️set-view-box/🦀️.rs"]
                                pub mod set_view_box;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/✏️set-transform/🦀️.rs"]
                                pub mod set_transform;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod tiny {
                        // 🏅️ SVG Tiny 1.1 (W3C Mobile SVG Profiles, REC-SVGMobile-20030114 §SVG
                        // Tiny 1.1). `schema` re-exports the ✳️any subset's `SvgSnapshot`
                        // verbatim (same Rust type, same `s.stdio.svg` schema id); `io` reuses
                        // the ✳️any subset's xml import/export leaves rather than duplicating
                        // them. Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES.
                        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod basic {
                        // 🏅️ SVG Basic 1.1 (W3C Mobile SVG Profiles, REC-SVGMobile-20030114 §SVG
                        // Basic 1.1). Same shape as `✳️tiny` above.
                        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_1::subsets::base::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1_1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1_1::subsets::base::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod bmp {
        #[path = "../../🗿️artifacts/🖼️bmp/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_v3 {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::any::io` (codec/register/io_registry —
                // `register` stays reachable since bmp is one of stdio's 10 deliberate imperative
                // `engine::register()` plugin-root calls) and `subsets::any::schema` (document
                // helpers); this stays an inline barrel so every existing
                // `standards::v_v3::engine::*`/root `engine::*` path still resolves.
                pub mod engine {
                    pub use super::subsets::any::io::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod dimensions {
                                    #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐dimensions/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod top_level;
                                pub use top_level::*;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-header-fields/🦀️.rs"]
                                pub mod change_header_fields;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-palette-entry/🦀️.rs"]
                                pub mod insert_palette_entry;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📤️remove-palette-entry/🦀️.rs"]
                                pub mod remove_palette_entry;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️replace-palette-entry/🦀️.rs"]
                                pub mod replace_palette_entry;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟪️replace-pixel-data/🦀️.rs"]
                                pub mod replace_pixel_data;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                            pub mod operations;
                            #[cfg(test)]
                            #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧪️tests/🧬️mutation-regressions/🦀️.rs"]
                            mod mutation_regressions;
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_v3::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_v3::engine::*;
        }
        pub mod io {
            pub use super::standards::v_v3::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod dwg {
        #[path = "../../🗿️artifacts/🖊️dwg/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ac1018 {
                // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
                // so every existing `standards::v_ac1018::engine::*` path still resolves. ac1018's
                // own register()/schema/inference/language registration was confirmed dead
                // repo-wide and deleted outright — only composer entries + document helpers moved.
                pub mod engine {
                    pub use super::subsets::any::io::*;
                    pub use super::subsets::any::schema::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod structure {
                                    #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗂structure/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod v_ac1024 {
                // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
                // so every existing `standards::v_ac1024::engine::*`/root `engine::*` path
                // (aliased to THIS standard, the canonical one) still resolves.
                pub mod engine {
                    pub use super::subsets::any::io::*;
                    pub use super::subsets::any::schema::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod structure {
                                    #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗂structure/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        // 🎫️26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: default
        // standard switched ac1018 -> ac1024 (real R2004+ D1/D2 decode; ac1018 was never real per
        // Decision #5). ac1018 stays mounted above, fully untouched, ONLY because several other
        // plugins' own composer entries target `Dialect{standard: StandardId("ac1018")}` directly
        // -- those keep compiling/working unchanged regardless of this shim switch.
        pub mod schema {
            pub use super::standards::v_ac1024::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_ac1024::engine::*;
        }
        pub mod io {
            pub use super::standards::v_ac1024::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod architectural {
                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🧪️tests/🦀️.rs"]
                mod architectural_tests;
            }
        }
    }
    #[path = "."]
    pub mod png {
        #[path = "../../🗿️artifacts/📷️png/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_2 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod dimensions {
                                    #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐dimensions/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod top_level;
                                pub use top_level::*;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-header/🦀️.rs"]
                                pub mod change_header;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️replace-palette/🦀️.rs"]
                                pub mod replace_palette;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-transparency/🦀️.rs"]
                                pub mod change_transparency;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗️change-gamma/🦀️.rs"]
                                pub mod change_gamma;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌈️change-chromaticities/🦀️.rs"]
                                pub mod change_chromaticities;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️change-srgb-intent/🦀️.rs"]
                                pub mod change_srgb_intent;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-physical-dims/🦀️.rs"]
                                pub mod change_physical_dims;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕰️change-timestamp/🦀️.rs"]
                                pub mod change_timestamp;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-background/🦀️.rs"]
                                pub mod change_background;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-text-chunk/🦀️.rs"]
                                pub mod insert_text_chunk;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-text-chunk/🦀️.rs"]
                                pub mod remove_text_chunk;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️replace-text-chunk/🦀️.rs"]
                                pub mod replace_text_chunk;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟪️replace-pixels/🦀️.rs"]
                                pub mod replace_pixels;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️insert-unknown-chunk/🦀️.rs"]
                                pub mod insert_unknown_chunk;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📤️remove-unknown-chunk/🦀️.rs"]
                                pub mod remove_unknown_chunk;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                            pub mod operations;
                            #[cfg(test)]
                            #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧪️tests/🧬️mutation-regressions/🦀️.rs"]
                            mod mutation_regressions;
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod deflate {
                                            #[path = "."]
                                            pub mod v_rfc1950 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod deflate {
                                            #[path = "."]
                                            pub mod v_rfc1950 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_2::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1_2::subsets::any::io::*;
        }
        pub mod engine {
            pub use super::standards::v1_2::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]

    pub mod pdf {
        #[path = "../../🗿️artifacts/📄️pdf/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_4 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod deflate {
                                            #[path = "."]
                                            pub mod v_rfc1950 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod deflate {
                                            #[path = "."]
                                            pub mod v_rfc1950 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod a {
                        // 🏅️ PDF/A (1.4) -- ISO 19005-1 (PDF/A-1), the honestly-scope-limited
                        // reference case: `PdfSnapshot`(1.4) is a bare PageDoc, no object graph.
                        // Added in ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W2.
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod x {
                        // 🏅️ PDF/X (1.4) -- ISO 15930-1 (X-1a) / ISO 15930-3 (X-3), same
                        // honestly-scope-limited schema-gap shape as ✳️a above. Added in ticket
                        // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W2.
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                }
            }

            #[path = "."]
            pub mod v1_7 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod deflate {
                                            #[path = "."]
                                            pub mod v_rfc1950 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod deflate {
                                            #[path = "."]
                                            pub mod v_rfc1950 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod a {
                        // 🏅️ PDF/A (ISO 19005-2/-3) — the FIRST real, non-✳️any subset in the
                        // repo. Restructured from `✳️a-2b` in ticket
                        // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W2: the conformance
                        // LEVEL (2b/2u/3b/3u) is analyzer-detected DATA (`stdio.pdf.a.level`), not
                        // part of the subset id. `schema` re-exports the ✳️any subset's
                        // `PdfSnapshot` verbatim (same Rust type, same `s.stdio.pdf.1.7` schema
                        // id); `io` reuses the ✳️any subset's binary/deflate DAG leaves rather than
                        // duplicating them.
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod x {
                        // 🏅️ PDF/X-4 -- ISO 15930-7:2010, based on PDF 1.6/1.7. Real
                        // object-graph-backed analyzer/composer/builder (same shape as ✳️a).
                        // Added in ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️x/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️x/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod e {
                        // 🏅️ PDF/E-1 -- ISO 24517-1:2008, based on PDF 1.6. Added in ticket
                        // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod ua {
                        // 🏅️ PDF/UA-1 -- ISO 14289-1:2014. Added in ticket
                        // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️ua/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️ua/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod vt {
                        // 🏅️ PDF/VT-1/-2 -- ISO 16612-2:2010, layered on PDF/X-4 (ISO 15930-7):
                        // this subset's analyzer calls `x::analyzer::check_x_conformance`
                        // directly rather than duplicating those checks. Added in ticket
                        // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod h {
                        // 🏅️ PDF/H -- AIIM/ASTM PDF Healthcare Best Practices Guide (2008);
                        // industry best-practice, never ISO; all-soft profile, no hard checks,
                        // composer always Ok (pass-through). Added in ticket
                        // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️h/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️h/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        // 🔀️ S-6 twin (`.claude/plans/the-current-schemas-are-scalable-journal.md`; W0 recon's
        // "pdf has gif's exact S-6 problem" finding): 1.7 is the real object-graph engine (was
        // dodging this collision under its own `stdio.pdf.1.7` schema id) and is now canonical
        // here; 1.4 (the 87-line `PageDoc` stub) stays reachable at `standards::v1_4::`.
        pub mod schema {
            pub use super::standards::v1_7::subsets::base::schema::*;
        }
        pub mod io {
            pub use super::standards::v1_7::subsets::base::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod bachelor_thesis {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🧪️tests/🦀️.rs"]
                mod bachelor_thesis_tests;
            }
        }
    }

    #[path = "."]

    pub mod jpg {
        #[path = "../../🗿️artifacts/📷️jpg/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_jfif_1_01 {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::document::io` (codec/io_registry) and
                // `subsets::document::schema` (document helpers); this stays an inline barrel so every
                // existing `standards::v_jfif_1_01::engine::*`/root `engine::*` path still resolves
                // (`📸️remodel`'s own `jpg::engine::decode_jpg`/`encode_jpg`/`JpgError` consumer
                // included).
                pub mod engine {
                    pub use super::subsets::document::io::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod document {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod dimensions {
                                    #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/💡️inferences/📐dimensions/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod top_level;
                                pub use top_level::*;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🧬️mutations/📐️change-jfif-header/🦀️.rs"]
                                pub mod change_jfif_header;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🧬️mutations/📊️replace-quant-table/🦀️.rs"]
                                pub mod replace_quant_table;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🧬️mutations/📤️remove-quant-table/🦀️.rs"]
                                pub mod remove_quant_table;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🌳️replace-huffman-table/🦀️.rs"]
                                pub mod replace_huffman_table;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🪓️remove-huffman-table/🦀️.rs"]
                                pub mod remove_huffman_table;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🔁️change-restart-interval/🦀️.rs"]
                                pub mod change_restart_interval;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🧬️mutations/📥️insert-other-segment/🦀️.rs"]
                                pub mod insert_other_segment;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🗑️remove-other-segment/🦀️.rs"]
                                pub mod remove_other_segment;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🟪️replace-pixels/🦀️.rs"]
                                pub mod replace_pixels;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🎚️change-re-encode-quality/🦀️.rs"]
                                pub mod change_re_encode_quality;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧬️schema/⚙️operations/🦀️.rs"]
                            pub mod operations;
                            #[cfg(test)]
                            #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧪️tests/🧬️mutation-regressions/🦀️.rs"]
                            mod mutation_regressions;
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod baseline {
                        // 🏅️ ITU-T T.81 / ISO 10918-1 Annex F baseline sequential DCT (JFIF 1.01
                        // container) -- ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES.
                        // `schema` re-exports the ✳️any subset's `JpgSnapshot` verbatim (same Rust
                        // type, same `s.stdio.jpg` schema id); `io` reuses the ✳️any subset's
                        // `binary` DAG leaf rather than duplicating it. `JpgSnapshot` gained
                        // `frame`/`sof_marker`/`arithmetic`/`dc_huffman_table_count`/
                        // `ac_huffman_table_count` fields as part of this subset landing --
                        // `⚙️engine::decode_jpg` now persists what it already computed transiently.
                        #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_jfif_1_01::subsets::document::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_jfif_1_01::engine::*;
        }
        pub mod io {
            pub use super::standards::v_jfif_1_01::subsets::document::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]

    pub mod gif {
        #[path = "../../🗿️artifacts/🎞️gif/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v87a {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::any::io` (codec/register/io_registry — `register`
                // stays reachable since gif is one of stdio's 10 deliberate imperative
                // `engine::register()` plugin-root calls) and `subsets::any::schema` (document
                // helpers); this stays an inline barrel so every existing
                // `standards::v87a::engine::*`/`gif::engine::*` path still resolves — including
                // 89a's own `standards::v87a::engine as codec` cross-standard reuse import.
                pub mod engine {
                    pub use super::subsets::any::io::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod dimensions {
                                    #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐dimensions/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            #[path = "."]
            pub mod v89a {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `subsets::any::io` (codec/register/io_registry — `register`
                // stays reachable since gif is one of stdio's 10 deliberate imperative
                // `engine::register()` plugin-root calls) and `subsets::any::schema` (document
                // helpers); this stays an inline barrel so every existing
                // `standards::v89a::engine::*`/`gif::engine::*` path still resolves — including
                // `📚️examples/💃️dancing`'s own `standards::v89a::engine::decode_gif` call.
                pub mod engine {
                    pub use super::subsets::any::io::*;
                }
                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🧬️migrations/🦀️.rs"]
                pub mod migrations;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod dimensions {
                                    #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/💡️inferences/📐dimensions/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        // 🔀️ S-6 (`.claude/plans/the-current-schemas-are-scalable-journal.md`): 89a is the richer
        // standard (frames/GCE/loop vs. 87a's single-image model) and is now canonical here; 87a
        // stays reachable under its own explicit `standards::v87a::` path for callers that need it.
        pub mod schema {
            pub use super::standards::v89a::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v89a::engine::*;
            /// 📎 Registers BOTH standards' engines (89a canonical + 87a legacy) — a flat glob
            /// re-export can't do this (two `register` fns of the same name would collide), so this
            /// local definition shadows the glob-imported 89a one and calls both explicitly.
            // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
            pub fn register() {
                super::standards::v87a::engine::register();
                super::standards::v89a::engine::register();
            }
        }
        pub mod io {
            pub use super::standards::v89a::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod dancing {
                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🧪️tests/🦀️.rs"]
                mod dancing_tests;
            }
        }
    }

    #[path = "."]

    pub mod tiff {
        #[path = "../../🗿️artifacts/🖼️tiff/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v6_0 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod document {
                        // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                        // real code now lives in `io` (codec/io_registry) and `schema` (document
                        // helpers), both siblings within this same `any` module — this stays an
                        // inline barrel so every existing `subsets::document::engine::*` path (reached
                        // from the `v6_0::engine`/root `engine::*` barrels above it) still resolves.
                        pub mod engine {
                            pub use super::io::*;
                        }
                        #[path = "."]
                        pub mod examples {
                            #[path = "."]
                            pub mod demo {
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/📚️examples/🎬️demo/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod dimensions {
                                    #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/💡️inferences/📐dimensions/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod top_level;
                                pub use top_level::*;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🧭️change-byte-order/🦀️.rs"]
                                pub mod change_byte_order;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/🧬️mutations/📥️insert-ifd/🦀️.rs"]
                                pub mod insert_ifd;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/🧬️mutations/📤️remove-ifd/🦀️.rs"]
                                pub mod remove_ifd;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🏷️replace-tag/🦀️.rs"]
                                pub mod replace_tag;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🗑️remove-tag/🦀️.rs"]
                                pub mod remove_tag;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🟪️replace-pixels/🦀️.rs"]
                                pub mod replace_pixels;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧬️schema/⚙️operations/🦀️.rs"]
                            pub mod operations;
                            #[cfg(test)]
                            #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧪️tests/🧬️mutation-regressions/🦀️.rs"]
                            mod mutation_regressions;
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod binary {
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod baseline {
                        // 🏅️ Baseline TIFF (6.0) -- Adobe TIFF 6.0 Part 1 "Baseline TIFF", the
                        // honestly-scope-limited case: `TiffSnapshot`(6.0) retains only a decoded
                        // `RasterImage{width,height,rgba}`, no IFD. Added in ticket
                        // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                        #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                }
                pub mod engine {
                    pub use super::subsets::document::engine::*;
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v6_0::subsets::document::schema::*;
        }
        pub mod engine {
            pub use super::standards::v6_0::subsets::document::engine::*;
        }
        pub mod io {
            pub use super::standards::v6_0::subsets::document::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]

    pub mod docx {
        #[path = "../../🗿️artifacts/📜️docx/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // `DocxEngine` (zero construction sites) deleted outright; `register()`/
                // `register_artifact_inferences()`/`register_pilot_languages()` were already orphaned
                // (superseded by `docx::declaration()`) and deleted outright too; `build_minimal_docx`/
                // `sync_main_part`/`encode_docx` + the `*_to_xml` mapping moved to `subsets::base::io::
                // export::serializers`; `decode_docx`/`sniff_docx_bytes` + the `*_from_xml` mapping
                // moved to `subsets::base::io::import::deserializers`; `DocxError` + shared OPC/XML
                // constants moved to `subsets::base::io`; `io_registry` moved to `subsets::base::io`;
                // `empty_docx_snapshot`/`demo_docx_snapshot` + tests moved to `subsets::base::schema`.
                // docx is NOT one of stdio's 10 protected imperative plugin-root `engine::register()`
                // calls, so no `engine` shim remains — external callers (`✒️writer`) were repointed.
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod strict {
                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod transitional {
                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ecma_376::subsets::base::schema::*;
        }
        pub mod io {
            pub use super::standards::v_ecma_376::subsets::base::io::*;
        }
        pub mod engine {
            pub use super::standards::v_ecma_376::subsets::base::io::export::serializers::*;
            pub use super::standards::v_ecma_376::subsets::base::io::import::deserializers::*;
            pub use super::standards::v_ecma_376::subsets::base::io::io_registry;
            // 🎯 export::serializers and import::deserializers each define their own
            // artifacts submodule (per-dialect zip/xml helpers) -- disambiguate the resulting
            // glob collision by explicitly preferring serializers, the export side.
            pub use super::standards::v_ecma_376::subsets::base::io::export::serializers::artifacts;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]

    pub mod pptx {
        #[path = "../../🗿️artifacts/🎞️pptx/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // `PptxEngine` (zero construction sites) deleted outright; `register()`/
                // `register_artifact_inferences()`/`register_pilot_languages()` were already orphaned
                // (superseded by `pptx::declaration()`) and deleted outright too; `build_minimal_pptx`/
                // `encode_pptx` + the `*_to_xml` mapping moved to `subsets::any::io::export::
                // serializers`; `decode_pptx`/`sniff_pptx_bytes` + the `*_from_xml` mapping moved to
                // `subsets::any::io::import::deserializers`; `PptxError` + shared OPC/XML constants +
                // the minimal slideMaster/slideLayout/theme boilerplate moved to `subsets::any::io`;
                // `io_registry` moved to `subsets::any::io`; `empty_pptx_snapshot`/`demo_pptx_snapshot`
                // + tests moved to `subsets::any::schema`. pptx is NOT one of stdio's 10 protected
                // imperative plugin-root `engine::register()` calls, so no `engine` shim remains —
                // external callers only ever reached `PptxSnapshot`/`STDIO_PPTX_DOCUMENT_SCHEMA`
                // (unaffected).
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod strict {
                        // 🏅️ ISO/IEC 29500-1:2016 Strict -- presentationml main ns
                        // http://purl.oclc.org/ooxml/presentationml/main. Added in ticket
                        // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES. `schema`
                        // re-exports the ✳️any subset's `PptxSnapshot` verbatim (same Rust type,
                        // same `s.stdio.pptx` schema id); `io` reuses the ✳️any subset's
                        // zip/xml DAG leaves rather than duplicating them.
                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod transitional {
                        // 🏅️ ISO/IEC 29500-4:2016 Transitional -- presentationml main ns
                        // http://schemas.openxmlformats.org/presentationml/2006/main. Added in
                        // ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES. Same
                        // 5-leaf shape as ✳️strict above.
                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ecma_376::subsets::base::schema::*;
        }
        pub mod io {
            pub use super::standards::v_ecma_376::subsets::base::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]

    pub mod xlsx {
        #[path = "../../🗿️artifacts/📕️xlsx/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // `XlsxEngine` (zero construction sites) deleted outright; `register()`/
                // `register_artifact_inferences()`/`register_pilot_languages()` were already orphaned
                // (superseded by `xlsx::declaration()`) and deleted outright too; `build_minimal_xlsx`/
                // `encode_xlsx` + the `*_to_xml` mapping moved to `subsets::any::io::export::
                // serializers`; `decode_xlsx`/`sniff_xlsx_bytes` + the `*_from_xml` mapping moved to
                // `subsets::any::io::import::deserializers`; `XlsxError` + shared OPC/XML constants +
                // `column_letter`/`column_index` moved to `subsets::any::io`; `io_registry` moved to
                // `subsets::any::io`; `empty_xlsx_snapshot`/`demo_xlsx_snapshot` + tests moved to
                // `subsets::any::schema`. xlsx is NOT one of stdio's 10 protected imperative
                // plugin-root `engine::register()` calls, so no `engine` shim remains — external
                // callers only ever reached `XlsxSnapshot`/`STDIO_XLSX_DOCUMENT_SCHEMA` (unaffected).
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod strict {
                        // 🏅️ ISO/IEC 29500-1 Strict -- ticket
                        // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3. `schema`
                        // re-exports the ✳️any subset's `XlsxSnapshot` verbatim (same Rust type,
                        // same `s.stdio.xlsx` schema id); `io` reuses the ✳️any subset's
                        // zip/xml DAG leaves rather than duplicating them.
                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                    #[path = "."]
                    pub mod transitional {
                        // 🏅️ ISO/IEC 29500-4 Transitional -- ticket
                        // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3. Same shape as
                        // ✳️strict above, opposite polarity.
                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🧬️schema/🦀️.rs"]
                        pub mod schema;
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ecma_376::subsets::base::schema::*;
        }
        pub mod io {
            pub use super::standards::v_ecma_376::subsets::base::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]

    pub mod bcf {
        #[path = "../../🗿️artifacts/💬️bcf/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v2_1 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod topicstats {
                                    #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/💡️inferences/🗒topicstats/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v2_1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v2_1::subsets::any::io::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod semio {
        #[path = "../../🗿️artifacts/🧿️semio/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod animation {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod duration {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/💡️inferences/⏱duration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod base {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod kind {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/💡️inferences/🏷️kind/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/🧮️geometry/🦀️.rs"]
                            pub mod geometry;
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/🧰️triples/🦀️.rs"]
                            pub mod triples;
                        }
                    }
                    #[path = "."]
                    pub mod audio {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod duration {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/💡️inferences/⏱duration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod brep {
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod step {
                                            #[path = "."]
                                            pub mod v_ap214 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod step {
                                            #[path = "."]
                                            pub mod v_ap214 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️.rs"]
                            pub mod engine;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/🏟️arena/🦀️.rs"]
                                pub mod arena;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/➰️curve/🦀️.rs"]
                                pub mod curve;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/🚨️error/🦀️.rs"]
                                pub mod error;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/〰️polynomial/🦀️.rs"]
                                pub mod polynomial;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/🏄️surface/🦀️.rs"]
                                pub mod surface;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/📏️tolerance/🦀️.rs"]
                                pub mod tolerance;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/🕸️topology/🦀️.rs"]
                                pub mod topology;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/➡️vector/🦀️.rs"]
                                pub mod vector;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🎨️blend/🦀️.rs"]
                                pub mod blend;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🔀️boolean/🦀️.rs"]
                                pub mod boolean;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🔺️euler/🦀️.rs"]
                                pub mod euler;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/✂️intersect/🦀️.rs"]
                                pub mod intersect;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/↔️offset/🦀️.rs"]
                                pub mod offset;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🧱️primitives/🦀️.rs"]
                                pub mod primitives;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🧵️sew/🦀️.rs"]
                                pub mod sew;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/➡️sweep/🦀️.rs"]
                                pub mod sweep;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod validation_report {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/✅validation-report/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/🌳bounding-volume/🦀️.rs"]
                                pub mod bounding_volume;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/🏷classification/🦀️.rs"]
                                pub mod classification;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/📏mass-properties/🦀️.rs"]
                                pub mod mass_properties;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs"]
                                pub mod tessellation;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod delete_edge {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/✂️delete-edge/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/✂️delete-edge/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/✂️delete-edge/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_curve {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/➰replace-curve/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/➰replace-curve/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/➰replace-curve/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_vertex {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🏗️create-vertex/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🏗️create-vertex/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🏗️create-vertex/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_shell {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🐚create-shell/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🐚create-shell/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🐚create-shell/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_shell {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/💥delete-shell/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/💥delete-shell/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/💥delete-shell/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod move_vertex {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/📍move-vertex/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/📍move-vertex/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/📍move-vertex/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_edge {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🔗create-edge/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🔗create-edge/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🔗create-edge/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_face {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🔷create-face/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🔷create-face/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🔷create-face/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_solid {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🕳️delete-solid/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🕳️delete-solid/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🕳️delete-solid/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_vertex {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🗑️delete-vertex/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🗑️delete-vertex/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🗑️delete-vertex/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_surface {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🗺️replace-surface/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🗺️replace-surface/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🗺️replace-surface/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_face {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🚮delete-face/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🚮delete-face/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🚮delete-face/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_solid {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🧊create-solid/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🧊create-solid/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🧊create-solid/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod cad {
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod dxf {
                                            #[path = "."]
                                            pub mod v_r12 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dxf/🔖️r12/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1024 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod step {
                                            #[path = "."]
                                            pub mod v_ap214 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod dxf {
                                            #[path = "."]
                                            pub mod v_r12 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dxf/🔖️r12/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1024 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod step {
                                            #[path = "."]
                                            pub mod v_ap214 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod document {
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod docx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_7 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.7/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod docx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_7 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.7/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod drawing {
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dxf {
                                            #[path = "."]
                                            pub mod v_r12 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dxf/🔖️r12/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_7 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.7/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1024 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dxf {
                                            #[path = "."]
                                            pub mod v_r12 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dxf/🔖️r12/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_7 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.7/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1024 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod flattened_scene {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/💡️inferences/🎛flattened-scene/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod create_node {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/➕create-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/➕create-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/➕create-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_node {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/➖delete-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/➖delete-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/➖delete-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_layer {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🌱create-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🌱create-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🌱create-layer/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod unflatten_node {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🎈unflatten-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🎈unflatten-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🎈unflatten-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod ungroup_node {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/💫ungroup-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/💫ungroup-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/💫ungroup-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod move_node {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📍move-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📍move-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📍move-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod scale_node {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📏scale-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📏scale-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📏scale-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod change_stroke_width {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📐change-stroke-width/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📐change-stroke-width/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📐change-stroke-width/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod reorder_nodes {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🔀reorder-nodes/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🔀reorder-nodes/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🔀reorder-nodes/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rotate_node {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🔄rotate-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🔄rotate-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🔄rotate-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod change_stroke_color {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🖌️change-stroke-color/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🖌️change-stroke-color/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🖌️change-stroke-color/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod drag_nodes {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🖐️drag-nodes/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🖐️drag-nodes/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🖐️drag-nodes/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_layer {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🗑️delete-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🗑️delete-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🗑️delete-layer/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_path {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🛤️replace-path/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🛤️replace-path/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🛤️replace-path/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod group_nodes {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🧷group-nodes/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🧷group-nodes/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🧷group-nodes/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_fill {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🪣replace-fill/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🪣replace-fill/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🪣replace-fill/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod flatten_node {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🫓flatten-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🫓flatten-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🫓flatten-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod image {
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod jpg {
                                            #[path = "."]
                                            pub mod v_jfif_1_01 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️jpg/🔖️jfif-1.01/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod gif {
                                            #[path = "."]
                                            pub mod v89a {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎞️gif/🔖️89a/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod bmp {
                                            #[path = "."]
                                            pub mod v_v3 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖼️bmp/🔖️v3/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod tiff {
                                            #[path = "."]
                                            pub mod v6_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖼️tiff/🔖️6.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod jpg {
                                            #[path = "."]
                                            pub mod v_jfif_1_01 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️jpg/🔖️jfif-1.01/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod gif {
                                            #[path = "."]
                                            pub mod v89a {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️gif/🔖️89a/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod bmp {
                                            #[path = "."]
                                            pub mod v_v3 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖼️bmp/🔖️v3/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod tiff {
                                            #[path = "."]
                                            pub mod v6_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖼️tiff/🔖️6.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod dimensions {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/💡️inferences/📐dimensions/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod mesh {
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod gltf {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod ply {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️ply/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod las {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1024 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod gltf {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod ply {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️ply/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod las {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1024 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod aabb {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/💡️inferences/📦aabb/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod create_mesh {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🕸️create-mesh/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🕸️create-mesh/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🕸️create-mesh/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_mesh {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🗑️delete-mesh/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🗑️delete-mesh/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🗑️delete-mesh/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_primitive {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔺create-primitive/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔺create-primitive/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔺create-primitive/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_primitive {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/✂️delete-primitive/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/✂️delete-primitive/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/✂️delete-primitive/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod set_primitive_topology {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔀set-primitive-topology/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔀set-primitive-topology/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔀set-primitive-topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_primitive_geometry {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📐replace-primitive-geometry/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📐replace-primitive-geometry/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📐replace-primitive-geometry/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod set_primitive_material {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔗set-primitive-material/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔗set-primitive-material/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔗set-primitive-material/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_material {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🎨create-material/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🎨create-material/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🎨create-material/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_material {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🚮delete-material/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🚮delete-material/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🚮delete-material/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod change_material_base_color {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🌈change-material-base-color/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🌈change-material-base-color/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🌈change-material-base-color/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod change_material_metallic {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/⚙️change-material-metallic/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/⚙️change-material-metallic/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/⚙️change-material-metallic/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod change_material_roughness {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🧱change-material-roughness/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🧱change-material-roughness/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🧱change-material-roughness/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_texture {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🖼️create-texture/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🖼️create-texture/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🖼️create-texture/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_texture {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🕳️delete-texture/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🕳️delete-texture/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🕳️delete-texture/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod change_texture_mime {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🏷️change-texture-mime/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🏷️change-texture-mime/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🏷️change-texture-mime/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_texture_bytes {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📀replace-texture-bytes/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📀replace-texture-bytes/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📀replace-texture-bytes/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod move_vertex {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📍move-vertex/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📍move-vertex/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📍move-vertex/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                        #[path = "."]
                        pub mod examples {
                            #[path = "."]
                            pub mod cube {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/📚️examples/🧊️cube/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod model {
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod ifc {
                                            #[path = "."]
                                            pub mod v4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🏗️ifc/🔖️4/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod bcf {
                                            #[path = "."]
                                            pub mod v2_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💬️bcf/🔖️2.1/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod ifc {
                                            #[path = "."]
                                            pub mod v4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🏗️ifc/🔖️4/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod bcf {
                                            #[path = "."]
                                            pub mod v2_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💬️bcf/🔖️2.1/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod value {
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/➕️algebra-internals/🦀️.rs"]
                            pub mod algebra_internals;
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🌫️fuzzy-internals/🦀️.rs"]
                            pub mod fuzzy_internals;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod census {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/💡️inferences/🌳census/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod presentation {
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod pptx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod pptx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod video {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod duration {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/💡️inferences/⏱duration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod flow {
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "."]
                                pub mod profile {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/💡️inferences/📊profile/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod insert_run {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/📥insert-run/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/📥insert-run/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/📥insert-run/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod remove_run {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🗑️remove-run/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🗑️remove-run/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🗑️remove-run/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod edit_run {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/✏️edit-run/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/✏️edit-run/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/✏️edit-run/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod change_run_language {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🌐change-run-language/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🌐change-run-language/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🌐change-run-language/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod reorder_runs {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🔀reorder-runs/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🔀reorder-runs/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🔀reorder-runs/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod add_mark {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➕add-mark/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➕add-mark/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➕add-mark/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod remove_mark {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➖remove-mark/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➖remove-mark/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➖remove-mark/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod table {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🔗️causal-internals/🦀️.rs"]
                            pub mod causal_internals;
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🎲️entropy-internals/🦀️.rs"]
                            pub mod entropy_internals;
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🎲️probability-internals/🦀️.rs"]
                            pub mod probability_internals;
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/📊️statistics-internals/🦀️.rs"]
                            pub mod statistics_internals;
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/📋️tabular-internals/🦀️.rs"]
                            pub mod tabular_internals;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod shape {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/💡️inferences/📐shape/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod moments {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/💡️inferences/📊moments/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod entropy {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/💡️inferences/🎲entropy/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_column {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🏗️create-column/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🏗️create-column/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🏗️create-column/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_column {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🗑️delete-column/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🗑️delete-column/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🗑️delete-column/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_column {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🏷️rename-column/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🏷️rename-column/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🏷️rename-column/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod reorder_columns {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🔀reorder-columns/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🔀reorder-columns/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🔀reorder-columns/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod insert_row {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/📥insert-row/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/📥insert-row/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/📥insert-row/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod remove_row {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/➖remove-row/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/➖remove-row/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/➖remove-row/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod reorder_rows {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🔃reorder-rows/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🔃reorder-rows/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🔃reorder-rows/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod edit_cell {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/✏️edit-cell/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/✏️edit-cell/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/✏️edit-cell/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod examples {
                            #[path = "."]
                            pub mod sheet {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/📚️examples/📃️sheet/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod graph {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🔧️operators-internals/🦀️.rs"]
                            pub mod operators_internals;
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🚶️traversal-internals/🦀️.rs"]
                            pub mod traversal_internals;
                            #[path = "."]
                            pub mod normal_internals {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/➕️normal-internals/➡️directed/🦀️.rs"]
                                pub mod directed;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/➕️normal-internals/↔️undirected/🦀️.rs"]
                                pub mod undirected;
                            }
                            #[path = "."]
                            pub mod ports_internals {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🔌️ports-internals/↔️undirected/🦀️.rs"]
                                pub mod undirected;
                                #[path = "."]
                                pub mod directed {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🔌️ports-internals/➡️directed/➕️normal/🦀️.rs"]
                                    pub mod normal;
                                }
                            }
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod connectivity {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/💡️inferences/🔗connectivity/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_node {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🏗️create-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🏗️create-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🏗️create-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_node {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🗑️delete-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🗑️delete-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🗑️delete-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod change_node_kind {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔧change-node-kind/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔧change-node-kind/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔧change-node-kind/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod change_node_label {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🖍️change-node-label/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🖍️change-node-label/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🖍️change-node-label/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod move_node {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/📍move-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/📍move-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/📍move-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod add_node_port {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔌add-node-port/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔌add-node-port/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔌add-node-port/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod remove_node_port {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔚remove-node-port/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔚remove-node-port/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔚remove-node-port/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod add_node_property {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➕add-node-property/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➕add-node-property/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➕add-node-property/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod remove_node_property {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➖remove-node-property/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➖remove-node-property/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➖remove-node-property/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_edge {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔗create-edge/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔗create-edge/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔗create-edge/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_edge {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/✂️delete-edge/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/✂️delete-edge/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/✂️delete-edge/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod examples {
                            #[path = "."]
                            pub mod wires {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/📚️examples/🕸️wires/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod object {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod composition {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/💡️inferences/🧩composition/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod move_object {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚚move-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚚move-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚚move-object/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rotate_object {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🔄rotate-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🔄rotate-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🔄rotate-object/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod scale_object {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/📏scale-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/📏scale-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/📏scale-object/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_brep {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧱create-brep/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧱create-brep/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧱create-brep/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_brep {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/💥delete-brep/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/💥delete-brep/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/💥delete-brep/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_mesh {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🕸️create-mesh/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🕸️create-mesh/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🕸️create-mesh/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_mesh {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧨delete-mesh/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧨delete-mesh/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧨delete-mesh/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_properties {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🏷️create-properties/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🏷️create-properties/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🏷️create-properties/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_properties {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚫delete-properties/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚫delete-properties/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚫delete-properties/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod examples {
                            #[path = "."]
                            pub mod crate_ {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/📚️examples/📦️crate/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod kit {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod entries {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/💡️inferences/🗃entries/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_object {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏗️create-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏗️create-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏗️create-object/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_object {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🪓delete-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🪓delete-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🪓delete-object/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_model {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏛️create-model/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏛️create-model/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏛️create-model/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_model {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/💣delete-model/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/💣delete-model/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/💣delete-model/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_properties {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏷️create-properties/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏷️create-properties/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏷️create-properties/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_properties {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🚫delete-properties/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🚫delete-properties/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🚫delete-properties/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod bind_representation {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🔗bind-representation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🔗bind-representation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🔗bind-representation/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod unbind_representation {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/✂️unbind-representation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/✂️unbind-representation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/✂️unbind-representation/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod change_representation_pin {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/📌change-representation-pin/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/📌change-representation-pin/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/📌change-representation-pin/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod add_type {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/➕add-type/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/➕add-type/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/➕add-type/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod remove_type {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/➖remove-type/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/➖remove-type/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/➖remove-type/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_type {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/✏️rename-type/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/✏️rename-type/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/✏️rename-type/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod add_design {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🆕add-design/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🆕add-design/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🆕add-design/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod remove_design {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🗑️remove-design/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🗑️remove-design/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🗑️remove-design/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod edit_design {
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🖊️edit-design/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🖊️edit-design/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🖊️edit-design/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod examples {
                            #[path = "."]
                            pub mod furniture {
                                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/📚️examples/🪑️furniture/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod examples {
            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/📚️examples/🎬️demo/🦀️.rs"]
            pub mod demo;
            #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/📚️examples/📃️note/🦀️.rs"]
            pub mod note;
        }
    }

    #[path = "."]
    pub mod mp4 {
        #[path = "../../🗿️artifacts/🎥️mp4/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod isobmff {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod duration {
                                    #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/💡️inferences/⏱duration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod examples {
            #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
            pub mod demo;
        }
    }

    #[path = "."]
    pub mod avi {
        #[path = "../../🗿️artifacts/📼️avi/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_0 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod duration {
                                    #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/💡️inferences/⏱duration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod examples {
            #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/📚️examples/🎬️demo/🦀️.rs"]
            pub mod demo;
        }
    }

    #[path = "."]
    pub mod mp3 {
        #[path = "../../🗿️artifacts/🎵️mp3/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod mpeg1_layer3 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod duration {
                                    #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/💡️inferences/⏱duration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod examples {
            #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
            pub mod demo;
        }
    }

    #[path = "."]
    pub mod wav {
        #[path = "../../🗿️artifacts/🔊️wav/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod riff_pcm {
                /// 🗂️ ⚙️️→🚪️ dissolution shim (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                /// the real codec/`io_registry` moved to `subsets::any::io`; this inline module keeps
                /// only `register()` reachable at its historical path because the plugin root
                /// (`✏️s/🔌️plugins/🗄️stdio/🦀️.rs`) still calls
                /// `standards::riff_pcm::engine::register()` imperatively (one of the ticket's 10
                /// protected `dsl::registry` entrypoints — its call site is explicitly not to be
                /// touched).
                pub mod engine {
                    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
                    pub fn register() {
                        super::subsets::any::io::register();
                    }
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod duration {
                                    #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/💡️inferences/⏱duration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod examples {
            #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
            pub mod demo;
        }
    }

    #[path = "."]
    pub mod epw {
        #[path = "../../🗿️artifacts/🌦️epw/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod energyplus {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod climate {
                                    #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌡climate/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod examples {
            #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
            pub mod demo;
        }
    }

    #[path = "."]
    pub mod tsv {
        #[path = "../../🗿️artifacts/📑️tsv/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod iana {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // `sniff_real_bytes`/`decode_tsv`/`encode_tsv` moved to `subsets::any::schema::snapshot`
                // (mirrors `json`'s own `parse_json_text`/`write_json_text` placement — this artifact's
                // codec is a pure text<->snapshot round trip, no cross-artifact bridging), `io_registry`
                // moved to `subsets::any::io::io_registry`. `register()` is one of stdio's 10 protected
                // imperative plugin-root calls; the root `📑️tsv/🦀️.rs`'s OWN `register()` now
                // covers it directly (`crate::artifacts::tsv::register()` — see stdio's plugin root).
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod examples {
            #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
            pub mod demo;
        }
    }

    #[path = "."]
    pub mod html {
        #[path = "../../🗿️artifacts/🌐️html/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v5 {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // `sniff_real_bytes` moved to `subsets::any::io::import::deserializers`, `io_registry`
                // moved to `subsets::any::io::io_registry`. `register()` is one of stdio's 10 protected
                // imperative plugin-root calls (`crate::artifacts::html::standards::v5::engine::
                // register()` in `🗄️stdio/🦀️.rs`) — left callable at this exact path via a
                // pure re-export of `subsets::any::io::register` (itself unchanged).
                pub mod engine {
                    pub use super::subsets::any::io::register;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                        pub mod io;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod examples {
            #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
            pub mod demo;
        }
    }
}
//#endregion Artifacts

//#region ✏️Editor
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod png {
        #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod jpg_any {
        #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod jpg_baseline {
        #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod bmp {
        #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod tiff_any {
        #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod tiff_baseline {
        #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod gif_87a {
        #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod gif_89a {
        #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod svg_any {
        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod svg_basic {
        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod svg_tiny {
        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod mp4 {
        #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod mp3 {
        #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod wav {
        #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod avi {
        #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod html {
        #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod md {
        #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    //#region P3-stdio-geometry (semio/step/ifc/dwg/dxf/gltf/obj/stl/ply/las/bcf)
    #[path = "."]
    pub mod semio_animation {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_base {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_audio {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_brep {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_cad {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_document {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_drawing {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_flow {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_graph {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_image {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_kit {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_mesh {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_model {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_object {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_presentation {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_table {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_text {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_value {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_video {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step_any {
        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step_cc1 {
        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc1/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc1/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc1/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step_cc2 {
        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc2/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc2/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc2/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step_cc3 {
        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc3/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc3/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc3/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step_cc4 {
        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc4/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc4/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc4/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step_cc5 {
        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc5/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc5/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc5/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step_cc6 {
        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc2x3_any {
        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc2x3_cobie {
        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc2x3_cv20 {
        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc2x3_sav {
        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️sav/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️sav/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️sav/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc4_any {
        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod dwg_ac1018 {
        #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod dwg_ac1024 {
        #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod dxf {
        #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod gltf {
        #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod obj {
        #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod stl {
        #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ply {
        #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod las {
        #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod bcf {
        #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    //#endregion P3-stdio-geometry
    //#region P2-stdio-data (csv/tsv/txt/json/xml)
    #[path = "."]
    pub mod csv {
        #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod tsv {
        #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod txt {
        #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod json_any {
        #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod json_i_json {
        #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️i-json/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️i-json/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️i-json/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod xml_any {
        #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod xml_valid {
        #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    //#endregion P2-stdio-data
    //#region P2-stdio-data-pdf
    // 🧵 W2 packet P2-stdio-data: pdf, 10 subsets (1.4: a/any/x; 1.7: a/any/e/h/ua/vt/x) x {editor, viewer}.
    #[path = "."]
    pub mod pdf14a {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf14 {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf14x {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17a {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17 {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17e {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17h {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️h/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️h/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️h/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17ua {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️ua/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️ua/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️ua/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17vt {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17x {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️x/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️x/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️x/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    //#endregion P2-stdio-data-pdf
    //#region P2-stdio-data-docx-pptx
    // 🧵 W2 packet P2-stdio-data: docx(3)/pptx(3), 6 subsets x {editor, viewer}.
    #[path = "."]
    pub mod docx {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/✏️editor/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod edit {
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod strict {
                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/✏️editor/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod edit {
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod transitional {
                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/✏️editor/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod edit {
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pptx {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/✏️editor/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod edit {
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod strict {
                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/✏️editor/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod edit {
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod transitional {
                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/✏️editor/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod edit {
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    //#endregion P2-stdio-data-docx-pptx
    //#region P2-stdio-data-xlsx
    // 🧵 W2 packet P2-stdio-data: xlsx(3), 3 subsets x {editor, viewer}.
    #[path = "."]
    pub mod xlsx {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/✏️editor/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod edit {
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod strict {
                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/✏️editor/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod edit {
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod transitional {
                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/✏️editor/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod edit {
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    //#endregion P2-stdio-data-xlsx
    //#region P2-stdio-data-epw-zip-deflate-binary
    // 🧵 W2 packet P2-stdio-data: epw/zip(2)/deflate/binary, 5 subsets x {editor, viewer}.
    #[path = "."]
    pub mod epw {
        #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod zip {
        #[path = "."]
        pub mod base {
            #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/✏️editor/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod edit {
                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod windows {
                        #[path = "."]
                        pub mod main {
                            #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod iso21320 {
            #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/✏️editor/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod edit {
                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod windows {
                        #[path = "."]
                        pub mod main {
                            #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod deflate {
        #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod binary {
        #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    //#endregion P2-stdio-data-epw-zip-deflate-binary
}
//#endregion ✏️Editor

//#region 👁️Viewer
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod png {
        #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod jpg_any {
        #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod jpg_baseline {
        #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod bmp {
        #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod tiff_any {
        #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod tiff_baseline {
        #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod gif_87a {
        #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod gif_89a {
        #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod svg_any {
        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod svg_basic {
        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod svg_tiny {
        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod mp4 {
        #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod mp3 {
        #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod wav {
        #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod avi {
        #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod html {
        #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod md {
        #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    //#region P3-stdio-geometry (semio/step/ifc/dwg/dxf/gltf/obj/stl/ply/las/bcf)
    #[path = "."]
    pub mod semio_animation {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_base {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_audio {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_brep {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_cad {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_document {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_drawing {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_flow {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_graph {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_image {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_kit {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_mesh {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_model {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_object {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_presentation {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_table {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_text {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_value {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_video {
        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step_any {
        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step_cc1 {
        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc1/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc1/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc1/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step_cc2 {
        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc2/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc2/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc2/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step_cc3 {
        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc3/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc3/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc3/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step_cc4 {
        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc4/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc4/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc4/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step_cc5 {
        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc5/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc5/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc5/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step_cc6 {
        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc2x3_any {
        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc2x3_cobie {
        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc2x3_cv20 {
        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc2x3_sav {
        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️sav/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️sav/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️sav/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc4_any {
        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod dwg_ac1018 {
        #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod dwg_ac1024 {
        #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod dxf {
        #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod gltf {
        #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod obj {
        #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod stl {
        #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ply {
        #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod las {
        #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod bcf {
        #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    //#endregion P3-stdio-geometry
    //#region P2-stdio-data (csv/tsv/txt/json/xml)
    #[path = "."]
    pub mod csv {
        #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod tsv {
        #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod txt {
        #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod json_any {
        #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod json_i_json {
        #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️i-json/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️i-json/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️i-json/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod xml_any {
        #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod xml_valid {
        #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    //#endregion P2-stdio-data
    //#region P2-stdio-data-pdf
    // 🧵 W2 packet P2-stdio-data: pdf, 10 subsets (1.4: a/any/x; 1.7: a/any/e/h/ua/vt/x) x {editor, viewer}.
    #[path = "."]
    pub mod pdf14a {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf14 {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf14x {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17a {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17 {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17e {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17h {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️h/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️h/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️h/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17ua {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️ua/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️ua/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️ua/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17vt {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17x {
        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️x/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️x/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️x/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    //#endregion P2-stdio-data-pdf
    //#region P2-stdio-data-docx-pptx
    // 🧵 W2 packet P2-stdio-data: docx(3)/pptx(3), 6 subsets x {editor, viewer}.
    #[path = "."]
    pub mod docx {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/👁️viewer/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod view {
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod strict {
                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/👁️viewer/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod view {
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod transitional {
                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/👁️viewer/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod view {
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pptx {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/👁️viewer/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod view {
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod strict {
                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/👁️viewer/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod view {
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod transitional {
                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/👁️viewer/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod view {
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    //#endregion P2-stdio-data-docx-pptx
    //#region P2-stdio-data-xlsx
    // 🧵 W2 packet P2-stdio-data: xlsx(3), 3 subsets x {editor, viewer}.
    #[path = "."]
    pub mod xlsx {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/👁️viewer/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod view {
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod strict {
                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/👁️viewer/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod view {
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod transitional {
                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/👁️viewer/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod view {
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    //#endregion P2-stdio-data-xlsx
    //#region P2-stdio-data-epw-zip-deflate-binary
    // 🧵 W2 packet P2-stdio-data: epw/zip(2)/deflate/binary, 5 subsets x {editor, viewer}.
    #[path = "."]
    pub mod epw {
        #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod zip {
        #[path = "."]
        pub mod base {
            #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/👁️viewer/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod view {
                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod windows {
                        #[path = "."]
                        pub mod main {
                            #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod iso21320 {
            #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/👁️viewer/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod view {
                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod windows {
                        #[path = "."]
                        pub mod main {
                            #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod deflate {
        #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod binary {
        #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    //#endregion P2-stdio-data-epw-zip-deflate-binary
}
//#endregion 👁️Viewer
