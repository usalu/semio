//! 👻️ Tutorial ghost cursor glyphs: the Semio cursor assets (`🖼️assets/👆️cursor`) as filled polygons in their own viewBox space, one table per appearance, generated from `🖼️assets/👆️cursor/👻️ghost-glyphs/🔣️.json` — React paints the same assets through `--cursor-ghost-<glyph>`.

use semio_framework::onboarding::IntroductionCursor;

/// 🔷️ One filled and outlined polygon of a ghost cursor glyph; `fill` and `stroke` are sRGB bytes, `points` are in viewBox pixels.
pub struct GhostGlyphPolygon {
    pub fill: [u8; 3],
    pub stroke: [u8; 3],
    pub stroke_width: f32,
    pub points: &'static [[f32; 2]],
}

/// 🖱️ One ghost cursor glyph: its viewBox size and polygons in paint order.
pub struct GhostGlyph {
    pub width: f32,
    pub height: f32,
    pub polygons: &'static [GhostGlyphPolygon],
}

static LIGHT_DEFAULT: GhostGlyph = GhostGlyph { width: 14.6664, height: 23.8996, polygons: &[
    GhostGlyphPolygon { fill: [0, 1, 23], stroke: [247, 243, 227], stroke_width: 0.25, points: &[[0.125, 0.3101], [0.125, 23.7746], [5.7717, 23.7746], [5.7717, 6.1762]] },
    GhostGlyphPolygon { fill: [0, 1, 23], stroke: [247, 243, 227], stroke_width: 0.25, points: &[[6.5073, 6.9413], [14.4929, 15.2373], [10.5001, 19.3853], [6.5073, 15.2373]] },
] };

static LIGHT_POINTER: GhostGlyph = GhostGlyph { width: 14.6664, height: 23.8996, polygons: &[
    GhostGlyphPolygon { fill: [255, 52, 79], stroke: [247, 243, 227], stroke_width: 0.25, points: &[[0.125, 0.3101], [0.125, 23.7746], [5.7717, 23.7746], [5.7717, 6.1762]] },
    GhostGlyphPolygon { fill: [0, 1, 23], stroke: [247, 243, 227], stroke_width: 0.25, points: &[[6.5073, 6.9413], [14.4929, 15.2373], [10.5001, 19.3853], [6.5073, 15.2373]] },
] };

static LIGHT_GRAB: GhostGlyph = GhostGlyph { width: 14.6664, height: 23.8996, polygons: &[
    GhostGlyphPolygon { fill: [0, 166, 157], stroke: [247, 243, 227], stroke_width: 0.25, points: &[[0.125, 0.3101], [0.125, 23.7746], [5.7717, 23.7746], [5.7717, 6.1762]] },
    GhostGlyphPolygon { fill: [0, 1, 23], stroke: [247, 243, 227], stroke_width: 0.25, points: &[[6.5073, 6.9413], [14.4929, 15.2373], [10.5001, 19.3853], [6.5073, 15.2373]] },
] };

static LIGHT_GRABBING: GhostGlyph = GhostGlyph { width: 14.6664, height: 23.8996, polygons: &[
    GhostGlyphPolygon { fill: [0, 166, 157], stroke: [247, 243, 227], stroke_width: 0.25, points: &[[0.125, 0.3101], [0.125, 23.7746], [5.7717, 23.7746], [5.7717, 6.1762]] },
    GhostGlyphPolygon { fill: [0, 166, 157], stroke: [247, 243, 227], stroke_width: 0.25, points: &[[6.5073, 6.9413], [14.4929, 15.2373], [10.5001, 19.3853], [6.5073, 15.2373]] },
] };

static LIGHT_CROSSHAIR: GhostGlyph = GhostGlyph { width: 32.3198, height: 32.3419, polygons: &[
    GhostGlyphPolygon { fill: [0, 17, 23], stroke: [247, 243, 227], stroke_width: 0.2, points: &[[16.0782, 16.6303], [16.0782, 32.2419], [12.3802, 32.2419], [12.3802, 20.3241]] },
    GhostGlyphPolygon { fill: [0, 17, 23], stroke: [247, 243, 227], stroke_width: 0.2, points: &[[15.7115, 16.2615], [0.1, 16.2615], [0.1, 19.9596], [12.0018, 19.9596]] },
    GhostGlyphPolygon { fill: [0, 17, 23], stroke: [247, 243, 227], stroke_width: 0.2, points: &[[16.2416, 15.7115], [16.2416, 0.1], [19.9397, 0.1], [19.9397, 12.0178]] },
    GhostGlyphPolygon { fill: [0, 17, 23], stroke: [247, 243, 227], stroke_width: 0.2, points: &[[16.6083, 16.0804], [32.2198, 16.0804], [32.2198, 12.3823], [20.318, 12.3823]] },
] };

static LIGHT_MOVE: GhostGlyph = GhostGlyph { width: 31.9999, height: 28.154, polygons: &[
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 1, 23], stroke_width: 0.15, points: &[[31.8117, 12.226], [16.2002, 12.226], [16.2002, 15.9241], [27.9089, 15.9241]] },
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 1, 23], stroke_width: 0.15, points: &[[0.1882, 15.9241], [15.7997, 15.9241], [15.7997, 12.226], [4.0911, 12.226]] },
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 1, 23], stroke_width: 0.15, points: &[[17.849, 27.9662], [17.849, 16.3225], [14.151, 16.3225], [14.151, 24.0764]] },
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 1, 23], stroke_width: 0.15, points: &[[14.151, 0.1877], [14.151, 11.8315], [17.849, 11.8315], [17.849, 4.0776]] },
] };

