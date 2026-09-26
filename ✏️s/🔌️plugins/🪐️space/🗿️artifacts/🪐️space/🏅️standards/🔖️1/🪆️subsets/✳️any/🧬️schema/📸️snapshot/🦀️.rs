//! 🧬️ S Space index snapshot schema — artifact-lane fields only. Ticket
//! 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C4.

use crate::S_SPACE_INDEX_DOCUMENT_SCHEMA;
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Dialect
/// 🪪️ One artifact's coordinate inside a space's index — mirrors the freeze's
/// `dialect { artifactKind, standard, subset }` shape.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceArtifactDialect {
    pub artifact_kind: String,
    pub standard: String,
    pub subset: String,
}
//#endregion 🔖️Dialect

//#region 🔖️Row
/// 📇️ One row of a space's artifact index — persisted metadata only, never the artifact's own
/// document bytes (those live in their own backbone document, addressed by `id`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceArtifactRow {
    pub id: String,
    pub name: String,
    pub kind_id: String,
    pub schema: String,
    #[dsl(block)]
    pub dialect: SpaceArtifactDialect,
    pub created_at_ms: u64,
    pub created_by: String,
    pub updated_at_ms: u64,
    pub updated_by: String,
}
//#endregion 🔖️Row

//#region 🔖️Snapshot
/// 📸️ Persisted S Space index document snapshot — one per hub space, document id `index`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.space.space")]
#[dsl(extension = "sspace")]
#[dsl(layout = "lines")]
pub struct SSpaceSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub space_id: String,
    #[dsl(table)]
    #[state(artifact)]
    pub artifacts: Vec<SpaceArtifactRow>,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ Handcrafted `ArtifactDsl`/`ArtifactPack` — mirrors `SHomeSnapshot`'s own handcrafted pair.
