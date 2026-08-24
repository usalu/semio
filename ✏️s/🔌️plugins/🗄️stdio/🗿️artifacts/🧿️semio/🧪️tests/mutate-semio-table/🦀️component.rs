//! 🦀️ Semio TABLE exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-table-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️table/🧪️oracle/🔣️component.json`): `s.stdio.semio.table` is a semio-NATIVE
//! format with no third-party reader or writer, so `oracle` here reads the committed,
//! independently handcrafted per-kind specification fixtures — declared in `component.feature` as
//! `asset://` references into their own committed leaf directories (`../../🏅️standards/🔖️v1/
//! 🪆️subsets/✳️table/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`) and read through the host's
//! `Context::fixture_json` at run time — literally, no recomputation, no reimplementation of
//! mutation semantics. `subject` drives this repository's own `apply_semio_table_mutation`, the
//! entry point this ticket added, over the full 8-kind `SemioTableMutation` vocabulary. Both sides
//! project the snapshot to structural JSON and `ordered-json-v1` compares them. The oracle-only
//! build must never link the subject crate (fleet brief §5.3), so the subject module below carries
//! its own small, forward-only, hand-written JSON decoder that turns the SAME fixture bytes into
//! real `SemioTableSnapshot`/`SemioTableMutation` values — a mechanical structural decode, never a
//! reimplementation of mutation semantics, and never a hand-transcribed Rust-literal COPY of the
//! fixture that could silently drift from it (`Context::fixture_json` reads the committed file
//! itself, on every run). The generated test-host crate carries no `serde_json` dependency (only
//! `semio-repo-test-host` and, behind `sut`, this subset's own crate), so this hand-written decoder
//! is built on the framework's own dependency-free `protocol::Json`/`Context::fixture_json`, the
//! same type the oracle role uses. The subject half is gated behind the generated host's `sut`
//! feature so the oracle-only run never compiles the local implementation; the Rust SUBJECT phase
//! is blocked this wave by a concurrent os-kernel refactor (see the fleet brief), so it is written
//! and gated but not run.

use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioTableMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["create-column", "delete-column", "rename-column", "reorder-columns", "insert-row", "remove-row", "reorder-rows", "edit-cell"];
//#endregion 🔖️Kinds

