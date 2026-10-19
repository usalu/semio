//! 🅰️ Text styles and text system: the family choice, text style, alignment and run style, the font handles, slots and status, the font source and fetch and the `TextSystem` font registry. See [`super`].

use super::*;

//#region 🔖️Font
/// 🖋️ How to shape a run: family choice plus pixel size. Deliberately minimal — weight/style/features
/// are product-policy the caller resolves before reaching this crate, same posture `layout.rs` takes
/// on `Theme`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextStyle {
    pub family: FontFamilyChoice,
    pub size_px: f32,
}

/// ↔️ Paragraph alignment for [`TextSystem::shape_paragraph`] — our own enum so a caller never needs
/// `parley::Alignment` in scope; converted at the boundary by `TextSystem::to_parley_alignment`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Middle,
    Right,
    Justified,
}

/// 📝️ Full paragraph-shaping style for [`TextSystem::shape_paragraph`]: [`TextStyle`] plus
/// weight/line-height/letter-spacing/alignment — kept separate from `TextStyle` because
/// `measure`/`wrap`/cursor-movement callers never need these.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextRunStyle {
    pub base: TextStyle,
    pub weight: f32,
    pub line_height_relative: f32,
    pub letter_spacing_px: f32,
    pub alignment: TextAlignment,
}

/// 🔗️ A stable handle into [`TextSystem`]'s internal font table, carried by [`ShapedGlyph`] instead of
/// the raw `parley::Font`/`swash::FontRef` a real shape resolved to. Interning key is
/// `(fontique Blob::id(), font_index)` — `peniko`/`linebender_resource_handle`'s `Blob` already carries
/// a monotonic unique `id()` per allocation, so no content hashing or pointer arithmetic is needed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FontHandle(pub(super) u32);

pub(super) struct FontSlot {
    pub(super) data: Blob<u8>,
    pub(super) index: u32,
}

/// 🕓️ A host-requested font that has not necessarily loaded yet. `request_font` allocates one
/// synchronously (never blocking on the actual bytes); [`TextSystem::provide_font_bytes`] resolves it
/// once a host's [`FontSource`] impl finishes fetching.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FontDependencyId(pub(super) u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontStatus {
    Pending,
    Ready,
    Failed,
}

struct CustomFontSlot {
    pub(super) key: String,
    status: FontStatus,
}

/// 🌐️ The seam a future `ui-host` packet implements. Fetching font bytes is host I/O (network/disk) —
/// never this crate's concern under ruling U1, which is exactly why this trait has no async method:
/// `poll_font` is a synchronous, non-blocking status check the host calls from its own executor's poll
/// loop (or completion callback), and once it observes `FontFetch::Ready` it hands the bytes to
/// [`TextSystem::provide_font_bytes`] — a plain sync call, never awaited, never blocking a frame.
/// Generic per U3 (never `dyn FontSource`): a host picks its own transport/executor type and
/// monomorphizes over it, so this crate never names one. Unused within this crate itself — it exists
/// to be implemented outside it; see this packet's report for the registrar-request to wire it into
/// `ui-host` once that packet exists.
pub trait FontSource {
    fn poll_font(&mut self, key: &str) -> FontFetch;
}

/// 🌐️ One host font fetch's current state, as `FontSource::poll_font` reports it.
#[derive(Clone, Debug, PartialEq)]
pub enum FontFetch {
    Pending,
    Ready(Vec<u8>),
    Failed,
}

/// 🖋️ Font registration/fallback, a shaped-layout cache and the glyph atlas — the whole CPU text
/// pipeline for one window/surface. Every method is a plain sync `fn` (ruling U1): shaping and
/// rasterizing have no suspension point, so making them `async` would buy nothing and would cost the
/// frame its run-to-completion guarantee. A still-loading host font never blocks a call here — it
/// surfaces as [`Measurement::Pending`] with a placeholder metric (see `🔖️Measure`).
pub struct TextSystem {
    pub(super) font_cx: FontContext,
    pub(super) layout_cx: LayoutContext<[u8; 4]>,
    pub(super) scale_cx: ScaleContext,

    custom_fonts: HashMap<String, FontDependencyId>,
    custom_font_slots: Vec<CustomFontSlot>,
    ready_dependencies: Vec<FontDependencyId>,

    pub(super) font_table: Vec<FontSlot>,
    font_identity: HashMap<(u64, u32), FontHandle>,

    pub(super) shape_cache: HashMap<ShapeCacheKey, ShapedText>,
    pub(super) shape_hits: u32,
    pub(super) shape_misses: u32,

