//! 🪆️ The `animation` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
#[path = "."]
pub mod schema {
    #[path = "🧬️schema/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod snapshot {
        #[path = "🧬️schema/📸️snapshot/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod diff {
        #[path = "🧬️schema/🔺️diff/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod inferences {
        #[path = "🧬️schema/💡️inferences/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod duration {
            #[path = "🧬️schema/💡️inferences/⏱️duration/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod mutations {
        #[path = "🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod examples {
    #[path = "📚️examples/🚶️walk/🦀️.rs"]
    pub mod walk;
}
#[path = "."]
pub mod editor {
    #[path = "✏️editor/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod presence {
        #[path = "✏️editor/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "✏️editor/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            #[path = "✏️editor/🎭️modes/✏️edit/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "👁️viewer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod presence {
        #[path = "👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "👁️viewer/🎭️modes/👁️view/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

semio_framework_value::artifact_retire_leaf!(schema::snapshot::AnimInterpolation);
semio_framework_value::artifact_retire_struct!(schema::snapshot::AnimTarget { node, property });
semio_framework_value::artifact_retire_struct!(schema::snapshot::AnimKeyframe { t, value });
semio_framework_value::artifact_retire_struct!(schema::snapshot::AnimChannel { target, interpolation, keyframes });
semio_framework_value::artifact_retire_struct!(schema::snapshot::AnimTimeline { name, channels });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioAnimationSnapshot { schema, timelines });

semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioAnimationSnapshot, schema::mutations::SemioAnimationMutation);
