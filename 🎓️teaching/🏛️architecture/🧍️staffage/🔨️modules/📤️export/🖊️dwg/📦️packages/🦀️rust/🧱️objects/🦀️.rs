//! 🧱️ The drawing database of an AC1024 file: the nine table controls with their standard records, the model and paper space blocks, the layouts and dictionaries every reader expects, one layer per scene layer, and the LWPOLYLINE and TEXT entities of the scene, each as a framed object record.
//! <https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf> (20 Data section AcDb:AcDbObjects, 20.4 Object prescriptions)

use crate::bits::{aci, frame, Ref, Streams, BY_BLOCK, BY_LAYER};
use crate::scene::{Layer, ModelScene};

/// 🔖️ The fixed handles of the standard objects; scene layers and entities follow `FIRST_FREE`.
pub mod handle {
    pub const BLOCK_CONTROL: u64 = 0x1;
    pub const LAYER_CONTROL: u64 = 0x2;
    pub const STYLE_CONTROL: u64 = 0x3;
    pub const LTYPE_CONTROL: u64 = 0x4;
    pub const VIEW_CONTROL: u64 = 0x5;
    pub const UCS_CONTROL: u64 = 0x6;
    pub const VPORT_CONTROL: u64 = 0x7;
    pub const APPID_CONTROL: u64 = 0x8;
    pub const DIMSTYLE_CONTROL: u64 = 0x9;
    pub const LAYER_0: u64 = 0xA;
    pub const STYLE_STANDARD: u64 = 0xB;
    pub const LTYPE_BYBLOCK: u64 = 0xC;
    pub const LTYPE_BYLAYER: u64 = 0xD;
    pub const LTYPE_CONTINUOUS: u64 = 0xE;
    pub const APPID_ACAD: u64 = 0xF;
    pub const DIMSTYLE_STANDARD: u64 = 0x10;
    pub const VPORT_ACTIVE: u64 = 0x11;
    pub const MODEL_RECORD: u64 = 0x12;
    pub const MODEL_BLOCK: u64 = 0x13;
    pub const MODEL_END: u64 = 0x14;
    pub const PAPER_RECORD: u64 = 0x15;
    pub const PAPER_BLOCK: u64 = 0x16;
    pub const PAPER_END: u64 = 0x17;
    pub const NAMED_OBJECTS: u64 = 0x18;
    pub const GROUP_DICTIONARY: u64 = 0x19;
    pub const MLINESTYLE_DICTIONARY: u64 = 0x1A;
    pub const MLINESTYLE_STANDARD: u64 = 0x1B;
    pub const LAYOUT_DICTIONARY: u64 = 0x1C;
    pub const LAYOUT_MODEL: u64 = 0x1D;
    pub const LAYOUT_PAPER: u64 = 0x1E;
    pub const PLOTSETTINGS_DICTIONARY: u64 = 0x1F;
    pub const PLOTSTYLE_DICTIONARY: u64 = 0x20;
    pub const PLOTSTYLE_NORMAL: u64 = 0x21;
    pub const MATERIAL_DICTIONARY: u64 = 0x22;
    pub const COLOR_DICTIONARY: u64 = 0x23;
    pub const VISUALSTYLE_DICTIONARY: u64 = 0x24;
    pub const FIRST_FREE: u64 = 0x25;
}

const CLASS_DICTIONARY_WITH_DEFAULT: u16 = 500;
const CLASS_POLYLINE: u16 = 503;