    pub(super) alpha_page: AtlasPage,
    pub(super) color_page: AtlasPage,
    pub(super) glyph_cache: HashMap<GlyphCacheKey, GlyphEntry>,
}

impl TextSystem {
    /// 🏗️ Registers the four built-in families (Anta/Kelly Slab/Share Tech Mono/Noto Emoji) from the
    /// bytes the host supplied and wires `GenericFamily::{SansSerif,Serif,Monospace,Emoji}` to them —
    /// deterministic and dependency-free (no network, no system-font scan:
    /// `CollectionOptions { system_fonts: false, .. }`), same guarantee the wgpu-old target's
    /// `FontAtlas::builtin()`/`shaped()` merge gave. Fails naming the role whose bytes are not a font.
    pub fn new(fonts: &TextFontSet<'_>) -> Result<Self, TextFontError> {
        let mut collection = Self::empty_collection();
        let sans_id = Self::register_builtin(&mut collection, fonts, TextFontRole::Sans)?;
        let serif_id = Self::register_builtin(&mut collection, fonts, TextFontRole::Serif)?;
        let mono_id = Self::register_builtin(&mut collection, fonts, TextFontRole::Mono)?;
        let mut emoji_id: Option<FamilyId> = None;
        for role in TextFontRole::EMOJI {
            if let Some(id) = Self::register_font(&mut collection, fonts.get(role), role.family()) {
                emoji_id.get_or_insert(id);
            }
        }
        let emoji_id = emoji_id.ok_or(TextFontError { role: TextFontRole::EmojiRegions })?;
        collection.set_generic_families(GenericFamily::Emoji, std::iter::once(emoji_id));
        collection.set_generic_families(GenericFamily::SansSerif, std::iter::once(sans_id));
        collection.set_generic_families(GenericFamily::Serif, std::iter::once(serif_id));
        collection.set_generic_families(GenericFamily::Monospace, std::iter::once(mono_id));
        Ok(Self::with_collection(collection))
    }

    /// 🕳️ A text system with no built-in family: only fonts a host later provides through
    /// [`Self::request_font`]/[`Self::provide_font_bytes`] shape. `FontFamilyChoice::{SansSerif,Serif,
    /// Monospace}` and every generic fallback resolve to nothing, so a glyph the provided font lacks
    /// shapes as that font's own `.notdef`.
    pub fn without_builtin_fonts() -> Self {
        Self::with_collection(Self::empty_collection())
    }

    fn empty_collection() -> Collection {
        Collection::new(CollectionOptions { shared: false, system_fonts: false })
    }

    fn with_collection(collection: Collection) -> Self {
        Self {
            font_cx: FontContext { collection, source_cache: SourceCache::default() },
            layout_cx: LayoutContext::new(),
            scale_cx: ScaleContext::new(),
            custom_fonts: HashMap::new(),
            custom_font_slots: Vec::new(),
            ready_dependencies: Vec::new(),
            font_table: Vec::new(),
            font_identity: HashMap::new(),
            shape_cache: HashMap::new(),
            shape_hits: 0,
            shape_misses: 0,
            alpha_page: AtlasPage::new("ui-text-glyph-atlas-alpha", 1),
            color_page: AtlasPage::new("ui-text-glyph-atlas-color", 4),
            glyph_cache: HashMap::new(),
        }
    }

    fn register_builtin(collection: &mut Collection, fonts: &TextFontSet<'_>, role: TextFontRole) -> Result<FamilyId, TextFontError> {
        Self::register_font(collection, fonts.get(role), role.family()).ok_or(TextFontError { role })
    }

    fn register_font(collection: &mut Collection, bytes: &[u8], family: &'static str) -> Option<FamilyId> {
        let over = FontInfoOverride { family_name: Some(family), width: None, style: None, weight: None, axes: None };
        collection.register_fonts(Blob::new(Arc::new(bytes.to_vec())), Some(over)).into_iter().next().map(|(id, _)| id)
    }

    /// 🔗️ Interns `(font, glyph_id)`'s font identity to a stable [`FontHandle`], allocating on first
    /// sight. `Blob::id()` is already a unique monotonic id per allocation, so `(id, index)` is a
    /// correct identity key without hashing font bytes or comparing pointers.
    pub(super) fn intern_font(&mut self, data: Blob<u8>, index: u32) -> FontHandle {
        let key = (data.id(), index);
        if let Some(handle) = self.font_identity.get(&key) {
            return *handle;
        }
        let handle = FontHandle(self.font_table.len() as u32);
        self.font_table.push(FontSlot { data, index });
        self.font_identity.insert(key, handle);
        handle
    }

