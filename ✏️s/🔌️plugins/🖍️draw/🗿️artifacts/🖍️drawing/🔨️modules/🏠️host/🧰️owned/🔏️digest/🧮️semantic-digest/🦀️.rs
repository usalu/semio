//! 🧮️ Drawing semantic digest: the digest totals and credit, the fill, stroke, path segment, layer variant and layer digest authorities.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct DrawingSemanticDigestTotals {
    pub semantic_items: usize,
    pub semantic_bytes: usize,
    pub maximum_field_bytes: usize,
    pub source_owner_items: usize,
    pub source_owner_bytes: usize,
    pub derived_owner_items: usize,
    pub derived_owner_bytes: usize,
}

pub(super) struct DrawingSemanticDigestCredit {
    pub items: usize,
    pub bytes: usize,
    pub maximum_field_bytes: usize,
    pub source_owner_items: usize,
    pub source_owner_bytes: usize,
    pub derived_owner_items: usize,
    pub derived_owner_bytes: usize,
    pub owner_census: DrawingFixedOwnerCensus,
    pub semantic: Option<semio_framework_hash::Sha256>,
    pub text: Option<semio_s_artifact_draw_drawing_host_arena::native_text_footprint::DrawingTextDigestCursor>,
}

impl Default for DrawingSemanticDigestCredit {
    fn default() -> Self {
        Self {
            items: 0,
            bytes: 0,
            maximum_field_bytes: 0,
            source_owner_items: 1,
            source_owner_bytes: size_of::<DrawingMutation>(),
            derived_owner_items: 0,
            derived_owner_bytes: 0,
            owner_census: DrawingFixedOwnerCensus::new(),
            semantic: Some(semio_framework_hash::Sha256::new()),
            text: None,
        }
    }
}