static DARK_DEFAULT: GhostGlyph = GhostGlyph { width: 14.6664, height: 23.8996, polygons: &[
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 1, 23], stroke_width: 0.25, points: &[[0.125, 0.3101], [0.125, 23.7746], [5.7717, 23.7746], [5.7717, 6.1762]] },
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 1, 23], stroke_width: 0.25, points: &[[6.5073, 6.9413], [14.4929, 15.2373], [10.5001, 19.3853], [6.5073, 15.2373]] },
] };

static DARK_POINTER: GhostGlyph = GhostGlyph { width: 14.6664, height: 23.8996, polygons: &[
    GhostGlyphPolygon { fill: [255, 52, 79], stroke: [0, 17, 23], stroke_width: 0.25, points: &[[0.125, 0.3101], [0.125, 23.7746], [5.7717, 23.7746], [5.7717, 6.1762]] },
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 17, 23], stroke_width: 0.25, points: &[[6.5073, 6.9413], [14.4929, 15.2373], [10.5001, 19.3853], [6.5073, 15.2373]] },
] };

static DARK_GRAB: GhostGlyph = GhostGlyph { width: 14.6664, height: 23.8996, polygons: &[
    GhostGlyphPolygon { fill: [0, 166, 157], stroke: [0, 17, 23], stroke_width: 0.25, points: &[[0.125, 0.3101], [0.125, 23.7746], [5.7717, 23.7746], [5.7717, 6.1762]] },
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 17, 23], stroke_width: 0.25, points: &[[6.5073, 6.9413], [14.4929, 15.2373], [10.5001, 19.3853], [6.5073, 15.2373]] },
] };

static DARK_GRABBING: GhostGlyph = GhostGlyph { width: 14.6664, height: 23.8996, polygons: &[
    GhostGlyphPolygon { fill: [0, 166, 157], stroke: [247, 243, 227], stroke_width: 0.25, points: &[[0.125, 0.3101], [0.125, 23.7746], [5.7717, 23.7746], [5.7717, 6.1762]] },
    GhostGlyphPolygon { fill: [0, 166, 157], stroke: [247, 243, 227], stroke_width: 0.25, points: &[[6.5073, 6.9413], [14.4929, 15.2373], [10.5001, 19.3853], [6.5073, 15.2373]] },
] };

static DARK_CROSSHAIR: GhostGlyph = GhostGlyph { width: 32.3198, height: 32.3419, polygons: &[
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 1, 23], stroke_width: 0.2, points: &[[16.0782, 16.6303], [16.0782, 32.2419], [12.3802, 32.2419], [12.3802, 20.3241]] },
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 1, 23], stroke_width: 0.2, points: &[[15.7115, 16.2615], [0.1, 16.2615], [0.1, 19.9596], [12.0018, 19.9596]] },
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 1, 23], stroke_width: 0.2, points: &[[16.2416, 15.7115], [16.2416, 0.1], [19.9397, 0.1], [19.9397, 12.0178]] },
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 1, 23], stroke_width: 0.2, points: &[[16.6083, 16.0804], [32.2198, 16.0804], [32.2198, 12.3823], [20.318, 12.3823]] },
] };

static DARK_MOVE: GhostGlyph = GhostGlyph { width: 31.9999, height: 28.154, polygons: &[
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 1, 23], stroke_width: 0.15, points: &[[31.8117, 12.226], [16.2002, 12.226], [16.2002, 15.9241], [27.9089, 15.9241]] },
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 1, 23], stroke_width: 0.15, points: &[[0.1882, 15.9241], [15.7997, 15.9241], [15.7997, 12.226], [4.0911, 12.226]] },
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 1, 23], stroke_width: 0.15, points: &[[17.849, 27.9662], [17.849, 16.3225], [14.151, 16.3225], [14.151, 24.0764]] },
    GhostGlyphPolygon { fill: [247, 243, 227], stroke: [0, 1, 23], stroke_width: 0.15, points: &[[14.151, 0.1877], [14.151, 11.8315], [17.849, 11.8315], [17.849, 4.0776]] },
] };

/// 🖱️ The glyph of `cursor` for the light or dark appearance, matching React `--cursor-ghost-<glyph>`.
pub fn tutorial_ghost_glyph(cursor: IntroductionCursor, dark: bool) -> &'static GhostGlyph {
    match (cursor, dark) {
        (IntroductionCursor::Default, false) => &LIGHT_DEFAULT,
        (IntroductionCursor::Default, true) => &DARK_DEFAULT,
        (IntroductionCursor::Pointer, false) => &LIGHT_POINTER,
        (IntroductionCursor::Pointer, true) => &DARK_POINTER,
        (IntroductionCursor::Grab, false) => &LIGHT_GRAB,
        (IntroductionCursor::Grab, true) => &DARK_GRAB,
        (IntroductionCursor::Grabbing, false) => &LIGHT_GRABBING,
        (IntroductionCursor::Grabbing, true) => &DARK_GRABBING,
        (IntroductionCursor::Crosshair, false) => &LIGHT_CROSSHAIR,
        (IntroductionCursor::Crosshair, true) => &DARK_CROSSHAIR,
        (IntroductionCursor::Move, false) => &LIGHT_MOVE,
        (IntroductionCursor::Move, true) => &DARK_MOVE,
    }
}
