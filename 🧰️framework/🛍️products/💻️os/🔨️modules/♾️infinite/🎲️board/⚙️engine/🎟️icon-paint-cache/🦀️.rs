//! 🎟️ Icon paint cache of the board engine: the cached icon bodies, tokens, slots, registry, lease and the retirement of the cache. See [`super`].

use super::*;

pub enum CachedIconBody {
    Vector(Scene),
    Raster(Arc<RasterImage>),
}

struct CachedIconPaint {
    pub bx: f64,
    pub by: f64,
    pub bw: f64,
    pub bh: f64,
    pub body: CachedIconBody,
}

const ICON_PAINT_CACHE_CAPACITY: usize = 256;

#[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
const ICON_PAINT_CACHE_KEY_BYTE_CAPACITY: usize = 256;

#[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
const ICON_PAINT_SOURCE_BYTE_CAPACITY: usize = 16 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
struct IconPaintToken {
    pub slot: u16,
    pub generation: u64,
}

struct IconPaintSlot {
    pub key: Option<String>,
    epoch: u64,
    pub generation: u64,
    pub value: Option<CachedIconPaint>,
}

struct IconPaintRegistry {
    pub slots: Box<[IconPaintSlot; ICON_PAINT_CACHE_CAPACITY]>,
    epoch: u64,
    pub faulted: bool,
}

impl Default for IconPaintRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| IconPaintSlot { key: None, epoch: 0, generation: 0, value: None }), epoch: 1, faulted: false }
    }
}

impl IconPaintRegistry {
    #[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
    pub fn index(&self, key: &str) -> Option<usize> {
        self.slots.iter().position(|slot| slot.epoch == self.epoch && slot.key.as_deref() == Some(key) && slot.value.is_some())
    }

    #[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
    pub fn get(&self, key: &str) -> Option<&CachedIconPaint> {
        self.slots.get(self.index(key)?)?.value.as_ref()
    }

    #[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
    pub fn reserve(&mut self, key: &str) -> Option<IconPaintToken> {
        if key.len() > ICON_PAINT_CACHE_KEY_BYTE_CAPACITY || self.get(key).is_some() {
            self.faulted = true;
            return None;
        }
        let Some(index) = self.slots.iter().position(|slot| slot.key.is_none() && slot.value.is_none()) else {
            self.faulted = true;
            return None;
        };
        let slot = &mut self.slots[index];
        slot.generation = slot.generation.wrapping_add(1).max(1);
        slot.epoch = self.epoch;
        slot.key = Some(key.to_owned());
        Some(IconPaintToken { slot: index as u16, generation: slot.generation })
    }

    #[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
    fn publish(&mut self, token: IconPaintToken, value: CachedIconPaint) {
        let slot = self.slots.get_mut(usize::from(token.slot)).expect("reserved icon cache slot remains present");
        assert_eq!(slot.generation, token.generation, "reserved icon cache generation remains current");
        assert_eq!(slot.epoch, self.epoch, "reserved icon cache epoch remains current");
        assert!(slot.key.is_some() && slot.value.is_none(), "reserved icon cache slot remains unpublished");
        slot.value = Some(value);
    }

    #[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
    fn abort(&mut self, token: IconPaintToken) {
        let slot = self.slots.get_mut(usize::from(token.slot)).expect("reserved icon cache slot remains present");
        assert_eq!(slot.generation, token.generation, "aborted icon cache generation remains current");
        assert!(slot.value.is_none(), "only an unpublished icon cache reservation can abort");
        slot.key = None;
        slot.epoch = 0;
        slot.generation = slot.generation.wrapping_add(1).max(1);
    }

    fn invalidate(&mut self) {
        self.epoch = self.epoch.wrapping_add(1).max(1);
    }
}

/// 🖼️ Center and available screen extents for fitting one icon without coordinate conversion.
#[derive(Clone, Copy)]
pub struct IconScreenRect {
    pub center: Point,
    pub width: f64,
    pub height: f64,
}