impl DrawingSemanticDigestCredit {
    pub fn add_source_owner(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.owner_census.admit(items, bytes, 0, 0)?;
        self.source_owner_items = self.source_owner_items.checked_add(items).ok_or("drawing-store.mutation-source-owner-item-overflow")?;
        self.source_owner_bytes = self.source_owner_bytes.checked_add(bytes).ok_or("drawing-store.mutation-source-owner-byte-overflow")?;
        if self.source_owner_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-source-owner-item-capacity");
        }
        if self.source_owner_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-source-owner-byte-capacity");
        }
        Ok(())
    }

    pub fn add_derived_owner(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.owner_census.admit(0, 0, items, bytes)?;
        self.derived_owner_items = self.derived_owner_items.checked_add(items).ok_or("drawing-store.mutation-derived-owner-item-overflow")?;
        self.derived_owner_bytes = self.derived_owner_bytes.checked_add(bytes).ok_or("drawing-store.mutation-derived-owner-byte-overflow")?;
        if self.derived_owner_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-derived-owner-item-capacity");
        }
        if self.derived_owner_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-derived-owner-byte-capacity");
        }
        Ok(())
    }

    pub fn source_vec<T, const N: usize>(&mut self, value: &semio_framework_value::list::PagedList<T, N>) -> Result<(), &'static str> {
        let items = 1usize.checked_add(value.capacity()).ok_or("drawing-store.mutation-source-owner-item-overflow")?;
        let bytes = value.allocated_bytes().checked_add(size_of_val(value)).ok_or("drawing-store.mutation-source-owner-byte-overflow")?;
        self.add_source_owner(items, bytes)
    }

    pub fn derived_vec<T, const N: usize>(&mut self, value: &semio_framework_value::list::PagedList<T, N>) -> Result<(), &'static str> {
        let bytes = value.len().checked_mul(size_of::<T>()).ok_or("drawing-store.mutation-derived-owner-byte-overflow")?;
        let pages = bytes.checked_add(DRAWING_MUTATION_RETAINED_PAGE_BYTES - 1).ok_or("drawing-store.mutation-derived-owner-byte-overflow")? / DRAWING_MUTATION_RETAINED_PAGE_BYTES;
        self.add_derived_owner(pages.max(1), 0)
    }

    pub fn observe_owned_string(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: &DrawingNativeText, cloned: bool, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let cursor = self.text.get_or_insert_with(semio_s_artifact_draw_drawing_host_arena::native_text_footprint::DrawingTextDigestCursor::default);
        let semantic = self.semantic.as_mut().ok_or("drawing-store.mutation-digest-sealed")?;
        let step = cursor.step(value, tag, 1, DRAWING_OWNED_FIELD_BYTES, |bytes| { semantic.update(bytes); digest.observe(bytes); })?;
        cx.consume_fuel(1);
        if !step.complete { return Ok(false); }
        let totals = cursor.totals().ok_or("drawing-store.text-digest-false-terminal")?;
        cursor.close_step(1);
        self.text = None;
        self.add_source_owner(1usize.checked_add(totals.chunks).ok_or("drawing-store.mutation-source-owner-item-overflow")?, size_of_val(value).checked_add(totals.backing_bytes).ok_or("drawing-store.mutation-source-owner-byte-overflow")?)?;
        if cloned { self.add_derived_owner(totals.chunks.max(1), 0)?; }
        self.maximum_field_bytes = self.maximum_field_bytes.max(value.len());
        self.items = self.items.checked_add(1).ok_or("drawing-store.mutation-item-overflow")?;
        self.bytes = self.bytes.checked_add(11).and_then(|bytes| bytes.checked_add(value.len())).ok_or("drawing-store.mutation-byte-overflow")?;
        Ok(true)
    }

    pub fn observe(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        if value.len() > DRAWING_OWNED_FIELD_BYTES {
            return Err("drawing-store.mutation-field-capacity");
        }
        self.maximum_field_bytes = self.maximum_field_bytes.max(value.len());
        self.items = self.items.checked_add(1).ok_or("drawing-store.mutation-item-overflow")?;
        self.bytes = self.bytes.checked_add(11).and_then(|bytes| bytes.checked_add(value.len())).ok_or("drawing-store.mutation-byte-overflow")?;
        if self.items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-item-capacity");
        }
        if self.bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-byte-capacity");
        }
        let prefix = [0xd8];
        let tag = tag.to_be_bytes();
        let length = (value.len() as u64).to_be_bytes();
        let semantic = self.semantic.as_mut().ok_or("drawing-store.mutation-digest-sealed")?;
        semantic.update(&prefix);
        semantic.update(&tag);
        semantic.update(&length);
        semantic.update(value);
        digest.observe(&prefix);
        digest.observe(&tag);
        digest.observe(&length);
        digest.observe(value);
        cx.consume_fuel(1);
        Ok(())
    }

    pub fn scalar_f64(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: f64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        self.observe(digest, tag, &value.to_bits().to_be_bytes(), cx)
    }

    pub fn scalar_usize(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: usize, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        self.observe(digest, tag, &(value as u64).to_be_bytes(), cx)
    }

    pub fn seal(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        self.observe(digest, 99, &[], cx)?;
        let semantic = self.semantic.take().ok_or("drawing-store.mutation-digest-sealed")?.finalize();
        digest.observe(b"drawing.semantic.sha256");
        digest.observe(&semantic);
        Ok(())
    }

    pub fn totals(&self) -> Option<DrawingSemanticDigestTotals> {
        self.semantic.is_none().then_some(DrawingSemanticDigestTotals {
            semantic_items: self.items,
            semantic_bytes: self.bytes,
            maximum_field_bytes: self.maximum_field_bytes,
            source_owner_items: self.source_owner_items,
            source_owner_bytes: self.source_owner_bytes,
            derived_owner_items: self.derived_owner_items,
            derived_owner_bytes: self.derived_owner_bytes,
        })
    }
}

pub(super) struct DrawingFillDigestAuthority {
    pub phase: u8,
    pub index: usize,
    pub field: u8,
    pub terminal: bool,
}

impl DrawingFillDigestAuthority {
    pub fn new() -> Self {
        Self { phase: 0, index: 0, field: 0, terminal: false }
    }

