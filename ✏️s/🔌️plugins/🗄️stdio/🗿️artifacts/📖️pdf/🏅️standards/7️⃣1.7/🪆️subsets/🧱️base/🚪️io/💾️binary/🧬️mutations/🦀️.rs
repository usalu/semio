//! 💾️ Generic binary framing and direct-owner registry for the visible PDF mutation aggregate.

use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use protocol::OpBinary;

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner identities and binary tags in aggregate declaration order.
pub const BINARY_TAG_REGISTRY: &[(&str, &str, u8)] = &[
    ("InsertPage", "insertPage", insert_page::BINARY_TAG),
    ("RemovePage", "removePage", remove_page::BINARY_TAG),
    ("SetPageMediaBox", "setPageMediaBox", set_page_media_box::BINARY_TAG),
    ("SetPageCropBox", "setPageCropBox", set_page_crop_box::BINARY_TAG),
    ("AppendPageContent", "appendPageContent", append_page_content::BINARY_TAG),
    ("SetInfo", "setInfo", set_info::BINARY_TAG),
    ("InsertObject", "insertObject", insert_object::BINARY_TAG),
    ("RemoveObject", "removeObject", remove_object::BINARY_TAG),
    ("SetObjectValue", "setObjectValue", set_object_value::BINARY_TAG),
    ("SetDictEntry", "setDictEntry", set_dict_entry::BINARY_TAG),
    ("RemoveDictEntry", "removeDictEntry", remove_dict_entry::BINARY_TAG),
    ("SetTrailerEntry", "setTrailerEntry", set_trailer_entry::BINARY_TAG),
    ("RemoveTrailerEntry", "removeTrailerEntry", remove_trailer_entry::BINARY_TAG),
    ("MovePage", "movePage", move_page::BINARY_TAG),
    ("SetPageContent", "setPageContent", set_page_content::BINARY_TAG),
    ("SetPageRotation", "setPageRotation", set_page_rotation::BINARY_TAG),
    ("SetPageBox", "setPageBox", set_page_box::BINARY_TAG),
    ("SetPageUserUnit", "setPageUserUnit", set_page_user_unit::BINARY_TAG),
    ("InsertContent", "insertContent", insert_content::BINARY_TAG),
    ("RemoveContent", "removeContent", remove_content::BINARY_TAG),
    ("ReplaceContent", "replaceContent", replace_content::BINARY_TAG),
    ("InsertAnnotation", "insertAnnotation", insert_annotation::BINARY_TAG),
    ("RemoveAnnotation", "removeAnnotation", remove_annotation::BINARY_TAG),
    ("SetAnnotation", "setAnnotation", set_annotation::BINARY_TAG),
    ("SetFont", "setFont", set_font::BINARY_TAG),
    ("RemoveFont", "removeFont", remove_font::BINARY_TAG),
    ("SetImage", "setImage", set_image::BINARY_TAG),
    ("RemoveImage", "removeImage", remove_image::BINARY_TAG),
    ("SetForm", "setForm", set_form::BINARY_TAG),
    ("RemoveForm", "removeForm", remove_form::BINARY_TAG),
    ("SetExtGState", "setExtGState", set_ext_g_state::BINARY_TAG),
    ("RemoveExtGState", "removeExtGState", remove_ext_g_state::BINARY_TAG),
    ("SetShading", "setShading", set_shading::BINARY_TAG),
    ("RemoveShading", "removeShading", remove_shading::BINARY_TAG),
    ("SetPattern", "setPattern", set_pattern::BINARY_TAG),
    ("RemovePattern", "removePattern", remove_pattern::BINARY_TAG),
    ("SetColorSpace", "setColorSpace", set_color_space::BINARY_TAG),
    ("RemoveColorSpace", "removeColorSpace", remove_color_space::BINARY_TAG),
    ("SetProperties", "setProperties", set_properties::BINARY_TAG),
    ("RemoveProperties", "removeProperties", remove_properties::BINARY_TAG),
    ("SetEmbeddedFile", "setEmbeddedFile", set_embedded_file::BINARY_TAG),
    ("RemoveEmbeddedFile", "removeEmbeddedFile", remove_embedded_file::BINARY_TAG),
    ("SetOutlines", "setOutlines", set_outlines::BINARY_TAG),
    ("SetNamedDestination", "setNamedDestination", set_named_destination::BINARY_TAG),
    ("RemoveNamedDestination", "removeNamedDestination", remove_named_destination::BINARY_TAG),
    ("SetPageLabels", "setPageLabels", set_page_labels::BINARY_TAG),
    ("SetOutputIntents", "setOutputIntents", set_output_intents::BINARY_TAG),
    ("SetAcroForm", "setAcroForm", set_acro_form::BINARY_TAG),
    ("SetOptionalContent", "setOptionalContent", set_optional_content::BINARY_TAG),
    ("SetPageLayout", "setPageLayout", set_page_layout::BINARY_TAG),
    ("SetPageMode", "setPageMode", set_page_mode::BINARY_TAG),
    ("SetViewerPreferences", "setViewerPreferences", set_viewer_preferences::BINARY_TAG),
    ("SetOpenAction", "setOpenAction", set_open_action::BINARY_TAG),
    ("SetLanguage", "setLanguage", set_language::BINARY_TAG),
    ("SetMarkInfo", "setMarkInfo", set_mark_info::BINARY_TAG),
    ("SetMetadata", "setMetadata", set_metadata::BINARY_TAG),
    ("SetDocumentId", "setDocumentId", set_document_id::BINARY_TAG),
    ("SetEncryption", "setEncryption", set_encryption::BINARY_TAG),
    ("SetCatalogEntry", "setCatalogEntry", set_catalog_entry::BINARY_TAG),
    ("RemoveCatalogEntry", "removeCatalogEntry", remove_catalog_entry::BINARY_TAG),
    ("ReplacePage", "replacePage", replace_page::BINARY_TAG),
];
//#endregion 🧾️DerivedRegistry

mod receiving {
    //! 📩️ Receives format-1 archived PDF mutations using their original JSON extent.

    use super::{BINARY_TAG_REGISTRY, MAX_PAYLOAD_BYTES, PdfMutation};
    mod metadata {
        //! 🧾 Places canonical SetImage metadata in its concrete owner, one bounded field at a time.

        use super::{PdfMutation, receiving_invalid};
        use crate::standards::v1_7::subsets::base::schema::{
            mutations::SetImage,
            snapshot::{PdfColorSpace, PdfDictEntry, PdfImage, PdfImageMask, PdfObject},
        };
        use semio_framework_value::{
            DslValue, RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind,
            retained_clone::retained_clone_grant_admits,
        };

        const FIELD_COUNT: usize = 17;
        const FIELDS: [&str; FIELD_COUNT] = [
            "id", "width", "height", "colorSpace", "bitsPerComponent", "imageMask", "decode", "interpolate",
            "body", "softMask", "softMaskInData", "mask", "matte", "intent", "optionalContent", "structParent", "extra",
        ];
        const SMALL_RECORD: usize = 24;
        const SMALL_SEQUENCE: usize = 16;

        #[derive(Clone, Copy, Default)]
        enum Phase { #[default] Root, Image, Scope, Build, Field(usize), Done }

        #[derive(Default)]
        pub(super) struct MetadataReceiving {
            phase: Phase,
            scan: usize,
            image: Option<usize>,
            index: Option<usize>,
            positions: [Option<usize>; FIELD_COUNT],
        }

        pub(super) enum MetadataStep { Pending, Ordinary, Complete }

        pub(super) fn record(value: &DslValue) -> Result<&[(String, DslValue)], ValueError> {
            match value { DslValue::Object(fields) => Ok(fields), _ => Err(receiving_invalid("PDF image metadata expects an object")) }
        }

        fn record_mut(value: &mut DslValue) -> Result<&mut [(String, DslValue)], ValueError> {
            match value { DslValue::Object(fields) => Ok(fields), _ => Err(receiving_invalid("PDF image metadata expects an object")) }
        }

        pub(super) fn named<'a>(value: &'a DslValue, name: &str) -> Option<&'a DslValue> {
            record(value).ok()?.iter().find(|(field, _)| field == name).map(|(_, value)| value)
        }

