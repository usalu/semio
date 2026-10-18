//! 🔠️ Text faces and kerning: the raster grid and capacity limits, the font families, the text face, the emoji buckets, the font advance, the authored face bytes and the pair kerning. See [`super`].

use super::*;

// #region text
/// 🔤️ Raster grid steps per device pixel — glyph bitmaps snap to a quarter pixel, advances never do.
pub(super) const RASTER_GRID_STEPS_PER_PX: f32 = 4.0;

/// 🔤️ Exact-size glyph entries kept before the derived map is rebuilt — a continuous zoom must not grow it without bound.
pub(super) const SIZED_GLYPH_CAPACITY: usize = 16_384;

/// 🤝️ Kerned scalar pairs cached before the pair map is rebuilt — no document may grow it without bound.
pub(super) const KERNING_PAIR_CAPACITY: usize = 16_384;

pub(super) const FAMILY_SANS: &str = "Anta";

pub(super) const FAMILY_SERIF: &str = "Kelly Slab";

pub(super) const FAMILY_MONO: &str = "Share Tech Mono";

const FAMILY_EMOJI: &str = "Noto Emoji";

/// 🔤️ Authored Latin face requested by one text run; emoji fallback remains shared.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TextFace {
    #[default]
    #[serde(rename="Anta")]
    Sans,
    #[serde(rename="Kelly Slab")]
    Serif,
    #[serde(rename="Share Tech Mono")]
    Mono,
    #[serde(rename="Noto Emoji")]
    Emoji,
}

impl TextFace {    /// 🎨️ Resolves an explicit first-party catalog family without inferring a face.
    pub fn from_catalog_family(family:&str)->Option<Self>{match family{"Anta"=>Some(Self::Sans),"Kelly Slab"=>Some(Self::Serif),"Share Tech Mono"=>Some(Self::Mono),"Noto Emoji"=>Some(Self::Emoji),_=>None}}
    pub fn family(self) -> &'static str {
        match self {
            TextFace::Sans => FAMILY_SANS,
            TextFace::Serif => FAMILY_SERIF,
            TextFace::Mono => FAMILY_MONO,
            TextFace::Emoji => FAMILY_EMOJI,
        }
    }
}

pub(super) const EMOJI_BUCKET_FAMILIES:[&str;12]=["Noto Emoji Regions","Noto Emoji Flags","Noto Emoji Symbols","Noto Emoji Objects","Noto Emoji Activities","Noto Emoji Travel","Noto Emoji Food","Noto Emoji Nature","Noto Emoji People","Noto Emoji Faces","Noto Emoji Joined Forms","Noto Emoji Supplement"];

pub(super) const EMOJI_BUCKET_STACK:&str="\"Noto Emoji Regions\",\"Noto Emoji Flags\",\"Noto Emoji Symbols\",\"Noto Emoji Objects\",\"Noto Emoji Activities\",\"Noto Emoji Travel\",\"Noto Emoji Food\",\"Noto Emoji Nature\",\"Noto Emoji People\",\"Noto Emoji Faces\",\"Noto Emoji Joined Forms\",\"Noto Emoji Supplement\"";

pub static NOTO_EMOJI_BUCKETS:[&[u8];12]=[
 semio_framework_font_noto_emoji_regions_regular::OUTLINE,
 semio_framework_font_noto_emoji_flags_regular::OUTLINE,
 semio_framework_font_noto_emoji_symbols_regular::OUTLINE,
 semio_framework_font_noto_emoji_objects_regular::OUTLINE,
 semio_framework_font_noto_emoji_activities_regular::OUTLINE,
 semio_framework_font_noto_emoji_travel_regular::OUTLINE,
 semio_framework_font_noto_emoji_food_regular::OUTLINE,
 semio_framework_font_noto_emoji_nature_regular::OUTLINE,
 semio_framework_font_noto_emoji_people_regular::OUTLINE,
 semio_framework_font_noto_emoji_faces_regular::OUTLINE,
 semio_framework_font_noto_emoji_joined_forms_regular::OUTLINE,
 semio_framework_font_noto_emoji_supplement_regular::OUTLINE,
];

