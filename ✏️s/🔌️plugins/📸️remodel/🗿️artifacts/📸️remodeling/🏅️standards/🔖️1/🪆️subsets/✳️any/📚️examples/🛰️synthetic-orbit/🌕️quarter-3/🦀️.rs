//! 🛰️ The third quarter of the synthetic orbit: views 18–26 (180°–260° azimuth), in capture order.

/// 🎞️ `(asset id, PNG bytes)` of this quarter's nine views.
pub const FRAMES: [(&str, &[u8]); 9] = [
    ("synthetic-orbit-frame-18", include_bytes!("../🖼️assets/🎞️frame-18.png")),
    ("synthetic-orbit-frame-19", include_bytes!("../🖼️assets/🎞️frame-19.png")),
    ("synthetic-orbit-frame-20", include_bytes!("../🖼️assets/🎞️frame-20.png")),
    ("synthetic-orbit-frame-21", include_bytes!("../🖼️assets/🎞️frame-21.png")),
    ("synthetic-orbit-frame-22", include_bytes!("../🖼️assets/🎞️frame-22.png")),
    ("synthetic-orbit-frame-23", include_bytes!("../🖼️assets/🎞️frame-23.png")),
    ("synthetic-orbit-frame-24", include_bytes!("../🖼️assets/🎞️frame-24.png")),
    ("synthetic-orbit-frame-25", include_bytes!("../🖼️assets/🎞️frame-25.png")),
    ("synthetic-orbit-frame-26", include_bytes!("../🖼️assets/🎞️frame-26.png")),
];
