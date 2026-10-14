//! ⚙️ remodeling engine bridge: engine parameters and results from and to the document.


#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod remodeling {
        #[path = "."]
        pub mod engine {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod reconstruction {
                #[cfg(test)]
                #[path = "../../🏭️reconstruction/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
                #[cfg(test)]
                #[path = "../../🏭️reconstruction/🧪️tests/🔬️retention/🦀️.rs"]
                mod retention;
            }
        }
    }
}
