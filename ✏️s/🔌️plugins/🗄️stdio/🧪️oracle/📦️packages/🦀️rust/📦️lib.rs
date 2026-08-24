//! 🔮️ `semio-s-plugin-stdio-test-oracle` — the reference implementations for the artifacts this
//! plugin owns.
//!
//! This crate exists so the FRAMEWORK test platform never has to know that PDF, PNG, GIF, ZIP,
//! zlib, WAVE or CSV exist. It is contributed to the platform by
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️component.json`, which the platform discovers by convention;
//! adding a new artifact family here requires no framework edit at all.
//!
//! Every third-party reference library is linked ONLY here, behind an owned interface — no external
//! type appears in this crate's public API — and only under the `oracles` feature, which no
//! production target enables.

//#region 🔖️Modules
// ⚖️ The metamorphic laws every mutation case's `inverse-<kind>` and `identity-round-trip`
// scenarios claim, made assertable in role. Format-neutral and dependency-free — it knows the shape
// of the argument, never a format — so it sits beside the family modules rather than inside one.
#[path = "../../⚖️law/🦀️component.rs"]
pub mod law;

#[path = "../../📄️document/🦀️component.rs"]
pub mod document;

#[path = "../../🖼️raster/🦀️component.rs"]
pub mod raster;

#[path = "../../🎒️archive/🦀️component.rs"]
pub mod archive;

#[path = "../../🔊️audio/🦀️component.rs"]
pub mod audio;

#[path = "../../📊️tabular/🦀️component.rs"]
pub mod tabular;

#[path = "../../🧊️mesh/🦀️component.rs"]
pub mod mesh;

// 📰 Shared markup reference machinery (quick-xml tree, SVG geometry grammars, semantic
// projection). Contributed for the 🎨️svg 1.1 ✳️tiny and ✳️basic subset oracles, which are two
// profile restrictions of ONE schema and therefore genuinely share every parse, write, address
// and projection step — the family-module rule, not a copy in each subset.
#[path = "../../📰markup/🦀️component.rs"]
pub mod markup;
//#endregion 🔖️Modules

//#region 🔖️Artifacts
// 🪆️ Mirrors the plugin's own `artifacts::<format>::standards::<version>::subsets::<subset>` tree, so
// an oracle sits at the same address as the implementation it is evidence for. A mutation belongs to
// a subset — two standards of one format declare different vocabularies — and subsets that share an
// implementation reach it through the shared family modules above rather than duplicating it.
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod avi {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_0 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod bcf {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v2_1 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod binary {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_raw {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod bmp {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_v3 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod csv {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_rfc4180 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod deflate {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_rfc1950 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod docx {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    // 🏅️ The conformance-class subset — its oracle sits at the same taxonomy address as the
                    // implementation it is evidence for, beside `any` rather than inside it.
                    #[path = "."]
                    pub mod strict {
                        #[path = "../../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    // 🏅️ The conformance-class subset — its oracle sits at the same taxonomy address as the
                    // implementation it is evidence for, beside `any` rather than inside it.
                    #[path = "."]
                    pub mod transitional {
                        #[path = "../../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod dwg {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ac1018 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod v_ac1024 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod dxf {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_r12 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod epw {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_energyplus {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod gif {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v87a {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod v89a {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod gltf {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v2_0 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod html {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v5 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v2x3 {
                #[path = "."]
                pub mod reference {
                    #[path = "../../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🧪️oracle/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod cobie {
                        #[path = "../../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod cv20 {
                        #[path = "../../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod sav {
                        #[path = "../../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️sav/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod v4 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod jpg {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_jfif_1_01 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod json {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_rfc8259 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod las {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_0 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod md {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_commonmark {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod mp3 {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_mpeg1_layer3 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod mp4 {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_isobmff {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod obj {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v3_0 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_4 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    // 🏅️ A conformance subset of PDF 1.4 — its oracle sits at the same taxonomy address as
                    // the implementation it is evidence for, beside `any` rather than inside it.
                    #[path = "."]
                    pub mod a {
                        #[path = "../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    // 🏅️ A conformance subset of PDF 1.4 — its oracle sits at the same taxonomy address as
                    // the implementation it is evidence for, beside `any` rather than inside it.
                    #[path = "."]
                    pub mod x {
                        #[path = "../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod v1_7 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    // 🏅️ A conformance-class subset of PDF 1.7 — its oracle sits at the same taxonomy address as
                    // the implementation it is evidence for, beside `any` rather than inside it.
                    #[path = "."]
                    pub mod a {
                        #[path = "../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    // 🏅️ A conformance-class subset of PDF 1.7 — its oracle sits at the same taxonomy address as
                    // the implementation it is evidence for, beside `any` rather than inside it.
                    #[path = "."]
                    pub mod e {
                        #[path = "../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    // 🏅️ A conformance-class subset of PDF 1.7 — its oracle sits at the same taxonomy address as
                    // the implementation it is evidence for, beside `any` rather than inside it.
                    #[path = "."]
                    pub mod h {
                        #[path = "../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️h/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    // 🏅️ A conformance-class subset of PDF 1.7 — its oracle sits at the same taxonomy address as
                    // the implementation it is evidence for, beside `any` rather than inside it.
                    #[path = "."]
                    pub mod ua {
                        #[path = "../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️ua/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    // 🏅️ A conformance-class subset of PDF 1.7 — its oracle sits at the same taxonomy address as
                    // the implementation it is evidence for, beside `any` rather than inside it.
                    #[path = "."]
                    pub mod vt {
                        #[path = "../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    // 🏅️ A conformance-class subset of PDF 1.7 — its oracle sits at the same taxonomy address as
                    // the implementation it is evidence for, beside `any` rather than inside it.
                    #[path = "."]
                    pub mod x {
                        #[path = "../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️x/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ply {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_0 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod png {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_2 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pptx {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    // 🏅️ The conformance-class subset — its oracle sits at the same taxonomy address as the
                    // implementation it is evidence for, beside `any` rather than inside it.
                    #[path = "."]
                    pub mod strict {
                        #[path = "../../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    // 🏅️ The conformance-class subset — its oracle sits at the same taxonomy address as the
                    // implementation it is evidence for, beside `any` rather than inside it.
                    #[path = "."]
                    pub mod transitional {
                        #[path = "../../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod step {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ap214 {
                // 📐️ The standard-level reference codec and CC ladder, shared by every ap214 subset
                // so no class copies a Part-21 writer or a §4.3 classification.
                #[path = "."]
                pub mod reference {
                    #[path = "../../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🧪️oracle/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod cc1 {
                        #[path = "../../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc1/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod cc2 {
                        #[path = "../../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc2/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod cc3 {
                        #[path = "../../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc3/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod cc4 {
                        #[path = "../../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc4/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod cc5 {
                        #[path = "../../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc5/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod cc6 {
                        #[path = "../../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod stl {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ascii {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod svg {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_1 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod tiny {
                        #[path = "../../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod basic {
                        #[path = "../../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod tiff {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v6_0 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod tsv {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_iana {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod txt {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_utf_8 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod wav {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_riff_pcm {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod xlsx {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    // 🏅️ The conformance-class subset — its oracle sits at the same taxonomy address as the
                    // implementation it is evidence for, beside `any` rather than inside it.
                    #[path = "."]
                    pub mod strict {
                        #[path = "../../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    // 🏅️ The conformance-class subset — its oracle sits at the same taxonomy address as the
                    // implementation it is evidence for, beside `any` rather than inside it.
                    #[path = "."]
                    pub mod transitional {
                        #[path = "../../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod xml {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_0 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod valid {
                        #[path = "../../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod zip {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v2_0 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod iso21320 {
                        #[path = "../../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/🧪️oracle/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
//#endregion 🔖️Artifacts
