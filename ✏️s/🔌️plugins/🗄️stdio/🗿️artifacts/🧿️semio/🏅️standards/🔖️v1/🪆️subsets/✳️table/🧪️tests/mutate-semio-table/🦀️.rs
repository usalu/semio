//! 🦀️ Semio TABLE exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! **This file no longer serves the oracle role.** The reference for `semio-v1-table-mutate` is the
//! registered oracle `semio-table-python-independent` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️table/
//! 🔣️oracle.json`) — an independent Python implementation of the semio table carrier,
//! its `SemioValue` cell grammar and its eight verbs, written from the committed grammar, protocol
//! and JSON-schema documents, living beside this file as `🐍️component.py`. The runner dispatches the
//! oracle role to that adapter and the subject role here, and compares the two projections under
//! `@comparison-ordered-json-v1`. Registering oracle handlers here as well would put this
//! repository's own answer on both sides of that comparison, which is the precise failure the
//! platform exists to prevent.
//!
//! **What the handlers assert in role.** Parity across the two implementations is the primary
//! evidence, but each side still states its own law so a scenario can fail for the right reason with
//! a readable message: `inverse-<kind>` requires the mutation's OWN computed inverse to restore the
//! survey table, `spec-vector-<kind>` requires the applied snapshot to be the committed
//! after-snapshot, `payload-fidelity` requires the derived document to still carry exactly what this
//! repository's own RFC 4180 reader finds in the committed CSV, and `identity-round-trip` requires
//! all four committed encodings to be reproduced byte for byte through `law::carrier_is_exact`.
//!
//! **How the fixtures reach typed values.** The generated test host links only `semio-repo-test-host`
//! and, behind `sut`, this subset's own crate — no `serde`, no `serde_json`, and this crate's
//! `protocol`/`store` extern-crate aliases are private (`🦀️.rs`) — so the subset's own production
//! code exports the bridges this adapter needs, whose signatures name only reachable types:
//! `decode_semio_table_snapshot_json`/`encode_semio_table_snapshot_json`,
//! `decode_semio_table_mutation_json`/`inverse_semio_table_mutation`, and the DSL/pack
//! pass-throughs. Every input is read from a fixture the FEATURE declares — the mutation parameters
//! from the scenario's doc string, the specification vectors from the `asset://` URIs its steps name
//! — so neither adapter holds a transcription that could drift away from what the other one read.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioTableMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/
/// 🧬️mutations/🦀️.rs`) — duplicated, not imported, because the generated host builds this
/// file with and without the subject crate. The contract's mutation-coverage gate keeps this list
/// honest against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps
/// it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &["create-column", "delete-column", "rename-column", "reorder-columns", "insert-row", "remove-row", "reorder-rows", "edit-cell"];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::csv::standards::v_rfc4180::subsets::any::schema::snapshot::decode_csv;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::mutations::{apply_semio_table_mutation, decode_semio_table_mutation_json, inverse_semio_table_mutation, SemioTableMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{
        decode_semio_table_pack, decode_semio_table_snapshot_json, encode_semio_table_pack, encode_semio_table_snapshot_json, parse_semio_table_dsl, print_semio_table_dsl, SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot,
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;

    //#region 🔖️Input
    /// 📃️ The three-row demo sheet, in both encodings the domain commits for it — small, but the
    /// only `s.stdio.semio.table` bytes in this artifact a codec other than the Python one wrote.
    const SHEET_DSL: &str = "asset://📚️examples/📃️sheet/🖼️assets/🗣️.dsl.semio";
    const SHEET_PACK: &str = "asset://📚️examples/📃️sheet/🖼️assets/🎒️.pack.semio";
    /// 📊️ The real 50×12 survey table and its binary twin, derived once from the committed CSV
    /// beside them and re-derived on every run by `payload-fidelity`.
    const SURVEY_CSV: &str = "local://📊️reuse-marketplaces.csv";
    const SURVEY_DSL: &str = "local://📊️reuse-marketplaces.dsl.semio";
    const SURVEY_PACK: &str = "local://📊️reuse-marketplaces.pack.semio";

    fn utf8(bytes: Vec<u8>, what: &str) -> Result<String, String> {
        String::from_utf8(bytes).map_err(|error| format!("{what} is not UTF-8: {error}"))
    }

    /// 📊️ The real survey table, parsed through this repository's own DSL codec.
    fn survey(ctx: &Context) -> Result<SemioTableSnapshot, String> {
        parse_semio_table_dsl(&utf8(ctx.fixture_bytes(SURVEY_DSL)?, "the committed survey table")?)
    }

    /// 📜️ The scenario's own committed mutation parameters — the feature owns the vector.
    fn mutation(ctx: &Context) -> Result<SemioTableMutation, String> {
        decode_semio_table_mutation_json(ctx.doc_string()?).map_err(|error| format!("{}: the scenario's mutation payload must decode: {error}", ctx.scenario.id))
    }

    /// 🧫️ Every `asset://` URI the scenario's steps name, in step order. The feature is the single
    /// place the specification-vector paths are written down; both adapters read them from there.
    fn step_assets(ctx: &Context) -> Vec<String> {
        let mut found = Vec::new();
        for (_, text) in &ctx.scenario.steps {
            let mut rest = text.as_str();
            while let Some(at) = rest.find("asset://") {
                let tail = &rest[at..];
                let end = tail.find(char::is_whitespace).unwrap_or(tail.len());
                found.push(tail[..end].to_string());
                rest = &tail[end..];
            }
        }
        found
    }

    fn vector(ctx: &Context, position: usize, label: &str) -> Result<String, String> {
        let uri = step_assets(ctx).into_iter().nth(position).ok_or_else(|| format!("{}: the scenario names no {label} asset", ctx.scenario.id))?;
        utf8(ctx.fixture_bytes(&uri)?, &uri)
    }

    fn apply(current: &mut SemioTableSnapshot, step: &SemioTableMutation, what: &str) -> Result<(), String> {
        let outcome = apply_semio_table_mutation(current, step);
        let refusals = semio_mutation_refusals(&outcome);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{what}: the mutation was rejected: {refusals:?}"))
    }

    fn projection(snapshot: &SemioTableSnapshot) -> Result<Json, String> {
        parse_json(&encode_semio_table_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON both sides project, so a red
    /// scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioTableSnapshot, expected: &SemioTableSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_semio_table_snapshot_json(got), encode_semio_table_snapshot_json(expected))
    }
    //#endregion 🔖️Input

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real 50-row survey table by this repository's codec alone.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut current = survey(ctx)?;
        apply(&mut current, &mutation(ctx)?, &ctx.scenario.id)?;
        let projection = projection(&current)?;
        Ok(Outcome::with_raw(print_semio_table_dsl(&current).into_bytes(), projection))
    }

    /// ↩️ The metamorphic inverse law on the real survey: applying the verb and then its OWN computed
    /// inverse must restore the table exactly — a deleted column's POSITION and every one of the
    /// fifty cells it took with it included, not merely a column of the same name reappearing.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = survey(ctx)?;
        let step = mutation(ctx)?;
        let mut current = base.clone();
        apply(&mut current, &step, &ctx.scenario.id)?;
        let mutated = projection(&current)?;
        for undo in inverse_semio_table_mutation(&step, &base) {
            apply(&mut current, &undo, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the survey table", ctx.scenario.id), &current, &base));
        }
        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), projection(&current)?)])))
    }

    /// 🧫️ The same verb on its committed handcrafted `(before, mutation, after)` vector — a THIRD
    /// statement of what the verb means, independent of both implementations.
    pub fn spec_vector(ctx: &Context) -> Result<Outcome, String> {
        let mut current = decode_semio_table_snapshot_json(&vector(ctx, 0, "before-snapshot")?)?;
        let step = decode_semio_table_mutation_json(&vector(ctx, 1, "mutation")?)?;
        let expected = decode_semio_table_snapshot_json(&vector(ctx, 2, "after-snapshot")?)?;
        apply(&mut current, &step, &ctx.scenario.id)?;
        if current != expected {
            return Err(disagreement(&format!("{}: the applied table does not match the committed after-snapshot", ctx.scenario.id), &current, &expected));
        }
        Ok(Outcome::projection(projection(&current)?))
    }

    /// 📊️ The derived fixture against the real CSV it came from, re-read on every run by THIS
    /// repository's own RFC 4180 codec while the oracle re-reads it with Python's `csv` module. The
    /// derivation is a faithful transcription: the header record names the columns, every column is
    /// `Str` because every source field is text, and every cell carries its field verbatim.
    pub fn payload_fidelity(ctx: &Context) -> Result<Outcome, String> {
        let source = decode_csv(&utf8(ctx.fixture_bytes(SURVEY_CSV)?, "the committed survey source")?)?;
        let (header, records) = source.records.split_first().ok_or_else(|| "payload-fidelity: the committed survey source carries no header record".to_string())?;
        let derived = SemioTableSnapshot {
            schema: survey(ctx)?.schema.clone(),
            columns: header.fields.iter().map(|field| SemioTableColumn { name: field.value.clone(), kind: SemioTableCellKind::Str }).collect(),
            rows: records.iter().map(|record| SemioTableRow { cells: record.fields.iter().map(|field| SemioValue::Str { value: field.value.clone() }).collect() }).collect(),
        };
        for (at, row) in derived.rows.iter().enumerate() {
            if row.cells.len() != derived.columns.len() {
                return Err(format!("payload-fidelity: record {at} carries {} field(s), the header declares {}", row.cells.len(), derived.columns.len()));
            }
        }
        let committed = survey(ctx)?;
        if derived != committed {
            return Err(disagreement("payload-fidelity: the committed survey document no longer matches the CSV it was derived from", &derived, &committed));
        }
        let cells = derived.rows.iter().map(|row| row.cells.len()).sum::<usize>();
        Ok(Outcome::projection(Json::Object(vec![
            ("document".to_string(), projection(&derived)?),
            ("columns".to_string(), Json::Number(derived.columns.len() as f64)),
            ("rows".to_string(), Json::Number(derived.rows.len() as f64)),
            ("cells".to_string(), Json::Number(cells as f64)),
        ])))
    }

    /// 🔁️ All four committed encodings — the demo sheet's two and the survey table's two — each
    /// re-emitted from the parsed document.
    ///
    /// 🔒️ **The byte half of the identity law, asserted as `carrier_is_exact` and asserted in both
    /// directions.** `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary
    /// twin, so reproducing them BYTE FOR BYTE is the correct answer here and `law::reparsed_not_copied`
    /// would be exactly backwards — the same reading `mutate-dag-1` records for `.dag.dsl.semio`. It
    /// fails with the offset of the first differing byte the moment the printer or the packer drifts.
    /// Nor is it a self-comparison: the demo sheet's bytes were written by THIS codec and the Python
    /// oracle reproduces them from the grammar alone, while the survey table's bytes were written by
    /// the PYTHON implementation and this codec has to reproduce THOSE — each side is measured
    /// against bytes the other one emitted, and the runner compares the digests.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let sheet_dsl = ctx.fixture_bytes(SHEET_DSL)?;
        let sheet = parse_semio_table_dsl(&utf8(sheet_dsl.clone(), "the committed demo sheet")?)?;
        let sheet_printed = print_semio_table_dsl(&sheet);
        carrier_is_exact(sheet_printed.as_bytes(), &sheet_dsl)?;
        let sheet_pack = ctx.fixture_bytes(SHEET_PACK)?;
        let sheet_unpacked = decode_semio_table_pack(&sheet_pack)?;
        if sheet_unpacked != sheet {
            return Err(disagreement("identity-round-trip: the demo sheet's binary twin decodes to a different table than its text", &sheet_unpacked, &sheet));
        }
        let sheet_repacked = encode_semio_table_pack(&sheet);
        carrier_is_exact(&sheet_repacked, &sheet_pack)?;
        let survey_dsl = ctx.fixture_bytes(SURVEY_DSL)?;
        let table = parse_semio_table_dsl(&utf8(survey_dsl.clone(), "the committed survey table")?)?;
        let survey_printed = print_semio_table_dsl(&table);
        carrier_is_exact(survey_printed.as_bytes(), &survey_dsl)?;
        let reparsed = parse_semio_table_dsl(&survey_printed)?;
        if reparsed != table {
            return Err(disagreement("identity-round-trip: printing the survey table back to DSL and reparsing it lost content", &reparsed, &table));
        }
        let survey_pack = ctx.fixture_bytes(SURVEY_PACK)?;
        let survey_unpacked = decode_semio_table_pack(&survey_pack)?;
        if survey_unpacked != table {
            return Err(disagreement("identity-round-trip: the survey table's binary twin decodes to a different table than its text", &survey_unpacked, &table));
        }
        let survey_repacked = encode_semio_table_pack(&table);
        carrier_is_exact(&survey_repacked, &survey_pack)?;
        Ok(Outcome::projection(Json::Object(vec![
            ("sheet".to_string(), projection(&sheet)?),
            ("sheetDslDigest".to_string(), Json::String(digest(sheet_printed.as_bytes()))),
            ("sheetPackDigest".to_string(), Json::String(digest(&sheet_repacked))),
            ("surveyDslDigest".to_string(), Json::String(digest(survey_printed.as_bytes()))),
            ("surveyPackDigest".to_string(), Json::String(digest(&survey_repacked))),
            ("surveyDslLength".to_string(), Json::Number(survey_printed.len() as f64)),
            ("surveyPackLength".to_string(), Json::Number(survey_repacked.len() as f64)),
        ])))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. Only subject handlers are
/// registered: the oracle role belongs to `🐍️component.py`.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built
                .subject(&format!("mutate-{kind}"), subject::mutate)
                .subject(&format!("inverse-{kind}"), subject::inverse)
                .subject(&format!("spec-vector-{kind}"), subject::spec_vector);
        }
        built = built.subject("payload-fidelity", subject::payload_fidelity).subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