/// 🖼️ Shared SVG/raster icon decode cache for board and DAG hosts.
pub struct IconPaintCache {
    pub cache: RefCell<ManuallyDrop<IconPaintRegistry>>,
    retirement_cursor: Cell<u16>,
    retirement_credited_bytes: Cell<usize>,
    retirement_scene: Cell<Option<semio_framework_canvas::OpaqueSceneRetirementToken>>,
    pub closing: Cell<bool>,
    registry_released:Cell<bool>,
    pub themed_icon_lookup: semio_framework_canvas::icon_codec::ThemedSvgLookup,
}

/// 📸️ One icon-retirement turn with current credit and physical release split.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconPaintRetirementStep {
    Blocked,
    Pending { released_items: usize, credited_bytes: usize, released_bytes: usize },
    Complete,
}

pub struct CachedIconPaintLease<'a> {
    pub cache: std::cell::Ref<'a, ManuallyDrop<IconPaintRegistry>>,
    pub slot: usize,
}

impl CachedIconPaintLease<'_> {
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        let paint = self.cache.slots[self.slot].value.as_ref().expect("leased icon cache slot remains published");
        (paint.bx, paint.by, paint.bw, paint.bh)
    }

    pub fn body(&self) -> &CachedIconBody {
        &self.cache.slots[self.slot].value.as_ref().expect("leased icon cache slot remains published").body
    }
}

impl Default for IconPaintCache {
    fn default() -> Self {
        Self {
            cache: RefCell::new(ManuallyDrop::new(IconPaintRegistry::default())),
            retirement_cursor: Cell::new(0),
            retirement_credited_bytes: Cell::new(0),
            retirement_scene: Cell::new(None),
            closing: Cell::new(false),
            registry_released:Cell::new(false),
            themed_icon_lookup: |_| None,
        }
    }
}

impl Clone for IconPaintCache {
    fn clone(&self) -> Self {
        Self {
            cache: RefCell::new(ManuallyDrop::new(IconPaintRegistry::default())),
            retirement_cursor: Cell::new(0),
            retirement_credited_bytes: Cell::new(0),
            retirement_scene: Cell::new(None),
            closing: Cell::new(false),
            registry_released:Cell::new(false),
            themed_icon_lookup: self.themed_icon_lookup,
        }
    }
}

impl Drop for IconPaintCache {
    fn drop(&mut self) {
        if self.registry_released.get(){return}
        let terminal = self.terminal_is_empty();
        let never_admitted = self.retirement_scene.get().is_none() && self.cache.get_mut().slots.iter().all(|slot| slot.key.is_none() && slot.value.is_none());
        debug_assert!(terminal || never_admitted || std::thread::panicking(), "IconPaintCache with admitted resources must reach terminal-empty through close_step before release");
        if terminal || never_admitted {
            unsafe { ManuallyDrop::drop(self.cache.get_mut()) };
        }
    }
}

struct IconOwnedRetirement {
    pub source:ManuallyDrop<Option<IconPaintCache>>,
    pub key:Option<semio_framework_value::retirement::controlled::ControlledRetirement<String>>,
    image:Option<semio_framework_value::retirement::arc_custody::SharedControlledRetirement<RasterImage>>,
    metadata:Option<semio_framework_value::retirement::controlled::ControlledRetirement<(f64,f64,f64,f64)>>,
    pub cursor:usize,
}