const OT_TEXT: u16 = 1;
const OT_BLOCK: u16 = 4;
const OT_ENDBLK: u16 = 5;
const OT_DICTIONARY: u16 = 42;
const OT_BLOCK_CONTROL: u16 = 48;
const OT_BLOCK_HEADER: u16 = 49;
const OT_LAYER_CONTROL: u16 = 50;
const OT_LAYER: u16 = 51;
const OT_STYLE_CONTROL: u16 = 52;
const OT_STYLE: u16 = 53;
const OT_LTYPE_CONTROL: u16 = 56;
const OT_LTYPE: u16 = 57;
const OT_VIEW_CONTROL: u16 = 60;
const OT_UCS_CONTROL: u16 = 62;
const OT_VPORT_CONTROL: u16 = 64;
const OT_VPORT: u16 = 65;
const OT_APPID_CONTROL: u16 = 66;
const OT_APPID: u16 = 67;
const OT_DIMSTYLE_CONTROL: u16 = 68;
const OT_DIMSTYLE: u16 = 69;
const OT_MLINESTYLE: u16 = 73;
const OT_PLACEHOLDER: u16 = 80;
const OT_LAYOUT: u16 = 82;

const LAYOUT_PLOT_MODEL: i16 = 0x6B0;
const LAYOUT_PLOT_PAPER: i16 = 0x2B0;
const LINEWEIGHT_BY_LAYER: u8 = 29;
const LINEWEIGHT_DEFAULT: i16 = 31;
const PI_2: f64 = std::f64::consts::FRAC_PI_2;

/// 🌐️ The standard lineweights in 1/100 mm; the position is the DWG lineweight enumeration value.
pub const LINEWEIGHTS: [u32; 24] = [0, 5, 9, 13, 15, 18, 20, 25, 30, 35, 40, 50, 53, 60, 70, 80, 90, 100, 106, 120, 140, 158, 200, 211];

/// 🔢️ The enumeration value of the standard lineweight nearest to `hundredths_mm`.
pub fn lineweight_index(hundredths_mm: u32) -> u8 {
    LINEWEIGHTS.iter().enumerate().min_by_key(|(_, weight)| weight.abs_diff(hundredths_mm)).map_or(31, |(index, _)| index as u8)
}

/// 🗄️ The records of the object section with the facts the other sections need.
pub struct Database {
    pub records: Vec<(u64, Vec<u8>)>,
    pub next_handle: u64,
    pub extents: Option<([f64; 2], [f64; 2])>,
    pub polylines: usize,
}

struct Writer {
    records: Vec<(u64, Vec<u8>)>,
}

impl Writer {
    fn object(&mut self, kind: u16, handle: u64, owner: u64) -> Streams {
        let mut s = Streams::new();
        s.main.object_type(kind);
        s.main.handle(Ref::Absolute, handle);
        s.main.bs(0);
        s.main.bl(0);
        s.main.bit(true);
        s.reference(Ref::SoftPointer, owner);
        s
    }

    fn entity(&mut self, kind: u16, handle: u64, owner: u64, space: u8, layer: u64) -> Streams {
        let mut s = Streams::new();
        s.main.object_type(kind);
        s.main.handle(Ref::Absolute, handle);
        s.main.bs(0);
        s.main.bit(false);
        s.main.bits(u64::from(space), 2);
        s.main.bl(0);
        s.main.bit(true);
        s.main.bs(256);
        s.main.bd(1.0);
        s.main.bits(0, 2);
        s.main.bits(0, 2);
        s.main.rc(0);
        s.main.bits(0, 2);
        s.main.bit(false);
        s.main.bit(false);
        s.main.bit(false);
        s.main.bs(0);
        s.main.rc(LINEWEIGHT_BY_LAYER);
        if space == 0 {
            s.reference(Ref::SoftPointer, owner);
        }
        s.reference(Ref::HardPointer, layer);
        s
    }

    fn finish(&mut self, handle: u64, streams: Streams) {
        let (data, handle_bits) = streams.merge(false);
        self.records.push((handle, frame(&data, handle_bits)));
    }

    fn control(&mut self, kind: u16, handle: u64, entries: &[u64], owned: &[u64], flag: bool) {
        let mut s = self.object(kind, handle, 0);
        s.main.bl(entries.len() as i32);
        if flag {
            s.main.rc(0);
        }
        entries.iter().for_each(|entry| s.reference(Ref::SoftOwner, *entry));
        owned.iter().for_each(|entry| s.reference(Ref::HardOwner, *entry));
        self.finish(handle, s);
    }