//#region 🔖️OracleFixtures
/// 🗂️ `(leaf directory, fixture slug)` per kind — the SAME leaf directories `component.feature`'s
/// own `<dir>`/`<slug>` Examples columns declare as `asset://` fixture references, kept here purely
/// to build the identical URI strings the fixture-resolution contract already validated exist.
fn kind_fixture_dir(kind: &str) -> (&'static str, &'static str) {
    match kind {
        "create-column" => ("🏗️create-column", "appends-a-float-column-and-null-pads-every-row"),
        "delete-column" => ("🗑️delete-column", "drops-the-middle-column-and-cascades-into-every-row"),
        "rename-column" => ("🏷️rename-column", "renames-city-to-town-without-touching-any-row"),
        "reorder-columns" => ("🔀reorder-columns", "moves-the-area-column-to-the-front-and-realigns-every-row"),
        "insert-row" => ("📥insert-row", "inserts-a-row-between-the-two-existing-rows"),
        "remove-row" => ("➖remove-row", "removes-the-leading-row"),
        "reorder-rows" => ("🔃reorder-rows", "moves-the-last-row-to-the-front"),
        "edit-cell" => ("✏️edit-cell", "rewrites-the-population-cell-of-the-second-row"),
        other => panic!("mutate-semio-table: no fixture registered for kind {other:?}"),
    }
}
fn fixture_uri(kind: &str, leaf: &str) -> String {
    let (dir, slug) = kind_fixture_dir(kind);
    format!("asset://🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/{dir}/🧪️tests/{slug}/{leaf}")
}
fn before_uri(kind: &str) -> String {
    fixture_uri(kind, "📸️snapshot/⬅️before/🔣️component.json")
}
fn mutation_uri(kind: &str) -> String {
    fixture_uri(kind, "🦠️mutation/🔣️component.json")
}
fn after_uri(kind: &str) -> String {
    fixture_uri(kind, "📸️snapshot/➡️after/🔣️component.json")
}
//#endregion 🔖️OracleFixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally through the host.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let after = ctx.fixture_json(&after_uri(kind))?;
        let bytes = after.to_string().into_bytes();
        Ok(Outcome::with_raw(bytes, after))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let before = ctx.fixture_json(&before_uri(kind))?;
        let bytes = before.to_string().into_bytes();
        Ok(Outcome::with_raw(bytes, before))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{before_uri, mutation_uri};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::mutations::{create_column, delete_column, edit_cell, insert_row, remove_row, rename_column, reorder_columns, reorder_rows, SemioTableMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    use protocol::Mutation;

    //#region 🔖️Decode
    /// 🧫️ A small, forward-only, hand-written structural decoder — turns the fixture bytes
    /// `Context::fixture_json` reads STRAIGHT from the committed file (never a hand-transcribed
    /// copy) into real `SemioTableSnapshot`/`SemioTableMutation` values. This decodes JSON
    /// STRUCTURE only (field-by-field, mirroring each payload's own declared shape); it never
    /// invents or reimplements any mutation SEMANTICS — those still run through the real
    /// `apply_semio_table_mutation` below.
    fn decode_cell_kind(tag: &str) -> SemioTableCellKind {
        match tag {
            "null" => SemioTableCellKind::Null,
            "bool" => SemioTableCellKind::Bool,
            "int" => SemioTableCellKind::Int,
            "float" => SemioTableCellKind::Float,
            "str" => SemioTableCellKind::Str,
            "bytes" => SemioTableCellKind::Bytes,
            other => panic!("mutate-semio-table: unknown column kind tag {other:?}"),
        }
    }
    fn decode_column(json: &Json) -> SemioTableColumn {
        SemioTableColumn { name: json.str("name"), kind: decode_cell_kind(&json.str("kind")) }
    }
    fn decode_value(json: &Json) -> SemioValue {
        match json.str("kind").as_str() {
            "null" => SemioValue::Null,
            "bool" => SemioValue::Bool { value: matches!(json.get("value"), Some(Json::Bool(true))) },
            "int" => SemioValue::Int { lexeme: json.str("lexeme") },
            "float" => SemioValue::Float { lexeme: json.str("lexeme") },
            "str" => SemioValue::Str { value: json.str("value") },
            "bytes" => SemioValue::Bytes {
                value: json
                    .array("value")
                    .iter()
                    .map(|entry| match entry {
                        Json::Number(n) => *n as u8,
                        other => panic!("mutate-semio-table: expected a byte number, found {other:?}"),
                    })
                    .collect(),
            },
            other => panic!("mutate-semio-table: unknown cell value kind {other:?}"),
        }
    }
    fn decode_row(json: &Json) -> SemioTableRow {
        SemioTableRow { cells: json.array("cells").iter().map(decode_value).collect() }
    }
    fn decode_snapshot(json: &Json) -> SemioTableSnapshot {
        SemioTableSnapshot { schema: json.str("schema"), columns: json.array("columns").iter().map(decode_column).collect(), rows: json.array("rows").iter().map(decode_row).collect() }
    }
    fn usize_field(json: &Json, key: &str) -> usize {
        match json.get(key) {
            Some(Json::Number(n)) => *n as usize,
            other => panic!("mutate-semio-table: expected a numeric field {key:?}, found {other:?}"),
        }
    }
    fn optional_usize_field(json: &Json, key: &str) -> Option<usize> {
        match json.get(key) {
            Some(Json::Number(n)) => Some(*n as usize),
            _ => None,
        }
    }
    /// 🧫️ The committed mutation fixture is serde's default externally-tagged shape
    /// (`{"VariantName": {...fields...}}`) — matches `SemioTableMutation`'s own undecorated
    /// `#[derive(Serialize, Deserialize)]` (no `#[serde(tag = ...)]`) exactly.
    fn decode_mutation(json: &Json) -> SemioTableMutation {
        let (variant, payload) = match json {
            Json::Object(entries) if entries.len() == 1 => (entries[0].0.as_str(), &entries[0].1),
            other => panic!("mutate-semio-table: expected a single-variant mutation object, found {other:?}"),
        };
        match variant {
            "CreateColumn" => SemioTableMutation::CreateColumn(create_column::mutation::CreateColumn { name: payload.str("name"), kind: decode_cell_kind(&payload.str("kind")), index: optional_usize_field(payload, "index") }),
            "DeleteColumn" => SemioTableMutation::DeleteColumn(delete_column::mutation::DeleteColumn { name: payload.str("name") }),
            "RenameColumn" => SemioTableMutation::RenameColumn(rename_column::mutation::RenameColumn { name: payload.str("name"), new_name: payload.str("new_name") }),
            "ReorderColumns" => SemioTableMutation::ReorderColumns(reorder_columns::mutation::ReorderColumns { name: payload.str("name"), to_index: usize_field(payload, "to_index") }),
            "InsertRow" => SemioTableMutation::InsertRow(insert_row::mutation::InsertRow { index: usize_field(payload, "index"), row: decode_row(payload.get("row").expect("mutate-semio-table: InsertRow fixture must carry a row")) }),
            "RemoveRow" => SemioTableMutation::RemoveRow(remove_row::mutation::RemoveRow { index: usize_field(payload, "index") }),
            "ReorderRows" => SemioTableMutation::ReorderRows(reorder_rows::mutation::ReorderRows { from: usize_field(payload, "from"), to: usize_field(payload, "to") }),
            "EditCell" => SemioTableMutation::EditCell(edit_cell::mutation::EditCell { row_index: usize_field(payload, "row_index"), column_name: payload.str("column_name"), new_value: decode_value(payload.get("new_value").expect("mutate-semio-table: EditCell fixture must carry a new_value")) }),
            other => panic!("mutate-semio-table: no decoder for mutation variant {other:?}"),
        }
    }
    //#endregion 🔖️Decode

    //#region 🔖️Fixtures
    /// 🧫️ Reads the SAME committed fixture bytes `../🦀️component.rs`'s oracle role reads, decoded
    /// once into real typed values through `decode_snapshot`/`decode_mutation` above.
    fn fixture_for(kind: &str, ctx: &Context) -> Result<(SemioTableSnapshot, SemioTableMutation), String> {
        let before = decode_snapshot(&ctx.fixture_json(&before_uri(kind))?);
        let mutation = decode_mutation(&ctx.fixture_json(&mutation_uri(kind))?);
        Ok((before, mutation))
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️Projection
    fn cell_kind_str(kind: SemioTableCellKind) -> &'static str {
        match kind {
            SemioTableCellKind::Null => "null",
            SemioTableCellKind::Bool => "bool",
            SemioTableCellKind::Int => "int",
            SemioTableCellKind::Float => "float",
            SemioTableCellKind::Str => "str",
            SemioTableCellKind::Bytes => "bytes",
        }
    }
    fn column_json(column: &SemioTableColumn) -> Json {
        Json::Object(vec![("name".to_string(), Json::String(column.name.clone())), ("kind".to_string(), Json::String(cell_kind_str(column.kind).to_string()))])
    }
    /// 🎯️ Mirrors `SemioValue`'s internally-tagged (`tag = "kind"`) serde shape field for field —
    /// only the scalar variants a table cell can actually hold in this subset's committed fixtures
    /// (`null`/`bool`/`int`/`float`/`str`/`bytes`) are exercised.
    fn value_json(value: &SemioValue) -> Json {
        match value {
            SemioValue::Null => Json::Object(vec![("kind".to_string(), Json::String("null".to_string()))]),
            SemioValue::Bool { value } => Json::Object(vec![("kind".to_string(), Json::String("bool".to_string())), ("value".to_string(), Json::Bool(*value))]),
            SemioValue::Int { lexeme } => Json::Object(vec![("kind".to_string(), Json::String("int".to_string())), ("lexeme".to_string(), Json::String(lexeme.clone()))]),
            SemioValue::Float { lexeme } => Json::Object(vec![("kind".to_string(), Json::String("float".to_string())), ("lexeme".to_string(), Json::String(lexeme.clone()))]),
            SemioValue::Str { value } => Json::Object(vec![("kind".to_string(), Json::String("str".to_string())), ("value".to_string(), Json::String(value.clone()))]),
            SemioValue::Bytes { value } => Json::Object(vec![("kind".to_string(), Json::String("bytes".to_string())), ("value".to_string(), Json::Array(value.iter().map(|b| Json::Number(*b as f64)).collect()))]),
            other => panic!("mutate-semio-table: no JSON projection for non-scalar SemioValue {other:?}"),
        }
    }
    fn row_json(row: &SemioTableRow) -> Json {
        Json::Object(vec![("cells".to_string(), Json::Array(row.cells.iter().map(value_json).collect()))])
    }
    /// 🎯️ The projection every scenario compares under `ordered-json-v1`: the snapshot's own
    /// structural JSON shape, matching the committed fixtures field for field.
    fn snapshot_json(snapshot: &SemioTableSnapshot) -> Json {
        Json::Object(vec![
            ("schema".to_string(), Json::String(snapshot.schema.clone())),
            ("columns".to_string(), Json::Array(snapshot.columns.iter().map(column_json).collect())),
            ("rows".to_string(), Json::Array(snapshot.rows.iter().map(row_json).collect())),
        ])
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (mut base, mutation) = fixture_for(kind, ctx)?;
            let outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::mutations::apply_semio_table_mutation(&mut base, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("mutate-{kind}: mutation rejected: {:?}", outcome.messages()));
            }
            let projection = snapshot_json(&base);
            let bytes = projection.to_string().into_bytes();
            Ok(Outcome::with_raw(bytes, projection))
        }
    }

    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (base, mutation) = fixture_for(kind, ctx)?;
            let mut current = base.clone();
            let outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::mutations::apply_semio_table_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("inverse-{kind}: forward mutation rejected: {:?}", outcome.messages()));
            }
            let undo = mutation.inverse(&base);
            for step in &undo {
                let step_outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::mutations::apply_semio_table_mutation(&mut current, step);
                if !step_outcome.messages().is_empty() {
                    return Err(format!("inverse-{kind}: inverse step rejected: {:?}", step_outcome.messages()));
                }
            }
            let projection = snapshot_json(&current);
            let bytes = projection.to_string().into_bytes();
            Ok(Outcome::with_raw(bytes, projection))
        }
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built
}
//#endregion 🔖️Registration