impl IconOwnedRetirement {
    fn terminal(&self)->bool{self.source.is_none()&&self.key.is_none()&&self.image.is_none()&&self.metadata.is_none()}
    pub fn demand(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
        let deeper=|depth:usize|depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original icon owner depth overflow"));
        if let Some(owner)=self.key.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:deeper(owner.next_depth_demand()?)?})}
        if let Some(owner)=self.metadata.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:deeper(owner.next_depth_demand()?)?})}
        if let Some(owner)=self.image.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:deeper(owner.next_depth_demand()?)?})}
        let Some(source)=self.source.as_ref() else{return Ok(RetirementDemand::default())};
        if source.retirement_scene.get().is_some()||source.retirement_credited_bytes.get()!=0{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"existing icon scene authority requires its original full receipt"))}
        if self.cursor==ICON_PAINT_CACHE_CAPACITY{return Ok(RetirementDemand{release_bytes:size_of::<[IconPaintSlot;ICON_PAINT_CACHE_CAPACITY]>(),depth:1,..Default::default()})}
        if matches!(source.cache.borrow().slots[self.cursor].value.as_ref().map(|value|&value.body),Some(CachedIconBody::Vector(_))){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original vector icon producer has not declared full scene retirement authority"))}
        Ok(RetirementDemand{depth:1,..Default::default()})
    }
    pub fn step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{
        use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{controlled::ControlledRetirement,arc_custody::SharedControlledRetirement}};
        let empty=RetainedCloneProgress::default();if self.terminal(){return Ok(RetainedCloneStep::Complete(empty))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
        let demand=self.demand(grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original icon retirement exceeds admitted depth"))}if grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(empty))}
        macro_rules! child{($field:ident,$method:ident)=>{if let Some(owner)=self.$field.as_mut(){if owner.terminal_is_empty(){self.$field=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.$method(child)?;return Ok(RetainedCloneStep::Progress(semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original icon child")?.progress()))}}}
        child!(key,step);child!(metadata,step);child!(image,step);
        let source=self.source.as_mut().unwrap();
        if self.cursor==ICON_PAINT_CACHE_CAPACITY{unsafe{ManuallyDrop::drop(source.cache.get_mut())}source.registry_released.set(true);drop(self.source.take());return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..empty}))}
        let slot=&mut source.cache.get_mut().slots[self.cursor];
        if let Some(key)=slot.key.take(){self.key=Some(ControlledRetirement::new(key).unwrap_or_else(|_|unreachable!("original icon key is supported")));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
        if let Some(value)=slot.value.take(){let CachedIconPaint{bx,by,bw,bh,body}=value;let CachedIconBody::Raster(image)=body else{unreachable!("original vector scene authority is checked before source transfer")};self.metadata=Some(ControlledRetirement::new((bx,by,bw,bh)).unwrap_or_else(|_|unreachable!("icon numeric metadata is supported")));self.image=Some(SharedControlledRetirement::lease(image));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
        self.cursor+=1;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))
    }
}

impl semio_framework_value::retirement::RetirementCursor for IconOwnedRetirement {
    fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{use semio_framework_value::{retirement::RetirementStep,retained_clone::{RetainedCloneProgress,RetainedCloneStep}};match self.step(grant){Err(error)=>RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(progress))if progress==RetainedCloneProgress::default()=>RetirementStep::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress)}}
    fn terminal_is_empty(&self)->bool{self.terminal()}
    fn next_work_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.demand(0)?.copy_bytes)}
    fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.demand(copy).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.demand(0).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.demand(0)?.depth)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal().then_some(size_of::<Self>())}
}

impl Drop for IconOwnedRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal(),"original icon cache abandoned before physical retirement");if self.terminal(){unsafe{ManuallyDrop::drop(&mut self.source)}}}}

impl semio_framework_value::retirement::RetireOwned for IconPaintCache {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(IconOwnedRetirement{source:ManuallyDrop::new(Some(self)),key:None,image:None,metadata:None,cursor:0})}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<IconOwnedRetirement>())}
    fn controlled_retirement_supported()->bool{true}
}

