//! 🧬️ Precise native BMP samples and owned image metadata.

use crate::STDIO_BMP_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum BmpRowOrder { #[default] BottomUp, TopDown }

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum BmpProfile { IndexedRgb1, IndexedRgb4, IndexedRgb8, DirectRgb16, #[default] DirectRgb24, DirectRgb32, DirectBitfields16, DirectBitfields32 }

impl BmpProfile {
    pub const fn id(self) -> &'static str { match self { Self::IndexedRgb1 => "indexedRgb1", Self::IndexedRgb4 => "indexedRgb4", Self::IndexedRgb8 => "indexedRgb8", Self::DirectRgb16 => "directRgb16", Self::DirectRgb24 => "directRgb24", Self::DirectRgb32 => "directRgb32", Self::DirectBitfields16 => "directBitfields16", Self::DirectBitfields32 => "directBitfields32" } }
    pub const fn bits_per_pixel(self) -> u16 { match self { Self::IndexedRgb1 => 1, Self::IndexedRgb4 => 4, Self::IndexedRgb8 => 8, Self::DirectRgb16 | Self::DirectBitfields16 => 16, Self::DirectRgb24 => 24, Self::DirectRgb32 | Self::DirectBitfields32 => 32 } }
    pub const fn is_indexed(self) -> bool { matches!(self, Self::IndexedRgb1 | Self::IndexedRgb4 | Self::IndexedRgb8) }
    pub const fn is_direct(self) -> bool { !self.is_indexed() }
    pub const fn bitfields(self) -> bool { matches!(self, Self::DirectBitfields16 | Self::DirectBitfields32) }
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct BmpPaletteEntry { pub b: u8, pub g: u8, pub r: u8, pub reserved: u8 }

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[retained_clone(bitwise)]
pub struct BmpNativeSample { pub red: u32, pub green: u32, pub blue: u32, pub alpha: u32, pub reserved: u32 }

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct BmpRegion { pub x: u32, pub y: u32, pub width: u32, pub height: u32 }

/// 🧩 One rectangle of owned pixels, row-major: the sparse row a paint writes and its undo restores. Exactly the list of the image's storage is filled.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct BmpSampleRect {
    pub region: BmpRegion,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub indices: Vec<u8>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub samples: Vec<BmpNativeSample>,
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct BmpColor { pub red: u8, pub green: u8, pub blue: u8, pub alpha: u8 }

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "storage", rename_all = "camelCase")]
pub enum BmpPixels { Indexed { indices: Vec<u8> }, Direct { samples: Vec<BmpNativeSample> } }

impl semio_framework_dsl_record::BorrowedDslField for BmpPixels{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Statements(<Self as semio_framework_dsl_record::BorrowedDslVariants>::VARIANTS);}

impl semio_framework_dsl_record::DslField for BmpPixels{
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{control.step()?;let mut statements=control.allocate_vec(1)?;statements.push(<Self as semio_framework_dsl_record::DslVariants>::to_named_record_controlled(self,control)?);Ok(semio_framework_dsl_record::FieldValue::Statements(statements))}

    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{
        control.step()?;
        match value{semio_framework_dsl_record::FieldValue::Statements(values)if values.len()==1=>{let(keyword,record)=&values[0];<Self as semio_framework_dsl_record::DslVariants>::from_named_record_controlled(keyword,record,control)},_=>Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"BMP pixel storage requires exactly one typed choice"))}
    }
    fn shape()->semio_framework_dsl_record::Shape{semio_framework_dsl_record::Shape::Statements(<Self as semio_framework_dsl_record::DslVariants>::variants())}
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{<Self as semio_framework_dsl_record::DslVariants>::variants_controlled(control).map(semio_framework_dsl_record::Shape::Statements)}
    fn to_value(&self)->semio_framework_dsl_record::FieldValue{semio_framework_dsl_record::FieldValue::Statements(vec![<Self as semio_framework_dsl_record::DslVariants>::to_named_record(self)])}
    fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{match value{semio_framework_dsl_record::FieldValue::Statements(values)if values.len()==1=>{let(keyword,record)=&values[0];<Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword,record).map_err(|error|error.message)},_=>Err("BMP pixel storage requires exactly one typed choice".into())}}
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct BmpImage {
    pub width: u32,
    pub height: u32,
    pub row_order: BmpRowOrder,
    pub profile: BmpProfile,
    pub masks: [u32; 4],
    pub palette: Vec<BmpPaletteEntry>,
    #[dsl(block)]
    pub pixels: BmpPixels,
    pub x_pixels_per_meter: i32,
    pub y_pixels_per_meter: i32,
    pub colors_used: u32,
    pub colors_important: u32,
    pub reserved_1: u16,
    pub reserved_2: u16,
    pub opaque_gap: Vec<u8>,
    pub opaque_trailer: Vec<u8>,
}

