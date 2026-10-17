//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                    pub use semio_s_artifact_stdio_json_schema_2::standards::v_rfc8259::subsets::base::schema::*;
                    #[path = "../../../../🔨️modules/🧬️mutation-support/🦀️.rs"]
                    pub mod mutation_support;
                    pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::schema::inferences::*;
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "../../../📥️insert-array-element/🦀️.rs"]
                        pub mod insert_array_element;
                        #[path = "../../../📤️remove-array-element/🦀️.rs"]
                        pub mod remove_array_element;
                        #[path = "../../../🗑️remove-member/🦀️.rs"]
                        pub mod remove_member;
                        #[path = "../../../✏️set-member/🦀️.rs"]
                        pub mod set_member;
                        #[path = "../../../🔢️set-scalar/🦀️.rs"]
                        pub mod set_scalar;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_json_schema_2::standards::v_rfc8259::subsets::base::io::binary::snapshot;
                        #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "../../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::io::text::snapshot;
                        #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "../../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                }
            }
            #[path = "."]
            pub mod geojson {
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🌍️geojson/🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod json_any {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_json_schema::editor::json_any::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_json_schema::editor::json_any::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod json_i_json {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../🛜️i-json/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_json_schema::editor::json_i_json::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_json_schema::editor::json_i_json::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../../🛜️i-json/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod json_any {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_json_schema::viewer::json_any::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_json_schema::viewer::json_any::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
