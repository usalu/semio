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

    /// @emoji 🌓️ linear-sRGB → Oklab, Björn Ottosson's reference matrices — the twin of
    /// `🌗️mixing/🟦️.ts`'s `linearToOklab` (https://bottosson.github.io/posts/oklab/).
    // 🚫️async: E1 pure accessor consumed by sync render/paint call sites — see R9
    pub fn linear_to_oklab(r: f32, g: f32, b: f32) -> [f32; 3] {
        let l = (0.412_221_47 * r + 0.536_332_55 * g + 0.051_445_995 * b).cbrt();
        let m = (0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b).cbrt();
        let s = (0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b).cbrt();
        [0.210_454_26 * l + 0.793_617_8 * m - 0.004_072_047 * s, 1.977_998_5 * l - 2.428_592_2 * m + 0.450_593_7 * s, 0.025_904_037 * l + 0.782_771_77 * m - 0.808_675_77 * s]
    }

    /// @emoji 🌓️ Oklab → linear-sRGB, the inverse of [`linear_to_oklab`] and the twin of
    /// `🌗️mixing/🟦️.ts`'s `oklabToLinear`.
    // 🚫️async: E1 pure accessor consumed by sync render/paint call sites — see R9
    pub fn oklab_to_linear(lightness: f32, a: f32, b: f32) -> [f32; 3] {
        let l = (lightness + 0.396_337_78 * a + 0.215_803_76 * b).powi(3);
        let m = (lightness - 0.105_561_346 * a - 0.063_854_17 * b).powi(3);
        let s = (lightness - 0.089_484_18 * a - 1.291_485_5 * b).powi(3);
        [4.076_741_7 * l - 3.307_711_6 * m + 0.230_969_93 * s, -1.268_438 * l + 2.609_757_4 * m - 0.341_319_4 * s, -0.004_196_086 * l - 0.703_418_6 * m + 1.707_614_7 * s]
    }

    /// @emoji 🌓️ CSS `color-mix(in oklab, <a> <100·(1-t)%>, <b>)` over LINEAR paints — the twin of
    /// `🌗️mixing/🟦️.ts`'s `oklabMix` without its byte round-trip, because every Rust paint is
    /// already linear. `t = 0` answers `a`, `t = 1` answers `b`; alpha lerps linearly.
    // 🚫️async: E1 pure accessor consumed by sync render/paint call sites — see R9
    pub fn oklab_mix(a: [f32; 4], b: [f32; 4], t: f32) -> [f32; 4] {
        let (oa, ob) = (linear_to_oklab(a[0], a[1], a[2]), linear_to_oklab(b[0], b[1], b[2]));
        let mixed = oklab_to_linear(oa[0] * (1.0 - t) + ob[0] * t, oa[1] * (1.0 - t) + ob[1] * t, oa[2] * (1.0 - t) + ob[2] * t);
        [mixed[0].clamp(0.0, 1.0), mixed[1].clamp(0.0, 1.0), mixed[2].clamp(0.0, 1.0), a[3] * (1.0 - t) + b[3] * t]
    }
