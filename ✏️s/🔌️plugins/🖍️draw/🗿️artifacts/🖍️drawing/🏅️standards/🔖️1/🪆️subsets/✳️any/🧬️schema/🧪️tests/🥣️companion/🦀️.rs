//! 🎗 Companion test crate of `semio-s-artifact-draw-drawing-model`: the unit laws of the drawing model's fill, shape, affine, inference, snapshot-codec, validation, receive, vector, fill-sampling and snapshot-lookup modules. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

#[path="../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/🎟️work/🧪️tests/🦀️.rs"]
mod work_fixture;

/// 🧪️ Runs one original lookup turn under a real original action and returns its recorded step.
pub fn test_step(
    grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    mut step: impl FnMut(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError>,
) -> semio_framework_value::retained_clone::RetainedCloneStep {
    semio_framework_value_clone_source_testing::drive::drive_reply::<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError>(
        grant,
        256,
        |result| result.as_ref().map_or_else(semio_framework_value::ValueError::retained_progress, |step| step.progress()),
        |effect, reply, recover| {
            assert!(!recover);
            Ok(reply.complete(step(effect)))
        },
    ).result.unwrap()
}

#[path = "."]
pub mod affine {
    #[path = "../../🧮️geometry/↗️affine/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod fill_rule {
    #[path = "../../🎨️fill/🌀️rule/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod inferences {
    #[path = "../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
    #[path = "."]
    pub mod topology {
        #[path = "../../💡️inferences/🧭topology/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "."]
        pub mod snapshot {
            #[path = "../../../🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
    }
    #[path = "."]
    pub mod text {
        #[path = "."]
        pub mod snapshot {
            #[path = "../../../🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
    }
    #[path = "."]
    pub mod sqlite {
        #[path = "."]
        pub mod snapshot {
            #[path = "../../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
            pub mod tests;
        }
    }
}
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
pub mod unit;
#[path = "../../../🚪️io/🧪️tests/🔬️unit/🦀️.rs"]
pub mod io_unit;
#[path = "."]
pub mod shape_geometry {
    #[path = "../../🔷️shape/✏️coordinates/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}

#[path = "."]
pub mod validation {
    #[path = "../../✅️validation/🧪️tests.rs"]
    pub mod tests;
}
#[path = "."]
pub mod native_receive {
    #[path = "../../🛬️receive/🧪️tests/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod vector_source {
    #[path = "../../🧵️vector/🧪️tests.rs"]
    pub mod tests;
    #[path = "../../🧵️vector/🧪️preparation.rs"]
    pub mod preparation_tests;
}
#[path = "."]
pub mod fill {
    #[path = "."]
    pub mod sampling {
        #[path = "../../🎨️fill/🎨️sampling/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod snapshot {
    #[path = "."]
    pub mod lookup {
        #[path = "."]
        pub mod original_text {
            #[path = "../../📸️snapshot/🔎️lookup/📝️original-text/🧪️tests/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod equality {
            #[path = "../../📸️snapshot/🔎️lookup/⚖️equality/🧪️tests/🦀️.rs"]
            pub mod tests;
        }
    }
}
