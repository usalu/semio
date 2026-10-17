//! 🔬️ tests of `semio-s-artifact-stdio-xml` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;

#[cfg(test)]
pub use schema::snapshot::XmlSnapshot;
#[cfg(test)]
pub mod schema {
    pub use super::standards::v1_0::subsets::base::schema::*;
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::schema::mutations::*;
                        #[path = "."]
                        pub mod set_doctype {
                            pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::schema::mutations::set_doctype::*;
                            #[cfg(test)]
                            #[path = "../../../../📜️set-doctype/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[path = "."]
                        pub mod set_declaration {
                            pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::schema::mutations::set_declaration::*;
                            #[cfg(test)]
                            #[path = "../../../../📣️set-declaration/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[path = "."]
                        pub mod set_text {
                            pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::schema::mutations::set_text::*;
                            #[cfg(test)]
                            #[path = "../../../../✍️set-text/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[path = "."]
                        pub mod remove_element {
                            pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::schema::mutations::remove_element::*;
                            #[cfg(test)]
                            #[path = "../../../../🗑️remove-element/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[cfg(test)]
                        #[path = "../../🦀️.rs"]
                        mod tests;
                        #[path = "."]
                        pub mod insert_element {
                            pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::schema::mutations::insert_element::*;
                            #[cfg(test)]
                            #[path = "../../../../📥️insert-element/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[path = "."]
                        pub mod set_attribute {
                            pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::schema::mutations::set_attribute::*;
                            #[cfg(test)]
                            #[path = "../../../../🏷️set-attribute/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../../🚪️io/📝️text/🧬️mutations/📥️insert-element/🦀️.rs"]
                            pub mod insert_element;
                            #[path = "../../../../../../🚪️io/📝️text/🧬️mutations/🗑️remove-element/🦀️.rs"]
                            pub mod remove_element;
                            #[path = "../../../../../../🚪️io/📝️text/🧬️mutations/📜️set-doctype/🦀️.rs"]
                            pub mod set_doctype;
                            #[path = "../../../../../../🚪️io/📝️text/🧬️mutations/✍️set-text/🦀️.rs"]
                            pub mod set_text;
                            #[path = "../../../../../../🚪️io/📝️text/🧬️mutations/🏷️set-attribute/🦀️.rs"]
                            pub mod set_attribute;
                            #[path = "../../../../../../🚪️io/📝️text/🧬️mutations/📣️set-declaration/🦀️.rs"]
                            pub mod set_declaration;
                        }
                    }
                    #[path = "../../../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                }
            }
        }
    }
}