    fn entry(&mut self, kind: u16, handle: u64, owner: u64, name: &str) -> Streams {
        let mut s = self.object(kind, handle, owner);
        s.string(name);
        s.main.bs(0);
        s
    }

    fn layer(&mut self, handle: u64, name: &str, color: u16, lineweight: i16) {
        let mut s = self.entry(OT_LAYER, handle, handle::LAYER_CONTROL, name);
        s.main.bs((lineweight << 5) | 0x10);
        s.main.cmc(aci(color));
        s.reference(Ref::HardPointer, 0);
        s.reference(Ref::HardPointer, handle::PLOTSTYLE_NORMAL);
        s.reference(Ref::HardPointer, 0);
        s.reference(Ref::HardPointer, handle::LTYPE_CONTINUOUS);
        self.finish(handle, s);
    }

    fn style(&mut self) {
        let h = handle::STYLE_STANDARD;
        let mut s = self.entry(OT_STYLE, h, handle::STYLE_CONTROL, "Standard");
        s.main.bit(false);
        s.main.bit(false);
        s.main.bd(0.0);
        s.main.bd(1.0);
        s.main.bd(0.0);
        s.main.rc(0);
        s.main.bd(0.2);
        s.string("txt");
        s.string("");
        s.reference(Ref::HardPointer, 0);
        self.finish(h, s);
    }

    fn ltype(&mut self, handle: u64, name: &str, description: &str) {
        let mut s = self.object(OT_LTYPE, handle, handle::LTYPE_CONTROL);
        s.string(name);
        s.main.bs(0);
        s.string(description);
        s.main.bd(0.0);
        s.main.rc(b'A');
        s.main.rc(0);
        s.reference(Ref::HardPointer, 0);
        self.finish(handle, s);
    }

    fn appid(&mut self) {
        let h = handle::APPID_ACAD;
        let mut s = self.entry(OT_APPID, h, handle::APPID_CONTROL, "ACAD");
        s.main.rc(0);
        s.reference(Ref::HardPointer, 0);
        self.finish(h, s);
    }

    fn dimstyle(&mut self) {
        let h = handle::DIMSTYLE_STANDARD;
        let mut s = self.entry(OT_DIMSTYLE, h, handle::DIMSTYLE_CONTROL, "Standard");
        s.string("");
        s.string("");
        [1.0, 0.18, 0.0625, 0.38, 0.18, 0.0, 0.0, 0.0, 0.0, 1.0, std::f64::consts::FRAC_PI_4].into_iter().for_each(|value| s.main.bd(value));
        s.main.bs(0);
        s.main.cmc(BY_BLOCK);
        [false, false, true, true, false, false].into_iter().for_each(|flag| s.main.bit(flag));
        [0, 0, 0, 0].into_iter().for_each(|value| s.main.bs(value));
        [0.18, 0.09, 0.0, 25.4, 1.0, 0.0, 1.0, 0.09, 0.0].into_iter().for_each(|value| s.main.bd(value));
        s.main.bit(false);
        s.main.bs(2);
        [false, false, false, false].into_iter().for_each(|flag| s.main.bit(flag));
        (0..3).for_each(|_| s.main.cmc(BY_BLOCK));
        [0, 4, 4, 2, 2, 0, 0, 2, 46, 0, 0].into_iter().for_each(|value| s.main.bs(value));
        s.main.bit(false);
        s.main.bit(false);
        [1, 0, 0, 0].into_iter().for_each(|value| s.main.bs(value));
        s.main.bit(false);
        s.main.bs(3);
        s.main.bit(false);
        s.main.bit(false);
        s.main.bd(0.0);
        s.string("");
        s.main.bd(0.0);
        s.string("");
        s.main.bs(-2);
        s.main.bs(-2);
        s.main.bit(false);
        s.reference(Ref::HardPointer, 0);
        s.reference(Ref::HardPointer, handle::STYLE_STANDARD);
        (0..7).for_each(|_| s.reference(Ref::HardPointer, 0));
        self.finish(h, s);
    }

