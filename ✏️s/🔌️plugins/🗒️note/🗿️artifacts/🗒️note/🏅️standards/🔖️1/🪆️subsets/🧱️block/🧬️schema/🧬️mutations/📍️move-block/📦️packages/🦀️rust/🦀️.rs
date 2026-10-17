//! 📍️ mut-family-3 of `semio-s-artifact-note-note` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

pub use crate::schema::snapshot::NoteSnapshot;
pub use crate::standards::v1::subsets::any::schema;
pub use crate::schema::diff::NoteDiff;
pub use semio_s_artifact_note_note_schema::{NoteBlockNode};

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
            pub mod block {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod move_block {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod resize_block {
                            #[path = "../../../↔️resize-block/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../↔️resize-block/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../↔️resize-block/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_block_font_size {
                            #[path = "../../../🔤️change-block-font-size/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔤️change-block-font-size/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🔤️change-block-font-size/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod canvas {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_grid_visible {
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_grid_spacing {
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/📏️change-grid-spacing/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/📏️change-grid-spacing/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/📏️change-grid-spacing/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_grid_subdivisions {
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/🔢️change-grid-subdivisions/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/🔢️change-grid-subdivisions/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/🔢️change-grid-subdivisions/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_grid_opacity {
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/🌫️change-grid-opacity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/🌫️change-grid-opacity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/🌫️change-grid-opacity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_snap_enabled {
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/🧲️change-snap-enabled/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/🧲️change-snap-enabled/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/🧲️change-snap-enabled/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_snap_grid_spacing {
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🎨️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
