//! 🏭️ Production mutation bridge for `s.stdio.semio@v1/✳️mesh`.
//!
//! `test inventory` runs this and compares what it prints against the owner manifest and the claimed
//! test catalog; `contract` then requires all three to agree EXACTLY. The point is that the answer is
//! not written here — it is read out of `SemioMeshMutation::DESCRIPTORS`, which the `dsl::Mutations`
//! derive generates from the mutation leaves themselves. A mutation reachable in production but absent
//! from the manifest shows up as a breach rather than as a coverage footnote.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

// 🧬️ The production module tree, mirrored to the depth this subset actually needs. `🚪️io` is
// deliberately absent: mesh's importers and exporters reach into six sibling artifacts, none of which
// a mutation inventory consults.
#[path = "."]
mod artifacts {
    #[path = "."]
    pub mod semio {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
pub mod schema {
    #[path = "../../✳️base/🧬️schema/🧮️geometry/🦀️.rs"]
    pub mod geometry;
    #[path = "../../✳️base/🧬️schema/🧰️triples/🦀️.rs"]
    pub mod triples;
}
                    }
                    #[path = "."]
                    pub mod mesh {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../✳️mesh/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../✳️mesh/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../✳️mesh/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../✳️mesh/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../✳️mesh/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../✳️mesh/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/🕸️create-mesh/🦀️.rs"]
                                pub mod create_mesh;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/🗑️delete-mesh/🦀️.rs"]
                                pub mod delete_mesh;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/🔺create-primitive/🦀️.rs"]
                                pub mod create_primitive;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/✂️delete-primitive/🦀️.rs"]
                                pub mod delete_primitive;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/🔀set-primitive-topology/🦀️.rs"]
                                pub mod set_primitive_topology;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/📐replace-primitive-geometry/🦀️.rs"]
                                pub mod replace_primitive_geometry;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/🔗set-primitive-material/🦀️.rs"]
                                pub mod set_primitive_material;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/🎨create-material/🦀️.rs"]
                                pub mod create_material;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/🚮delete-material/🦀️.rs"]
                                pub mod delete_material;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/🌈change-material-base-color/🦀️.rs"]
                                pub mod change_material_base_color;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/⚙️change-material-metallic/🦀️.rs"]
                                pub mod change_material_metallic;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/🧱change-material-roughness/🦀️.rs"]
                                pub mod change_material_roughness;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/🖼️create-texture/🦀️.rs"]
                                pub mod create_texture;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/🕳️delete-texture/🦀️.rs"]
                                pub mod delete_texture;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/🏷️change-texture-mime/🦀️.rs"]
                                pub mod change_texture_mime;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/📀replace-texture-bytes/🦀️.rs"]
                                pub mod replace_texture_bytes;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/📍move-vertex/🦀️.rs"]
                                pub mod move_vertex;
                                #[path = "../../✳️mesh/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                        }
                    }
                }
            }
        }
    }
}

use artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use protocol::Mutation;

/// 🎯️ Maps production's outcome severities onto the PROTOCOL's outcome classes. `Info`/`Warning` are
/// diagnostics ON an outcome — a mutation that applies with a warning still applied — and `Error`/
/// `Fatal` are the refusal. The test platform collapses them identically in `outcomeClassesOf`, in one
/// place and for the same reason; a bridge emitting raw severities would disagree with the manifest on
/// every mutation while both described the same behaviour.
fn protocol_outcomes(classes: &[protocol::MutationOutcomeClass]) -> Vec<&'static str> {
    let mut seen: Vec<&'static str> = Vec::new();
    for outcome in classes.iter() {
        let mapped = match format!("{outcome:?}").as_str() {
            "Applied" | "Info" | "Warning" => "applied",
            "Error" | "Fatal" => "rejected",
            _ => continue,
        };
        if !seen.contains(&mapped) {
            seen.push(mapped);
        }
    }
    if seen.is_empty() {
        seen.push("applied");
    }
    seen
}

fn main() {
    let descriptors = <SemioMeshMutation as Mutation<_>>::DESCRIPTORS;
    let rows: Vec<pack::JsonValue> = descriptors
        .iter()
        .map(|d| {
            pack::json_object([
                ("id".to_string(), pack::JsonValue::from(d.semantic_kind)),
                ("variant".to_string(), pack::JsonValue::from(d.aggregate_variant)),
                ("outcomes".to_string(), pack::json_array(protocol_outcomes(d.outcome_classes).into_iter().map(pack::JsonValue::from))),
            ])
        })
        .collect();
    let out = pack::json_object([
        ("schema".to_string(), pack::JsonValue::from("semio.repository-test.runtime-inventory/v2")),
        ("artifact".to_string(), pack::JsonValue::from("s.stdio.semio")),
        ("standard".to_string(), pack::JsonValue::from("v1")),
        ("subset".to_string(), pack::JsonValue::from("mesh")),
        ("bridgeVersion".to_string(), pack::JsonValue::from(1_i64)),
        ("mutations".to_string(), pack::json_array(rows)),
    ]);
    println!("{}", pack::json_to_string(&out));
}