    fn vport(&mut self, center: [f64; 2], height: f64) {
        let h = handle::VPORT_ACTIVE;
        let mut s = self.entry(OT_VPORT, h, handle::VPORT_CONTROL, "*Active");
        s.main.bd(height);
        s.main.bd(height);
        s.main.rd2(center[0], center[1]);
        s.main.bd3(0.0, 0.0, 0.0);
        s.main.bd3(0.0, 0.0, 1.0);
        s.main.bd(0.0);
        s.main.bd(50.0);
        s.main.bd(0.0);
        s.main.bd(0.0);
        [false, false, false, false].into_iter().for_each(|flag| s.main.bit(flag));
        s.main.rc(0);
        s.main.bit(true);
        s.main.rc(1);
        s.main.bd(0.0);
        s.main.bd(0.0);
        s.main.cmc(aci(250));
        s.main.rd2(0.0, 0.0);
        s.main.rd2(1.0, 1.0);
        s.main.bit(false);
        s.main.bs(100);
        s.main.bit(true);
        s.main.bit(true);
        s.main.bit(true);
        s.main.bit(false);
        s.main.rd2(10.0, 10.0);
        s.main.bit(false);
        s.main.bit(false);
        s.main.bs(0);
        s.main.bd(0.0);
        s.main.rd2(0.0, 0.0);
        s.main.rd2(0.5, 0.5);
        s.main.bit(false);
        s.main.bit(true);
        s.main.bd3(0.0, 0.0, 0.0);
        s.main.bd3(1.0, 0.0, 0.0);
        s.main.bd3(0.0, 1.0, 0.0);
        s.main.bd(0.0);
        s.main.bs(0);
        s.main.bs(2);
        s.main.bs(5);
        s.reference(Ref::HardPointer, 0);
        s.reference(Ref::SoftPointer, 0);
        s.reference(Ref::HardPointer, 0);
        s.reference(Ref::HardOwner, 0);
        s.reference(Ref::HardPointer, 0);
        s.reference(Ref::HardPointer, 0);
        self.finish(h, s);
    }

    fn block_record(&mut self, record: u64, name: &str, begin: u64, end: u64, layout: u64, owned: &[u64]) {
        let mut s = self.entry(OT_BLOCK_HEADER, record, handle::BLOCK_CONTROL, name);
        s.main.bit(false);
        s.main.bit(false);
        s.main.bit(false);
        s.main.bit(false);
        s.main.bit(false);
        s.main.bl(owned.len() as i32);
        s.main.bd3(0.0, 0.0, 0.0);
        s.string("");
        s.main.rc(0);
        s.string("");
        s.main.bl(0);
        s.main.bs(0);
        s.main.bit(true);
        s.main.rc(0);
        s.reference(Ref::HardPointer, 0);
        s.reference(Ref::HardOwner, begin);
        owned.iter().for_each(|entity| s.reference(Ref::HardOwner, *entity));
        s.reference(Ref::HardOwner, end);
        s.reference(Ref::HardPointer, layout);
        self.finish(record, s);
    }

    fn block_pair(&mut self, record: u64, begin: u64, end: u64, name: &str, space: u8) {
        let mut s = self.entity(OT_BLOCK, begin, record, space, handle::LAYER_0);
        s.string(name);
        self.finish(begin, s);
        let s = self.entity(OT_ENDBLK, end, record, space, handle::LAYER_0);
        self.finish(end, s);
    }

    fn dictionary(&mut self, kind: u16, handle: u64, owner: u64, entries: &[(&str, u64)], default: Option<u64>) {
        let mut s = self.object(kind, handle, owner);
        s.main.bl(entries.len() as i32);
        s.main.bs(1);
        s.main.rc(0);
        for (name, target) in entries {
            s.string(name);
            s.reference(Ref::SoftOwner, *target);
        }
        if let Some(target) = default {
            s.reference(Ref::HardPointer, target);
        }
        self.finish(handle, s);
    }

