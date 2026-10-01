use store::sqlite_snapshot::{SqliteDatabase, SqliteRow, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase};

impl store::ArtifactSqliteSnapshot for super::ProbeSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        fn node(value: &serde_json::Value, parent: Option<i64>, position: Option<usize>, name: Option<&str>, depth: usize, rows: &mut Vec<SqliteRow>, control: &mut SqliteSnapshotControl<'_>) -> Result<(), String> {
            if depth > 1024 { return Err("probe syntax exceeds maximum depth".into()); }
            if rows.len() % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, rows.len(), 0)?; }
            control.check_rows(rows.len().checked_add(1).ok_or_else(|| "probe node count overflow".to_string())?)?;
            if let serde_json::Value::String(value) = value { control.check_value_bytes(value.len())?; }
            if let Some(name) = name { control.check_value_bytes(name.len())?; }
            let id = i64::try_from(rows.len() + 1).map_err(|error| error.to_string())?;
            let (kind, boolean, number, string) = match value {
                serde_json::Value::Null => ("null", SqliteValue::Null, SqliteValue::Null, SqliteValue::Null),
                serde_json::Value::Bool(value) => ("boolean", SqliteValue::Integer(i64::from(*value)), SqliteValue::Null, SqliteValue::Null),
                serde_json::Value::Number(value) => ("number", SqliteValue::Null, SqliteValue::Text(value.to_string()), SqliteValue::Null),
                serde_json::Value::String(value) => ("string", SqliteValue::Null, SqliteValue::Null, SqliteValue::Text(value.clone())),
                serde_json::Value::Array(_) => ("array", SqliteValue::Null, SqliteValue::Null, SqliteValue::Null),
                serde_json::Value::Object(_) => ("object", SqliteValue::Null, SqliteValue::Null, SqliteValue::Null),
            };
            rows.push(SqliteRow { rowid: id, values: vec![SqliteValue::Integer(id), parent.map(SqliteValue::Integer).unwrap_or(SqliteValue::Null), position.map(|value| SqliteValue::Integer(value as i64)).unwrap_or(SqliteValue::Null), name.map(|value| SqliteValue::Text(value.to_string())).unwrap_or(SqliteValue::Null), SqliteValue::Text(kind.to_string()), boolean, number, string] });
            match value {
                serde_json::Value::Array(values) => for (position, value) in values.iter().enumerate() { node(value, Some(id), Some(position), None, depth + 1, rows, control)?; },
                serde_json::Value::Object(values) => for (name, value) in values { node(value, Some(id), None, Some(name), depth + 1, rows, control)?; },
                _ => {}
            }
            Ok(())
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA).map_err(|error| error.to_string())?;
        let rows = &mut database.table_mut("probe_node")?.rows;
        node(&self.0, None, None, None, 0, rows, control)?;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, rows.len(), rows.len())?;
        Ok(database)
    }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        use std::collections::{BTreeMap, BTreeSet};
        let rows = &database.table("probe_node")?.rows;
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, rows.len())?;
        let mut nodes = BTreeMap::new();
        let mut children = BTreeMap::<i64, Vec<&SqliteRow>>::new();
        let mut roots = Vec::new();
        for (index, row) in rows.iter().enumerate() {
            if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, index, rows.len())?; }
            if row.values.len() != 8 || row.rowid != row.integer(0)? || nodes.insert(row.rowid, row).is_some() { return Err("invalid probe syntax node identity".into()); }
            match row.values[1] { SqliteValue::Integer(parent) => children.entry(parent).or_default().push(row), SqliteValue::Null => roots.push(row), _ => return Err("invalid probe parent identity".into()) }
        }
        if roots.len() != 1 || roots[0].values[2] != SqliteValue::Null || roots[0].values[3] != SqliteValue::Null { return Err("probe syntax requires one root".into()); }
        for parent in children.keys() { if !nodes.contains_key(parent) { return Err("probe parent does not exist".into()); } }
        fn node(row: &SqliteRow, children: &BTreeMap<i64, Vec<&SqliteRow>>, visited: &mut BTreeSet<i64>, depth: usize, total: usize, control: &mut SqliteSnapshotControl<'_>) -> Result<serde_json::Value, String> {
            if depth > 1024 || !visited.insert(row.rowid) { return Err("cyclic or excessively nested probe syntax".into()); }
            if visited.len() % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, visited.len(), total)?; }
            let members = children.get(&row.rowid).map(Vec::as_slice).unwrap_or(&[]);
            let kind = row.text(4)?;
            let scalars = &row.values[5..];
            let empty = [SqliteValue::Null, SqliteValue::Null, SqliteValue::Null];
            if kind != "array" && kind != "object" && !members.is_empty() { return Err("scalar probe node cannot own children".into()); }
            match kind {
                "null" if scalars == empty => Ok(serde_json::Value::Null),
                "boolean" if scalars[1..] == empty[1..] => match row.integer(5)? { 0 => Ok(serde_json::Value::Bool(false)), 1 => Ok(serde_json::Value::Bool(true)), _ => Err("probe boolean must be zero or one".into()) },
                "number" if scalars[0] == SqliteValue::Null && scalars[2] == SqliteValue::Null => row.text(6)?.parse::<serde_json::Number>().map(serde_json::Value::Number).map_err(|error| error.to_string()),
                "string" if scalars[..2] == empty[..2] => Ok(serde_json::Value::String(row.text(7)?.to_string())),
                "array" if scalars == empty => {
                    let mut ordered = members.to_vec();
                    ordered.sort_by_key(|member| member.integer(2).unwrap_or(-1));
                    let mut values = Vec::new();
                    for (position, member) in ordered.into_iter().enumerate() {
                        if member.integer(2)? != position as i64 || member.values[3] != SqliteValue::Null { return Err("invalid probe array order".into()); }
                        values.push(node(member, children, visited, depth + 1, total, control)?);
                    }
                    Ok(serde_json::Value::Array(values))
                }
                "object" if scalars == empty => {
                    let mut values = serde_json::Map::new();
                    for member in members {
                        if member.values[2] != SqliteValue::Null { return Err("probe object member cannot have an array position".into()); }
                        let name = member.text(3)?;
                        if values.insert(name.to_string(), node(member, children, visited, depth + 1, total, control)?).is_some() { return Err("duplicate probe object member".into()); }
                    }
                    Ok(serde_json::Value::Object(values))
                }
                _ => Err("invalid probe syntax scalar columns".into()),
            }
        }
        let mut visited = BTreeSet::new();
        let value = node(roots[0], &children, &mut visited, 0, rows.len(), control)?;
        if visited.len() != rows.len() { return Err("unreachable probe syntax node".into()); }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, rows.len(), rows.len())?;
        Ok(Self(value))
    }
}
