//! 🥣️ Companion test crate of `semio-s-artifact-draw-drawing-model`: the unit laws of the drawing model's fill, shape, affine, inference and snapshot-codec modules. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

#[path = "."]
pub mod affine {
    #[path = "../../🧮️geometry/↗️affine/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
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
