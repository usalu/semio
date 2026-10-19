//! 📌️ The catalogue, artifact, inspection and settings panel mounts of the puzzle 2d editor, held by their own crate so the retained crate stays inside the source gate.

#![allow(clippy::result_large_err)]

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "."]
        pub mod panels {
            #[path = "../../📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../📌️panels/🗿️artifact/🦀️.rs"]
            pub mod artifact;
            #[path = "../../📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
            #[path = "../../📌️panels/⚙️settings/🦀️.rs"]
            pub mod settings;
        }
    }
}