    fn mlinestyle(&mut self) {
        let h = handle::MLINESTYLE_STANDARD;
        let mut s = self.object(OT_MLINESTYLE, h, handle::MLINESTYLE_DICTIONARY);
        s.string("Standard");
        s.string("");
        s.main.bs(0);
        s.main.cmc(BY_LAYER);
        s.main.bd(PI_2);
        s.main.bd(PI_2);
        s.main.rc(2);
        for offset in [0.5, -0.5] {
            s.main.bd(offset);
            s.main.cmc(BY_LAYER);
            s.main.bs(0x7FFF);
        }
        self.finish(h, s);
    }

    fn placeholder(&mut self) {
        let h = handle::PLOTSTYLE_NORMAL;
        let s = self.object(OT_PLACEHOLDER, h, handle::PLOTSTYLE_DICTIONARY);
        self.finish(h, s);
    }

    fn layout(&mut self, h: u64, name: &str, tab: i16, plot: i16, record: u64) {
        let mut s = self.object(OT_LAYOUT, h, handle::LAYOUT_DICTIONARY);
        s.string("");
        s.string("");
        s.main.bs(plot);
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0].into_iter().for_each(|value| s.main.bd(value));
        s.string("");
        s.main.bd(0.0);
        s.main.bd(0.0);
        s.main.bs(0);
        s.main.bs(0);
        s.main.bs(5);
        [0.0, 0.0, 0.0, 0.0, 1.0, 1.0].into_iter().for_each(|value| s.main.bd(value));
        s.string("");
        s.main.bs(0);
        s.main.bd(1.0);
        s.main.bd(0.0);
        s.main.bd(0.0);
        s.main.bs(0);
        s.main.bs(0);
        s.main.bs(300);
        s.reference(Ref::SoftPointer, 0);
        s.reference(Ref::SoftPointer, 0);
        s.string(name);
        s.main.bs(tab);
        s.main.bs(1);
        s.main.bd3(0.0, 0.0, 0.0);
        s.main.rd2(0.0, 0.0);
        s.main.rd2(12.0, 9.0);
        s.main.bd3(0.0, 0.0, 0.0);
        s.main.bd3(1.0, 0.0, 0.0);
        s.main.bd3(0.0, 1.0, 0.0);
        s.main.bd(0.0);
        s.main.bs(0);
        s.main.bd3(0.0, 0.0, 0.0);
        s.main.bd3(12.0, 9.0, 0.0);
        s.main.bl(0);
        s.reference(Ref::SoftPointer, record);
        s.reference(Ref::SoftPointer, 0);
        s.reference(Ref::HardPointer, 0);
        s.reference(Ref::HardPointer, 0);
        self.finish(h, s);
    }

    fn polyline(&mut self, h: u64, layer: u64, points: &[crate::scene::Point], closed: bool) {
        let mut s = self.entity(CLASS_POLYLINE, h, handle::MODEL_RECORD, 2, layer);
        s.main.bs(if closed { 0x200 } else { 0 });
        s.main.bl(points.len() as i32);
        s.main.rd2(points[0].x, points[0].y);
        for pair in points.windows(2) {
            s.main.dd(pair[1].x, pair[0].x);
            s.main.dd(pair[1].y, pair[0].y);
        }
        self.finish(h, s);
    }

    fn text(&mut self, h: u64, layer: u64, text: &crate::scene::Text) {
        let mut s = self.entity(OT_TEXT, h, handle::MODEL_RECORD, 2, layer);
        s.main.rc(0xFF);
        s.main.rd2(text.at.x, text.at.y);
        s.main.bit(true);
        s.main.bit(true);
        s.main.rd(text.height_m);
        s.string(&text.text);
        s.reference(Ref::HardPointer, handle::STYLE_STANDARD);
        self.finish(h, s);
    }
}

