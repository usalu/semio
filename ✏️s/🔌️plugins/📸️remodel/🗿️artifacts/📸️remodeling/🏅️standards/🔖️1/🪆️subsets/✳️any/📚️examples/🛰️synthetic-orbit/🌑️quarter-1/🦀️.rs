//! 🛰️ The first quarter of the synthetic orbit: views 0–8 (0°–80° azimuth), in capture order.

/// 🎞️ `(asset id, PNG bytes)` of this quarter's nine views.
pub const FRAMES: [(&str, &[u8]); 9] = [
    ("synthetic-orbit-frame-0", include_bytes!("../🖼️assets/🎞️frame-00.png")),
    ("synthetic-orbit-frame-1", include_bytes!("../🖼️assets/🎞️frame-01.png")),
    ("synthetic-orbit-frame-2", include_bytes!("../🖼️assets/🎞️frame-02.png")),
    ("synthetic-orbit-frame-3", include_bytes!("../🖼️assets/🎞️frame-03.png")),
    ("synthetic-orbit-frame-4", include_bytes!("../🖼️assets/🎞️frame-04.png")),
    ("synthetic-orbit-frame-5", include_bytes!("../🖼️assets/🎞️frame-05.png")),
    ("synthetic-orbit-frame-6", include_bytes!("../🖼️assets/🎞️frame-06.png")),
    ("synthetic-orbit-frame-7", include_bytes!("../🖼️assets/🎞️frame-07.png")),
    ("synthetic-orbit-frame-8", include_bytes!("../🖼️assets/🎞️frame-08.png")),
];