impl Default for BmpImage {
    fn default() -> Self { Self { width: 1, height: 1, row_order: BmpRowOrder::BottomUp, profile: BmpProfile::DirectRgb24, masks: [0xff0000, 0xff00, 0xff, 0], palette: Vec::new(), pixels: BmpPixels::Direct { samples: vec![BmpNativeSample { red: 255, green: 255, blue: 255, alpha: 0, reserved: 0 }] }, x_pixels_per_meter: 0, y_pixels_per_meter: 0, colors_used: 0, colors_important: 0, reserved_1: 0, reserved_2: 0, opaque_gap: Vec::new(), opaque_trailer: Vec::new() } }
}

pub fn mask_maximum(mask: u32) -> u32 { if mask == 0 { 0 } else { mask >> mask.trailing_zeros() } }

impl BmpImage {
    /// 🧭 Ordinal of the first pixel of `region`, or `None` when the region is empty or leaves the image.
    pub fn region_start(&self, region: BmpRegion) -> Option<usize> {
        let inside = region.width != 0 && region.height != 0 && region.x.checked_add(region.width).is_some_and(|right| right <= self.width) && region.y.checked_add(region.height).is_some_and(|bottom| bottom <= self.height);
        inside.then(|| region.y as usize * self.width as usize + region.x as usize)
    }

    /// 📋 The pixels `region` currently holds (`None` when the region is outside the image).
    pub fn region_rect(&self, region: BmpRegion) -> Option<BmpSampleRect> {
        let start = self.region_start(region)?;
        let (stride, row) = (self.width as usize, region.width as usize);
        let rows = 0..region.height as usize;
        Some(match &self.pixels {
            BmpPixels::Indexed { indices } => BmpSampleRect { region, indices: rows.flat_map(|line| indices[start + line * stride..start + line * stride + row].iter().copied()).collect(), samples: Vec::new() },
            BmpPixels::Direct { samples } => BmpSampleRect { region, indices: Vec::new(), samples: rows.flat_map(|line| samples[start + line * stride..start + line * stride + row].iter().copied()).collect() },
        })
    }

    /// ✍️ Writes `rect` into the pixel rows it addresses and nothing else.
    pub fn write_rect(&mut self, rect: &BmpSampleRect) -> Result<(), String> {
        let start = self.region_start(rect.region).ok_or("bmp: sample rectangle exceeds the owned image or is empty")?;
        let (stride, row) = (self.width as usize, rect.region.width as usize);
        let count = row * rect.region.height as usize;
        match &mut self.pixels {
            BmpPixels::Indexed { indices } if rect.indices.len() == count && rect.samples.is_empty() => rect.indices.chunks_exact(row).enumerate().for_each(|(line, values)| indices[start + line * stride..start + line * stride + row].copy_from_slice(values)),
            BmpPixels::Direct { samples } if rect.samples.len() == count && rect.indices.is_empty() => rect.samples.chunks_exact(row).enumerate().for_each(|(line, values)| samples[start + line * stride..start + line * stride + row].copy_from_slice(values)),
            _ => return Err("bmp: sample rectangle storage or cardinality differs from its region".into()),
        }
        Ok(())
    }

    /// 🔎 Refuses a rectangle that is out of bounds, mis-sized, of another storage, past the palette or over a native mask precision.
    pub fn validate_rect(&self, rect: &BmpSampleRect) -> Result<(), String> {
        self.validate_header()?;
        self.region_start(rect.region).ok_or("bmp: sample rectangle exceeds the owned image or is empty")?;
        let count = rect.region.width as usize * rect.region.height as usize;
        match &self.pixels {
            BmpPixels::Indexed { .. } if rect.indices.len() == count && rect.samples.is_empty() => {
                if rect.indices.iter().any(|index| usize::from(*index) >= self.palette.len()) {
                    return Err("bmp: sample references an absent palette entry".into());
                }
            }
            BmpPixels::Direct { .. } if rect.samples.len() == count && rect.indices.is_empty() => rect.samples.iter().try_for_each(|sample| self.validate_direct_sample(sample))?,
            _ => return Err("bmp: sample rectangle storage or cardinality differs from its region".into()),
        }
        Ok(())
    }

