    /// @emoji 🌈️ Converts sRGB8888 bytes to linear-sRGB `f32` components for GPU paints.
    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn rgba8_to_linear(r: u8, g: u8, b: u8, a: u8) -> [f32; 4] {
        // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
        fn ch(c: u8) -> f32 {
            let x = f64::from(c) / 255.0;
            let lin = if x <= 0.04045 { x / 12.92 } else { ((x + 0.055) / 1.055).powf(2.4) };
            lin as f32
        }
        [ch(r), ch(g), ch(b), f32::from(a) / 255.0]
    }

    /// @emoji 🌈️ Converts linear-sRGB `f32` components to sRGB8888 bytes.
    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn linear_to_rgba8(lr: f32, lg: f32, lb: f32, la: f32) -> [u8; 4] {
        // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
        fn ch(l: f32) -> u8 {
            let x = l as f64;
            let s = if x <= 0.0031308 { x * 12.92 } else { 1.055 * x.powf(1.0 / 2.4) - 0.055 };
            (s * 255.0).round().clamp(0.0, 255.0) as u8
        }
        [ch(lr), ch(lg), ch(lb), (f64::from(la) * 255.0).round().clamp(0.0, 255.0) as u8]
    }