impl store::ArtifactDsl for SSpaceSnapshot {
    const EXTENSION: &'static str = "sspace";
    fn envelope_id() -> &'static str {
        "s.space"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SSpaceSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for SSpaceSnapshot {
    fn default() -> Self {
        Self { schema: S_SPACE_INDEX_DOCUMENT_SCHEMA.into(), space_id: String::new(), artifacts: Vec::new() }
    }
}

//#region 🔖️DocumentHelpers
/// 🆕️ A fresh, empty index for a newly created hub space.
pub fn empty_space_index_snapshot(space_id: &str) -> SSpaceSnapshot {
    SSpaceSnapshot { schema: S_SPACE_INDEX_DOCUMENT_SCHEMA.into(), space_id: space_id.into(), artifacts: Vec::new() }
}

/// 🆔️ Mints a fresh artifact id for `create-artifact` — `handle()` is pure/no-IO, so this derives
/// uniqueness from the mutation's own inputs (creation instant + a collision-probed counter) rather
/// than a random/host-global source. Unique within `existing` (one space's index).
pub fn mint_artifact_id(existing: &[SpaceArtifactRow], now_ms: u64) -> String {
    let mut n = existing.len() as u64;
    loop {
        let candidate = format!("artifact-{now_ms}-{n}");
        if !existing.iter().any(|row| row.id == candidate) {
            return candidate;
        }
        n += 1;
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️TableProjection
semio_framework_plugin::app_labels! {
    /// 🗣️ The space app table's strings (worker-brief task 1: name · id · kind · subset · updated · updated-by ·
    /// presence) — the single source both the editor's and the viewer's `main` window render from (neutral
    /// schema-layer helper so the viewer never has to import from `✏️editor`, `policyViewerPurityBreaches`). The
    /// NAME leads: the table grid names every row and every row button by its first cell ("Öffnen: Werkstattplan"),
    /// and a leading artifact id read "Open: artifact-5bd9…" to a screen reader (ticket 26/09/23 S16); the row's
    /// identity is its key (`artifact:<id>`), never a cell.
    pub struct SpaceIndexTableLabels {
        table_name: native_en "Artifacts", native_de "Artefakte", reuse_en "Artifacts", reuse_de "Artefakte";
        column_id: native_en "ID", native_de "ID", reuse_en "ID", reuse_de "ID";
        column_name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        column_kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        column_subset: native_en "Subset", native_de "Teilmenge", reuse_en "Subset", reuse_de "Teilmenge";
        column_updated: native_en "Updated", native_de "Aktualisiert", reuse_en "Updated", reuse_de "Aktualisiert";
        column_updated_by: native_en "Updated By", native_de "Aktualisiert von", reuse_en "Updated By", reuse_de "Aktualisiert von";
        column_presence: native_en "Presence", native_de "Anwesenheit", reuse_en "Presence", reuse_de "Anwesenheit";
        column_actions: native_en "Actions", native_de "Aktionen", reuse_en "Actions", reuse_de "Aktionen";
        action_open: native_en "Open", native_de "Öffnen", reuse_en "Open", reuse_de "Öffnen";
        create_artifact: native_en "Create Artifact", native_de "Artefakt erstellen", reuse_en "Create Artifact", reuse_de "Artefakt erstellen";
    }
}

impl SpaceIndexTableLabels {
    /// 📊️ The seven column headers, in cell order.
    pub fn columns(&self) -> [&str; 7] {
        [self.column_name.as_str(), self.column_id.as_str(), self.column_kind.as_str(), self.column_subset.as_str(), self.column_updated.as_str(), self.column_updated_by.as_str(), self.column_presence.as_str()]
    }

    /// 📊️ One table row for `row` in these labels' language; `presence` is a display-ready summary (empty
    /// when the caller has no live presence data, e.g. the viewer, which folds no presence of its own).
    pub fn row(&self, row: &SpaceArtifactRow, presence: &str) -> [String; 7] {
        [row.name.clone(), row.id.clone(), row.kind_id.clone(), row.dialect.subset.clone(), utc_minute_text(row.updated_at_ms, self.locale), row.updated_by.clone(), presence.into()]
    }
}

/// 🕰️ One UTC instant to the minute, in the order and punctuation of `locale`: `2026-09-25 21:05 UTC` (en) and
/// `25.09.2026, 21:05 UTC` (de) — the Home and space tables' "Updated" cell. The guest has no time zone, so it
/// says UTC instead of pretending to be local. Cases: `🧫️fixtures/🕰️utc-minute/🔣️.json` beside this module,
/// cross-checked against `Intl.DateTimeFormat` by its TS unit test.
pub fn utc_minute_text(epoch_ms: u64, locale: semio_framework_plugin::Locale) -> String {
    let minutes = epoch_ms / 60_000;
    let (hour, minute) = ((minutes / 60) % 24, minutes % 60);
    let days = (minutes / 1_440) as i64 + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days.rem_euclid(146_097);
    let year_of_era = (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 { month_index + 3 } else { month_index - 9 };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    match locale {
        semio_framework_plugin::Locale::En => format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02} UTC"),
        semio_framework_plugin::Locale::De => format!("{day:02}.{month:02}.{year:04}, {hour:02}:{minute:02} UTC"),
    }
}

/// 🕰️ Epoch milliseconds (whole seconds) of a local catalog entry's RFC 3339 UTC `saved_at`
/// (`2026-09-25T21:05:00Z`, optional `.fraction`); `None` for the never-saved `"0"` and for anything else.
pub fn rfc3339_utc_epoch_ms(value: &str) -> Option<u64> {
    let bytes = value.as_bytes();
    let fraction = bytes.get(19..bytes.len().checked_sub(1)?)?;
    let digits = |range: std::ops::Range<usize>| bytes.get(range.clone()).filter(|part| part.iter().all(u8::is_ascii_digit)).and_then(|_| value.get(range)?.parse::<i64>().ok());
    if bytes.len() < 20 || bytes[4] != b'-' || bytes[7] != b'-' || bytes[10] != b'T' || bytes[13] != b':' || bytes[16] != b':' || bytes.last() != Some(&b'Z') {
        return None;
    }
    if !(fraction.is_empty() || (fraction.len() > 1 && fraction[0] == b'.' && fraction[1..].iter().all(u8::is_ascii_digit))) {
        return None;
    }
    let (year, month, day, hour, minute, second) = (digits(0..4)?, digits(5..7)?, digits(8..10)?, digits(11..13)?, digits(14..16)?, digits(17..19)?);
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) || hour > 23 || minute > 59 || second > 60 {
        return None;
    }
    let shifted = if month <= 2 { year - 1 } else { year };
    let era = shifted.div_euclid(400);
    let year_of_era = shifted.rem_euclid(400);
    let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = era * 146_097 + day_of_era - 719_468;
    u64::try_from(((days * 24 + hour) * 60 + minute) * 60 + second).ok().map(|seconds| seconds * 1_000)
}
//#endregion 🔖️TableProjection

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🌉️IdentityBridge
/// 🔁️ One JSON report of carrying `dsl_text` through this subset's own codecs, for a
/// language-neutral test adapter. Same reachability wall as `s_space_mutation_report_json`:
/// `store::ArtifactDsl`/`store::ArtifactPack` and their error types are unnameable outside this
/// crate, so the identity law's evidence has to be produced here and handed over as text.
///
/// `canonicalText` is `print_dsl` of the parsed document and `canonicalTextAgain` is `print_dsl` of
/// re-parsing that — [`store::ArtifactDsl`]'s own documented LAW is that canonical output is a
/// `parse_dsl` fixpoint (hand-written text may normalize on the way in), so the two must be
/// byte-identical while neither is required to equal the committed file. `packDecoded` comes back
/// through a SEPARATE binary codec, so agreeing on one snapshot cannot be achieved by carrying text
/// bytes across.
pub fn s_space_identity_report_json(dsl_text: &str) -> Result<String, String> {
    let parsed = <SSpaceSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).map_err(|error| error.to_string())?;
    let canonical = <SSpaceSnapshot as store::ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <SSpaceSnapshot as store::ArtifactDsl>::parse_dsl(&canonical).map_err(|error| error.to_string())?;
    let canonical_again = <SSpaceSnapshot as store::ArtifactDsl>::print_dsl(&reparsed);
    let packed = <SSpaceSnapshot as store::ArtifactPack>::encode_pack(&reparsed);
    let unpacked = <SSpaceSnapshot as store::ArtifactPack>::decode_pack(&packed).map_err(|error| error.to_string())?;
    let report = pack::json!({
        "parsed": pack::json_from_dsl_value(&dsl::ToValue::to_value(&parsed)),
        "reparsed": pack::json_from_dsl_value(&dsl::ToValue::to_value(&reparsed)),
        "packDecoded": pack::json_from_dsl_value(&dsl::ToValue::to_value(&unpacked)),
        "canonicalText": canonical,
        "canonicalTextAgain": canonical_again,
    });
    Ok(report.to_string())
}
//#endregion 🌉️IdentityBridge
