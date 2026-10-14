//! 🧪️ remodeling editor integration tests: every law that drives the play app through its command dispatch.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod synthetic_orbit {
        #[cfg(test)]
        #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod remodeling {
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod add_stream {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱️add-stream/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod edit_calibration {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛠️edit-calibration/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod import_frames {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎞️import-frames/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod import_video_frame_payload {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📼️import-video-frame-payload/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod reset_placeholder_mesh {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/♻️reset-placeholder-mesh/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod set_active_example {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎬️set-active-example/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod set_camera {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📷️set-camera/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod set_ingest_params {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🥣️set-ingest-params/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod model {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod model {
                        #[cfg(test)]
                        #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                        #[path = "."]
                        pub mod config {
                            #[cfg(test)]
                            #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs"]
                            mod window_ownership_tests;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod capture {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod frames {
                        #[cfg(test)]
                        #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
            #[path = "."]
            pub mod analyze {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod report {
                        #[cfg(test)]
                        #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔍️analyze/🪟️windows/📊️report/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod calibration {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🎯️calibration/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod document {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod media {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗂️media/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod parameters {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/⚙️parameters/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod quality {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/✅️quality/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod results {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🧵️results/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod tracks {
                #[cfg(test)]
                #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🏃️tracks/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
        }
        #[cfg(test)]
        #[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub(crate) mod unit_tests;
    }
}
