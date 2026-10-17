//! 🥣️ Companion test crate of `semio-s-artifact-draw-drawing-scene`: the unit laws of the drawing scene preparation, painting, raster, boolean, trace, text and view modules. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod booleans {
    #[path = "../../🔀️booleans/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod identity {
    #[path = "."]
    pub mod admission {
        #[path = "../../🪪️identity/🚦️admission/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    #[path = "../../🪪️identity/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod paint {
    #[path = "."]
    pub mod scene {
        #[path = "."]
        pub mod query {
            #[path = "../../🎨️paint/📋️prepare/🎯️query/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "../../🎨️paint/📋️prepare/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    #[path = "../../🎨️paint/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod picking {
    #[path = "../../../🧮️geometry/🎯️picking/🎨️paint/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod placement {
    #[path = "../../📍️placement/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod preparation {
    #[path = "../../📋️prepare/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod raster {
    #[path = "../../📷️raster/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod retirement {
    #[path = "../../🧹️retire/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod text {
    #[path = "../../🔤️text/🧪️tests/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod trace {
    #[path = "../../🔍️trace/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod view {
    #[path = "../../👁️view/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
