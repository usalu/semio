//! 📤️ `SemioValueToCsv` — mirror of `SemioValueFromCsv`. `root` must be a `List` of rows shaped
//! consistently with the FIRST row (all `Map`s sharing that Map's exact key set/order -> a
//! header'd table; all `List`s -> a headerless table) — a real, honest constraint CSV's own flat
//! grid imposes, never silently patched over:
//! - Rows are read in strict lock-step with the first row's shape; a Map row with a different key
//!   SET or ORDER than the first row, or a mix of Map/List rows, is a hard `PackError` (RFC 4180
//!   has exactly one column set per file — there is no honest per-row header to fall back to).
//! - A cell value must be a SCALAR (`Str`/`Int`/`Float`/`Bool`/`Null`) — `Int`/`Float` lexemes and
//!   `Bool` (`"true"`/`"false"`) are stringified verbatim (a faithful text rendering, not a
//!   fabrication), `Null` becomes an empty field. `List`/`Map`/`Bytes` nested inside a cell have no
//!   flat-grid representation and are a hard error, never silently flattened or dropped.
//! - `Ref{id}` is dereferenced the same way the json/xml serializers do; dangling refs and cycles
//!   are hard errors.
//! - Every regenerated `CsvField.quoted` is `false` — RFC 4180 §2 rule 6 (structural necessity:
//!   embedded comma/quote/newline) still forces re-quoting where needed at the engine's OWN
//!   `encode_csv` layer; only the "quoted even though not structurally required" bit the
//!   deserializer never captured in the first place is what's absent here.

use crate::artifacts::csv::schema::snapshot::{CsvField, CsvRecord};
use crate::artifacts::csv::CsvSnapshot;
use crate::artifacts::csv::STDIO_CSV_DOCUMENT_SCHEMA;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueSnapshot, ValueId};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use std::collections::{HashMap, HashSet};

//#region 🔖️Serializer
pub struct SemioValueToCsv;

impl ArtifactSerializer for SemioValueToCsv {
    type From = SemioValueSnapshot;
    type Into = CsvSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("value") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let nodes: HashMap<&ValueId, &SemioValue> = from.nodes.iter().map(|n| (&n.id, &n.value)).collect();
        let mut visiting: HashSet<ValueId> = HashSet::new();
        csv_from_semio(&from.root, &nodes, &mut visiting)
    }
}

pub fn register() {}
//#endregion 🔖️Serializer

//#region 🔖️Convert
fn err(msg: impl Into<String>) -> store::PackError {
    store::PackError::Schema(msg.into())
}

fn resolve(v: &SemioValue, nodes: &HashMap<&ValueId, &SemioValue>, visiting: &mut HashSet<ValueId>) -> Result<SemioValue, store::PackError> {
    match v {
        SemioValue::Ref { id } => {
            if !visiting.insert(id.clone()) {
                return Err(err(format!("value->csv: reference cycle detected at id {:?}", id.value)));
            }
            let target = *nodes.get(id).ok_or_else(|| err(format!("value->csv: dangling Ref{{id: {:?}}} — not found in `nodes`", id.value)))?;
            let result = resolve(target, nodes, visiting);
            visiting.remove(id);
            result
        }
        other => Ok(other.clone()),
    }
}

fn scalar_to_field(v: &SemioValue) -> Result<CsvField, store::PackError> {
    match v {
        SemioValue::Str { value } => Ok(CsvField { value: value.clone(), quoted: false }),
        SemioValue::Int { lexeme } | SemioValue::Float { lexeme } => Ok(CsvField { value: lexeme.clone(), quoted: false }),
        SemioValue::Bool { value } => Ok(CsvField { value: if *value { "true" } else { "false" }.into(), quoted: false }),
        SemioValue::Null => Ok(CsvField { value: String::new(), quoted: false }),
        other => Err(err(format!("value->csv: cell value must be a scalar (Str/Int/Float/Bool/Null), got {other:?}"))),
    }
}

