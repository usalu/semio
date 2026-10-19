//! 🪟️ The editor config, editor presence and viewer presence schema mounts of `semio-s-artifact-stdio-tiff`'s snapshot, held by their own crate so the snapshot crate stays inside the source gate.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod tiff_any {
        #[path = "."]
        pub mod presence {
            #[path = "../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod tiff_baseline {
        #[path = "../../../../🧱️baseline/✏️editor/🧭️edit-rules/🦀️.rs"]
        pub mod edit_rules;
        #[path = "."]
        pub mod presence {
            #[path = "../../../../🧱️baseline/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../🧱️baseline/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod tiff_any {
        #[path = "."]
        pub mod presence {
            #[path = "../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
