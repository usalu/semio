//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../🦀️.rs"]
        pub mod mutations;
    }
    #[path = "."]
    pub mod text {
        pub use semio_s_artifact_stdio_semio_kit_snapshot::io::text::snapshot;
        #[path = "../../../../../📝️text/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_kit_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_kit_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "../../../../../../🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::create_object;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::delete_object;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::create_model;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::delete_model;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::create_properties;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::delete_properties;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::bind_representation;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::unbind_representation;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::change_representation_pin;
        #[path = "."]
        pub mod add_type {
            #[path = "../../../../../../🧬️schema/🧬️mutations/➕add-type/🦀️.rs"]
            mod component;
            #[path = "../../../../../../🧬️schema/🧬️mutations/➕add-type/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../../../🧬️schema/🧬️mutations/➕add-type/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_type {
            #[path = "../../../../../../🧬️schema/🧬️mutations/➖remove-type/🦀️.rs"]
            mod component;
            #[path = "../../../../../../🧬️schema/🧬️mutations/➖remove-type/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../../../🧬️schema/🧬️mutations/➖remove-type/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod rename_type {
            #[path = "../../../../../../🧬️schema/🧬️mutations/✏️rename-type/🦀️.rs"]
            mod component;
            #[path = "../../../../../../🧬️schema/🧬️mutations/✏️rename-type/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../../../🧬️schema/🧬️mutations/✏️rename-type/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod add_design {
            #[path = "../../../../../../🧬️schema/🧬️mutations/🆕add-design/🦀️.rs"]
            mod component;
            #[path = "../../../../../../🧬️schema/🧬️mutations/🆕add-design/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../../../🧬️schema/🧬️mutations/🆕add-design/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_design {
            #[path = "../../../../../../🧬️schema/🧬️mutations/🗑️remove-design/🦀️.rs"]
            mod component;
            #[path = "../../../../../../🧬️schema/🧬️mutations/🗑️remove-design/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../../../🧬️schema/🧬️mutations/🗑️remove-design/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod edit_design {
            #[path = "../../../../../../🧬️schema/🧬️mutations/🖊️edit-design/🦀️.rs"]
            mod component;
            #[path = "../../../../../../🧬️schema/🧬️mutations/🖊️edit-design/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../../../🧬️schema/🧬️mutations/🖊️edit-design/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                pub use semio_s_artifact_stdio_semio_kit_examples::editor::modes::edit::windows::*;
                #[path = "."]
                pub mod main {
                    pub use semio_s_artifact_stdio_semio_kit_examples::editor::modes::edit::windows::main::*;
                    #[cfg(test)]
                    #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                pub use semio_s_artifact_stdio_semio_kit_examples::viewer::modes::view::windows::*;
                #[path = "."]
                pub mod main {
                    pub use semio_s_artifact_stdio_semio_kit_examples::viewer::modes::view::windows::main::*;
                    #[cfg(test)]
                    #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
        }
    }
}
