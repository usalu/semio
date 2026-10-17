//! 🆕️ mut-family-2 of `semio-s-artifact-note-note` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

pub use crate::schema::snapshot::NoteSnapshot;
pub use crate::standards::v1::subsets::any::schema;
pub use crate::schema::diff::NoteDiff;
pub use semio_s_artifact_note_note_schema::{NoteBlockNode, NoteImageAsset};

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
                    pub use semio_s_artifact_note_note_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_note_note_diff::standards::v1::subsets::any::schema::diff;
                }
            }
            #[path = "."]
            pub mod asset {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_asset {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_asset_payload {
                            #[path = "../../../🔁️replace-asset-payload/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔁️replace-asset-payload/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🔁️replace-asset-payload/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_asset {
                            #[path = "../../../🗑️delete-asset/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🗑️delete-asset/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🗑️delete-asset/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod block {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_block {
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/➕️create-block/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/➕️create-block/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/➕️create-block/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_block {
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/❌️delete-block/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/❌️delete-block/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/❌️delete-block/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod move_block_to_container {
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/🚚️move-block-to-container/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/🚚️move-block-to-container/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/🚚️move-block-to-container/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod drag_blocks {
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod rename_block {
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/🔖️rename-block/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/🔖️rename-block/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/🔖️rename-block/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_block_visible {
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/👀️change-block-visible/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/👀️change-block-visible/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🧱️block/🧬️schema/🧬️mutations/👀️change-block-visible/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
