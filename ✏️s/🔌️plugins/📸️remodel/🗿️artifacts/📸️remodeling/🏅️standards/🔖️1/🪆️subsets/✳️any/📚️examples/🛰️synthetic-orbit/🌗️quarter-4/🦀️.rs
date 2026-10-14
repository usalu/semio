//! 🛰️ The fourth quarter of the synthetic orbit: views 27–35 (270°–350° azimuth), in capture order.

/// 🎞️ `(asset id, PNG bytes)` of this quarter's nine views.
pub const FRAMES: [(&str, &[u8]); 9] = [
    ("synthetic-orbit-frame-27", include_bytes!("../🖼️assets/🎞️frame-27.png")),
    ("synthetic-orbit-frame-28", include_bytes!("../🖼️assets/🎞️frame-28.png")),
    ("synthetic-orbit-frame-29", include_bytes!("../🖼️assets/🎞️frame-29.png")),
    ("synthetic-orbit-frame-30", include_bytes!("../🖼️assets/🎞️frame-30.png")),
    ("synthetic-orbit-frame-31", include_bytes!("../🖼️assets/🎞️frame-31.png")),
    ("synthetic-orbit-frame-32", include_bytes!("../🖼️assets/🎞️frame-32.png")),
    ("synthetic-orbit-frame-33", include_bytes!("../🖼️assets/🎞️frame-33.png")),
    ("synthetic-orbit-frame-34", include_bytes!("../🖼️assets/🎞️frame-34.png")),
    ("synthetic-orbit-frame-35", include_bytes!("../🖼️assets/🎞️frame-35.png")),
];
