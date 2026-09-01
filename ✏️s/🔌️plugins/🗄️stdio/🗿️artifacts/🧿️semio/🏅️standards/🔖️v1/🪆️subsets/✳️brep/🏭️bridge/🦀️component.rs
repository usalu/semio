//! 🏭️ Production mutation bridge for `s.stdio.semio@v1/✳️brep`.
//!
//! `test inventory` runs this and compares what it prints against the owner manifest and the claimed
//! test catalog; `contract` then requires all three to agree EXACTLY. The point is that the answer is
//! not written here — it is read out of `SemioBrepMutation::DESCRIPTORS`, which the `dsl::Mutations`
//! derive generates from the mutation leaves themselves. A mutation reachable in production but absent
//! from the manifest shows up as a breach rather than as a coverage footnote.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

// 🧬️ The production module tree, mirrored to the depth this subset actually needs. `🚪️io` is
// deliberately absent: brep's importers and exporters reach into six sibling artifacts, none of which
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
                            #[path = "../../✳️any/🧬️schema/🧮️geometry/🦀️component.rs"]
                            pub mod geometry;
                            #[path = "../../✳️any/🧬️schema/🧰️triples/🦀️component.rs"]
                            pub mod triples;
                        }
                    }
                    #[path = "."]
                    pub mod brep {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../✳️brep/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../✳️brep/🧬️schema/📸️snapshot/🏟️arena/🦀️component.rs"]
                                pub mod arena;
                                #[path = "../../✳️brep/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../✳️brep/🧬️schema/📸️snapshot/➰️curve/🦀️component.rs"]
                                pub mod curve;
                                #[path = "../../✳️brep/🧬️schema/📸️snapshot/🚨️error/🦀️component.rs"]
                                pub mod error;
                                #[path = "../../✳️brep/🧬️schema/📸️snapshot/〰️polynomial/🦀️component.rs"]
                                pub mod polynomial;
                                #[path = "../../✳️brep/🧬️schema/📸️snapshot/🏄️surface/🦀️component.rs"]
                                pub mod surface;
                                #[path = "../../✳️brep/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../✳️brep/🧬️schema/📸️snapshot/📏️tolerance/🦀️component.rs"]
                                pub mod tolerance;
                                #[path = "../../✳️brep/🧬️schema/📸️snapshot/➡️vector/🦀️component.rs"]
                                pub mod vector;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../✳️brep/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../✳️brep/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../✳️brep/🧬️schema/🔺️diff/✂️intersect/🦀️component.rs"]
                                pub mod intersect;
                                #[path = "../../✳️brep/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/✂️delete-edge/🦀️.rs"]
                                pub mod delete_edge;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/➰replace-curve/🦀️.rs"]
                                pub mod replace_curve;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/🏗️create-vertex/🦀️.rs"]
                                pub mod create_vertex;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/🐚create-shell/🦀️.rs"]
                                pub mod create_shell;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/💥delete-shell/🦀️.rs"]
                                pub mod delete_shell;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/📍move-vertex/🦀️.rs"]
                                pub mod move_vertex;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/🔗create-edge/🦀️.rs"]
                                pub mod create_edge;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/🔷create-face/🦀️.rs"]
                                pub mod create_face;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/🕳️delete-solid/🦀️.rs"]
                                pub mod delete_solid;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/🗑️delete-vertex/🦀️.rs"]
                                pub mod delete_vertex;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/🗺️replace-surface/🦀️.rs"]
                                pub mod replace_surface;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/🚮delete-face/🦀️.rs"]
                                pub mod delete_face;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/🧊create-solid/🦀️.rs"]
                                pub mod create_solid;
                                #[path = "../../✳️brep/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                            }
                        }
                    }
                }
            }
        }
    }
}

use artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
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
    let descriptors = <SemioBrepMutation as Mutation<_>>::DESCRIPTORS;
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
        ("subset".to_string(), pack::JsonValue::from("brep")),
        ("bridgeVersion".to_string(), pack::JsonValue::from(1_i64)),
        ("mutations".to_string(), pack::json_array(rows)),
    ]);
    println!("{}", pack::json_to_string(&out));
}
