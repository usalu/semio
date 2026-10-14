//! 📄️ File-level sections: header variables, classes, summary, application, dependencies, template, auxiliary header, revision history and preview.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_HEADER_BEGIN;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_HEADER_END;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_PREVIEW_BEGIN;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_PREVIEW_END;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_SENTINEL_CLASSES_BEGIN;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DwgBitReader;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::dwg_crc16;
use crate::standards::v_ac1024::subsets::any::io::import::records::container::{decode_r2004_sections, r2004_section_data};
use crate::standards::v_ac1024::subsets::any::io::import::records::fields::{r2010_string_stream};

//#region 🔖️DwgStructuralCodec
//#region DwgRead
struct DwgSectionCursor<'a> {
    bytes: &'a [u8],
    position: usize,
}
impl<'a> DwgSectionCursor<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn take(&mut self, length: usize) -> Result<&'a [u8], String> {
        let end = self.position.checked_add(length).ok_or("DWG section cursor overflow")?;
        let bytes = self.bytes.get(self.position..end).ok_or_else(|| format!("DWG section value at {} needs {length} bytes, only {} remain", self.position, self.bytes.len().saturating_sub(self.position)))?;
        self.position = end;
        Ok(bytes)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn u16(&mut self) -> Result<u16, String> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn i32(&mut self) -> Result<i32, String> {
        Ok(i32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn utf16_z(&mut self) -> Result<String, String> {
        let count = usize::from(self.u16()?);
        let units = self.take(count.checked_mul(2).ok_or("DWG UTF-16 length overflow")?)?.as_chunks::<2>().0.iter().map(|unit| u16::from_le_bytes([unit[0], unit[1]])).take_while(|unit| *unit != 0).collect::<Vec<_>>();
        String::from_utf16(&units).map_err(|error| format!("invalid DWG UTF-16 string: {error}"))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn utf16_bytes(&mut self) -> Result<String, String> {
        let byte_count = self.u32()? as usize;
        if !byte_count.is_multiple_of(2) {
            return Err("DWG UTF-16 byte string has an odd length".into());
        }
        let units = self.take(byte_count)?.as_chunks::<2>().0.iter().map(|unit| u16::from_le_bytes([unit[0], unit[1]])).collect::<Vec<_>>();
        String::from_utf16(&units).map_err(|error| format!("invalid DWG UTF-16 byte string: {error}"))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn bytes_z(&mut self) -> Result<String, String> {
        let count = usize::from(self.u16()?);
        let bytes = self.take(count)?;
        Ok(String::from_utf8_lossy(bytes.strip_suffix(&[0]).unwrap_or(bytes)).into_owned())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn has_more(&self) -> bool {
        self.position < self.bytes.len()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn finish(self, section: &str) -> Result<(), String> {
        if self.position == self.bytes.len() {
            Ok(())
        } else {
            Err(format!("{section} has {} trailing bytes", self.bytes.len() - self.position))
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn checksum_text(bytes: &[u8]) -> String {
    bytes.iter().enumerate().map(|(index, byte)| format!("{}{:02x}", if [4, 6, 8, 10].contains(&index) { "-" } else { "" }, byte)).collect::<Vec<_>>().join("")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_summary_info(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgSummaryInfo, String> {
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgCustomProperty;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgJulianDate;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgSummaryInfo;
    let mut cursor = DwgSectionCursor::new(bytes);
    let title = cursor.utf16_z()?;
    let subject = cursor.utf16_z()?;
    let author = cursor.utf16_z()?;
    let keywords = cursor.utf16_z()?;
    let comments = cursor.utf16_z()?;
    let last_saved_by = cursor.utf16_z()?;
    let revision_number = cursor.utf16_z()?;
    let hyperlink_base = cursor.utf16_z()?;
    let total_editing_time = cursor.u64()?;
    let created_at = DwgJulianDate { days: cursor.u32()?, milliseconds: cursor.u32()? };
    let modified_at = DwgJulianDate { days: cursor.u32()?, milliseconds: cursor.u32()? };
    let property_count = usize::from(cursor.u16()?);
    let mut custom_properties = Vec::with_capacity(property_count);
    for _ in 0..property_count {
        custom_properties.push(DwgCustomProperty { key: cursor.utf16_z()?, value: cursor.utf16_z()? });
    }
    let _reserved_one = cursor.u32()?;
    let _reserved_two = cursor.u32()?;
    Ok(DwgSummaryInfo { title, subject, author, keywords, comments, last_saved_by, revision_number, hyperlink_base, total_editing_time, created_at, modified_at, custom_properties })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_application_info(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationInfo, String> {
    let mut cursor = DwgSectionCursor::new(bytes);
    let _format = cursor.u32()?;
    let name = cursor.utf16_z()?;
    let _field_count = cursor.u32()?;
    let version_checksum = checksum_text(cursor.take(16)?);
    let version = cursor.utf16_z()?;
    let comment_checksum = checksum_text(cursor.take(16)?);
    let comment = cursor.utf16_z()?;
    let product_checksum = checksum_text(cursor.take(16)?);
    let product = cursor.utf16_z()?;
    let application_version = if cursor.has_more() { cursor.bytes_z()? } else { String::new() };
    Ok(semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationInfo { name, version_checksum, version, comment_checksum, comment, product_checksum, product, application_version })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_dependencies(bytes: &[u8]) -> Result<Vec<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgDependency>, String> {
    let mut cursor = DwgSectionCursor::new(bytes);
    let feature_count = cursor.u32()? as usize;
    let mut features = Vec::with_capacity(feature_count);
    for _ in 0..feature_count {
        features.push(cursor.utf16_bytes()?);
    }
    let file_count = cursor.u32()? as usize;
    let mut dependencies = Vec::with_capacity(file_count);
    for _ in 0..file_count {
        let full_path = cursor.utf16_bytes()?;
        let relative_path = cursor.utf16_bytes()?;
        let fingerprint = cursor.utf16_bytes()?;
        let version = cursor.utf16_bytes()?;
        let feature_index = cursor.u32()? as usize;
        let timestamp = cursor.u32()?;
        let file_size = cursor.u32()?;
        let affects_graphics = cursor.u16()? != 0;
        let reference_count = cursor.u32()?;
        let feature = features.get(feature_index).cloned().ok_or("DWG dependency feature index is out of bounds")?;
        dependencies.push(semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgDependency { feature, full_path, relative_path, fingerprint, version, timestamp, file_size, affects_graphics, reference_count });
    }
    Ok(dependencies)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_template(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgTemplate, String> {
    let mut cursor = DwgSectionCursor::new(bytes);
    let description = cursor.utf16_z()?;
    let measurement = match cursor.u16()? {
        0 => semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgMeasurement::English,
        1 => semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgMeasurement::Metric,
        value => return Err(format!("unsupported DWG measurement value {value}")),
    };
    Ok(semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgTemplate { description, measurement })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_auxiliary_header(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgAuxiliaryHeader, String> {
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgAuxiliaryHeader;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgCompatibilityProfile;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgJulianDate;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgVersionStamp;
    let mut cursor = DwgSectionCursor::new(bytes);
    if [cursor.u8()?, cursor.u8()?, cursor.u8()?] != [255, 119, 1] {
        return Err("unsupported auxiliary-header intro".into());
    }
    let version = cursor.u16()?;
    let maintenance = cursor.u16()?;
    if version != 29 || maintenance != 2 {
        return Err(format!("unsupported auxiliary-header target {version}.{maintenance}"));
    }
    let total_saves = cursor.u32()?;
    if cursor.i32()? != -1 {
        return Err("auxiliary-header minus-one marker changed".into());
    }
    let save_partition_one = cursor.u16()?;
    let save_partition_two = cursor.u16()?;
    let save_generation = cursor.u32()?;
    let legacy_stamp_one = DwgVersionStamp { version: cursor.u16()?, maintenance: cursor.u16()? };
    let legacy_stamp_two = DwgVersionStamp { version: cursor.u16()?, maintenance: cursor.u16()? };
    let profile_shorts = [cursor.u16()?, cursor.u16()?, cursor.u16()?, cursor.u16()?, cursor.u16()?, cursor.u16()?];
    let profile_longs = [cursor.u32()?, cursor.u32()?, cursor.u32()?, cursor.u32()?, cursor.u32()?];
    if profile_shorts != [4, 1381, 261, 2600, 0, 1] || profile_longs != [0, 0, 0, 16_908_544, 65_538] {
        return Err("unsupported auxiliary-header compatibility profile".into());
    }
    let created_at = DwgJulianDate { days: cursor.u32()?, milliseconds: cursor.u32()? };
    let updated_at = DwgJulianDate { days: cursor.u32()?, milliseconds: cursor.u32()? };
    let handle_seed = cursor.u64()?;
    if cursor.u16()? != 0 {
        return Err("auxiliary-header reserved handle marker changed".into());
    }
    let terminal_save_generation = cursor.u16()?;
    let terminal = [cursor.u32()?, cursor.u32()?, cursor.u32()?, cursor.u32()?, cursor.u32()?, cursor.u32()?, cursor.u32()?, cursor.u32()?];
    if terminal != [0, 0, 0, total_saves, 0, 0, 0, 0] {
        return Err("auxiliary-header terminal profile changed".into());
    }
    cursor.finish("AcDb:AuxHeader")?;
    Ok(DwgAuxiliaryHeader {
        total_saves,
        save_partition_one,
        save_partition_two,
        save_generation,
        legacy_stamp_one,
        legacy_stamp_two,
        compatibility_profile: DwgCompatibilityProfile::Autocad2009,
        created_at,
        updated_at,
        handle_seed,
        terminal_save_generation,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_revision_history(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgRevisionHistory, String> {
    let mut cursor = DwgSectionCursor::new(bytes);
    let format_major = cursor.u32()?;
    let format_minor = cursor.u32()?;
    let count = cursor.u32()? as usize;
    let mut revisions = Vec::with_capacity(count);
    for _ in 0..count {
        revisions.push(cursor.u32()?);
    }
    cursor.finish("AcDb:RevHistory")?;
    Ok(semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgRevisionHistory { format_major, format_minor, revisions })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_indexed_preview(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgIndexedPreview, String> {
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgIndexedPreview;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgPreviewOrigin;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgRgba;
    let mut cursor = DwgSectionCursor::new(bytes);
    if cursor.take(16)? != DWG_PREVIEW_BEGIN {
        return Err("preview start sentinel changed".into());
    }
    if cursor.u32()? as usize != bytes.len() - 36 || cursor.u8()? != 2 {
        return Err("preview record envelope changed".into());
    }
    let (code_one, start_one, size_one) = (cursor.u8()?, cursor.u32()?, cursor.u32()?);
    let (code_two, start_two, size_two) = (cursor.u8()?, cursor.u32()?, cursor.u32()?);
    if (code_one, size_one, code_two) != (1, 80, 2) || start_two != start_one + 80 || size_two as usize != bytes.len().saturating_sub(16 + 4 + 1 + 18 + 80 + 16) {
        return Err("unsupported preview record table".into());
    }
    if cursor.take(80)?.iter().any(|byte| *byte != 0) {
        return Err("preview header record changed".into());
    }
    let header_size = cursor.u32()?;
    let width = cursor.i32()?;
    let height = cursor.i32()?;
    let planes = cursor.u16()?;
    let depth = cursor.u16()?;
    let compression = cursor.u32()?;
    let image_size = cursor.u32()?;
    let x_resolution = cursor.i32()?;
    let y_resolution = cursor.i32()?;
    let colors = cursor.u32()?;
    let important = cursor.u32()?;
    if header_size != 40 || width < 0 || height < 0 || planes != 1 || depth != 8 || compression != 0 || x_resolution != 0 || y_resolution != 0 || colors != 256 || important != 0 {
        return Err("unsupported indexed preview bitmap header".into());
    }
    let mut palette = Vec::with_capacity(256);
    for _ in 0..256 {
        let blue = cursor.u8()?;
        let green = cursor.u8()?;
        let red = cursor.u8()?;
        let alpha = cursor.u8()?;
        palette.push(DwgRgba { red, green, blue, alpha });
    }
    let width = width as usize;
    let height = height as usize;
    let stride = (width + 3) & !3;
    if image_size as usize != stride * height {
        return Err("preview image size disagrees with dimensions".into());
    }
    let mut pixel_indices = Vec::with_capacity(width * height);
    let mut background_palette_index = None;
    for _ in 0..height {
        pixel_indices.extend_from_slice(cursor.take(width)?);
        let padding = cursor.take(stride - width)?;
        if let Some(index) = padding.first().copied() {
            if padding.iter().any(|value| *value != index) {
                return Err("preview row padding is inconsistent".into());
            }
            if background_palette_index.get_or_insert(index) != &index {
                return Err("preview background palette index changed between rows".into());
            }
        }
    }
    if cursor.take(16)? != DWG_PREVIEW_END {
        return Err("preview end sentinel changed".into());
    }
    cursor.finish("AcDb:Preview")?;
    Ok(DwgIndexedPreview { width: width as u32, height: height as u32, origin: DwgPreviewOrigin::BottomUp, palette, pixel_indices, background_palette_index: background_palette_index.unwrap_or(0) })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_digest128(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
/// 🧾️ The inverse of [`render_application_properties`]: the property-set format identifier and its
/// `<prop id="…"><string|datetime>…</…></prop>` entries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_application_properties(rendered: &str) -> Result<(String, Vec<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationProperty>), String> {
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationProperty;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationPropertyKind;
    let body = rendered.strip_prefix("<prop_set fmt_id=\"{").and_then(|rest| rest.strip_suffix("</prop_set>")).ok_or("application property set is not a <prop_set>")?;
    let (format_identifier, mut rest) = body.split_once("}\">").ok_or("application property set has no format identifier")?;
    let mut properties = Vec::new();
    while !rest.is_empty() {
        let entry = rest.strip_prefix("<prop id=\"").ok_or("application property is not a <prop>")?;
        let (id, entry) = entry.split_once("\"><").ok_or("application property has no id")?;
        let (tag, entry) = entry.split_once('>').ok_or("application property has no value tag")?;
        let kind = match tag {
            "string" => DwgApplicationPropertyKind::String,
            "datetime" => DwgApplicationPropertyKind::DateTime,
            other => return Err(format!("application property value tag {other:?} is unknown")),
        };
        let (value, entry) = entry.split_once(&format!("</{tag}></prop>")).ok_or("application property is not closed")?;
        properties.push(DwgApplicationProperty { id: id.parse().map_err(|_| format!("application property id {id:?} is not a number"))?, kind, value: value.into() });
        rest = entry;
    }
    Ok((format_identifier.into(), properties))
}
/// 🧾️ The inverse of [`render_product_information`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_product_information(rendered: &str) -> Result<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgProductInformation, String> {
    let mut values = Vec::new();
    let mut rest = rendered.strip_prefix("\"<ProductInformation name =\\\"").and_then(|rest| rest.strip_suffix("\\\"/>\"")).ok_or("product information is not a <ProductInformation>")?;
    for key in ["build_version", "registry_version", "install_id_string", "registry_localeID"] {
        let (value, next) = rest.split_once(&format!("\\\" {key}=\\\"")).ok_or_else(|| format!("product information has no {key}"))?;
        values.push(value.to_string());
        rest = next;
    }
    values.push(rest.to_string());
    let [name, build_version, registry_version, install_id, locale_id]: [String; 5] = values.try_into().map_err(|_| "product information field count")?;
    Ok(semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgProductInformation { name, build_version, registry_version, install_id, locale_id })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_application_history(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationHistory, String> {
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationHistory;
    let mut cursor = DwgSectionCursor::new(bytes);
    let history_identifier_one = decode_digest128(cursor.take(16)?);
    let history_identifier_two = decode_digest128(cursor.take(16)?);
    let class_version = cursor.u32()?;
    if cursor.utf16_z()? != "AppInfoDataList" || cursor.u32()? != 4 {
        return Err("unsupported application-history list".into());
    }
    let application_version_digest = decode_digest128(cursor.take(16)?);
    let application_version = cursor.utf16_z()?;
    let trust_comment_digest = decode_digest128(cursor.take(16)?);
    let trust_comment = cursor.utf16_z()?;
    let property_set_digest = decode_digest128(cursor.take(16)?);
    let rendered_properties = cursor.utf16_z()?;
    let product_digest = decode_digest128(cursor.take(16)?);
    let rendered_product = cursor.utf16_z()?;
    cursor.finish("AcDb:AppInfoHistory")?;
    let (property_format_identifier, properties) = parse_application_properties(&rendered_properties)?;
    let product = parse_product_information(&rendered_product)?;
    Ok(DwgApplicationHistory {
        history_identifier_one,
        history_identifier_two,
        class_version,
        application_version_digest,
        application_version,
        trust_comment_digest,
        trust_comment,
        property_set_digest,
        property_format_identifier,
        properties,
        product_digest,
        product,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_header_time(reader: &mut DwgBitReader<'_>) -> Result<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgJulianDate, String> {
    Ok(semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgJulianDate { days: reader.read_bl()?, milliseconds: reader.read_bl()? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_header_color(reader: &mut DwgBitReader<'_>, expected_rgb: u32, name: &str) -> Result<u16, String> {
    let index = reader.read_bs()?;
    let rgb = reader.read_bl()?;
    let flags = reader.read_rc()?;
    if rgb != expected_rgb || flags != 0 {
        return Err(format!("unsupported AC1024 Header color {name} index={index} rgb={rgb:#x} flags={flags:#x}"));
    }
    Ok(index)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_header_space(reader: &mut DwgBitReader<'_>) -> Result<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderSpaceGeometry, String> {
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderSpaceGeometry;
    Ok(DwgHeaderSpaceGeometry {
        insertion_base: reader.read_3bd().map_err(|error| format!("insertion base: {error}"))?.to_vec(),
        extents_minimum: reader.read_3bd().map_err(|error| format!("extents minimum: {error}"))?.to_vec(),
        extents_maximum: reader.read_3bd().map_err(|error| format!("extents maximum: {error}"))?.to_vec(),
        limits_minimum: reader.read_2rd()?.to_vec(),
        limits_maximum: reader.read_2rd()?.to_vec(),
        elevation: reader.read_bd().map_err(|error| format!("elevation: {error}"))?,
        ucs_origin: reader.read_3bd().map_err(|error| format!("UCS origin: {error}"))?.to_vec(),
        ucs_x_axis: reader.read_3bd().map_err(|error| format!("UCS X axis: {error}"))?.to_vec(),
        ucs_y_axis: reader.read_3bd().map_err(|error| format!("UCS Y axis: {error}"))?.to_vec(),
        ucs_orthographic_view: reader.read_bs()?,
        ucs_origin_top: reader.read_3bd().map_err(|error| format!("UCS top: {error}"))?.to_vec(),
        ucs_origin_bottom: reader.read_3bd().map_err(|error| format!("UCS bottom: {error}"))?.to_vec(),
        ucs_origin_left: reader.read_3bd().map_err(|error| format!("UCS left: {error}"))?.to_vec(),
        ucs_origin_right: reader.read_3bd().map_err(|error| format!("UCS right: {error}"))?.to_vec(),
        ucs_origin_front: reader.read_3bd().map_err(|error| format!("UCS front: {error}"))?.to_vec(),
        ucs_origin_back: reader.read_3bd().map_err(|error| format!("UCS back: {error}"))?.to_vec(),
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_header_relation(reader: &mut DwgBitReader<'_>, name: &str) -> Result<u64, String> {
    let (code, value) = reader.read_handle()?;
    let expected = match name {
        "block_control" | "layer_control" | "style_control" | "linetype_control" | "view_control" | "ucs_control" | "viewport_control" | "appid_control" | "dimension_style_control" | "named_objects_dictionary" => 3,
        _ => 5,
    };
    if code != expected {
        return Err(format!("AC1024 Header relation {name} uses handle code {code}, expected {expected}"));
    }
    Ok(value)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_optional_header_relation(reader: &mut DwgBitReader<'_>, name: &str) -> Result<Option<u64>, String> {
    Ok(Some(read_header_relation(reader, name)?).filter(|value| *value != 0))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_r2010_header_section(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderVariables, String> {
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderVariables;
    if bytes.len() < 58 || bytes[..16] != DWG_HEADER_BEGIN {
        return Err("AC1024 Header begin sentinel is missing".into());
    }
    let size = u32::from_le_bytes(bytes[16..20].try_into().unwrap()) as usize;
    let bit_size = u32::from_le_bytes(bytes[20..24].try_into().unwrap()) as usize;
    let crc_end = size.checked_add(20).filter(|end| *end >= 24 && end + 18 <= bytes.len()).ok_or("AC1024 Header size exceeds its section")?;
    if bytes[crc_end + 2..crc_end + 18] != DWG_HEADER_END {
        return Err("AC1024 Header end sentinel is missing".into());
    }
    let expected_crc = u16::from_le_bytes(bytes[crc_end..crc_end + 2].try_into().unwrap());
    if dwg_crc16(0xc0c1, &bytes[16..crc_end]) != expected_crc {
        return Err("AC1024 Header CRC mismatch".into());
    }
    let stream = &bytes[24..crc_end];
    let boundary = bit_size.checked_sub(32).filter(|bits| *bits <= stream.len() * 8).ok_or("AC1024 Header bit size exceeds its stream")?;
    let mut main = DwgBitReader::new(stream);
    let mut value = DwgHeaderVariables::default();
    let u = &mut value.units;
    u.unit1_conversion = main.read_bd()?;
    u.unit2_conversion = main.read_bd()?;
    u.unit3_conversion = main.read_bd()?;
    u.unit4_conversion = main.read_bd()?;
    if main.read_bl()? != 2_454_805 || main.read_bl()? != 60_784_745 {
        return Err("unsupported AC1024 Header producer date profile".into());
    }
    let m = &mut value.modes;
    m.dimension_associative = main.read_b()?;
    m.dimension_show = main.read_b()?;
    m.polyline_generation = main.read_b()?;
    m.orthographic_mode = main.read_b()?;
    m.regeneration_mode = main.read_b()?;
    m.fill_mode = main.read_b()?;
    m.quick_text_mode = main.read_b()?;
    m.paper_space_linetype_scale = main.read_b()?;
    m.limits_check = main.read_b()?;
    if main.read_b()? {
        return Err("unsupported AC1024 Header limits feature marker".into());
    }
    m.user_timer = main.read_b()?;
    m.sketch_polyline = main.read_b()?;
    m.angle_direction = main.read_b()?;
    m.spline_frame = main.read_b()?;
    m.mirror_text = main.read_b()?;
    m.world_view = main.read_b()?;
    m.tile_mode = main.read_b()?;
    m.paper_limits_check = main.read_b()?;
    m.visual_retain = main.read_b()?;
    m.display_silhouette = main.read_b()?;
    m.polyline_ellipse = main.read_b()?;
    let i = &mut value.integers;
    i.proxy_graphics = main.read_bs()?;
    i.tree_depth = main.read_bs()? as i16;
    i.linear_units = main.read_bs()?;
    i.linear_precision = main.read_bs()?;
    i.angular_units = main.read_bs()?;
    i.angular_precision = main.read_bs()?;
    i.attribute_mode = main.read_bs()?;
    i.point_display_mode = main.read_bs()?;
    if [main.read_bl()?, main.read_bl()?, main.read_bl()?] != [0x30303030, 0x1d050900, 0x4d353930] {
        return Err("unsupported AC1024 Header producer generation".into());
    }
    i.user_integer1 = main.read_bs()? as i16;
    i.user_integer2 = main.read_bs()? as i16;
    i.user_integer3 = main.read_bs()? as i16;
    i.user_integer4 = main.read_bs()? as i16;
    i.user_integer5 = main.read_bs()? as i16;
    i.spline_segments = main.read_bs()?;
    i.surface_u = main.read_bs()?;
    i.surface_v = main.read_bs()?;
    i.surface_type = main.read_bs()?;
    i.surface_tab1 = main.read_bs()?;
    i.surface_tab2 = main.read_bs()?;
    i.spline_type = main.read_bs()?;
    i.shade_edge = main.read_bs()?;
    i.shade_difference = main.read_bs()?;
    i.unit_mode = main.read_bs()?;
    i.maximum_active_viewports = main.read_bs()?;
    i.isolines = main.read_bs()?;
    i.multiline_justification = main.read_bs()?;
    i.text_quality = main.read_bs()?;
    let s = &mut value.scalars;
    s.linetype_scale = main.read_bd()?;
    s.text_size = main.read_bd()?;
    s.trace_width = main.read_bd()?;
    s.sketch_increment = main.read_bd()?;
    s.fillet_radius = main.read_bd()?;
    s.thickness = main.read_bd()?;
    s.angle_base = main.read_bd()?;
    s.point_display_size = main.read_bd()?;
    s.polyline_width = main.read_bd()?;
    s.user_real1 = main.read_bd()?;
    s.user_real2 = main.read_bd()?;
    s.user_real3 = main.read_bd()?;
    s.user_real4 = main.read_bd()?;
    s.user_real5 = main.read_bd()?;
    s.chamfer_a = main.read_bd()?;
    s.chamfer_b = main.read_bd()?;
    s.chamfer_c = main.read_bd()?;
    s.chamfer_d = main.read_bd()?;
    s.facet_resolution = main.read_bd()?;
    s.multiline_scale = main.read_bd()?;
    s.current_entity_linetype_scale = main.read_bd()?;
    value.time.created_at = read_header_time(&mut main)?;
    value.time.updated_at = read_header_time(&mut main)?;
    if [main.read_bl()?, main.read_bl()?, main.read_bl()?] != [0x44443434, 0x140d0102, 0x39343531] {
        return Err("unsupported AC1024 Header time producer profile".into());
    }
    value.time.editing_duration = read_header_time(&mut main)?;
    value.time.user_timer_duration = read_header_time(&mut main)?;
    s.current_entity_color_index = read_header_color(&mut main, 0xc0000000, "CECOLOR")?;
    let (seed_code, seed) = main.read_handle()?;
    if seed_code != 0 {
        return Err(format!("AC1024 HANDSEED code {seed_code} != 0"));
    }
    value.relations.handle_seed = seed;
    s.paper_space_viewport_scale = main.read_bd()?;
    value.paper_space = read_header_space(&mut main).map_err(|error| format!("paper-space Header geometry: {error}"))?;
    value.model_space = read_header_space(&mut main).map_err(|error| format!("model-space Header geometry: {error}"))?;
    let d = &mut value.dimensions;
    d.scale = main.read_bd()?;
    d.arrow_size = main.read_bd()?;
    d.extension_offset = main.read_bd()?;
    d.line_increment = main.read_bd()?;
    d.extension = main.read_bd()?;
    d.rounding = main.read_bd()?;
    d.line_extension = main.read_bd()?;
    d.tolerance_plus = main.read_bd()?;
    d.tolerance_minus = main.read_bd()?;
    d.fixed_extension_length = main.read_bd()?;
    d.jog_angle = main.read_bd()?;
    d.text_fill = main.read_bs()?;
    d.text_fill_color_index = read_header_color(&mut main, 0xc1000000, "DIMTFILLCLR")?;
    d.tolerance = main.read_b()?;
    d.limits = main.read_b()?;
    d.text_inside_horizontal = main.read_b()?;
    d.text_outside_horizontal = main.read_b()?;
    d.suppress_extension1 = main.read_b()?;
    d.suppress_extension2 = main.read_b()?;
    d.text_above = main.read_bs()?;
    d.zero_suppression = main.read_bs()?;
    d.angular_zero_suppression = main.read_bs()?;
    d.arc_symbol = main.read_bs()?;
    d.text_height = main.read_bd()?;
    d.center_mark = main.read_bd()?;
    d.tick_size = main.read_bd()?;
    d.alternate_scale = main.read_bd()?;
    d.linear_factor = main.read_bd()?;
    d.text_vertical_position = main.read_bd()?;
    d.text_factor = main.read_bd()?;
    d.gap = main.read_bd()?;
    d.alternate_rounding = main.read_bd()?;
    d.alternate_units = main.read_b()?;
    d.alternate_decimal_places = main.read_bs()?;
    d.text_outside_force_line = main.read_b()?;
    d.separate_arrows = main.read_b()?;
    d.text_inside = main.read_b()?;
    d.suppress_outside = main.read_b()?;
    d.line_color_index = read_header_color(&mut main, 0xc1000000, "DIMCLRD")?;
    d.extension_color_index = read_header_color(&mut main, 0xc1000000, "DIMCLRE")?;
    d.text_color_index = read_header_color(&mut main, 0xc1000000, "DIMCLRT")?;
    d.angular_decimal_places = main.read_bs()?;
    d.decimal_places = main.read_bs()?;
    d.tolerance_decimal_places = main.read_bs()?;
    d.alternate_units_format = main.read_bs()?;
    d.alternate_tolerance_decimal_places = main.read_bs()?;
    d.angular_unit_format = main.read_bs()?;
    d.fractional_format = main.read_bs()?;
    d.linear_unit_format = main.read_bs()?;
    d.decimal_separator = main.read_bs()?;
    d.text_movement = main.read_bs()?;
    d.justification = main.read_bs()?;
    d.suppress_dimension1 = main.read_b()?;
    d.suppress_dimension2 = main.read_b()?;
    d.tolerance_justification = main.read_bs()?;
    d.tolerance_zero_suppression = main.read_bs()?;
    d.alternate_zero_suppression = main.read_bs()?;
    d.alternate_tolerance_zero_suppression = main.read_bs()?;
    d.user_positioned_text = main.read_b()?;
    d.fit = main.read_bs()?;
    d.fixed_extension_enabled = main.read_b()?;
    d.text_direction = main.read_b()?;
    d.alternate_measurement_scale = main.read_bd()?;
    d.measurement_scale = main.read_bd()?;
    d.dimension_line_weight = main.read_bs()? as i16;
    d.extension_line_weight = main.read_bs()? as i16;
    let p = &mut value.policy;
    p.text_stack_alignment = main.read_bs()?;
    p.text_stack_size = main.read_bs()?;
    if main.read_bl()? != 0x2a1d {
        return Err("unsupported AC1024 Header packed drawing policy".into());
    }
    p.current_entity_lineweight = -1;
    p.end_caps = 0;
    p.join_style = 0;
    p.lineweight_display = false;
    p.external_reference_editing = true;
    p.extended_names = true;
    p.plot_style_mode = true;
    p.ole_startup = false;
    p.insertion_units = main.read_bs()?;
    p.current_plot_style_type = main.read_bs()?;
    p.sort_entities = main.read_rc()?;
    p.index_control = main.read_rc()?;
    p.hide_text = main.read_rc()?;
    p.xclip_frame = main.read_rc()?;
    p.dimension_association = main.read_rc()?;
    p.halo_gap = main.read_rc()?;
    p.obscured_color = main.read_bs()?;
    p.intersection_color = main.read_bs()?;
    p.obscured_linetype = main.read_rc()?;
    p.intersection_display = main.read_rc()?;
    p.camera_display = main.read_b()?;
    if [main.read_bl()?, main.read_bl()?] != [0, 10] || main.read_bd()? != 1.0 {
        return Err("unsupported AC1024 Header render profile".into());
    }
    p.steps_per_second = main.read_bd()?;
    p.step_size = main.read_bd()?;
    p.dwf_3d_precision = main.read_bd()?;
    p.lens_length = main.read_bd()?;
    p.camera_height = main.read_bd()?;
    p.solid_history = main.read_rc()?;
    p.show_history = main.read_rc()?;
    p.polysolid_width = main.read_bd()?;
    p.polysolid_height = main.read_bd()?;
    p.loft_angle1 = main.read_bd()?;
    p.loft_angle2 = main.read_bd()?;
    p.loft_magnitude1 = main.read_bd()?;
    p.loft_magnitude2 = main.read_bd()?;
    p.loft_parameter = main.read_bs()?;
    p.loft_normals = main.read_rc()?;
    p.latitude = main.read_bd()?;
    p.longitude = main.read_bd()?;
    p.north_direction = main.read_bd()?;
    p.timezone = main.read_bl()? as i32;
    p.light_glyph_display = main.read_rc()?;
    p.tile_mode_light_sync = main.read_rc()?;
    p.dwf_frame = main.read_rc()?;
    p.dgn_frame = main.read_rc()?;
    p.real_world_scale = main.read_b()?;
    if read_header_color(&mut main, 0xc3000001, "INTERFERECOLOR")? != 0 {
        return Err("unsupported AC1024 INTERFERECOLOR index".into());
    }
    p.interfere_color_index = 256;
    p.shadow_mode = main.read_rc()?;
    p.shadow_plane_location = main.read_bd()?;
    let strings_start = main.bit_position();
    let mut strings = DwgBitReader::at_bit(stream, strings_start)?;
    u.unit1_name = strings.read_tu()?;
    u.unit2_name = strings.read_tu()?;
    u.unit3_name = strings.read_tu()?;
    u.unit4_name = strings.read_tu()?;
    value.strings.menu = strings.read_tu()?;
    value.strings.dimension_postfix = strings.read_tu()?;
    value.strings.dimension_alternate_postfix = strings.read_tu()?;
    value.strings.dimension_alternate_measurement_zero_suffix = strings.read_tu()?;
    value.strings.dimension_measurement_zero_suffix = strings.read_tu()?;
    value.strings.hyperlink_base = strings.read_tu()?;
    value.strings.stylesheet = strings.read_tu()?;
    value.strings.fingerprint_guid = strings.read_tu()?;
    value.strings.version_guid = strings.read_tu()?;
    value.strings.project_name = strings.read_tu()?;
    let string_bits = strings.bit_position() - strings_start;
    if usize::from(strings.read_rs()?) != string_bits || !strings.read_b()? || strings.bit_position() != boundary {
        return Err(format!("AC1024 Header string footer does not close the {string_bits}-bit string stream at bit {boundary}"));
    }
    let mut handles = DwgBitReader::at_bit(stream, boundary)?;
    let r = &mut value.relations;
    r.current_layer = read_header_relation(&mut handles, "CLAYER")?;
    r.text_style = read_header_relation(&mut handles, "TEXTSTYLE")?;
    r.current_linetype = read_header_relation(&mut handles, "CELTYPE")?;
    r.current_material = read_header_relation(&mut handles, "CMATERIAL")?;
    r.dimension_style = read_header_relation(&mut handles, "DIMSTYLE")?;
    r.multiline_style = read_header_relation(&mut handles, "CMLSTYLE")?;
    r.paper_ucs_name = read_optional_header_relation(&mut handles, "PUCSNAME")?;
    r.paper_ucs_orthographic_reference = read_optional_header_relation(&mut handles, "PUCSORTHOREF")?;
    r.paper_ucs_base = read_optional_header_relation(&mut handles, "PUCSBASE")?;
    r.model_ucs_name = read_optional_header_relation(&mut handles, "UCSNAME")?;
    r.model_ucs_orthographic_reference = read_optional_header_relation(&mut handles, "UCSORTHOREF")?;
    r.model_ucs_base = read_optional_header_relation(&mut handles, "UCSBASE")?;
    r.dimension_text_style = read_header_relation(&mut handles, "DIMTXSTY")?;
    r.dimension_leader_block = read_optional_header_relation(&mut handles, "DIMLDRBLK")?;
    r.dimension_block = read_optional_header_relation(&mut handles, "DIMBLK")?;
    r.dimension_block1 = read_optional_header_relation(&mut handles, "DIMBLK1")?;
    r.dimension_block2 = read_optional_header_relation(&mut handles, "DIMBLK2")?;
    r.dimension_linetype = read_optional_header_relation(&mut handles, "DIMLTYPE")?;
    r.dimension_extension_linetype1 = read_optional_header_relation(&mut handles, "DIMLTEX1")?;
    r.dimension_extension_linetype2 = read_optional_header_relation(&mut handles, "DIMLTEX2")?;
    macro_rules! relation {
        ($field:ident) => {
            r.$field = read_header_relation(&mut handles, stringify!($field))?;
        };
    }
    relation!(block_control);
    relation!(layer_control);
    relation!(style_control);
    relation!(linetype_control);
    relation!(view_control);
    relation!(ucs_control);
    relation!(viewport_control);
    relation!(appid_control);
    relation!(dimension_style_control);
    relation!(group_dictionary);
    relation!(multiline_style_dictionary);
    relation!(named_objects_dictionary);
    relation!(layout_dictionary);
    relation!(plot_settings_dictionary);
    relation!(plot_style_name_dictionary);
    relation!(material_dictionary);
    relation!(color_dictionary);
    relation!(visual_style_dictionary);
    relation!(paper_space_block_record);
    relation!(model_space_block_record);
    relation!(by_layer_linetype);
    relation!(by_block_linetype);
    relation!(continuous_linetype);
    r.interfere_object_visual_style = read_optional_header_relation(&mut handles, "INTERFEREOBJVS")?;
    r.interfere_viewport_visual_style = read_optional_header_relation(&mut handles, "INTERFEREVPVS")?;
    r.drag_visual_style = read_optional_header_relation(&mut handles, "DRAGVS")?;
    for expected in [0xbfc4, 0x122d, 0xa23e, 0xb717] {
        let actual = handles.read_bs()?;
        if actual != expected {
            return Err(format!("unsupported AC1024 Header producer compatibility value {actual:#x}"));
        }
    }
    if handles.read_bl()? != 0 || handles.read_bl()? != 0 || !handles.read_b()? || !(handles.read_b()? && handles.read_b()? && handles.read_b()?) || handles.bit_position() != stream.len() * 8 {
        return Err("unsupported AC1024 Header terminal framing".into());
    }
    Ok(value)
}
/// 📑️ The typed R2004 document sections a round trip reads back: header, dependencies, summary, application, template, revisions and preview.
pub struct DwgDocumentSections {
    pub header: semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderVariables,
    pub dependencies: Vec<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgDependency>,
    pub summary: semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgSummaryInfo,
    pub application: semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationInfo,
    pub template: semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgTemplate,
    pub auxiliary_header: semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgAuxiliaryHeader,
    pub revision_history: semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgRevisionHistory,
    pub preview: semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgIndexedPreview,
    pub application_history: semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationHistory,
}
/// 📑️ Decodes the R2004 document sections (summary, preview, application history) that the round-trip laws read back.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2004_document_sections(bytes: &[u8]) -> Result<DwgDocumentSections, String> {
    let sections = decode_r2004_sections(bytes)?;
    let data = |name: &str| -> Result<Vec<u8>, String> {
        let section = sections.iter().find(|section| section.name == name).ok_or_else(|| format!("R2004 {name} section missing"))?;
        r2004_section_data(section)
    };
    Ok(DwgDocumentSections {
        header: decode_r2010_header_section(&data("AcDb:Header")?).map_err(|error| format!("AcDb:Header: {error}"))?,
        dependencies: decode_dependencies(&data("AcDb:FileDepList")?).map_err(|error| format!("AcDb:FileDepList: {error}"))?,
        summary: decode_summary_info(&data("AcDb:SummaryInfo")?).map_err(|error| format!("AcDb:SummaryInfo: {error}"))?,
        application: decode_application_info(&data("AcDb:AppInfo")?).map_err(|error| format!("AcDb:AppInfo: {error}"))?,
        template: decode_template(&data("AcDb:Template")?).map_err(|error| format!("AcDb:Template: {error}"))?,
        auxiliary_header: decode_auxiliary_header(&data("AcDb:AuxHeader")?).map_err(|error| format!("AcDb:AuxHeader: {error}"))?,
        revision_history: decode_revision_history(&data("AcDb:RevHistory")?).map_err(|error| format!("AcDb:RevHistory: {error}"))?,
        preview: decode_indexed_preview(&data("AcDb:Preview")?).map_err(|error| format!("AcDb:Preview: {error}"))?,
        application_history: decode_application_history(&data("AcDb:AppInfoHistory")?).map_err(|error| format!("AcDb:AppInfoHistory: {error}"))?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2010_classes_section(bytes: &[u8]) -> Result<Vec<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgClass>, String> {
    if bytes.len() < 24 || bytes[..16] != DWG_SENTINEL_CLASSES_BEGIN {
        return Err("R2010 classes section sentinel is missing".into());
    }
    let total_bits = u32::from_le_bytes(bytes[20..24].try_into().unwrap()) as usize;
    let end_bit = 20usize.checked_mul(8).and_then(|value| value.checked_add(total_bits)).ok_or("R2010 classes end overflow")?;
    let (mut strings, data_end_bit) = r2010_string_stream(bytes, end_bit)?;
    let mut data = DwgBitReader::at_bit(bytes, 24 * 8)?;
    let maximum_class = data.read_bl()? as u16;
    if !data.read_b()? {
        return Err("R2010 classes section standard marker is unset".into());
    }
    let expected = maximum_class.saturating_sub(499) as usize;
    let mut classes = Vec::with_capacity(expected);
    while classes.len() < expected && data.bit_position() < data_end_bit {
        let number = data.read_bs()?;
        let proxy_flags = u32::from(data.read_bs()?);
        let application_name = strings.read_tu()?;
        let cpp_class_name = strings.read_tu()?;
        let dxf_name = strings.read_tu()?;
        let was_zombie = data.read_b()?;
        let item_class_id = data.read_bs()?;
        let object_count = data.read_bl()?;
        let dwg_version = data.read_bl()?;
        let maintenance_version = data.read_bl()?;
        let reserved_values = vec![data.read_bl()?, data.read_bl()?];
        classes.push(semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgClass { number, proxy_flags, application_name, cpp_class_name, dxf_name, was_zombie, item_class_id, object_count, dwg_version, maintenance_version, reserved_values });
    }
    Ok(classes)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2004_classes(bytes: &[u8]) -> Result<Vec<semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgClass>, String> {
    let sections = decode_r2004_sections(bytes)?;
    let classes = sections.iter().find(|section| section.name == "AcDb:Classes").ok_or("R2004 Classes section missing")?;
    decode_r2010_classes_section(&r2004_section_data(classes)?)
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