pub fn csv_from_semio(root: &SemioValue, nodes: &HashMap<&ValueId, &SemioValue>, visiting: &mut HashSet<ValueId>) -> Result<CsvSnapshot, store::PackError> {
    let resolved_root = resolve(root, nodes, visiting)?;
    let items = match resolved_root {
        SemioValue::List { items } => items,
        other => return Err(err(format!("value->csv: root must be a List of rows, got {other:?}"))),
    };
    if items.is_empty() {
        return Ok(CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records: Vec::new() });
    }

    let first = resolve(&items[0], nodes, visiting)?;
    match first {
        SemioValue::Map { entries: first_entries } => {
            let header: Vec<String> = first_entries.iter().map(|e| e.key.clone()).collect();
            let mut records = vec![CsvRecord { fields: header.iter().map(|h| CsvField { value: h.clone(), quoted: false }).collect() }];
            for item in &items {
                let resolved = resolve(item, nodes, visiting)?;
                let entries = match resolved {
                    SemioValue::Map { entries } => entries,
                    other => return Err(err(format!("value->csv: every row must be a Map since the first row was, got {other:?}"))),
                };
                let keys: Vec<String> = entries.iter().map(|e| e.key.clone()).collect();
                if keys != header {
                    return Err(err(format!("value->csv: row key set/order {keys:?} does not match the first row's {header:?} — RFC 4180 has exactly one column set per file")));
                }
                let fields = entries.iter().map(|e| scalar_to_field(&resolve(&e.value, nodes, visiting)?)).collect::<Result<Vec<_>, store::PackError>>()?;
                records.push(CsvRecord { fields });
            }
            Ok(CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records })
        }
        SemioValue::List { .. } => {
            let mut records = Vec::new();
            for item in &items {
                let resolved = resolve(item, nodes, visiting)?;
                let row_items = match resolved {
                    SemioValue::List { items } => items,
                    other => return Err(err(format!("value->csv: every row must be a List since the first row was, got {other:?}"))),
                };
                let fields = row_items.iter().map(|v| scalar_to_field(&resolve(v, nodes, visiting)?)).collect::<Result<Vec<_>, store::PackError>>()?;
                records.push(CsvRecord { fields });
            }
            Ok(CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: false, records })
        }
        other => Err(err(format!("value->csv: each row must be a Map or List, got {other:?}"))),
    }
}
//#endregion 🔖️Convert

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::csv::schema::snapshot::CsvField as CsvFieldT;
    use crate::artifacts::semio::standards::v1::subsets::value::io::import::deserializers::artifacts::csv::v_rfc4180::any::semio_value_from_csv;

    fn field(s: &str) -> CsvFieldT {
        CsvFieldT { value: s.into(), quoted: false }
    }

    fn round_trip(snapshot: &CsvSnapshot) -> CsvSnapshot {
        let value = semio_value_from_csv(snapshot);
        let nodes = HashMap::new();
        let mut visiting = HashSet::new();
        csv_from_semio(&value, &nodes, &mut visiting).expect("value->csv")
    }

    /// 🧪️ Required proof: csv -> value -> csv -> value round trip preserves everything the
    /// value subset can represent (the `quoted` flag excepted — documented lossy field).
    #[test]
    fn csv_to_value_to_csv_to_value_round_trips() {
        let snapshot = CsvSnapshot {
            schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
            has_header: true,
            records: vec![CsvRecord { fields: vec![field("name"), field("age"), field("city")] }, CsvRecord { fields: vec![field("Ada"), field("36"), field("London")] }, CsvRecord { fields: vec![field("Grace"), field("85"), field("New York")] }],
        };
        let s1 = semio_value_from_csv(&snapshot);
        let csv_x = round_trip(&snapshot);
        let s2 = semio_value_from_csv(&csv_x);
        assert_eq!(s1, s2);
        assert_eq!(csv_x.has_header, true);
        assert_eq!(csv_x.records.len(), 3);
    }

    #[test]
    fn headerless_round_trips() {
        let snapshot = CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: false, records: vec![CsvRecord { fields: vec![field("x"), field("y")] }, CsvRecord { fields: vec![field("1"), field("2")] }] };
        let csv_x = round_trip(&snapshot);
        assert!(!csv_x.has_header);
        assert_eq!(csv_x.records, snapshot.records);
    }

    #[test]
    fn mismatched_row_shape_is_a_hard_error() {
        let value = SemioValue::List {
            items: vec![
                SemioValue::Map { entries: vec![crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueEntry { key: "a".into(), value: SemioValue::Str { value: "1".into() } }] },
                SemioValue::Map { entries: vec![crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueEntry { key: "b".into(), value: SemioValue::Str { value: "2".into() } }] },
            ],
        };
        let nodes = HashMap::new();
        let mut visiting = HashSet::new();
        assert!(csv_from_semio(&value, &nodes, &mut visiting).is_err());
    }

    #[test]
    fn nested_container_cell_is_a_hard_error() {
        let value = SemioValue::List { items: vec![SemioValue::List { items: vec![SemioValue::List { items: vec![] }] }] };
        let nodes = HashMap::new();
        let mut visiting = HashSet::new();
        assert!(csv_from_semio(&value, &nodes, &mut visiting).is_err());
    }
}
//#endregion 🧪️Tests