    pub fn validate_header(&self) -> Result<(), String> {
        if self.width > i32::MAX as u32 || self.height > i32::MAX as u32 || (self.width == 0) != (self.height == 0) { return Err("bmp: invalid owned dimensions".into()); }
        let count = usize::try_from(self.width).ok().and_then(|width| width.checked_mul(self.height as usize)).ok_or("bmp: owned sample count overflow")?;
        if self.profile.is_indexed() {
            let BmpPixels::Indexed { indices } = &self.pixels else { return Err("bmp: indexed profile needs indexed samples".into()); };
            let maximum = 1usize << self.profile.bits_per_pixel();
            if self.masks != [0; 4] || self.palette.is_empty() || self.palette.len() > maximum || indices.len() != count { return Err("bmp: invalid indexed sample or palette ownership".into()); }
            if (if self.colors_used == 0 { maximum } else { self.colors_used as usize }) != self.palette.len() { return Err("bmp: colors-used must match the owned palette".into()); }
        } else {
            let BmpPixels::Direct { samples } = &self.pixels else { return Err("bmp: direct profile needs direct samples".into()); };
            if samples.len() != count { return Err("bmp: direct sample count does not match dimensions".into()); }
            let expected = match self.profile { BmpProfile::DirectRgb16 => Some([0x7c00, 0x3e0, 0x1f, 0]), BmpProfile::DirectRgb24 | BmpProfile::DirectRgb32 => Some([0xff0000, 0xff00, 0xff, 0]), _ => None };
            if expected.is_some_and(|masks| masks != self.masks) { return Err("bmp: RGB profile has fixed native channel masks".into()); }
            if self.masks[3] != 0 || !self.palette.is_empty() { return Err("bmp: v3 direct samples own three color masks and no indexed palette".into()); }
            let depth = self.profile.bits_per_pixel();
            let valid_bits = if depth == 32 { u32::MAX } else { (1u32 << depth) - 1 };
            let mut assigned = 0;
            for (index, mask) in self.masks.iter().copied().enumerate() {
                let maximum = mask_maximum(mask);
                if index < 3 && mask == 0 || mask & !valid_bits != 0 || mask & assigned != 0 || maximum != 0 && maximum & maximum.wrapping_add(1) != 0 { return Err("bmp: native channel masks must be nonoverlapping contiguous lanes".into()); }
                assigned |= mask;
            }

        }
        Ok(())
    }
    /// 🎨 Refuses a direct sample whose component exceeds its native mask precision or whose reserved bits fall outside the unassigned ones.
    pub fn validate_direct_sample(&self,sample:&BmpNativeSample)->Result<(),String> {let assigned=self.masks.into_iter().fold(0,|bits,mask|bits|mask);let depth=self.profile.bits_per_pixel();let valid_bits=if depth==32 {u32::MAX}else{(1u32<<depth)-1};if [sample.red,sample.green,sample.blue,sample.alpha].into_iter().zip(self.masks).any(|(component,mask)|component>mask_maximum(mask))||sample.reserved&assigned!=0||sample.reserved&!valid_bits!=0 {return Err("bmp: owned component exceeds its native mask precision".into());}Ok(())
    }
    pub fn validate_sample(&self,ordinal:usize)->Result<(),String> {
        match &self.pixels {
            BmpPixels::Indexed {indices}=>{if indices.get(ordinal).is_none_or(|index|usize::from(*index)>=self.palette.len()) {return Err("bmp: sample references an absent palette entry".into());}},
            BmpPixels::Direct {samples}=>{let sample=samples.get(ordinal).ok_or("bmp: direct sample is absent")?;self.validate_direct_sample(sample)?;},
        }Ok(())
    }
    pub fn validate(&self)->Result<(),String> {self.validate_header()?;let count=match &self.pixels {BmpPixels::Indexed {indices}=>indices.len(),BmpPixels::Direct {samples}=>samples.len()};for ordinal in 0..count {self.validate_sample(ordinal)?;}Ok(())}
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.stdio.bmp")]
pub struct BmpSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[dsl(block)]
    pub image: BmpImage,
}

impl Default for BmpSnapshot { fn default() -> Self { Self { schema: STDIO_BMP_DOCUMENT_SCHEMA.into(), image: BmpImage::default() } } }

impl BmpSnapshot { pub fn validate(&self) -> Result<(), String> { if self.schema != STDIO_BMP_DOCUMENT_SCHEMA { return Err("bmp: undeclared semantic schema".into()); } self.image.validate() } }

#[cfg(test)]
#[path = "🧪️tests/🧬️owned-native-contract/🦀️.rs"]
mod owned_native_contract_tests;