/// 📏️ One scalar's advance per logical pixel of font size, straight from the face's own units (`hmtx` / unitsPerEm) — the ONE
/// advance source both the retained layout's text worker (off the main thread, no atlas) and the atlas answer to: the authored
/// face, then the emoji buckets (the atlas's own fallback), then the bitmap fallback's fixed `0.625` em; a default-ignorable
/// format control advances by nothing.
pub fn font_advance_em(face: TextFace, ch: char) -> f32 {
    if is_zero_width_format_char(ch) {
        return 0.0;
    }
    let advance = |bytes: &'static [u8]| {
        let font = SwashFontRef::from_index(bytes, 0)?;
        let glyph = font.charmap().map(ch);
        let units = f32::from(font.metrics(&[]).units_per_em);
        (glyph != 0 && units > 0.0).then(|| font.glyph_metrics(&[]).advance_width(glyph) / units)
    };
    advance(authored_face_bytes(face)).or_else(|| NOTO_EMOJI_BUCKETS.iter().find_map(|bytes| advance(bytes))).unwrap_or(BITMAP_ADVANCE_EM)
}

/// 📏️ The bitmap fallback's advance per logical pixel of font size.
const BITMAP_ADVANCE_EM: f32 = 0.625;

/// 🔤️ The embedded font file an authored face is drawn from — Anta for [`TextFace::Sans`] (React's `--font-sans`, and the file
/// every host's atlas fetches), Share Tech Mono for [`TextFace::Mono`].
pub(super) fn authored_face_bytes(face: TextFace) -> &'static [u8] {
    match face {
        TextFace::Sans => ANTA_LATIN,
        TextFace::Serif => KELLY_SLAB_LATIN,
        TextFace::Mono => SHARE_TECH_MONO_LATIN,
        TextFace::Emoji => NOTO_EMOJI_BUCKETS[0],
    }
}

/// 🤝️ Reads the authored faces through borrowed font IO without retaining a shaping cache.
#[derive(Clone,Copy)]
pub struct PairKerning {
    pub(super) sans: Option<SwashFontRef<'static>>,
    serif: Option<SwashFontRef<'static>>,
    pub(super) mono: Option<SwashFontRef<'static>>,
    sans_pairs: BorrowedFontPairSource<'static>,
    serif_pairs: BorrowedFontPairSource<'static>,
    mono_pairs: BorrowedFontPairSource<'static>,
}

impl Default for PairKerning {
    fn default() -> Self {
        Self {
            sans: SwashFontRef::from_index(authored_face_bytes(TextFace::Sans), 0),
            serif: SwashFontRef::from_index(authored_face_bytes(TextFace::Serif), 0),
            mono: SwashFontRef::from_index(authored_face_bytes(TextFace::Mono), 0),
            sans_pairs: BorrowedFontPairSource::read(authored_face_bytes(TextFace::Sans),*b"latn").expect("embedded Anta pair table"),
            serif_pairs: BorrowedFontPairSource::read(authored_face_bytes(TextFace::Serif),*b"latn").expect("embedded Kelly Slab pair table"),
            mono_pairs: BorrowedFontPairSource::read(authored_face_bytes(TextFace::Mono),*b"latn").expect("embedded Share Tech Mono pair table"),
        }
    }
}

impl PairKerning {
    /// 👓️ Borrowed font views own no physical allocation requiring retirement.
    pub fn terminal_is_empty(&self)->bool {true}
    /// 📏️ Projects the original glyph pair's design-unit adjustment to logical em units.
    pub fn em(&mut self, face: TextFace, left: char, right: char) -> f32 {
        if is_zero_width_format_char(left) || is_zero_width_format_char(right) { return 0.0; }
        let (font,pairs)=match face {TextFace::Sans=>(self.sans,&self.sans_pairs),TextFace::Serif=>(self.serif,&self.serif_pairs),TextFace::Mono=>(self.mono,&self.mono_pairs),TextFace::Emoji=>return 0.0};
        let Some(font)=font else{return 0.0};let charmap=font.charmap();let units=f32::from(font.metrics(&[]).units_per_em);
        let query=FontPairQuery{left_glyph:charmap.map(left),right_glyph:charmap.map(right)};
        if units<=0.0||query.left_glyph==0||query.right_glyph==0{return 0.0;}
        pairs.adjustment(query).expect("embedded authored pair adjustment").advance_units as f32/units
    }
}

pub(super) const BITMAP_GLYPH_W: u32 = 8;

pub(super) const BITMAP_GLYPH_H: u32 = 16;
// #endregion text
