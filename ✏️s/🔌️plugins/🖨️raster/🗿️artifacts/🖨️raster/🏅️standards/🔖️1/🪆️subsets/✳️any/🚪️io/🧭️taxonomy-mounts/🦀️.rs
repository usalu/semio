//! 🧭️ Raster io taxonomy mounts: the export serializers and import deserializers of every stdio format, mounted below the io root.

#[path = "."]
pub mod export {
    #[path = "."]
    pub mod serializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod bmp {
                #[path = "."]
                pub mod v_v3 {
                    #[path = "../📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod gif {
                #[path = "."]
                pub mod v87a {
                    #[path = "../📤️export/🧵️serializers/🗿️artifacts/🎞️gif/🔖️87a/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod jpg {
                #[path = "."]
                pub mod v_jfif_1_01 {
                    #[path = "../📤️export/🧵️serializers/🗿️artifacts/📸️jpg/🔖️jfif-1.01/♾️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod v_rfc8259 {
                    #[path = "../📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod v1_2 {
                    #[path = "../📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod v1_1 {
                    #[path = "../📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod tiff {
                #[path = "."]
                pub mod v6_0 {
                    #[path = "../📤️export/🧵️serializers/🗿️artifacts/🖼️tiff/🔖️6.0/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
        }
    }
}
#[path = "."]
pub mod import {
    #[path = "."]
    pub mod deserializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod bmp {
                #[path = "."]
                pub mod v_v3 {
                    #[path = "../📥️import/🧩️deserializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod v_ac1018 {
                    #[path = "../📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod gif {
                #[path = "."]
                pub mod v87a {
                    #[path = "../📥️import/🧩️deserializers/🗿️artifacts/🎞️gif/🔖️87a/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod jpg {
                #[path = "."]
                pub mod v_jfif_1_01 {
                    #[path = "../📥️import/🧩️deserializers/🗿️artifacts/📸️jpg/🔖️jfif-1.01/♾️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod v_rfc8259 {
                    #[path = "../📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod v1_2 {
                    #[path = "../📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod v1_1 {
                    #[path = "../📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod tiff {
                #[path = "."]
                pub mod v6_0 {
                    #[path = "../📥️import/🧩️deserializers/🗿️artifacts/🖼️tiff/🔖️6.0/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
        }
    }
}
