//! 🧬️ mut-base-3 of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🔑️set-dict-entry/🦀️.rs"]
                        pub mod set_dict_entry;
                        #[path = "../../🚫️remove-dict-entry/🦀️.rs"]
                        pub mod remove_dict_entry;
                        #[path = "../../🧳️set-trailer-entry/🦀️.rs"]
                        pub mod set_trailer_entry;
                        #[path = "../../🧽️remove-trailer-entry/🦀️.rs"]
                        pub mod remove_trailer_entry;
                        #[path = "../../🔀️move-page/🦀️.rs"]
                        pub mod move_page;
                        #[path = "../../✏️set-page-content/🦀️.rs"]
                        pub mod set_page_content;
                        #[path = "../../🔄️set-page-rotation/🦀️.rs"]
                        pub mod set_page_rotation;
                        #[path = "../../🖼️set-page-box/🦀️.rs"]
                        pub mod set_page_box;
                        #[path = "../../📏️set-page-user-unit/🦀️.rs"]
                        pub mod set_page_user_unit;
                        #[path = "../../🖋️insert-content/🦀️.rs"]
                        pub mod insert_content;
                        #[path = "../../🧻️remove-content/🦀️.rs"]
                        pub mod remove_content;
                        #[path = "../../🔁️replace-content/🦀️.rs"]
                        pub mod replace_content;
                        #[path = "../../📌️insert-annotation/🦀️.rs"]
                        pub mod insert_annotation;
                        #[path = "../../📍️remove-annotation/🦀️.rs"]
                        pub mod remove_annotation;
                        #[path = "../../📝️set-annotation/🦀️.rs"]
                        pub mod set_annotation;
                        #[path = "../../🔤️set-font/🦀️.rs"]
                        pub mod set_font;
                        #[path = "../../🅾️remove-font/🦀️.rs"]
                        pub mod remove_font;
                        #[path = "../../🏞️set-image/🦀️.rs"]
                        pub mod set_image;
                        #[path = "../../🌫️remove-image/🦀️.rs"]
                        pub mod remove_image;
                        #[path = "../../📄️set-form/🦀️.rs"]
                        pub mod set_form;
                        #[path = "../../🗞️remove-form/🦀️.rs"]
                        pub mod remove_form;
                        #[path = "../../🎛️set-ext-g-state/🦀️.rs"]
                        pub mod set_ext_g_state;
                        #[path = "../../🎚️remove-ext-g-state/🦀️.rs"]
                        pub mod remove_ext_g_state;
                        #[path = "../../🌅️set-shading/🦀️.rs"]
                        pub mod set_shading;
                        #[path = "../../🌄️remove-shading/🦀️.rs"]
                        pub mod remove_shading;
                        #[path = "../../🧩️set-pattern/🦀️.rs"]
                        pub mod set_pattern;
                        #[path = "../../🪡️remove-pattern/🦀️.rs"]
                        pub mod remove_pattern;
                    }
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                }
            }
        }
    }
}
