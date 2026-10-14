//! 🚪️ remodeling io: the subset io declaration, builder/analyzer/composer facets and the diff and inference codecs.


#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