    /// 🕓️ Idempotent: requesting the same `key` twice returns the same [`FontDependencyId`], same
    /// interning idiom `ResourceRegistry::intern_texture` uses for texture keys. Never blocks — the
    /// actual bytes arrive later via [`Self::provide_font_bytes`], driven by a host's [`FontSource`].
    pub fn request_font(&mut self, key: &str) -> FontDependencyId {
        if let Some(&id) = self.custom_fonts.get(key) {
            return id;
        }
        let id = FontDependencyId(self.custom_font_slots.len() as u32);
        self.custom_font_slots.push(CustomFontSlot { key: key.to_string(), status: FontStatus::Pending });
        self.custom_fonts.insert(key.to_string(), id);
        id
    }

    /// 📥️ Registers `bytes` under `id`'s requested key and marks it `Ready`, forcing the family name
    /// exactly like the built-in fonts [`Self::new`] registers. Returns `false` (no-op) for an unknown id, an already
    /// -resolved id, or empty/unparseable bytes (marked `Failed` in the last case) — never panics on a
    /// bad host response. On success, `id` is queued for [`Self::take_ready_dependencies`] so whoever
    /// owns this window's `FrameScheduler` can invalidate it with `InvalidationReason::RESOURCE_READY`.
    pub fn provide_font_bytes(&mut self, id: FontDependencyId, bytes: &[u8]) -> bool {
        let Some(slot) = self.custom_font_slots.get(id.0 as usize) else { return false };
        if slot.status == FontStatus::Ready || bytes.is_empty() {
            if bytes.is_empty() {
                if let Some(slot) = self.custom_font_slots.get_mut(id.0 as usize) {
                    slot.status = FontStatus::Failed;
                }
            }
            return false;
        }
        let key = slot.key.clone();
        let over = FontInfoOverride { family_name: Some(&key), width: None, style: None, weight: None, axes: None };
        let registered = self.font_cx.collection.register_fonts(Blob::new(Arc::new(bytes.to_vec())), Some(over));
        let Some(slot) = self.custom_font_slots.get_mut(id.0 as usize) else { return false };
        if registered.is_empty() {
            slot.status = FontStatus::Failed;
            return false;
        }
        slot.status = FontStatus::Ready;
        self.ready_dependencies.push(id);
        true
    }

    pub fn font_status(&self, id: FontDependencyId) -> FontStatus {
        self.custom_font_slots.get(id.0 as usize).map_or(FontStatus::Failed, |slot| slot.status)
    }

    /// 🔔️ Drains the dependencies that resolved since the last drain — the invalidation half of
    /// `Measurement::Pending`'s contract (`layout.rs`'s own docstring: "whoever completes that
    /// dependency ... is responsible for invalidating the window once it lands"). Same take-and-reset
    /// idiom as `ResourceRegistry::drain_ops`/the old `FontAtlas::take_dirty`.
    pub fn take_ready_dependencies(&mut self) -> Vec<FontDependencyId> {
        std::mem::take(&mut self.ready_dependencies)
    }

    pub(super) fn family_ready(&self, family: FontFamilyChoice) -> bool {
        match family {
            FontFamilyChoice::Custom(id) => matches!(self.custom_font_slots.get(id.0 as usize).map(|slot| slot.status), Some(FontStatus::Ready)),
            _ => true,
        }
    }

    /// 🧭️ Resolves a [`TextStyle`] to the `FontStack` a shape actually uses. A still-pending or failed
    /// custom font falls back to `FAMILY_SANS` here — this is the paint-phase fallback; the
    /// layout-phase `Measurement::Pending` short-circuit in `🔖️Measure` is what actually keeps a
    /// pending font from being measured with the wrong metrics.
    pub(super) fn font_stack_for(&self, family: FontFamilyChoice) -> FontStack<'static> {
        match family {
            FontFamilyChoice::SansSerif => FontStack::Source(Cow::Borrowed(FAMILY_SANS)),
            FontFamilyChoice::Serif => FontStack::Source(Cow::Borrowed(FAMILY_SERIF)),
            FontFamilyChoice::Monospace => FontStack::Source(Cow::Borrowed(FAMILY_MONO)),
            FontFamilyChoice::Custom(id) => match self.custom_font_slots.get(id.0 as usize) {
                Some(slot) if slot.status == FontStatus::Ready => FontStack::List(Cow::Owned(vec![FontFamily::Named(Cow::Owned(slot.key.clone())), FontFamily::Generic(GenericFamily::SansSerif)])),
                _ => FontStack::Source(Cow::Borrowed(FAMILY_SANS)),
            },
        }
    }
}
//#endregion 🔖️Font
