//! 🧩️ The xlsx ecma-376 strict subset: composer, builder and analyzer, conversions, editor, viewer and examples.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_ecma_376 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod strict {
                #[path = "../../🚪️io/🦀️.rs"]
                pub mod io;
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod xlsx {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod strict {
                        #[path = "../../✏️editor/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod presence {
                            #[path = "../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
                            pub mod schema;
                        }
                        #[path = "."]
                        pub mod config {
                            #[path = "../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
                            pub mod schema;
                        }
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod edit {
                                #[path = "../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod xlsx {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod strict {
                        #[path = "../../👁️viewer/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod presence {
                            #[path = "../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
                            pub mod schema;
                        }
                        #[path = "."]
                        pub mod config {
                            #[path = "../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
                            pub mod schema;
                        }
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod view {
                                #[path = "../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