        pub(super) fn named_mut<'a>(value: &'a mut DslValue, name: &str) -> Option<&'a mut DslValue> {
            record_mut(value).ok()?.iter_mut().find(|(field, _)| field == name).map(|(_, value)| value)
        }

        pub(super) fn array(value: Option<&DslValue>) -> Result<&[DslValue], ValueError> {
            match value { None => Ok(&[]), Some(DslValue::Array(values)) => Ok(values),
                _ => Err(receiving_invalid("PDF image metadata expects an array")) }
        }

        pub(super) fn small_record(value: &DslValue) -> bool {
            record(value).map_or(true, |fields| fields.len() <= SMALL_RECORD)
        }

        pub(super) fn small_array(value: Option<&DslValue>) -> bool {
            array(value).map_or(true, |values| values.len() <= SMALL_SEQUENCE)
        }

        fn scope_color(value: Option<&DslValue>) -> bool {
            let Some(value) = value else { return true; };
            small_record(value) && matches!(named(value, "kind").and_then(DslValue::as_str),
                None | Some("deviceGray" | "deviceRgb" | "deviceCmyk" | "named"))
        }

        fn scope_mask(value: Option<&DslValue>) -> bool {
            let Some(value) = value else { return true; };
            small_record(value) && small_array(named(value, "ranges"))
        }

        pub(super) fn scope_extra(value: Option<&DslValue>) -> bool {
            let Ok(values) = array(value) else { return true; };
            values.len() <= SMALL_SEQUENCE && values.iter().all(|entry| {
                small_record(entry) && named(entry, "value").is_none_or(|value| {
                    small_record(value) && matches!(named(value, "kind").and_then(DslValue::as_str),
                        None | Some("null" | "bool" | "int" | "text" | "name"))
                })
            })
        }

        pub(super) fn unsigned<T: TryFrom<u64>>(value: Option<&DslValue>) -> Result<T, ValueError> {
            value.and_then(DslValue::as_u64).and_then(|value| T::try_from(value).ok())
                .ok_or_else(|| receiving_invalid("PDF image metadata expects an exact unsigned integer"))
        }

        pub(super) fn optional<T>(value: Option<&DslValue>, decode: impl FnOnce(Option<&DslValue>) -> Result<T, ValueError>) -> Result<Option<T>, ValueError> {
            match value { None | Some(DslValue::Null) => Ok(None), value => decode(value).map(Some) }
        }

        pub(super) fn boolean(value: Option<&DslValue>) -> Result<bool, ValueError> {
            match value { Some(DslValue::Bool(value)) => Ok(*value), _ => Err(receiving_invalid("PDF image metadata expects a boolean")) }
        }

        fn float(value: &DslValue) -> Result<f64, ValueError> {
            value.as_f64().ok_or_else(|| receiving_invalid("PDF image metadata expects a number"))
        }

        pub(super) fn string(value: Option<&mut DslValue>) -> Result<String, ValueError> {
            match value { Some(DslValue::String(value)) => Ok(std::mem::take(value)),
                _ => Err(receiving_invalid("PDF image metadata expects text")) }
        }

        pub(super) fn optional_string(value: Option<&mut DslValue>) -> Result<Option<String>, ValueError> {
            match value { None | Some(DslValue::Null) => Ok(None), value => string(value).map(Some) }
        }

        fn color(value: Option<&mut DslValue>) -> Result<Option<PdfColorSpace>, ValueError> {
            let Some(value) = value.filter(|value| !matches!(value, DslValue::Null)) else { return Ok(None); };
            Ok(Some(match named(value, "kind").and_then(DslValue::as_str) {
                Some("deviceGray") => PdfColorSpace::DeviceGray,
                Some("deviceRgb") => PdfColorSpace::DeviceRgb,
                Some("deviceCmyk") => PdfColorSpace::DeviceCmyk,
                Some("named") => PdfColorSpace::Named { name: string(named_mut(value, "name"))? },
                _ => return Err(receiving_invalid("invalid PDF image color-space discriminator")),
            }))
        }

        pub(super) fn reserve<T>(count: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<Vec<T>, ValueError> {
            let mut output = Vec::new();
            if count == 0 { return Ok(output); }
            if effect.actual().is_some() { return Err(ValueError::invariant("PDF metadata allocation post is occupied")); }
            *effect.actual() = Some(0);
            output.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "PDF image metadata reserve failed"))?;
            *effect.actual() = Some(output.capacity() * size_of::<T>());
            Ok(output)
        }

        pub(super) fn sequence<T>(values: &[DslValue], decode: impl Fn(&DslValue) -> Result<T, ValueError>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Vec<T>, ValueError> {
            for value in values { decode(value)?; }
            let mut output = reserve(values.len(), effect)?;
            output.extend(values.iter().map(|value| decode(value).ok().expect("validated PDF image metadata sample")));
            Ok(output)
        }

        fn mask(value: Option<&mut DslValue>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<PdfImageMask>, ValueError> {
            let Some(value) = value.filter(|value| !matches!(value, DslValue::Null)) else { return Ok(None); };
            Ok(Some(match named(value, "kind").and_then(DslValue::as_str) {
                Some("stencil") => PdfImageMask::Stencil { image: string(named_mut(value, "image"))? },
                Some("colorKey") => {
                    let ranges = named(value, "ranges").ok_or_else(|| receiving_invalid("PDF color-key mask requires ranges"))?;
                    PdfImageMask::ColorKey { ranges: sequence(array(Some(ranges))?, |value| unsigned(Some(value)), effect)? }
                }
                _ => return Err(receiving_invalid("invalid PDF image mask discriminator")),
            }))
        }

        fn scalar_cos(value: &DslValue) -> Result<PdfObject, ValueError> {
            let field = named(value, "value");
            match named(value, "kind").and_then(DslValue::as_str) {
                Some("null") => Ok(PdfObject::Null),
                Some("bool") => boolean(field).map(PdfObject::Bool),
                Some("int") => field.and_then(DslValue::as_i64).map(PdfObject::Int)
                    .ok_or_else(|| receiving_invalid("PDF COS integer expects an exact signed integer")),
                Some("text" | "name") if field.and_then(DslValue::as_str).is_some() => Ok(PdfObject::Null),
                _ => Err(receiving_invalid("invalid PDF image extra COS value")),
            }
        }

        pub(super) fn extra(value: Option<&mut DslValue>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Vec<PdfDictEntry>, ValueError> {
            let Some(value) = value else { return Ok(Vec::new()); };
            for entry in array(Some(value))? {
                if named(entry, "key").and_then(DslValue::as_str).is_none() {
                    return Err(receiving_invalid("PDF image extra entry requires a text key"));
                }
                scalar_cos(named(entry, "value").ok_or_else(|| receiving_invalid("PDF image extra entry requires a value"))?)?;
            }
            let DslValue::Array(entries) = value else { unreachable!() };
            let mut output = reserve(entries.len(), effect)?;
            for entry in entries {
                let key = string(named_mut(entry, "key")).expect("validated extra key");
                let raw = named_mut(entry, "value").expect("validated COS value");
                let value = match named(raw, "kind").and_then(DslValue::as_str) {
                    Some("text") => PdfObject::Text(string(named_mut(raw, "value")).expect("validated COS text")),
                    Some("name") => PdfObject::Name(string(named_mut(raw, "value")).expect("validated COS name")),
                    _ => scalar_cos(raw).expect("validated COS scalar"),
                };
                output.push(PdfDictEntry { key, value });
            }
            Ok(output)
        }

        impl MetadataReceiving {
            fn image<'a>(&self, value: &'a DslValue) -> Result<&'a [(String, DslValue)], ValueError> {
                let ordinal = self.image.ok_or_else(|| receiving_invalid("SetImage requires image"))?;
                record(&record(value)?[ordinal].1)
            }

            fn image_mut<'a>(&self, value: &'a mut DslValue) -> Result<&'a mut [(String, DslValue)], ValueError> {
                let ordinal = self.image.ok_or_else(|| receiving_invalid("SetImage requires image"))?;
                record_mut(&mut record_mut(value)?[ordinal].1)
            }

            fn field<'a>(&self, value: &'a DslValue, field: usize) -> Result<Option<&'a DslValue>, ValueError> {
                Ok(self.positions[field].map(|index| &self.image(value).expect("validated image record")[index].1))
            }

            pub(super) fn demand(&self, value: &DslValue) -> Result<(RetirementDemand, u64), ValueError> {
                let base_copy = size_of::<Self>() + size_of::<PdfImage>() + size_of::<Option<PdfMutation>>();
                let (count, element, work) = match self.phase {
                    Phase::Scope => (0, 0, 8 + 4 * SMALL_RECORD as u64 * SMALL_SEQUENCE as u64),
                    Phase::Field(field @ (6 | 11 | 12 | 16)) => {
                        let value = self.field(value, field)?;
                        let count = if field == 11 {
                            value.filter(|value| named(value, "kind").and_then(DslValue::as_str) == Some("colorKey"))
                                .and_then(|value| array(named(value, "ranges")).ok()).map_or(0, <[DslValue]>::len)
                        } else { array(value).map_or(0, <[DslValue]>::len) };
                        let element = match field { 11 => size_of::<u32>(), 16 => size_of::<PdfDictEntry>(), _ => size_of::<f64>() };
                        let work = if field == 16 { 8 + 8 * SMALL_RECORD as u64 * count as u64 }
                            else { 2 + 4 * count as u64 + 2 * SMALL_RECORD as u64 };
                        (count, element, work)
                    }
                    Phase::Field(3 | 8) => (0, 0, 2 + 2 * SMALL_RECORD as u64),
                    Phase::Image => (0, 0, FIELDS.len() as u64 + 1),
                    _ => (0, 0, 1),
                };
                Ok((RetirementDemand { copy_bytes: base_copy + count * element * 2,
                    capacity_bytes: count * element, depth: 4, ..Default::default() }, work))
            }

            pub(super) fn advance(
                &mut self, value: &mut DslValue, output: &mut Option<PdfMutation>, effect: &mut RetainedAllocationEffect<'_>,
            ) -> Result<(MetadataStep, RetainedCloneProgress), ValueError> {
                let (demand, work) = self.demand(value)?;
                if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? < work || !effect.performed(work)? {
                    return Ok((MetadataStep::Pending, Default::default()));
                }
                let action = self.perform(value, output, effect);
                let receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes,
                    retained_capacity_bytes: effect.actual().unwrap_or(0), ..Default::default() };
                action.map(|state| (state, receipt)).map_err(|error| error.with_retained_progress(receipt))
            }

            fn perform(
                &mut self, value: &mut DslValue, output: &mut Option<PdfMutation>, effect: &mut RetainedAllocationEffect<'_>,
            ) -> Result<MetadataStep, ValueError> {
                match self.phase {
                    Phase::Root => {
                        if let Some((key, _)) = record(value)?.get(self.scan) {
                            match key.as_str() { "image" => self.image = Some(self.scan), "index" => self.index = Some(self.scan), _ => {} }
                            self.scan += 1;
                        } else { self.scan = 0; self.phase = Phase::Image; }
                    }
                    Phase::Image => {
                        if let Some((key, _)) = self.image(value)?.get(self.scan) {
                            if let Some(field) = FIELDS.iter().position(|field| field == key) { self.positions[field] = Some(self.scan); }
                            self.scan += 1;
                        } else { self.phase = Phase::Scope; }
                    }
                    Phase::Scope => {
                        if !(scope_color(self.field(value, 3)?) && small_array(self.field(value, 6)?)
                            && self.field(value, 8)?.is_none_or(small_record)
                            && scope_mask(self.field(value, 11)?) && small_array(self.field(value, 12)?)
                            && scope_extra(self.field(value, 16)?))
                        {
                            return Ok(MetadataStep::Ordinary);
                        }
                        self.phase = Phase::Build;
                    }
                    Phase::Build => {
                        if output.is_some() { return Err(ValueError::invariant("PDF metadata target occupied")); }
                        let index = self.index.map(|index| &record(value).expect("validated SetImage record")[index].1);
                        let index = optional(index, unsigned::<usize>)?;
                        let mut image = PdfImage::samples(String::new(), 0, 0, PdfColorSpace::DeviceRgb, 0, Vec::new());
                        image.color_space = None;
                        *output = Some(PdfMutation::SetImage(SetImage { image, index }));
                        self.phase = Phase::Field(0);
                    }
                    Phase::Field(field) => {
                        let Some(PdfMutation::SetImage(target)) = output.as_mut() else {
                            return Err(ValueError::invariant("PDF metadata target disappeared"));
                        };
                        let value = self.positions[field].map(|index| &mut self.image_mut(value).expect("validated image record")[index].1);
                        let shared = value.as_deref();
                        let image = &mut target.image;
                        match field {
                            0 => image.id = string(value)?,
                            1 => image.width = unsigned(shared)?,
                            2 => image.height = unsigned(shared)?,
                            3 => image.color_space = color(value)?,
                            4 => image.bits_per_component = shared.map_or(Ok(0), |value| unsigned(Some(value)))?,
                            5 => image.image_mask = shared.map_or(Ok(false), |value| boolean(Some(value)))?,
                            6 => image.decode = sequence(array(shared)?, float, effect)?,
                            7 => image.interpolate = shared.map_or(Ok(false), |value| boolean(Some(value)))?,
                            8 => {
                                if shared.and_then(|body| named(body, "kind")).and_then(DslValue::as_str) != Some("samples")
                                    || !matches!(shared.and_then(|body| named(body, "values")), Some(DslValue::Array(values)) if values.is_empty())
                                {
                                    return Err(ValueError::invariant("PDF image metadata lost its sample donation marker"));
                                }
                            }
                            9 => image.soft_mask = optional_string(value)?,
                            10 => image.soft_mask_in_data = optional(shared, unsigned::<u32>)?,
                            11 => image.mask = mask(value, effect)?,
                            12 => image.matte = match shared { None | Some(DslValue::Null) => None,
                                _ => Some(sequence(array(shared)?, float, effect)?) },
                            13 => image.intent = optional_string(value)?,
                            14 => image.optional_content = optional_string(value)?,
                            15 => image.struct_parent = optional(shared, unsigned::<u32>)?,
                            16 => image.extra = extra(value, effect)?,
                            _ => unreachable!(),
                        }
                        if field + 1 == FIELD_COUNT { self.phase = Phase::Done; } else { self.phase = Phase::Field(field + 1); }
                    }
                    Phase::Done => return Ok(MetadataStep::Complete),
                }
                Ok(MetadataStep::Pending)
            }
        }
    }
    use metadata::{MetadataReceiving, MetadataStep};
    mod object {
        //! 📦 Receives simple COS insert and remove payloads with owned, paid stream-role rows.

        use super::{PdfMutation, receiving_invalid, metadata::{
            array, boolean, named, named_mut, optional, record, reserve, sequence, small_array, small_record, string, unsigned,
        }};
        use crate::standards::v1_7::subsets::base::schema::{
            diff::{PdfIndexedDiff, PdfRoleItem},
            mutations::{InsertObject, RemoveObject},
            snapshot::{ObjRef, PdfObject},
            stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfStreamRoleValue},
        };
        use semio_framework_value::{
            DslValue, RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError,
            retained_clone::retained_clone_grant_admits,
        };

        const FIELDS: [&str; 4] = ["id", "value", "index", "admittedStreamRoles"];
        const ROW_KEYS: [&str; 3] = ["removed", "modified", "added"];
        const RECORD_SEARCH: u64 = 24;
        const SMALL_ROLE_ROWS: usize = 4;

        #[derive(Clone, Copy, Default)]
        enum Phase { #[default] Find, Scope, Build, Id, Cos, Index, Role, Removed, Modified, Added, Done }

        #[derive(Default)]
        pub(super) struct ObjectReceiving {
            phase: Phase,
            scan: usize,
            positions: [Option<usize>; FIELDS.len()],
        }

        pub(super) enum ObjectStep { Pending, Ordinary, Complete }

        fn required<'a>(value: &'a DslValue, key: &str) -> Result<&'a DslValue, ValueError> {
            named(value, key).ok_or_else(|| receiving_invalid("PDF archive object needs its declared field"))
        }

        fn identity(value: Option<&DslValue>) -> Result<ObjRef, ValueError> {
            let value = value.ok_or_else(|| receiving_invalid("PDF archive object needs an id"))?;
            let num = unsigned(Some(required(value, "num")?))?;
            let generation = unsigned(Some(required(value, "gen")?))?;
            Ok(ObjRef { num, r#gen: generation })
        }

        fn tag(value: &DslValue) -> Option<&str> {
            named(value, "kind").and_then(DslValue::as_str)
        }

        fn simple_cos(value: &DslValue) -> bool {
            small_record(value) && matches!(tag(value), None | Some("null" | "bool" | "int" | "text" | "name" | "ref"))
        }

        fn cos(value: Option<&mut DslValue>) -> Result<PdfObject, ValueError> {
            let value = value.ok_or_else(|| receiving_invalid("insertObject needs a COS value"))?;
            match tag(value) {
                Some("null") => Ok(PdfObject::Null),
                Some("bool") => boolean(named(value, "value")).map(PdfObject::Bool),
                Some("int") => required(value, "value")?.as_i64().map(PdfObject::Int)
                    .ok_or_else(|| receiving_invalid("PDF COS integer needs an exact signed integer")),
                Some("text") => string(named_mut(value, "value")).map(PdfObject::Text),
                Some("name") => string(named_mut(value, "value")).map(PdfObject::Name),
                Some("ref") => identity(Some(value)).map(PdfObject::Ref),
                _ => Err(receiving_invalid("invalid PDF COS discriminator")),
            }
        }

        fn no_children(value: Option<&DslValue>) -> bool {
            array(value).map_or(true, <[DslValue]>::is_empty)
        }

        fn simple_role(item: &DslValue) -> bool {
            small_record(item) && named(item, "value").is_none_or(|role| {
                small_record(role) && no_children(named(role, "dependencies"))
                    && named(role, "identity").is_none_or(|id| {
                        small_record(id) && no_children(named(id, "path"))
                            && named(id, "owner").is_none_or(small_record)
                    })
                    && named(role, "value").is_none_or(|value| {
                        small_record(value) && matches!(tag(value), None | Some("metadataText" | "calculatorProgram"))
                    })
            })
        }

        fn small_role_diff(value: &DslValue) -> bool {
            small_record(value) && small_array(named(value, "removed"))
                && ROW_KEYS[1..].iter().all(|field| {
                    array(named(value, field)).map_or(true, |rows| rows.len() <= SMALL_ROLE_ROWS && rows.iter().all(simple_role))
                })
        }

        #[derive(Clone, Copy)]
        enum RoleText { Metadata, Calculator }

        fn role_shape(item: &DslValue) -> Result<(usize, ObjRef, RoleText), ValueError> {
            let index = unsigned(Some(required(item, "index")?))?;
            let role = required(item, "value")?;
            let graph = required(role, "identity")?;
            let owner = identity(Some(required(graph, "owner")?))?;
            if !array(Some(required(graph, "path")?))?.is_empty()
                || !array(Some(required(role, "dependencies")?))?.is_empty()
            {
                return Err(ValueError::invariant("archive stream role scope changed"));
            }
            let value = required(role, "value")?;
            let (kind, name) = match tag(value) {
                Some("metadataText") => (RoleText::Metadata, "text"),
                Some("calculatorProgram") => (RoleText::Calculator, "code"),
                _ => return Err(receiving_invalid("invalid PDF archive stream role")),
            };
            if required(value, name)?.as_str().is_none() {
                return Err(receiving_invalid("PDF archive stream role needs text"));
            }
            Ok((index, owner, kind))
        }

        fn role_rows(
            raw: Option<&mut DslValue>, effect: &mut RetainedAllocationEffect<'_>,
        ) -> Result<Vec<PdfRoleItem>, ValueError> {
            let Some(raw) = raw else { return Ok(Vec::new()); };
            type Row = (usize, ObjRef, RoleText);
            let mut shape: [Option<Row>; SMALL_ROLE_ROWS] = [None; SMALL_ROLE_ROWS];
            for (index, item) in array(Some(raw))?.iter().enumerate() {
                shape[index] = Some(role_shape(item)?);
            }
            let DslValue::Array(items) = raw else { unreachable!() };
            let mut output = reserve(items.len(), effect)?;
            for (item, data) in items.iter_mut().zip(shape) {
                let (index, owner, kind) = data.expect("validated role row");
                let role = named_mut(item, "value").expect("validated role entry");
                let value = named_mut(role, "value").expect("validated role body");
                let value = match kind {
                    RoleText::Metadata => PdfStreamRoleValue::MetadataText {
                        text: string(named_mut(value, "text")).expect("validated metadata text"),
                    },
                    RoleText::Calculator => PdfStreamRoleValue::CalculatorProgram {
                        code: string(named_mut(value, "code")).expect("validated calculator text"),
                    },
                };
                output.push(PdfRoleItem { index, value: PdfAdmittedStreamRole {
                    identity: PdfGraphIdentity { owner, path: Vec::new() }, dependencies: Vec::new(), value,
                } });
            }
            Ok(output)
        }

        fn roles_target(target: &mut PdfMutation) -> &mut Option<crate::standards::v1_7::subsets::base::schema::diff::PdfRoleDiff> {
            match target {
                PdfMutation::InsertObject(operation) => &mut operation.admitted_stream_roles,
                PdfMutation::RemoveObject(operation) => &mut operation.admitted_stream_roles,
                _ => unreachable!("selected PDF object mutation"),
            }
        }

        impl ObjectReceiving {
            fn field<'a>(&self, value: &'a DslValue, field: usize) -> Result<Option<&'a DslValue>, ValueError> {
                let fields = record(value)?;
                Ok(self.positions[field].map(|index| &fields[index].1))
            }

            fn row_fields(&self) -> Option<(usize, usize)> {
                match self.phase {
                    Phase::Removed => Some((0, size_of::<usize>())),
                    Phase::Modified => Some((1, size_of::<PdfRoleItem>())),
                    Phase::Added => Some((2, size_of::<PdfRoleItem>())),
                    _ => None,
                }
            }

            pub(super) fn demand(&self, value: &DslValue) -> Result<(RetirementDemand, u64), ValueError> {
                let mut demand = RetirementDemand { depth: 4, copy_bytes: size_of::<Self>() + 2 * size_of::<Option<PdfMutation>>(), ..Default::default() };
                let work = if let Some((field, element)) = self.row_fields() {
                    let count = self.field(value, 3)?.and_then(|roles| array(named(roles, ROW_KEYS[field])).ok()).map_or(0, <[DslValue]>::len);
                    demand.capacity_bytes = count * element;
                    demand.copy_bytes += 2 * demand.capacity_bytes;
                    if field == 0 { 2 * RECORD_SEARCH + 4 * count as u64 + 1 }
                    else { 2 * RECORD_SEARCH + 18 * RECORD_SEARCH * count as u64 + 1 }
                } else { match self.phase {
                    Phase::Find => FIELDS.len() as u64 + 1,
                    Phase::Scope => 24 * RECORD_SEARCH * SMALL_ROLE_ROWS as u64 + 1,
                    Phase::Id | Phase::Cos => 4 * RECORD_SEARCH + 1,
                    _ => 1,
                }};
                Ok((demand, work))
            }

            pub(super) fn advance(
                &mut self, insert: bool, value: &mut DslValue, target: &mut Option<PdfMutation>, effect: &mut RetainedAllocationEffect<'_>,
            ) -> Result<(ObjectStep, RetainedCloneProgress), ValueError> {
                let (demand, work) = self.demand(value)?;
                if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? < work || !effect.performed(work)? {
                    return Ok((ObjectStep::Pending, Default::default()));
                }
                let action = self.perform(insert, value, target, effect);
                let receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes,
                    retained_capacity_bytes: effect.actual().unwrap_or(0), ..Default::default() };
                action.map(|step| (step, receipt)).map_err(|error| error.with_retained_progress(receipt))
            }

            fn perform(
                &mut self, insert: bool, value: &mut DslValue, target: &mut Option<PdfMutation>, effect: &mut RetainedAllocationEffect<'_>,
            ) -> Result<ObjectStep, ValueError> {
                if let Some((field, _)) = self.row_fields() {
                    let roles = roles_target(target.as_mut().expect("retained typed object")).as_mut().expect("retained admitted roles");
                    let DslValue::Object(fields) = value else { unreachable!() };
                    let raw = &mut fields[self.positions[3].expect("admitted role field")].1;
                    let raw = named_mut(raw, ROW_KEYS[field]);
                    self.phase = match field {
                        0 => {
                            roles.removed = sequence(array(raw.as_deref())?, |value| unsigned(Some(value)), effect)?;
                            Phase::Modified
                        }
                        1 => { roles.modified = role_rows(raw, effect)?; Phase::Added }
                        _ => { roles.added = role_rows(raw, effect)?; Phase::Done }
                    };
                    return Ok(ObjectStep::Pending);
                }
                self.phase = match self.phase {
                    Phase::Find => {
                        if let Some((key, _)) = record(value)?.get(self.scan) {
                            if let Some(field) = FIELDS.iter().position(|field| field == key) {
                                self.positions[field] = Some(self.scan);
                            }
                            self.scan += 1;
                            Phase::Find
                        } else { Phase::Scope }
                    }
                    Phase::Scope => {
                        if self.field(value, 0)?.is_some_and(|id| !small_record(id))
                            || insert && self.field(value, 1)?.is_some_and(|cos| !simple_cos(cos))
                            || self.field(value, 3)?.is_some_and(|roles| !small_role_diff(roles))
                        { return Ok(ObjectStep::Ordinary); }
                        Phase::Build
                    }
                    Phase::Build => {
                        if target.is_some() { return Err(ValueError::invariant("PDF archive object target occupied")); }
                        *target = Some(if insert { PdfMutation::InsertObject(InsertObject {
                            id: ObjRef::default(), value: PdfObject::Null, index: None, admitted_stream_roles: None,
                        }) } else { PdfMutation::RemoveObject(RemoveObject { id: ObjRef::default(), admitted_stream_roles: None }) });
                        Phase::Id
                    }
                    Phase::Id => {
                        let id = identity(self.field(value, 0)?)?;
                        match target.as_mut().expect("retained typed object") {
                            PdfMutation::InsertObject(operation) => { operation.id = id; Phase::Cos }
                            PdfMutation::RemoveObject(operation) => { operation.id = id; Phase::Role }
                            _ => unreachable!("selected PDF object mutation"),
                        }
                    }
                    Phase::Cos => {
                        let Some(PdfMutation::InsertObject(operation)) = target.as_mut() else { unreachable!() };
                        let DslValue::Object(fields) = value else { unreachable!() };
                        operation.value = cos(self.positions[1].map(|index| &mut fields[index].1))?;
                        Phase::Index
                    }
                    Phase::Index => {
                        let Some(PdfMutation::InsertObject(operation)) = target.as_mut() else { unreachable!() };
                        operation.index = optional(self.field(value, 2)?, unsigned::<usize>)?;
                        Phase::Role
                    }
                    Phase::Role => match self.field(value, 3)? {
                        None | Some(DslValue::Null) => Phase::Done,
                        Some(roles) => {
                            record(roles)?;
                            *roles_target(target.as_mut().expect("retained typed object")) = Some(Default::default());
                            Phase::Removed
                        }
                    },
                    Phase::Done => return Ok(ObjectStep::Complete),
                    _ => unreachable!(),
                };
                Ok(ObjectStep::Pending)
            }
        }
    }
    use object::{ObjectReceiving, ObjectStep};
    mod leaves {
        //! 🌿 Receives the ordinary page rotation, document information and language mutations.

        use super::{PdfMutation, receiving_invalid, metadata::{
            array, extra, named, optional, optional_string, record, scope_extra, small_record, unsigned,
        }};
        use crate::standards::v1_7::subsets::base::schema::{
            mutations::{SetInfo, SetLanguage, SetPageRotation},
            snapshot::{PdfDate, PdfDictEntry, PdfInfo},
        };
        use semio_framework_value::{
            DslValue, RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError,
            retained_clone::retained_clone_grant_admits,
        };

        const ROOT: [&str; 4] = ["index", "rotation", "language", "info"];
        const INFO: [&str; 10] = [
            "title", "author", "subject", "keywords", "creator", "producer",
            "creationDate", "modificationDate", "trapped", "extra",
        ];

        #[derive(Clone, Copy)]
        pub(super) enum Leaf { Rotation, Info, Language }

        impl Leaf {
            pub(super) fn for_kind(kind: Option<&str>) -> Option<Self> {
                match kind {
                    Some("setPageRotation") => Some(Self::Rotation),
                    Some("setInfo") => Some(Self::Info),
                    Some("setLanguage") => Some(Self::Language),
                    _ => None,
                }
            }
        }

        #[derive(Clone, Copy, Default)]
        enum Phase { #[default] Find, Info, Scope, Build, Field(usize), Done }

        #[derive(Default)]
        pub(super) struct LeafReceiving {
            phase: Phase,
            scan: usize,
            root: [Option<usize>; ROOT.len()],
            info: [Option<usize>; INFO.len()],
        }

        pub(super) enum LeafStep { Pending, Ordinary, Complete }

        fn signed(value: Option<&DslValue>) -> Result<i32, ValueError> {
            value.and_then(DslValue::as_i64).and_then(|number| i32::try_from(number).ok())
                .ok_or_else(|| receiving_invalid("PDF date needs an exact signed integer"))
        }

        fn date(value: Option<&DslValue>) -> Result<Option<PdfDate>, ValueError> {
            let Some(value) = value.filter(|node| !matches!(node, DslValue::Null)) else { return Ok(None); };
            record(value)?;
            let part = |name, default| named(value, name).map_or(Ok(default), |node| unsigned(Some(node)));
            Ok(Some(PdfDate {
                year: signed(named(value, "year"))?,
                month: part("month", 1)?,
                day: part("day", 1)?,
                hour: part("hour", 0)?,
                minute: part("minute", 0)?,
                second: part("second", 0)?,
                offset_minutes: optional(named(value, "offsetMinutes"), signed)?,
            }))
        }

        fn selected<'a>(value: &'a DslValue, position: Option<usize>) -> Option<&'a DslValue> {
            position.and_then(|pos| value.as_object()?.get(pos).map(|(_, value)| value))
        }

        fn selected_mut(value: &mut DslValue, position: Option<usize>) -> Option<&mut DslValue> {
            let DslValue::Object(fields) = value else { return None; };
            position.and_then(|pos| fields.get_mut(pos).map(|(_, value)| value))
        }

        impl LeafReceiving {
            fn info<'a>(&self, root: &'a DslValue) -> Result<&'a [(String, DslValue)], ValueError> {
                let value = selected(root, self.root[3]).ok_or_else(|| receiving_invalid("setInfo needs an info record"))?;
                record(value)
            }

            pub(super) fn demand(&self, leaf: Leaf, root: &DslValue) -> (RetirementDemand, u64) {
                let (count, work) = match self.phase {
                    Phase::Find => (0, ROOT.len() as u64 + 1),
                    Phase::Info => (0, INFO.len() as u64 + 1),
                    Phase::Scope => (0, 8 + 4 * 24 * 16),
                    Phase::Field(6 | 7) if matches!(leaf, Leaf::Info) => (0, 8 * 24 + 1),
                    Phase::Field(9) if matches!(leaf, Leaf::Info) => {
                        let count = selected(root, self.root[3]).and_then(|value| array(selected(value, self.info[9])).ok())
                            .map_or(0, <[DslValue]>::len);
                        (count, 8 + 8 * 24 * count as u64)
                    }
                    _ => (0, 2),
                };
                let bytes = count * size_of::<PdfDictEntry>();
                (RetirementDemand { depth: 4, copy_bytes: size_of::<Self>() + 2 * size_of::<Option<PdfMutation>>() + 2 * bytes,
                    capacity_bytes: bytes, ..Default::default() }, work)
            }

            pub(super) fn advance(
                &mut self, leaf: Leaf, value: &mut DslValue, output: &mut Option<PdfMutation>,
                effect: &mut RetainedAllocationEffect<'_>,
            ) -> Result<(LeafStep, RetainedCloneProgress), ValueError> {
                let (demand, work) = self.demand(leaf, value);
                if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? < work || !effect.performed(work)? {
                    return Ok((LeafStep::Pending, Default::default()));
                }
                let action = self.perform(leaf, value, output, effect);
                let receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes,
                    retained_capacity_bytes: effect.actual().unwrap_or(0), ..Default::default() };
                action.map(|step| (step, receipt)).map_err(|error| error.with_retained_progress(receipt))
            }

            fn perform(
                &mut self, leaf: Leaf, value: &mut DslValue, output: &mut Option<PdfMutation>,
                effect: &mut RetainedAllocationEffect<'_>,
            ) -> Result<LeafStep, ValueError> {
                self.phase = match self.phase {
                    Phase::Find => {
                        if let Some((name, _)) = record(value)?.get(self.scan) {
                            if let Some(index) = ROOT.iter().position(|key| key == name) { self.root[index] = Some(self.scan); }
                            self.scan += 1;
                            Phase::Find
                        } else if matches!(leaf, Leaf::Info) {
                            self.scan = 0;
                            Phase::Info
                        } else { Phase::Build }
                    }
                    Phase::Info => {
                        if let Some((name, _)) = self.info(value)?.get(self.scan) {
                            if let Some(index) = INFO.iter().position(|key| key == name) { self.info[index] = Some(self.scan); }
                            self.scan += 1;
                            Phase::Info
                        } else { Phase::Scope }
                    }
                    Phase::Scope => {
                        let info = selected(value, self.root[3]).expect("validated info field");
                        if !selected(info, self.info[6]).is_none_or(small_record)
                            || !selected(info, self.info[7]).is_none_or(small_record)
                            || !scope_extra(selected(info, self.info[9]))
                        { return Ok(LeafStep::Ordinary); }
                        Phase::Build
                    }
                    Phase::Build => {
                        if output.is_some() { return Err(ValueError::invariant("ordinary PDF mutation target is occupied")); }
                        *output = Some(match leaf {
                            Leaf::Rotation => PdfMutation::SetPageRotation(SetPageRotation { index: 0, rotation: 0 }),
                            Leaf::Language => PdfMutation::SetLanguage(SetLanguage { language: None }),
                            Leaf::Info => PdfMutation::SetInfo(SetInfo { info: PdfInfo::default() }),
                        });
                        Phase::Field(0)
                    }
                    Phase::Field(field) => match output.as_mut().expect("admitted ordinary PDF mutation") {
                        PdfMutation::SetPageRotation(target) => {
                            if field == 0 {
                                target.index = unsigned(selected(value, self.root[0]))?;
                                Phase::Field(1)
                            } else {
                                target.rotation = unsigned(selected(value, self.root[1]))?;
                                Phase::Done
                            }
                        }
                        PdfMutation::SetLanguage(target) => {
                            target.language = optional_string(selected_mut(value, self.root[2]))?;
                            Phase::Done
                        }
                        PdfMutation::SetInfo(target) => {
                            let raw = selected_mut(value, self.root[3]).expect("admitted info metadata");
                            let field_value = selected_mut(raw, self.info[field]);
                            let info = &mut target.info;
                            match field {
                                0 => info.title = optional_string(field_value)?,
                                1 => info.author = optional_string(field_value)?,
                                2 => info.subject = optional_string(field_value)?,
                                3 => info.keywords = optional_string(field_value)?,
                                4 => info.creator = optional_string(field_value)?,
                                5 => info.producer = optional_string(field_value)?,
                                6 => info.creation_date = date(field_value.as_deref())?,
                                7 => info.modification_date = date(field_value.as_deref())?,
                                8 => info.trapped = optional_string(field_value)?,
                                9 => info.extra = extra(field_value, effect)?,
                                _ => unreachable!(),
                            }
                            if field + 1 == INFO.len() { Phase::Done } else { Phase::Field(field + 1) }
                        }
                        _ => unreachable!("ordinary PDF mutation family changed"),
                    },
                    Phase::Done => return Ok(LeafStep::Complete),
                };
                Ok(LeafStep::Pending)
            }
        }
    }
    use leaves::{Leaf, LeafReceiving, LeafStep};
    use pack::format::{RetainedVarintCursor, RetainedVarintStep};
    use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor, OriginalReceivingPrimary};
    use semio_framework_dsl_record::receiving::OwnedSlot;
    use semio_framework_pack_json::{JsonError, JsonGrammarCursor, JsonMemberPolicy, JsonU32ArrayCursor};
    use semio_framework_value::{
        DslValue, FromValue, NativeDecodeControl, OriginalRetirementPoll,
        RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep,
        RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError, ValueRefusalKind,
        retained_clone::{NativeAllocationIdentity, retained_clone_grant_admits},
        retirement::{RetireOwned, RetirementCursor, RetirementStep, erased_cursor_turn},
    };
    use std::mem::{ManuallyDrop, size_of, size_of_val};

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum FramePhase { Version, Tag, Length(RetainedVarintCursor), Payload }

    struct FramedMutation {
        position: usize,
        end: usize,
        phase: FramePhase,
        kind: Option<&'static str>,
    }

    fn receiving_invalid(message: &'static str) -> ValueError {
        ValueError::literal(ValueRefusalKind::InvalidValue, message)
    }

    impl FramedMutation {
        fn new(range: (usize, usize)) -> Self {
            Self { position: range.0, end: range.1, phase: FramePhase::Version, kind: None }
        }

        fn advance(&mut self, source: &[u8]) -> Result<(), ValueError> {
            let byte = *source.get(self.position).filter(|_| self.position < self.end)
                .ok_or_else(|| receiving_invalid("PDF mutation frame is truncated"))?;
            match self.phase {
                FramePhase::Version => {
                    if byte != store::pack_rt::OP_BINARY_FORMAT { return Err(receiving_invalid("PDF mutation frame version is invalid")); }
                    self.position += 1;
                    self.phase = FramePhase::Tag;
                }
                FramePhase::Tag => {
                    self.kind = BINARY_TAG_REGISTRY.iter().find(|(_, _, tag)| *tag == byte).map(|(_, kind, _)| *kind);
                    if self.kind.is_none() { return Err(receiving_invalid("PDF mutation frame tag is not declared")); }
                    self.position += 1;
                    self.phase = FramePhase::Length(RetainedVarintCursor::default());
                }
                FramePhase::Length(mut cursor) => {
                    let step = cursor.admit(byte, self.position as u64)
                        .map_err(|_| receiving_invalid("PDF mutation frame length varint is invalid"))?;
                    self.position += 1;
                    match step {
                        RetainedVarintStep::Pending => self.phase = FramePhase::Length(cursor),
                        RetainedVarintStep::Complete(length) => {
                            if length > MAX_PAYLOAD_BYTES as u64 {
                                return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "PDF mutation frame length exceeds its declared bound"));
                            }
                            if length != (self.end - self.position) as u64 {
                                return Err(receiving_invalid("PDF mutation payload extent differs from its frame"));
                            }
                            self.phase = FramePhase::Payload;
                        }
                    }
                }
                FramePhase::Payload => return Err(ValueError::invariant("PDF mutation frame already owns a payload range")),
            }
            Ok(())
        }

        fn payload<'a>(&self, source: &'a [u8]) -> &'a [u8] { &source[self.position..self.end] }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum ReceivingPhase { Frame, StartJson, Json, Samples, SamplesMarker, SamplesTake, Identity(usize), Bind, BindMetadata, BindObject, BindLeaf, Ready, Closing, Closed }

    /// 🧾️ Receives the decoded aggregate over its original version, tag, length and JSON bytes.
    pub struct PdfMutationReceiving {
        source: (usize, usize),
        frame: FramedMutation,
        phase: ReceivingPhase,
        authority: Option<(NativeAllocationIdentity, Option<u64>)>,
        parser: OwnedSlot<JsonGrammarCursor<DslValue>>,
        json_error: OwnedSlot<JsonError>,
        value: OwnedSlot<DslValue>,
        output: OwnedSlot<PdfMutation>,
        retained_json: usize,
        sample_cursor: Option<JsonU32ArrayCursor>,
        samples: OwnedSlot<Vec<u32>>,
        metadata: MetadataReceiving,
        object: ObjectReceiving,
        leaf: LeafReceiving,
        failed: bool,
    }

    fn receiving_metadata(copy_bytes: usize) -> RetirementDemand {
        RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
    }

    fn receiving_progress(demand: RetirementDemand) -> RetainedCloneProgress {
        RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }
    }

    fn receiving_work(effect: &mut RetainedAllocationEffect<'_>) -> Result<(), ValueError> {
        if effect.performed(1)? { Ok(()) }
        else { Err(ValueError::literal(ValueRefusalKind::WorkLimit, "PDF mutation receiving lacks one funded work unit")) }
    }

    fn receiving_child(grant: RetainedCloneGrant, copy: usize) -> RetainedCloneGrant {
        RetainedCloneGrant {
            maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(copy),
            maximum_depth: grant.maximum_depth.saturating_sub(1),
            ..grant
        }
    }

    const JSON_HANDOFF: usize = size_of::<ReceivingPhase>() + size_of::<usize>()
        + 2 * size_of::<Option<DslValue>>() + size_of::<Option<JsonError>>();
    const OUTPUT_HANDOFF: usize = 2 * size_of::<Option<PdfMutation>>();

    impl OpBinaryReceiving for PdfMutation {
        type DomainFailure = ValueError;
        type Receiving = PdfMutationReceiving;

        fn admit_receiving(source: &[u8], range: (usize, usize), grant: RetainedCloneGrant)
            -> RetainedWorkOutcome<Option<(Self::Receiving, RetainedCloneProgress)>> {
            if range.0 > range.1 || range.1 > source.len() {
                return RetainedWorkOutcome { result: Err(receiving_invalid("PDF mutation escaped its original source")), work: 0 };
            }
            let demand = receiving_metadata(0);
            if !retained_clone_grant_admits(grant, demand) {
                return RetainedWorkOutcome { result: Ok(None), work: 0 };
            }
            let receiver = PdfMutationReceiving {
                source: (source.as_ptr() as usize, source.len()), frame: FramedMutation::new(range),
                phase: ReceivingPhase::Frame, authority: None, parser: Default::default(), json_error: Default::default(),
                value: Default::default(), output: Default::default(), retained_json: 0, failed: false,
                sample_cursor: None, samples: Default::default(), metadata: Default::default(), object: Default::default(), leaf: Default::default(),
            };
            RetainedWorkOutcome { result: Ok(Some((receiver, receiving_progress(demand)))), work: 1 }
        }
    }

    impl PdfMutationReceiving {
        fn validate_source(&self, source: &[u8]) -> Result<(), ValueError> {
            if self.source == (source.as_ptr() as usize, source.len()) { Ok(()) }
            else { Err(receiving_invalid("PDF mutation original source identity changed")) }
        }

        fn advance_json(&mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, effect: &mut RetainedAllocationEffect<'_>)
            -> Result<RetainedCloneProgress, OriginalReceivingPrimary> {
            let parser = self.parser.value.as_mut().expect("admitted PDF mutation JSON parser");
            let child = receiving_child(effect.grant(), JSON_HANDOFF);
            let step = parser.step_source(self.frame.payload(source), 1, native, &mut effect.nested(child), false);
            let mut progress = parser.normal_step_progress();
            if effect.performed_work() != 0 {
                progress.copied_bytes += JSON_HANDOFF;
                self.retained_json = self.retained_json.checked_add(progress.retained_capacity_bytes)
                    .and_then(|bytes| bytes.checked_sub(progress.released_bytes))
                    .ok_or_else(|| ValueError::invariant("PDF mutation JSON backing receipt has no original owner").with_retained_progress(progress))?;
            }
            match step {
                Ok(Some(value)) => {
                    *self.value.value = Some(value);
                    self.phase = ReceivingPhase::Identity(0);
                }
                Ok(None) => {}
                Err(JsonError::Native(error)) => return Err(error.with_retained_progress(progress).into()),
                Err(error) => {
                    let kind = error.kind();
                    *self.json_error.value = Some(error);
                    return Err(ValueError::literal(kind, "PDF mutation JSON payload was refused").with_retained_progress(progress).into());
                }
            }
            Ok(progress)
        }

        fn check_identity(&mut self, index: usize) -> Result<(), ValueError> {
            let Some(DslValue::Object(entries)) = self.value.value.as_ref() else {
                return Err(receiving_invalid("PDF mutation payload is not an object"));
            };
            let (key, value) = entries.get(index).ok_or_else(|| receiving_invalid("PDF mutation payload has no mutation identity"))?;
            if key == "mutation" {
                if value.as_str() != self.frame.kind { return Err(receiving_invalid("PDF mutation tag differs from its JSON identity")); }
                self.phase = if self.samples.value.is_some() {
                    ReceivingPhase::BindMetadata
                } else if matches!(self.frame.kind, Some("insertObject" | "removeObject")) {
                    ReceivingPhase::BindObject
                } else if Leaf::for_kind(self.frame.kind).is_some() {
                    ReceivingPhase::BindLeaf
                } else {
                    ReceivingPhase::Bind
                };
            } else {
                self.phase = ReceivingPhase::Identity(index + 1);
            }
            Ok(())
        }

        fn turn(&mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, body: usize, effect: &mut RetainedAllocationEffect<'_>)
            -> Result<RetainedCloneProgress, OriginalReceivingPrimary> {
            if self.failed || matches!(self.phase, ReceivingPhase::Ready | ReceivingPhase::Closing | ReceivingPhase::Closed) { return Ok(Default::default()); }
            let demand = self.demands(source, body)?;
            if effect.maximum_work_units() == 0 || demand.copy_bytes > body || !retained_clone_grant_admits(effect.grant(), demand) { return Ok(Default::default()); }
            native.checkpoint()?;
            let authority = (native.ledger_identity(), native.retirement_recipient_identity());
            if self.authority.is_some_and(|bound| bound != authority) {
                return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "PDF mutation original receiving authority changed").into());
            }
            if matches!(self.phase, ReceivingPhase::Samples | ReceivingPhase::SamplesMarker | ReceivingPhase::SamplesTake) {
                return self.advance_samples(source, effect).map_err(Into::into);
            }
            if self.phase == ReceivingPhase::BindLeaf {
                let (step, receipt) = self.leaf.advance(
                    Leaf::for_kind(self.frame.kind).expect("admitted PDF archive leaf"),
                    self.value.value.as_mut().expect("retained PDF archive leaf"),
                    &mut self.output.value, effect)?;
                match step {
                    LeafStep::Pending => {},
                    LeafStep::Ordinary => self.phase = ReceivingPhase::Bind,
                    LeafStep::Complete => self.phase = ReceivingPhase::Ready,
                }
                return Ok(receipt);
            }
            if self.phase == ReceivingPhase::BindObject {
                let (step, receipt) = self.object.advance(
                    self.frame.kind == Some("insertObject"),
                    self.value.value.as_mut().expect("retained PDF archive object"),
                    &mut self.output.value, effect)?;
                match step {
                    ObjectStep::Pending => {},
                    ObjectStep::Ordinary => self.phase = ReceivingPhase::Bind,
                    ObjectStep::Complete => self.phase = ReceivingPhase::Ready,
                }
                return Ok(receipt);
            }
            if self.phase == ReceivingPhase::BindMetadata {
                let (step, receipt) = self.metadata.advance(
                    self.value.value.as_mut().expect("retained PDF image metadata"),
                    &mut self.output.value, effect)?;
                match step {
                    MetadataStep::Pending => {},
                    MetadataStep::Ordinary => self.phase = ReceivingPhase::Bind,
                    MetadataStep::Complete => {
                        donate_samples(self.output.value.as_mut().expect("typed PDF image"), &mut self.samples.value)
                            .map_err(|error| error.with_retained_progress(receipt))?;
                        self.phase = ReceivingPhase::Ready;
                    }
                }
                return Ok(receipt);
            }
            let samples_at = if self.phase == ReceivingPhase::Json {
                let start = self.samples_start(source)?;
                if start.is_none() { return self.advance_json(source, native, effect); }
                start
            } else { None };
            receiving_work(effect)?;
            if self.authority.is_none() { self.authority = Some(authority); }
            let progress = receiving_progress(demand);
            let result = match self.phase {
                ReceivingPhase::Frame => self.frame.advance(source).map(|()| {
                    if self.frame.phase == FramePhase::Payload { self.phase = ReceivingPhase::StartJson; }
                }),
                ReceivingPhase::StartJson => {
                    *self.parser.value = Some(JsonGrammarCursor::new(JsonMemberPolicy::Reject));
                    self.phase = ReceivingPhase::Json;
                    Ok(())
                }
                ReceivingPhase::Json => {
                    self.sample_cursor = JsonU32ArrayCursor::admit(self.frame.payload(source),
                        samples_at.expect("selected PDF Samples start"), SAMPLE_BACKING_BYTES, SAMPLE_BACKING_BYTES, effect.grant())?;
                    self.phase = ReceivingPhase::Samples;
                    Ok(())
                }
                ReceivingPhase::Identity(index) => self.check_identity(index),
                ReceivingPhase::Bind => self.bind_mutation(),
                _ => unreachable!(),
            };
            result.map(|()| progress).map_err(|error| error.with_retained_progress(progress).into())
        }

        fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
            if self.phase == ReceivingPhase::Closed { return Ok(Default::default()); }
            if self.phase != ReceivingPhase::Closing { return Ok(receiving_metadata(size_of::<ReceivingPhase>())); }
            let demand = if let Some(samples) = self.sample_cursor.as_ref() { Ok(samples.close_demand()) }
            else if !self.samples.terminal_is_empty() { self.samples.close_demands(body) }
            else if !self.output.terminal_is_empty() { self.output.close_demands(body) }
            else if !self.value.terminal_is_empty() { self.value.close_demands(body) }
            else if !self.parser.terminal_is_empty() { self.parser.close_demands(body) }
            else if !self.json_error.terminal_is_empty() { self.json_error.close_demands(body) }
            else { return Ok(receiving_metadata(size_of::<ReceivingPhase>())); }?;
            Ok(RetirementDemand { depth: demand.depth + 1, ..demand })
        }

        fn close_work(&self, body: usize) -> Result<u64, ValueError> {
            if self.phase == ReceivingPhase::Closed { return Ok(0); }
            if self.phase != ReceivingPhase::Closing { return Ok(1); }
            if self.sample_cursor.is_some() { return Ok(1); }
            if !self.samples.terminal_is_empty() { return self.samples.close_work_demand(body); }
            if !self.output.terminal_is_empty() { return self.output.close_work_demand(body); }
            if !self.value.terminal_is_empty() { return self.value.close_work_demand(body); }
            if !self.parser.terminal_is_empty() { return self.parser.close_work_demand(body); }
            if !self.json_error.terminal_is_empty() { return self.json_error.close_work_demand(body); }
            Ok(1)
        }

        fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
            let grant = effect.grant();
            let done = |result, effect: &RetainedAllocationEffect<'_>| RetainedWorkPoll::Complete(RetainedWorkOutcome { result, work: effect.performed_work() });
            if self.phase == ReceivingPhase::Closed { return done(Ok(RetainedCloneStep::Complete(Default::default())), effect); }
            let demand = match self.close_demand(grant.maximum_copy_bytes) {
                Ok(demand) => demand,
                Err(error) => return done(Err(error.into()), effect),
            };
            if effect.maximum_work_units() == 0 || !retained_clone_grant_admits(grant, demand) {
                return done(Ok(RetainedCloneStep::Progress(Default::default())), effect);
            }
            if self.phase == ReceivingPhase::Closing {
                let effect = &mut effect.nested(receiving_child(grant, 0));
                if let Some(samples) = self.sample_cursor.as_mut() {
                    let turn = samples.close(effect);
                    if samples.terminal_is_empty() { self.sample_cursor = None; }
                    return match turn {
                        RetainedWorkPoll::Pending { work } => RetainedWorkPoll::Pending { work },
                        RetainedWorkPoll::Complete(turn) => RetainedWorkPoll::Complete(RetainedWorkOutcome {
                            result: turn.result.map(|step| RetainedCloneStep::Progress(step.progress())), work: turn.work,
                        }),
                    };
                }
                let child = if !self.samples.terminal_is_empty() { Some(self.samples.close_step(effect)) }
                else if !self.output.terminal_is_empty() { Some(self.output.close_step(effect)) }
                else if !self.value.terminal_is_empty() { Some(self.value.close_step(effect)) }
                else if !self.parser.terminal_is_empty() { Some(self.parser.close_step(effect)) }
                else if !self.json_error.terminal_is_empty() { Some(self.json_error.close_step(effect)) }
                else { None };
                if let Some(child) = child {
                    return match child {
                        RetainedWorkPoll::Pending { work } => RetainedWorkPoll::Pending { work },
                        RetainedWorkPoll::Complete(step) => RetainedWorkPoll::Complete(RetainedWorkOutcome {
                            result: step.result.map(RetainedCloneStep::Progress), work: step.work,
                        }),
                    };
                }
            }
            if let Err(error) = receiving_work(effect) { return done(Err(error.into()), effect); }
            let progress = receiving_progress(demand);
            if self.phase == ReceivingPhase::Closing {
                self.phase = ReceivingPhase::Closed;
                done(Ok(RetainedCloneStep::Complete(progress)), effect)
            } else {
                self.phase = ReceivingPhase::Closing;
                done(Ok(RetainedCloneStep::Progress(progress)), effect)
            }
        }
    }

    impl OpBinaryReceivingCursor<PdfMutation> for PdfMutationReceiving {
        fn demands(&self, source: &[u8], body: usize) -> Result<RetirementDemand, ValueError> {
            self.validate_source(source)?;
            Ok(match self.phase {
                ReceivingPhase::Frame => receiving_metadata(size_of::<FramedMutation>() + size_of::<ReceivingPhase>() + size_of_val(&self.authority)),
                ReceivingPhase::StartJson => receiving_metadata(2 * size_of::<Option<JsonGrammarCursor<DslValue>>>()),
                ReceivingPhase::Json if self.samples_start(source)?.is_some() => JsonU32ArrayCursor::admission_demand(),
                ReceivingPhase::Samples => self.sample_cursor.as_ref().expect("admitted PDF Samples cursor").demand(self.frame.payload(source))?.0,
                ReceivingPhase::SamplesMarker => JsonGrammarCursor::<DslValue>::number_array_marker_demand(),
                ReceivingPhase::SamplesTake => JsonU32ArrayCursor::take_demand(),
                ReceivingPhase::Json => {
                    let parser = self.parser.value.as_ref().expect("admitted PDF mutation JSON parser");
                    let demand = parser.normal_step_demands(self.frame.payload(source))
                        .map_err(|error| ValueError::literal(error.kind(), "PDF mutation JSON demand was refused"))?;
                    RetirementDemand { copy_bytes: demand.copy_bytes.saturating_add(JSON_HANDOFF), depth: demand.depth + 1, ..demand }
                }
                ReceivingPhase::Identity(_) => receiving_metadata(size_of::<ReceivingPhase>()),
                ReceivingPhase::BindLeaf => self.leaf.demand(
                    Leaf::for_kind(self.frame.kind).expect("admitted PDF archive leaf"),
                    self.value.value.as_ref().expect("retained PDF archive leaf")).0,
                ReceivingPhase::BindObject => self.object.demand(self.value.value.as_ref().expect("retained PDF archive object"))?.0,
                ReceivingPhase::BindMetadata => self.metadata.demand(self.value.value.as_ref().expect("retained PDF image metadata"))?.0,
                ReceivingPhase::Bind => receiving_metadata(self.retained_json.saturating_add(OUTPUT_HANDOFF + size_of::<DslValue>() + 2 * size_of::<Option<Vec<u32>>>())),
                ReceivingPhase::Ready => receiving_metadata(OUTPUT_HANDOFF),
                ReceivingPhase::Closing | ReceivingPhase::Closed => return self.close_demand(body),
            })
        }

        fn step_work_demand(&self, source: &[u8], _: usize) -> Result<u64, ValueError> {
            self.validate_source(source)?;
            if self.phase == ReceivingPhase::BindLeaf {
                return Ok(self.leaf.demand(
                    Leaf::for_kind(self.frame.kind).expect("admitted PDF archive leaf"),
                    self.value.value.as_ref().expect("retained PDF archive leaf")).1);
            }
            if self.phase == ReceivingPhase::BindObject {
                return self.object.demand(self.value.value.as_ref().expect("retained PDF archive object")).map(|quote| quote.1);
            }
            if self.phase == ReceivingPhase::BindMetadata {
                return self.metadata.demand(self.value.value.as_ref().expect("retained PDF image metadata")).map(|quote| quote.1);
            }
            if self.phase == ReceivingPhase::Samples {
                return self.sample_cursor.as_ref().expect("admitted PDF Samples cursor").demand(self.frame.payload(source)).map(|quote| quote.1);
            }
            Ok(1)
        }

        fn step_in(&mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, body: usize, effect: &mut RetainedAllocationEffect<'_>)
            -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary> {
            let result = self.turn(source, native, body, effect);
            if result.is_err() { self.failed = true; }
            RetainedWorkOutcome { result, work: effect.performed_work() }
        }

        fn domain_failure(&self) -> Option<&ValueError> { None }
        fn domain_failure_demand(&self) -> Result<RetirementDemand, ValueError> { Ok(Default::default()) }
        fn domain_failure_work_demand(&self) -> Result<u64, ValueError> { Ok(0) }
        fn take_domain_failure_into(&mut self, recipient: &mut Option<ValueError>, _: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress> {
            let result = if recipient.is_some() { Err(receiving_invalid("PDF mutation refusal recipient is occupied")) } else { Ok(Default::default()) };
            RetainedWorkOutcome { result, work: 0 }
        }

        fn is_ready(&self) -> bool { !self.failed && self.phase == ReceivingPhase::Ready && self.output.value.is_some() }
        fn take_into(&mut self, recipient: &mut Option<PdfMutation>, grant: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress> {
            let demand = receiving_metadata(OUTPUT_HANDOFF);
            if !self.is_ready() || recipient.is_some() || !retained_clone_grant_admits(grant, demand) {
                return RetainedWorkOutcome { result: Ok(Default::default()), work: 0 };
            }
            *recipient = self.output.value.take();
            RetainedWorkOutcome { result: Ok(receiving_progress(demand)), work: 1 }
        }

        fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.close_demand(body) }
        fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll { self.close(effect) }
        fn terminal_is_empty(&self) -> bool {
            self.phase == ReceivingPhase::Closed && self.sample_cursor.is_none() && self.samples.terminal_is_empty() && self.output.terminal_is_empty()
                && self.value.terminal_is_empty() && self.parser.terminal_is_empty() && self.json_error.terminal_is_empty()
        }
    }

    /// ♻️ Retires every unpublished JSON and mutation owner through the original allocation effect.
    pub struct PdfMutationReceivingRetirement(ManuallyDrop<PdfMutationReceiving>);
    impl RetireOwned for PdfMutationReceiving {
        type RetirementCursor = PdfMutationReceivingRetirement;
        fn retirement_seed(self) -> Self::RetirementCursor { PdfMutationReceivingRetirement(ManuallyDrop::new(self)) }
        fn controlled_retirement_supported() -> bool { true }
    }
    impl PdfMutationReceivingRetirement {
        fn close_step(&mut self, _: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetirementStep { erased_cursor_turn(self.0.close(effect)) }
    }
    impl RetirementCursor for PdfMutationReceivingRetirement {
        semio_framework_value::native_retirement_reply!();
        fn terminal_is_empty(&self) -> bool { self.0.terminal_is_empty() }
        fn next_computational_work(&self, body: usize) -> Result<u64, ValueError> { self.0.close_work(body) }
        fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.0.close_demand(usize::MAX)?.depth) }
        fn next_work_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.0.close_demand(usize::MAX)?.copy_bytes) }
        fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
        fn next_selected_work_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.0.close_demand(body)?.copy_bytes) }
        fn next_close_byte_demand(&self) -> Option<usize> { self.0.close_demand(usize::MAX).ok().map(|demand| demand.release_bytes) }
        fn next_birth_bytes(&self, body: usize) -> Option<usize> { self.0.close_demand(body).ok().map(|demand| demand.capacity_bytes) }
    }
    impl Drop for PdfMutationReceivingRetirement {
        fn drop(&mut self) {
            assert!(self.0.terminal_is_empty() || std::thread::panicking(), "PDF original mutation receiving requires paid closure");
            if self.0.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.0) } }
        }
    }

    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfImageBody;

    const SAMPLE_PATH: [&str; 3] = ["image", "body", "values"];
    const SAMPLE_TAGS: [(usize, &str); 2] = [(0, "mutation"), (2, "kind")];
    const SAMPLE_BACKING_BYTES: usize = 1 << 20;

    fn donate_samples(mutation: &mut PdfMutation, donor: &mut Option<Vec<u32>>) -> Result<(), ValueError> {
        let Some(_) = donor else { return Ok(()); };
        let PdfMutation::SetImage(operation) = mutation else {
            return Err(ValueError::invariant("PDF sample donor changed mutation family"));
        };
        let PdfImageBody::Samples { values } = &mut operation.image.body else {
            return Err(ValueError::invariant("PDF sample donor changed image body family"));
        };
        if !values.is_empty() || values.capacity() != 0 {
            return Err(ValueError::invariant("PDF sample donor destination is occupied"));
        }
        *values = donor.take().expect("validated PDF sample donor");
        Ok(())
    }

    impl PdfMutationReceiving {
        fn samples_start(&self, source: &[u8]) -> Result<Option<usize>, ValueError> {
            if self.frame.kind != Some("setImage") || self.samples.value.is_some() {
                return Ok(None);
            }
            let selected = self.parser.value.as_ref().and_then(|parser|
                parser.selected_number_array_tags(self.frame.payload(source), &SAMPLE_PATH, &SAMPLE_TAGS));
            let Some((start, [mutation, body])) = selected else { return Ok(None); };
            if mutation.is_some_and(|tag| tag.as_str() != Some("setImage")) {
                return Err(receiving_invalid("PDF mutation tag differs from its JSON identity"));
            }
            match body.map(DslValue::as_str) {
                Some(Some("samples")) if mutation.is_some() => Ok(Some(start)),
                Some(Some("samples" | "artifact")) | None => Ok(None),
                _ => Err(receiving_invalid("unknown PDF image body discriminator")),
            }
        }

        fn advance_samples(&mut self, source: &[u8], effect: &mut RetainedAllocationEffect<'_>)
            -> Result<RetainedCloneProgress, ValueError> {
            let payload = self.frame.payload(source);
            let cursor = self.sample_cursor.as_mut().expect("admitted PDF Samples cursor");
            match self.phase {
                ReceivingPhase::Samples => {
                    let progress = cursor.advance(payload, effect, || true).result?;
                    if cursor.is_ready() { self.phase = ReceivingPhase::SamplesMarker; }
                    Ok(progress)
                }
                ReceivingPhase::SamplesMarker => {
                    let span = cursor.finished_span(payload)?.expect("validated PDF Samples span");
                    let parser = self.parser.value.as_mut().expect("admitted PDF mutation JSON parser");
                    let progress = parser.accept_number_array_marker(payload, &SAMPLE_PATH, &span, effect)?;
                    if progress.copied_items != 0 { self.phase = ReceivingPhase::SamplesTake; }
                    Ok(progress)
                }
                ReceivingPhase::SamplesTake => {
                    let progress = cursor.take(&mut self.samples.value, effect)?;
                    if self.samples.value.is_some() {
                        self.sample_cursor = None;
                        self.phase = ReceivingPhase::Json;
                    }
                    Ok(progress)
                }
                _ => unreachable!(),
            }
        }

        fn bind_mutation(&mut self) -> Result<(), ValueError> {
            let value = self.value.value.take().expect("admitted PDF mutation JSON value");
            let mutation = PdfMutation::from_value(value)?;
            *self.output.value = Some(mutation);
            donate_samples(self.output.value.as_mut().expect("resident PDF mutation output"), &mut self.samples.value)?;
            self.phase = ReceivingPhase::Ready;
            Ok(())
        }
    }
}
pub use receiving::{PdfMutationReceiving, PdfMutationReceivingRetirement};