impl IconPaintCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&self) {
        assert!(self.retirement_scene.get().is_none(), "icon cache invalidation cannot detach an admitted scene retirement");
        assert_eq!(self.retirement_credited_bytes.get(), 0, "icon cache invalidation cannot detach admitted byte credit");
        self.cache.borrow_mut().invalidate();
        self.retirement_cursor.set(0);
    }

    pub fn close_step(&self) -> bool {
        matches!(self.close_page(1, usize::MAX), IconPaintRetirementStep::Complete)
    }

    pub fn close_page(&self, maximum_items: usize, maximum_bytes: usize) -> IconPaintRetirementStep {
        self.closing.set(true);
        if maximum_items == 0 || maximum_bytes == 0 {
            return IconPaintRetirementStep::Blocked;
        }
        if let Some(token) = self.retirement_scene.get() {
            return match semio_framework_canvas::advance_opaque_scene_retirement(token, maximum_items, maximum_bytes) {
                semio_framework_canvas::OpaqueSceneRetirementStep::Blocked => IconPaintRetirementStep::Blocked,
                semio_framework_canvas::OpaqueSceneRetirementStep::Pending { released_items, credited_bytes, released_bytes } => IconPaintRetirementStep::Pending { released_items, credited_bytes, released_bytes },
                semio_framework_canvas::OpaqueSceneRetirementStep::Complete { released_items, credited_bytes, released_bytes } => {
                    self.retirement_scene.set(None);
                    IconPaintRetirementStep::Pending { released_items, credited_bytes, released_bytes }
                }
                semio_framework_canvas::OpaqueSceneRetirementStep::Fault => {
                    self.cache.borrow_mut().faulted = true;
                    IconPaintRetirementStep::Blocked
                }
            };
        }
        let index = usize::from(self.retirement_cursor.get());
        if index == ICON_PAINT_CACHE_CAPACITY {
            return IconPaintRetirementStep::Complete;
        }
        let mut cache = self.cache.borrow_mut();
        let slot = &mut cache.slots[index];
        let released_bytes = slot.key.as_ref().map_or(0, String::capacity).saturating_add(match slot.value.as_ref().map(|value| &value.body) {
            Some(CachedIconBody::Raster(image)) if Arc::strong_count(image) == 1 => image.retirement_exclusive_backing_bytes(),
            Some(CachedIconBody::Raster(_)) | Some(CachedIconBody::Vector(_)) | None => 0,
        });
        let remaining_bytes = released_bytes.saturating_sub(self.retirement_credited_bytes.get());
        let credited_bytes = maximum_bytes.min(remaining_bytes);
        self.retirement_credited_bytes.set(self.retirement_credited_bytes.get().saturating_add(credited_bytes));
        if self.retirement_credited_bytes.get() != released_bytes {
            return IconPaintRetirementStep::Pending { released_items: 0, credited_bytes, released_bytes: 0 };
        }
        if let Some(CachedIconPaint { body: CachedIconBody::Vector(_), .. }) = slot.value.as_ref() {
            let Some(token) = semio_framework_canvas::reserve_opaque_scene_retirement() else {
                cache.faulted = true;
                return IconPaintRetirementStep::Blocked;
            };
            let paint = slot.value.take().expect("vector icon retirement slot remains occupied");
            let CachedIconBody::Vector(scene) = paint.body else {
                unreachable!("vector icon retirement was witnessed before ownership transfer");
            };
            semio_framework_canvas::publish_opaque_scene_retirement(token, scene);
            self.retirement_scene.set(Some(token));
        } else {
            slot.value = None;
        }
        slot.key = None;
        slot.epoch = 0;
        slot.generation = slot.generation.wrapping_add(1).max(1);
        self.retirement_cursor.set((index + 1) as u16);
        self.retirement_credited_bytes.set(0);
        IconPaintRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.registry_released.get() || self.closing.get()
            && usize::from(self.retirement_cursor.get()) == ICON_PAINT_CACHE_CAPACITY
            && self.retirement_credited_bytes.get() == 0
            && self.retirement_scene.get().is_none()
            && self.cache.borrow().slots.iter().all(|slot| slot.key.is_none() && slot.value.is_none())
    }

    pub fn faulted(&self) -> bool {
        self.cache.borrow().faulted
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn has_retiring_scene(&self) -> bool {
        self.retirement_scene.get().is_some()
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn occupied_slots(&self) -> usize {
        self.cache.borrow().slots.iter().filter(|slot| slot.key.is_some()).count()
    }

    #[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
    fn icon_vector_cache_key(tag: &str, svg: &str, fg: Color, bg: Color) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        svg.hash(&mut hasher);
        let hx = hasher.finish();
        let f = fg.to_rgba8();
        let b = bg.to_rgba8();
        format!("v8|{tag}|{hx:x}|{}|{:02x}{:02x}{:02x}{:02x}|{:02x}{:02x}{:02x}{:02x}", svg.len(), f.r, f.g, f.b, f.a, b.r, b.g, b.b, b.a)
    }

    #[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
    fn icon_raster_cache_key(rgba: &Arc<[u8]>, w: u32, h: u32) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        rgba.as_ref().hash(&mut hasher);
        let hx = hasher.finish();
        format!("v8|r|{w}x{h}|{hx:x}|{}", rgba.len())
    }

    /// 🖌️ Builds (or reuses a cached) icon paint — rasterizes SVG via `usvg`/`vello_svg`
    /// or decodes raster bytes via `image`, producing real pixels/vector paint for `Scene`.
    /// Host/browser only: a `wasm32-wasip2` guest has no display to paint onto, so this
    /// target has its own arm below that returns `None` unconditionally — the same value
    /// every caller here already treats as "nothing to paint", so no caller
    /// (`append_icon_at_screen_rect`, `paint_scene`, `build_vector_scene`, and trinity's own
    /// `TrinityBridge::paint_scene`, which is unreachable repo-wide as of this ticket, see
    /// `🔍️research/📓️intrinsic-size-wiring.md`) needs to change. Ticket
    /// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn get_or_build(&self, encoded: &str, fg: Color, bg: Color, preserve_original_style: bool) -> Option<CachedIconPaintLease<'_>> {
        if self.closing.get() {
            return None;
        }
        if encoded.len() > ICON_PAINT_SOURCE_BYTE_CAPACITY {
            self.cache.borrow_mut().faulted = true;
            return None;
        }
        let resolved = semio_framework_canvas::icon_codec::resolve_icon_kind(encoded, self.themed_icon_lookup);
        let key = match &resolved {
            semio_framework_canvas::icon_codec::ResolvedIcon::None => return None,
            semio_framework_canvas::icon_codec::ResolvedIcon::SvgThemed(s) | semio_framework_canvas::icon_codec::ResolvedIcon::SvgPlain(s) => Self::icon_vector_cache_key(if preserve_original_style { "p" } else { "t" }, s.as_str(), fg, bg),
            semio_framework_canvas::icon_codec::ResolvedIcon::RasterRgba8 { rgba, w, h } => Self::icon_raster_cache_key(rgba, *w, *h),
        };
        {
            let g = self.cache.borrow();
            if let Some(slot) = g.index(&key) {
                return Some(CachedIconPaintLease { cache: g, slot });
            }
        }
        let token = self.cache.borrow_mut().reserve(&key)?;
        let (bx, by, bw, bh, body) = match resolved {
            semio_framework_canvas::icon_codec::ResolvedIcon::None => {
                self.cache.borrow_mut().abort(token);
                return None;
            }
            semio_framework_canvas::icon_codec::ResolvedIcon::SvgThemed(s) => {
                let Some(doc) = SvgDocument::parse_icons(s.trim()).ok() else {
                    self.cache.borrow_mut().abort(token);
                    return None;
                };
                let (bx, by, bw, bh) = doc.content_bounds();
                if !(bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite()) {
                    self.cache.borrow_mut().abort(token);
                    return None;
                }
                let mut s = Scene::new();
                if preserve_original_style {
                    append_svg_document(&mut s, &doc);
                } else {
                    doc.render_themed(&mut s, fg, bg);
                }
                (bx, by, bw, bh, CachedIconBody::Vector(s))
            }
            semio_framework_canvas::icon_codec::ResolvedIcon::SvgPlain(s) => {
                let Some(doc) = SvgDocument::parse_icons(s.trim()).ok() else {
                    self.cache.borrow_mut().abort(token);
                    return None;
                };
                let (bx, by, bw, bh) = doc.content_bounds();
                if !(bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite()) {
                    self.cache.borrow_mut().abort(token);
                    return None;
                }
                let mut s = Scene::new();
                if preserve_original_style {
                    append_svg_document(&mut s, &doc);
                } else {
                    doc.render_themed(&mut s, fg, bg);
                }
                (bx, by, bw, bh, CachedIconBody::Vector(s))
            }
            semio_framework_canvas::icon_codec::ResolvedIcon::RasterRgba8 { rgba, w, h } => {
                let bx = 0.0_f64;
                let by = 0.0_f64;
                let bw = f64::from(w);
                let bh = f64::from(h);
                let img = RasterImage::rgba8(w, h, Arc::new(rgba.as_ref().to_vec()));
                (bx, by, bw, bh, CachedIconBody::Raster(Arc::new(img)))
            }
        };
        self.cache.borrow_mut().publish(token, CachedIconPaint { bx, by, bw, bh, body });
        let cache = self.cache.borrow();
        let slot = cache.index(&key).expect("published icon cache key remains indexed");
        Some(CachedIconPaintLease { cache, slot })
    }

    /// 🚫️ `wasm32-wasip2` arm: icon *painting* (rasterizing SVG/raster icon sources to
    /// `Scene` pixels/paths) is host-only by nature — a WASI guest component has no display
    /// to paint onto, and every real caller of icon painting on this target is either a
    /// browser bridge already excluded from `wasm32-wasip2` (`target_arch = "wasm32"` is TRUE
    /// for `wasm32-wasip2`, so those bridges use the narrower `not(target_env = "p2")` gate)
    /// or, for `semio-s-plugin-trinity`'s `TrinityBridge::paint_scene`, unreachable from any
    /// caller repo-wide. `None` here is not a stub for exercised behavior — it is the value
    /// every caller already treats as "nothing to paint", so nothing regresses if it is
    /// literally the only value this target ever produces. Ticket
    /// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,
    /// `🔍️research/📓️intrinsic-size-wiring.md`.
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    pub fn get_or_build(&self, _encoded: &str, _fg: Color, _bg: Color, _preserve_original_style: bool) -> Option<CachedIconPaintLease<'_>> {
        None
    }

    /// 🖼️ Paints an icon centered in a screen-space rectangle.
    pub fn append_icon_at_screen_rect(&self, scene: &mut Scene, icon_kind: &str, rect: IconScreenRect, fg: Color, bg: Color, preserve_original_style: bool) {
        let IconScreenRect { center, width: avail_w, height: avail_h } = rect;
        let Some(paint) = self.get_or_build(icon_kind, fg, bg, preserve_original_style) else {
            return;
        };
        let (bx, by, bw, bh) = paint.bounds();
        if !(avail_w > 0.0 && avail_h > 0.0) {
            return;
        }
        let fit_inset = ui_styling::metrics::icon::FIT_INSET;
        let sx_half = avail_w * fit_inset * 0.5;
        let sy_half = avail_h * fit_inset * 0.5;
        let cx = bx + bw * 0.5;
        let cy = by + bh * 0.5;
        let scale = (2.0 * sx_half / bw).min(2.0 * sy_half / bh);
        let aff = Affine::IDENTITY.translate((center.x - scale * cx, center.y - scale * cy)) * Affine::IDENTITY.scale(scale);
        let clip_inset = ui_styling::metrics::icon::CLIP_INSET;
        let hw = avail_w * clip_inset * 0.5;
        let hh = avail_h * clip_inset * 0.5;
        let clip_r = Rect::from_points(Point::new(center.x - hw, center.y - hh), Point::new(center.x + hw, center.y + hh));
        scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &clip_r);
        match paint.body() {
            CachedIconBody::Vector(icon_scene) => {
                scene.append(icon_scene, Some(aff));
            }
            CachedIconBody::Raster(img) => {
                scene.draw_image(img, aff);
            }
        }
        scene.pop_layer();
    }

    /// 🎨️ Themed SVG icon fg/bg from centralized canvas tokens (not node chrome stroke/fill).
    pub fn board_icon_paint_colors(canvas_theme: &CanvasPalette) -> (Color, Color) {
        let rgba = canvas_theme.raster_clear.to_rgba8();
        let lum = f64::from(rgba.r) * 0.299 + f64::from(rgba.g) * 0.587 + f64::from(rgba.b) * 0.114;
        let canvas = if lum < 128.0 { &ui_styling::CANVAS_DARK } else { &ui_styling::CANVAS_LIGHT };
        (Color::new(canvas.icon_fg), Color::new(canvas.icon_bg))
    }
}