fn extents(scene: &ModelScene) -> Option<([f64; 2], [f64; 2])> {
    let points = scene.layers.iter().flat_map(|layer| layer.polylines.iter().flat_map(|line| line.points.iter().map(|p| [p.x, p.y])).chain(layer.texts.iter().map(|text| [text.at.x, text.at.y])));
    points.fold(None, |found, [x, y]| match found {
        None => Some(([x, y], [x, y])),
        Some((min, max)) => Some(([min[0].min(x), min[1].min(y)], [max[0].max(x), max[1].max(y)])),
    })
}

fn layer_record(layer: &Layer) -> (u16, i16) {
    (u16::from(layer.aci), i16::from(lineweight_index(layer.lineweight)))
}

/// 🏗️ Builds every object record of the drawing for `scene`.
pub fn build(scene: &ModelScene) -> Database {
    let mut w = Writer { records: Vec::new() };
    let found = extents(scene);
    let mut layers: Vec<(u64, &Layer)> = Vec::new();
    let mut zero: Option<&Layer> = None;
    let mut next = handle::FIRST_FREE;
    for layer in &scene.layers {
        if layer.name == "0" {
            zero = Some(layer);
        } else {
            layers.push((next, layer));
            next += 1;
        }
    }
    let layer_handles: Vec<u64> = std::iter::once(handle::LAYER_0).chain(layers.iter().map(|(h, _)| *h)).collect();
    let mut owned: Vec<u64> = Vec::new();
    let mut placed: Vec<(u64, u64, Placed)> = Vec::new();
    for (layer_handle, layer) in zero.map(|layer| (handle::LAYER_0, layer)).into_iter().chain(layers.iter().map(|(h, layer)| (*h, *layer))) {
        for line in &layer.polylines {
            placed.push((next, layer_handle, Placed::Line(&line.points, line.closed)));
            owned.push(next);
            next += 1;
        }
        for text in &layer.texts {
            placed.push((next, layer_handle, Placed::Text(text)));
            owned.push(next);
            next += 1;
        }
    }
    let (center, height) = found.map_or(([0.0, 0.0], 10.0), |(min, max)| ([(min[0] + max[0]) / 2.0, (min[1] + max[1]) / 2.0], ((max[0] - min[0]).max(max[1] - min[1]) * 1.2).max(1.0)));

    w.control(OT_BLOCK_CONTROL, handle::BLOCK_CONTROL, &[], &[handle::MODEL_RECORD, handle::PAPER_RECORD], false);
    w.control(OT_LAYER_CONTROL, handle::LAYER_CONTROL, &layer_handles, &[], false);
    w.control(OT_STYLE_CONTROL, handle::STYLE_CONTROL, &[handle::STYLE_STANDARD], &[], false);
    w.control(OT_LTYPE_CONTROL, handle::LTYPE_CONTROL, &[handle::LTYPE_CONTINUOUS], &[handle::LTYPE_BYBLOCK, handle::LTYPE_BYLAYER], false);
    w.control(OT_VIEW_CONTROL, handle::VIEW_CONTROL, &[], &[], false);
    w.control(OT_UCS_CONTROL, handle::UCS_CONTROL, &[], &[], false);
    w.control(OT_VPORT_CONTROL, handle::VPORT_CONTROL, &[handle::VPORT_ACTIVE], &[], false);
    w.control(OT_APPID_CONTROL, handle::APPID_CONTROL, &[handle::APPID_ACAD], &[], false);
    w.control(OT_DIMSTYLE_CONTROL, handle::DIMSTYLE_CONTROL, &[handle::DIMSTYLE_STANDARD], &[], true);

    let (zero_color, zero_weight) = zero.map_or((7, LINEWEIGHT_DEFAULT), layer_record);
    w.layer(handle::LAYER_0, "0", zero_color, zero_weight);
    for (h, layer) in &layers {
        let (color, weight) = layer_record(layer);
        w.layer(*h, &layer.name, color, weight);
    }
    w.style();
    w.ltype(handle::LTYPE_BYBLOCK, "ByBlock", "");
    w.ltype(handle::LTYPE_BYLAYER, "ByLayer", "");
    w.ltype(handle::LTYPE_CONTINUOUS, "Continuous", "Solid line");
    w.vport(center, height);
    w.appid();
    w.dimstyle();

    w.block_record(handle::MODEL_RECORD, "*Model_Space", handle::MODEL_BLOCK, handle::MODEL_END, handle::LAYOUT_MODEL, &owned);
    w.block_pair(handle::MODEL_RECORD, handle::MODEL_BLOCK, handle::MODEL_END, "*Model_Space", 2);
    for (h, layer, item) in &placed {
        match item {
            Placed::Line(points, closed) => w.polyline(*h, *layer, points, *closed),
            Placed::Text(text) => w.text(*h, *layer, text),
        }
    }
    w.block_record(handle::PAPER_RECORD, "*Paper_Space", handle::PAPER_BLOCK, handle::PAPER_END, handle::LAYOUT_PAPER, &[]);
    w.block_pair(handle::PAPER_RECORD, handle::PAPER_BLOCK, handle::PAPER_END, "*Paper_Space", 1);

    w.dictionary(
        OT_DICTIONARY,
        handle::NAMED_OBJECTS,
        0,
        &[
            ("ACAD_COLOR", handle::COLOR_DICTIONARY),
            ("ACAD_GROUP", handle::GROUP_DICTIONARY),
            ("ACAD_LAYOUT", handle::LAYOUT_DICTIONARY),
            ("ACAD_MATERIAL", handle::MATERIAL_DICTIONARY),
            ("ACAD_MLINESTYLE", handle::MLINESTYLE_DICTIONARY),
            ("ACAD_PLOTSETTINGS", handle::PLOTSETTINGS_DICTIONARY),
            ("ACAD_PLOTSTYLENAME", handle::PLOTSTYLE_DICTIONARY),
            ("ACAD_VISUALSTYLE", handle::VISUALSTYLE_DICTIONARY),
        ],
        None,
    );
    for empty in [handle::GROUP_DICTIONARY, handle::PLOTSETTINGS_DICTIONARY, handle::MATERIAL_DICTIONARY, handle::COLOR_DICTIONARY, handle::VISUALSTYLE_DICTIONARY] {
        w.dictionary(OT_DICTIONARY, empty, handle::NAMED_OBJECTS, &[], None);
    }
    w.dictionary(OT_DICTIONARY, handle::MLINESTYLE_DICTIONARY, handle::NAMED_OBJECTS, &[("Standard", handle::MLINESTYLE_STANDARD)], None);
    w.mlinestyle();
    w.dictionary(OT_DICTIONARY, handle::LAYOUT_DICTIONARY, handle::NAMED_OBJECTS, &[("Layout1", handle::LAYOUT_PAPER), ("Model", handle::LAYOUT_MODEL)], None);
    w.layout(handle::LAYOUT_MODEL, "Model", 0, LAYOUT_PLOT_MODEL, handle::MODEL_RECORD);
    w.layout(handle::LAYOUT_PAPER, "Layout1", 1, LAYOUT_PLOT_PAPER, handle::PAPER_RECORD);
    w.dictionary(CLASS_DICTIONARY_WITH_DEFAULT, handle::PLOTSTYLE_DICTIONARY, handle::NAMED_OBJECTS, &[("Normal", handle::PLOTSTYLE_NORMAL)], Some(handle::PLOTSTYLE_NORMAL));
    w.placeholder();

    let polylines = placed.iter().filter(|(_, _, item)| matches!(item, Placed::Line(..))).count();
    Database { records: w.records, next_handle: next, extents: found, polylines }
}

enum Placed<'a> {
    Line(&'a [crate::scene::Point], bool),
    Text(&'a crate::scene::Text),
}