//#region 🧱️Framing
const MAX_PAYLOAD_BYTES: usize = 64 * 1024 * 1024;

fn malformed(offset: u64, detail: impl Into<String>) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "PDF mutation aggregate", offset, detail: detail.into() }
}

fn identity(payload: &[u8]) -> Result<String, protocol::ProtocolError> {
    let value = semio_framework_pack_json::parse_bytes(payload, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| malformed(2, error.to_string()))?;
    value.get("mutation").and_then(semio_framework_pack_json::Value::as_str).map(str::to_owned).ok_or_else(|| malformed(2, "missing mutation identity"))
}

impl OpBinary for PdfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let payload = semio_framework_pack_json::to_json_string(self).into_bytes();
        if payload.len() > MAX_PAYLOAD_BYTES {
            return Err(protocol::ProtocolError::LimitExceeded("PDF mutation payload"));
        }
        let identity = identity(&payload)?;
        let tag = BINARY_TAG_REGISTRY.iter().find(|(_, json, _)| *json == identity).map(|(_, _, tag)| *tag).ok_or_else(|| malformed(1, format!("unknown mutation identity {identity:?}")))?;
        let mut writer = dsl::ByteWriter::new();
        writer.write_u8(store::pack_rt::OP_BINARY_FORMAT);
        writer.write_u8(tag);
        writer.write_varint_u64(payload.len() as u64);
        writer.write_bytes(&payload);
        Ok(writer.into_bytes())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = dsl::ByteReader::new(bytes);
        let format = reader.read_u8().map_err(|error| malformed(0, error.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed(0, format!("expected format {}, got {format}", store::pack_rt::OP_BINARY_FORMAT)));
        }
        let tag = reader.read_u8().map_err(|error| malformed(1, error.to_string()))?;
        let (_, expected_identity, _) = BINARY_TAG_REGISTRY.iter().find(|(_, _, candidate)| *candidate == tag).ok_or_else(|| malformed(1, format!("unknown mutation tag {tag}")))?;
        let payload_len = usize::try_from(reader.read_varint_u64().map_err(|error| malformed(2, error.to_string()))?).map_err(|_| malformed(2, "payload length exceeds usize"))?;
        if payload_len > MAX_PAYLOAD_BYTES {
            return Err(protocol::ProtocolError::LimitExceeded("PDF mutation payload"));
        }
        let payload = reader.read_bytes(payload_len).map_err(|error| malformed(2, error.to_string()))?;
        if reader.remaining() != 0 {
            return Err(malformed((bytes.len() - reader.remaining()) as u64, "trailing bytes"));
        }
        let actual_identity = identity(payload)?;
        if actual_identity != *expected_identity {
            return Err(malformed(2, format!("tag {tag} declares {expected_identity}, payload declares {actual_identity}")));
        }
        let parsed = semio_framework_pack_json::parse_bytes(payload, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| malformed(2, error.to_string()))?;
        semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| malformed(2, error.to_string()))
    }
}
//#endregion 🧱️Framing

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_color_space;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_page_media_box;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_pattern;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::remove_shading;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_page_rotation;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::move_page;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_open_action;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::remove_pattern;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::remove_page;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::insert_object;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_viewer_preferences;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_page_user_unit;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_language;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_page_crop_box;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_image;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_named_destination;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_form;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_catalog_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_mark_info;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::remove_ext_g_state;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_annotation;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_page_layout;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_output_intents;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_acro_form;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_annotation;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_embedded_file;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_content;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_metadata;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::insert_annotation;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_embedded_file;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_dict_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_ext_g_state;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_shading;

pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::base::io::binary::mutations::append_page_content;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_form;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_font;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_object;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_properties;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_trailer_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_named_destination;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_outlines;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_dict_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_catalog_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_trailer_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::insert_page;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::replace_content;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_encryption;

pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::base::io::binary::mutations::set_object_value;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_image;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::set_optional_content;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::set_page_labels;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::remove_properties;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::set_page_content;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::set_page_box;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::remove_font;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::remove_color_space;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::set_page_mode;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::insert_content;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::set_info;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::set_document_id;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::replace_page;
