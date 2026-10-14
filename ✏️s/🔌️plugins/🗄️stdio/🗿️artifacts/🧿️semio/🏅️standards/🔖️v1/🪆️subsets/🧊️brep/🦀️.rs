//! 🪆️ The `brep` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "🚪️io/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod schema {
    #[path = "🧬️schema/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod snapshot {
        #[path = "🧬️schema/📸️snapshot/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "🧬️schema/📸️snapshot/🔁️body/🦀️.rs"]
        pub mod body;
    }
    #[path = "."]
    pub mod diff {
        #[path = "🧬️schema/🔺️diff/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod inferences {
        #[path = "🧬️schema/💡️inferences/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod validation_report {
            #[path = "🧬️schema/💡️inferences/✅validation-report/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod mutations {
        #[path = "🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod delete_edge {
            #[path = "🧬️schema/🧬️mutations/✂️delete-edge/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/✂️delete-edge/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/✂️delete-edge/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod replace_curve {
            #[path = "🧬️schema/🧬️mutations/➰replace-curve/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/➰replace-curve/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/➰replace-curve/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_vertex {
            #[path = "🧬️schema/🧬️mutations/🏗️create-vertex/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🏗️create-vertex/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🏗️create-vertex/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_shell {
            #[path = "🧬️schema/🧬️mutations/🐚create-shell/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🐚create-shell/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🐚create-shell/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_shell {
            #[path = "🧬️schema/🧬️mutations/💥delete-shell/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/💥delete-shell/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/💥delete-shell/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod move_vertex {
            #[path = "🧬️schema/🧬️mutations/📍move-vertex/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/📍move-vertex/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/📍move-vertex/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_edge {
            #[path = "🧬️schema/🧬️mutations/🖇️create-edge/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🖇️create-edge/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🖇️create-edge/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_face {
            #[path = "🧬️schema/🧬️mutations/🔷create-face/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🔷create-face/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🔷create-face/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_solid {
            #[path = "🧬️schema/🧬️mutations/🕳️delete-solid/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🕳️delete-solid/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🕳️delete-solid/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_vertex {
            #[path = "🧬️schema/🧬️mutations/🗑️delete-vertex/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🗑️delete-vertex/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🗑️delete-vertex/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod replace_surface {
            #[path = "🧬️schema/🧬️mutations/🗺️replace-surface/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🗺️replace-surface/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🗺️replace-surface/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_face {
            #[path = "🧬️schema/🧬️mutations/🚮delete-face/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🚮delete-face/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🚮delete-face/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_solid {
            #[path = "🧬️schema/🧬️mutations/🧊create-solid/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🧊create-solid/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🧊create-solid/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "📚️examples/🧊️solid/🦀️.rs"]
    pub mod solid;
}
#[path = "."]
pub mod editor {
    #[path = "✏️editor/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod presence {
        #[path = "✏️editor/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "✏️editor/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            #[path = "✏️editor/🎭️modes/✏️edit/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "👁️viewer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod presence {
        #[path = "👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "👁️viewer/🎭️modes/👁️view/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

semio_framework_value::artifact_retire_struct!(schema::snapshot::BrepVertex { id, point, tol });
semio_framework_value::artifact_retire_struct!(schema::snapshot::BrepEdge { id, start_vertex, end_vertex, curve, tol });
semio_framework_value::artifact_retire_struct!(schema::snapshot::BrepLoopEdge { edge, orientation });
semio_framework_value::artifact_retire_struct!(schema::snapshot::BrepLoop { id, edges });
semio_framework_value::artifact_retire_struct!(schema::snapshot::BrepFace { id, outer_loop, inner_loops, surface, orientation, tol });
semio_framework_value::artifact_retire_struct!(schema::snapshot::BrepShellFace { face, orientation });
semio_framework_value::artifact_retire_struct!(schema::snapshot::BrepShell { id, faces });
semio_framework_value::artifact_retire_struct!(schema::snapshot::BrepSolidShell { shell, is_void });
semio_framework_value::artifact_retire_struct!(schema::snapshot::BrepSolid { id, shells });
semio_framework_value::artifact_retire_struct!(schema::snapshot::BrepCoedge { id, edge, forward, pcurve, prange, loop_id, next, prev });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioBrepSnapshot { schema, vertices, edges, loops, faces, shells, solids, coedges, next_label });

semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!(schema::snapshot::SemioBrepSnapshot);
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioBrepSnapshot, schema::mutations::SemioBrepMutation);
