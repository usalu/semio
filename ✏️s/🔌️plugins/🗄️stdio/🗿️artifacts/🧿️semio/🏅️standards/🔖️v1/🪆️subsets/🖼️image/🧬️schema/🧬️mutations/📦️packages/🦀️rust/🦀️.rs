//! 🧬️ mut-family of `semio-s-artifact-stdio-semio-image` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_image_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_image_diff::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "../../🔀️move-frame/🦀️.rs"]
        pub mod move_frame;
        #[path = "../../🗑️remove-metadata-entry/🦀️.rs"]
        pub mod remove_metadata_entry;
        #[path = "../../🔢️set-bit-depth/🦀️.rs"]
        pub mod set_bit_depth;
        #[path = "../../🌈️set-colorspace/🦀️.rs"]
        pub mod set_colorspace;
        #[path = "../../📐️set-dimensions/🦀️.rs"]
        pub mod set_dimensions;
        #[path = "../../⏱️set-frame-delay/🦀️.rs"]
        pub mod set_frame_delay;
        #[path = "../../🖌️set-frame-pixels/🦀️.rs"]
        pub mod set_frame_pixels;
        #[path = "../../🎨️set-icc/🦀️.rs"]
        pub mod set_icc;
        #[path = "../../🏷️set-metadata-entry/🦀️.rs"]
        pub mod set_metadata_entry;
    }
}
