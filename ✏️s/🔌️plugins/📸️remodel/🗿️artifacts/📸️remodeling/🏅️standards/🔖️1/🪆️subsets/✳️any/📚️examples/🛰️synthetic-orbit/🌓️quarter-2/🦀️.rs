//! 🛰️ The second quarter of the synthetic orbit: views 9–17 (90°–170° azimuth), in capture order.

/// 🎞️ `(asset id, PNG bytes)` of this quarter's nine views.
pub const FRAMES: [(&str, &[u8]); 9] = [
    ("synthetic-orbit-frame-9", include_bytes!("../🖼️assets/🎞️frame-09.png")),
    ("synthetic-orbit-frame-10", include_bytes!("../🖼️assets/🎞️frame-10.png")),
    ("synthetic-orbit-frame-11", include_bytes!("../🖼️assets/🎞️frame-11.png")),
    ("synthetic-orbit-frame-12", include_bytes!("../🖼️assets/🎞️frame-12.png")),
    ("synthetic-orbit-frame-13", include_bytes!("../🖼️assets/🎞️frame-13.png")),
    ("synthetic-orbit-frame-14", include_bytes!("../🖼️assets/🎞️frame-14.png")),
    ("synthetic-orbit-frame-15", include_bytes!("../🖼️assets/🎞️frame-15.png")),
    ("synthetic-orbit-frame-16", include_bytes!("../🖼️assets/🎞️frame-16.png")),
    ("synthetic-orbit-frame-17", include_bytes!("../🖼️assets/🎞️frame-17.png")),
];