    pub fn step(&mut self, value: Option<&FillStyle>, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            credit.observe(digest, 200, &[u8::from(value.is_some())], cx)?;
            self.phase = 1;
            if value.is_none() {
                self.terminal = true;
            }
            return Ok(self.terminal);
        }
        let value = value.ok_or("drawing-store.digest-fill-missing")?;
        if self.phase == 1 {
            let variant = match value {
                FillStyle::Solid { .. } => 1,
                FillStyle::LinearGradient { .. } => 2,
                FillStyle::RadialGradient { .. } => 3,
            };
            credit.observe(digest, 201, &[variant], cx)?;
            self.phase = 2;
            return Ok(false);
        }
        match value {
            FillStyle::Solid { color } => {
                let Some(value) = color.get((self.phase - 2) as usize) else {
                    self.terminal = true;
                    return Ok(true);
                };
                credit.scalar_f64(digest, 202 + u16::from(self.phase - 2), *value, cx)?;
                self.phase += 1;
                self.terminal = self.phase == 6;
            }
            FillStyle::LinearGradient { x1, y1, x2, y2, stops } => {
                if self.phase <= 5 {
                    let fields = [*x1, *y1, *x2, *y2];
                    credit.scalar_f64(digest, 210 + u16::from(self.phase - 2), fields[(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                } else if self.phase == 6 {
                    credit.source_vec(stops)?;
                    credit.derived_vec(stops)?;
                    credit.scalar_usize(digest, 214, stops.len(), cx)?;
                    self.phase = 7;
                    self.terminal = stops.is_empty();
                } else {
                    let stop = stops.get(self.index).ok_or("drawing-store.digest-linear-stop")?;
                    if self.field == 0 {
                        credit.scalar_f64(digest, 215, stop.offset, cx)?;
                    } else {
                        credit.scalar_f64(digest, 215 + u16::from(self.field), stop.color[(self.field - 1) as usize], cx)?;
                    }
                    self.field += 1;
                    if self.field == 5 {
                        self.field = 0;
                        self.index += 1;
                        self.terminal = self.index == stops.len();
                    }
                }
            }
            FillStyle::RadialGradient { cx: center_x, cy: center_y, r, stops } => {
                if self.phase <= 4 {
                    let fields = [*center_x, *center_y, *r];
                    credit.scalar_f64(digest, 220 + u16::from(self.phase - 2), fields[(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                } else if self.phase == 5 {
                    credit.source_vec(stops)?;
                    credit.derived_vec(stops)?;
                    credit.scalar_usize(digest, 223, stops.len(), cx)?;
                    self.phase = 6;
                    self.terminal = stops.is_empty();
                } else {
                    let stop = stops.get(self.index).ok_or("drawing-store.digest-radial-stop")?;
                    if self.field == 0 {
                        credit.scalar_f64(digest, 224, stop.offset, cx)?;
                    } else {
                        credit.scalar_f64(digest, 224 + u16::from(self.field), stop.color[(self.field - 1) as usize], cx)?;
                    }
                    self.field += 1;
                    if self.field == 5 {
                        self.field = 0;
                        self.index += 1;
                        self.terminal = self.index == stops.len();
                    }
                }
            }
        }
        Ok(self.terminal)
    }
}

pub(super) struct DrawingStrokeDigestAuthority {
    pub phase: u8,
    pub index: usize,
    pub terminal: bool,
}

impl DrawingStrokeDigestAuthority {
    pub fn new() -> Self {
        Self { phase: 0, index: 0, terminal: false }
    }

    pub fn step(&mut self, value: Option<&StrokeStyle>, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            credit.observe(digest, 240, &[u8::from(value.is_some())], cx)?;
            self.phase = 1;
            if value.is_none() {
                self.terminal = true;
            }
            return Ok(self.terminal);
        }
        let value = value.ok_or("drawing-store.digest-stroke-missing")?;
        match self.phase {
            1..=4 => credit.scalar_f64(digest, 240 + u16::from(self.phase), value.color[(self.phase - 1) as usize], cx)?,
            5 => credit.scalar_f64(digest, 245, value.width, cx)?,
            6 => credit.observe(digest, 246, value.cap.as_str().as_bytes(), cx)?,
            7 => credit.observe(digest, 247, value.join.as_str().as_bytes(), cx)?,
            8 => credit.observe(digest, 248, &[u8::from(value.dash.is_some())], cx)?,
            9 => {
                let dash = value.dash.as_ref().ok_or("drawing-store.digest-dash-missing")?;
                credit.source_vec(dash)?;
                credit.derived_vec(dash)?;
                credit.scalar_usize(digest, 249, dash.len(), cx)?;
                self.terminal = dash.is_empty();
            }
            _ => {
                let dash = value.dash.as_ref().ok_or("drawing-store.digest-dash-missing")?;
                let item = *dash.get(self.index).ok_or("drawing-store.digest-dash-index")?;
                credit.scalar_f64(digest, 250, item, cx)?;
                self.index += 1;
                self.terminal = self.index == dash.len();
            }
        }
        if !self.terminal {
            self.phase = match self.phase {
                8 if value.dash.is_none() => {
                    self.terminal = true;
                    8
                }
                phase => phase + 1,
            };
        }
        Ok(self.terminal)
    }
}

pub(super) struct DrawingPathSegmentDigestAuthority {
    pub phase: u8,
    pub terminal: bool,
}

impl DrawingPathSegmentDigestAuthority {
    pub fn new() -> Self {
        Self { phase: 0, terminal: false }
    }

    pub fn step(&mut self, value: &PathSegment, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            let variant = match value {
                PathSegment::Move { .. } => 1,
                PathSegment::Line { .. } => 2,
                PathSegment::Quad { .. } => 3,
                PathSegment::Cubic { .. } => 4,
                PathSegment::Arc { .. } => 5,
                PathSegment::Close => 6,
            };
            credit.observe(digest, 300, &[variant], cx)?;
            self.phase = 1;
            self.terminal = matches!(value, PathSegment::Close);
            return Ok(self.terminal);
        }
        let index = (self.phase - 1) as usize;
        match value {
            PathSegment::Move { to } | PathSegment::Line { to } => {
                let Some(field) = to.get(index) else {
                    self.terminal = true;
                    return Ok(true);
                };
                credit.scalar_f64(digest, 301 + index as u16, *field, cx)?;
                self.phase += 1;
                self.terminal = self.phase == 3;
            }
            PathSegment::Quad { ctrl, to } => {
                let fields = [ctrl[0], ctrl[1], to[0], to[1]];
                credit.scalar_f64(digest, 303 + index as u16, fields[index], cx)?;
                self.phase += 1;
                self.terminal = self.phase == 5;
            }
            PathSegment::Cubic { ctrl1, ctrl2, to } => {
                let fields = [ctrl1[0], ctrl1[1], ctrl2[0], ctrl2[1], to[0], to[1]];
                credit.scalar_f64(digest, 307 + index as u16, fields[index], cx)?;
                self.phase += 1;
                self.terminal = self.phase == 7;
            }
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
                if index < 3 {
                    credit.scalar_f64(digest, 313 + index as u16, [*rx, *ry, *rotation][index], cx)?;
                } else if index < 5 {
                    credit.observe(digest, 313 + index as u16, &[u8::from([*large_arc, *sweep][index - 3])], cx)?;
                } else {
                    credit.scalar_f64(digest, 313 + index as u16, to[index - 5], cx)?;
                }
                self.phase += 1;
                self.terminal = self.phase == 8;
            }
            PathSegment::Close => self.terminal = true,
        }
        Ok(self.terminal)
    }
}

#[derive(Clone, Copy)]
struct DrawingOptionalDigestField {
    pub tag: u16,
    pub present: bool,
    pub next: u8,
    pub absent: u8,
}

struct DrawingLayerVariantDigestAuthority {
    pub phase: u8,
    pub index: usize,
    pub field: u8,
    pub segment: Option<DrawingPathSegmentDigestAuthority>,
    pub terminal: bool,
}

impl DrawingLayerVariantDigestAuthority {
    pub fn new() -> Self {
        Self { phase: 1, index: 0, field: 0, segment: None, terminal: false }
    }

    pub fn option(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, field: DrawingOptionalDigestField, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        let DrawingOptionalDigestField { tag, present, next, absent } = field;
        credit.observe(digest, tag, &[u8::from(present)], cx)?;
        self.phase = if present { next } else { absent };
        Ok(())
    }

    pub fn step(&mut self, layer: &DrawingLayerNode, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        match layer {
            DrawingLayerNode::Shape(value) => match self.phase {
                1 => {
                    if !credit.observe_owned_string(digest, 340, &value.shape_kind, true, cx)? { return Ok(false); }
                    self.phase = 2;
                }
                2 => self.option(digest, credit, DrawingOptionalDigestField { tag: 341, present: value.rect.is_some(), next: 3, absent: 7 }, cx)?,
                3..=6 => {
                    let rect = value.rect.as_ref().ok_or("drawing-store.digest-rect-missing")?;
                    credit.scalar_f64(digest, 342 + u16::from(self.phase - 3), [rect.x, rect.y, rect.width, rect.height][(self.phase - 3) as usize], cx)?;
                    self.phase += 1;
                }
                7 => self.option(digest, credit, DrawingOptionalDigestField { tag: 346, present: value.ellipse.is_some(), next: 8, absent: 12 }, cx)?,
                8..=11 => {
                    let ellipse = value.ellipse.as_ref().ok_or("drawing-store.digest-ellipse-missing")?;
                    credit.scalar_f64(digest, 347 + u16::from(self.phase - 8), [ellipse.cx, ellipse.cy, ellipse.rx, ellipse.ry][(self.phase - 8) as usize], cx)?;
                    self.phase += 1;
                }
                12 => self.option(digest, credit, DrawingOptionalDigestField { tag: 351, present: value.circle.is_some(), next: 13, absent: 16 }, cx)?,
                13..=15 => {
                    let circle = value.circle.as_ref().ok_or("drawing-store.digest-circle-missing")?;
                    credit.scalar_f64(digest, 352 + u16::from(self.phase - 13), [circle.cx, circle.cy, circle.r][(self.phase - 13) as usize], cx)?;
                    self.phase += 1;
                }
                16 => self.option(digest, credit, DrawingOptionalDigestField { tag: 355, present: value.line.is_some(), next: 17, absent: 21 }, cx)?,
                17..=20 => {
                    let line = value.line.as_ref().ok_or("drawing-store.digest-line-missing")?;
                    credit.scalar_f64(digest, 356 + u16::from(self.phase - 17), [line.x1, line.y1, line.x2, line.y2][(self.phase - 17) as usize], cx)?;
                    self.phase += 1;
                }
                21 => self.option(digest, credit, DrawingOptionalDigestField { tag: 360, present: value.polygon.is_some(), next: 22, absent: 24 }, cx)?,
                22 => {
                    let points = &value.polygon.as_ref().ok_or("drawing-store.digest-polygon-missing")?.points;
                    credit.source_vec(points)?;
                    credit.derived_vec(points)?;
                    credit.scalar_usize(digest, 361, points.len(), cx)?;
                    self.phase = 23;
                    self.terminal = points.is_empty();
                }
                23 => {
                    let points = &value.polygon.as_ref().ok_or("drawing-store.digest-polygon-missing")?.points;
                    let point = points.get(self.index).ok_or("drawing-store.digest-point-index")?;
                    credit.scalar_f64(digest, 362 + u16::from(self.field), point[self.field as usize], cx)?;
                    self.field += 1;
                    if self.field == 2 {
                        self.field = 0;
                        self.index += 1;
                        self.terminal = self.index == points.len();
                    }
                }
                _ => self.terminal = true,
            },
            DrawingLayerNode::Path(value) => match self.phase {
                1 => {
                    credit.source_vec(&value.segments)?;
                    credit.derived_vec(&value.segments)?;
                    credit.scalar_usize(digest, 370, value.segments.len(), cx)?;
                    self.phase = 2;
                    self.terminal = value.segments.is_empty();
                }
                _ => {
                    let segment = value.segments.get(self.index).ok_or("drawing-store.digest-segment-index")?;
                    let cursor = self.segment.get_or_insert_with(DrawingPathSegmentDigestAuthority::new);
                    if cursor.step(segment, digest, credit, cx)? {
                        self.segment = None;
                        self.index += 1;
                        self.terminal = self.index == value.segments.len();
                    }
                }
            },
            DrawingLayerNode::Text(value) => {
                match self.phase {
                    1 => credit.scalar_f64(digest, 380, value.x, cx)?,
                    2 => credit.scalar_f64(digest, 381, value.y, cx)?,
                    3 => { if !credit.observe_owned_string(digest, 382, &value.content, true, cx)? { return Ok(false); } },
                    4 => credit.scalar_f64(digest, 383, value.size, cx)?,
                    _ => {
                        self.terminal = true;
                        return Ok(true);
                    }
                }
                self.phase += 1;
                self.terminal = self.phase == 5;
            }
            DrawingLayerNode::Image(value) => {
                match self.phase {
                    1 => { if !credit.observe_owned_string(digest, 390, &value.image_key, true, cx)? { return Ok(false); } },
                    2 => credit.scalar_f64(digest, 391, value.width, cx)?,
                    3 => credit.scalar_f64(digest, 392, value.height, cx)?,
                    _ => {
                        self.terminal = true;
                        return Ok(true);
                    }
                }
                self.phase += 1;
                self.terminal = self.phase == 4;
            }
            DrawingLayerNode::Group(value) => {
                if self.phase==1 {
                    credit.source_vec(&value.children)?;
                    credit.derived_vec(&value.children)?;
                    credit.scalar_usize(digest,400,value.children.len(),cx)?;
                    self.phase=2;
                } else {credit.observe(digest,401,&[u8::from(value.isolation)],cx)?;self.terminal=true;}
            }
            DrawingLayerNode::Boolean(value) => match self.phase {
                1 => {
                    if !credit.observe_owned_string(digest, 410, &value.operation, true, cx)? { return Ok(false); }
                    self.phase = 2;
                }
                2 => {
                    credit.source_vec(&value.children)?;
                    credit.derived_vec(&value.children)?;
                    credit.scalar_usize(digest, 411, value.children.len(), cx)?;
                    self.phase = 3;
                    self.terminal = value.children.is_empty();
                }
                _ => {
                    let item = value.children.get(self.index).ok_or("drawing-store.digest-boolean-child")?;
                    if !credit.observe_owned_string(digest, 412, item, true, cx)? { return Ok(false); }
                    self.index += 1;
                    self.terminal = self.index == value.children.len();
                }
            },
            DrawingLayerNode::Trace(value) => {
                match self.phase {
                    1 => { if !credit.observe_owned_string(digest, 420, &value.source_key, true, cx)? { return Ok(false); } },
                    2 => credit.scalar_f64(digest, 421, value.params.threshold, cx)?,
                    3 => credit.scalar_f64(digest, 422, value.params.simplify_epsilon, cx)?,
                    _ => {
                        self.terminal = true;
                        return Ok(true);
                    }
                }
                self.phase += 1;
                self.terminal = self.phase == 4;
            }
        }
        Ok(self.terminal)
    }
}

pub(super) struct DrawingLayerDigestAuthority {
    pub depth: usize,
    pub path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub fill: Option<DrawingFillDigestAuthority>,
    pub stroke: Option<DrawingStrokeDigestAuthority>,
    pub variant: Option<DrawingLayerVariantDigestAuthority>,
    pub terminal: bool,
}

impl DrawingLayerDigestAuthority {
    pub fn new() -> Self {
        Self { depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], fill: None, stroke: None, variant: None, terminal: false }
    }

    pub fn step(&mut self, root: &DrawingLayerNode, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let node = DrawingSnapshotBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("drawing-store.digest-layer-path")?;
        let base = semio_s_artifact_draw_drawing_model::layer_base(node);
        let phase = self.frames[self.depth].phase;
        match phase {
            0 => {
                credit.add_source_owner(1, size_of::<DrawingLayerNode>())?;
                credit.add_derived_owner(1, size_of::<DrawingLayerNode>())?;
                let variant = match node {
                    DrawingLayerNode::Shape(_) => 1,
                    DrawingLayerNode::Path(_) => 2,
                    DrawingLayerNode::Text(_) => 3,
                    DrawingLayerNode::Image(_) => 4,
                    DrawingLayerNode::Group(_) => 5,
                    DrawingLayerNode::Boolean(_) => 6,
                    DrawingLayerNode::Trace(_) => 7,
                };
                credit.observe(digest, 100, &[variant], cx)?;
                self.frames[self.depth].phase = 1;
            }
            1 => {
                if !credit.observe_owned_string(digest, 101, &base.id, true, cx)? { return Ok(false); }
                self.frames[self.depth].phase = 2;
            }
            2 => {
                if !credit.observe_owned_string(digest, 102, &base.name, true, cx)? { return Ok(false); }
                self.frames[self.depth].phase = 3;
            }
            3 => {
                credit.observe(digest, 103, &[u8::from(base.visible)], cx)?;
                self.frames[self.depth].phase = 4;
            }
            4 => {
                credit.observe(digest, 104, &[u8::from(base.locked)], cx)?;
                self.frames[self.depth].phase = 5;
            }
            5 => {
                credit.scalar_f64(digest, 105, base.opacity, cx)?;
                self.frames[self.depth].phase = 6;
            }
            6 => {
                if !credit.observe_owned_string(digest, 106, &base.blend_mode, true, cx)? { return Ok(false); }
                self.frames[self.depth].phase = 7;
            }
            7..=12 => {
                let fields = [base.transform.x, base.transform.y, base.transform.scale_x, base.transform.scale_y, base.transform.rotation, base.transform.shear];
                credit.scalar_f64(digest, 107 + u16::from(phase - 7), fields[(phase - 7) as usize], cx)?;
                self.frames[self.depth].phase += 1;
            }
            13 => {
                let fill = self.fill.get_or_insert_with(DrawingFillDigestAuthority::new);
                if fill.step(base.attributes.fill.as_ref(), digest, credit, cx)? {
                    self.fill = None;
                    self.frames[self.depth].phase = 14;
                }
            }
            14 => {
                let stroke = self.stroke.get_or_insert_with(DrawingStrokeDigestAuthority::new);
                if stroke.step(base.attributes.stroke.as_ref(), digest, credit, cx)? {
                    self.stroke = None;
                    self.frames[self.depth].phase = 15;
                }
            }
            15 => {
                credit.observe(digest, 113, &[u8::from(base.attributes.fill_rule == semio_s_artifact_draw_drawing_model::fill_rule::FillRule::Nonzero)], cx)?;
                self.frames[self.depth].phase = 16;
            }
            16 => {
                let variant = self.variant.get_or_insert_with(DrawingLayerVariantDigestAuthority::new);
                if variant.step(node, digest, credit, cx)? {
                    self.variant = None;
                    self.frames[self.depth].phase = 17;
                }
            }
            17 => {
                if let DrawingLayerNode::Group(group) = node {
                    let child = self.frames[self.depth].child;
                    if child < group.children.len() {
                        if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH {
                            return Err("drawing-store.digest-layer-depth");
                        }
                        self.frames[self.depth].child += 1;
                        self.path[self.depth] = child;
                        self.depth += 1;
                        self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                }
                self.frames[self.depth].phase = 18;
                return Ok(false);
            }
            _ => {
                credit.observe(digest, 199, &[], cx)?;
                if self.depth == 0 {
                    self.terminal = true;
                } else {
                    self.depth -= 1;
                }
            }
        }
        Ok(self.terminal)
    }

    pub fn close(&mut self) {
        self.fill = None;
        self.stroke = None;
        self.variant = None;
        self.terminal = true;
    }
}
