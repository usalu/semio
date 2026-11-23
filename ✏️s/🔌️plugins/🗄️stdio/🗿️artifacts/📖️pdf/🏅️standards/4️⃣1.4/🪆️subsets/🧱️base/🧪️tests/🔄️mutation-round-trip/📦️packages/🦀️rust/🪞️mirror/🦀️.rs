//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::base::schema::diff;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::base::io::mutation_bridge;
                }
            }
            #[path = "."]
            pub mod a {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::a::schema::mutations::*;
                        #[path = "."]
                        pub mod set_page_text {
                            pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::a::schema::mutations::set_page_text::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🗄️a/🧬️schema/🧬️mutations/📝️set-page-text/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod clear_page_text {
                            pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::a::schema::mutations::clear_page_text::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🗄️a/🧬️schema/🧬️mutations/🧹️clear-page-text/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod x {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::x::schema::mutations::*;
                        #[path = "."]
                        pub mod set_page_size {
                            pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::x::schema::mutations::set_page_size::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🖨️x/🧬️schema/🧬️mutations/📐️set-page-size/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod collapse_page_size {
                            pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::x::schema::mutations::collapse_page_size::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🖨️x/🧬️schema/🧬️mutations/📉️collapse-page-size/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
