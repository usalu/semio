
fn native_value_bound(value:&PlyValue,bound:&mut semio_framework_os_kernel::sqlite_snapshot::artifact::NativeEncodingBound<'_,'_>)->Result<(),String>{match value{PlyValue::List(values)=>{bound.add(64)?;for value in values{native_value_bound(value,bound)?;}Ok(())},PlyValue::Float(_)|PlyValue::Double(_)=>bound.add(1164),_=>bound.add(96)}}
use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,artifact::NativeEncodingBound};
use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_os_kernel::sqlite_snapshot::artifact::{Cell,Projection,FloatColumn,FloatRow,insert_ieee754};
const BINARY32:&[FloatColumn]=&[FloatColumn::Binary32(5)];
const BINARY64:&[FloatColumn]=&[FloatColumn::Binary64(5)];
fn scalar_bytes(value:&PlyValue)->usize{let value=match value{PlyValue::Float(value)=>f64::from(*value),PlyValue::Double(value)=>*value,_=>return 8};if value.is_nan(){11}else if value.is_infinite(){32}else{22}}

fn kind_name(kind: PlyScalarType) -> &'static str { match kind { PlyScalarType::Char => "char", PlyScalarType::UChar => "uchar", PlyScalarType::Short => "short", PlyScalarType::UShort => "ushort", PlyScalarType::Int => "int", PlyScalarType::UInt => "uint", PlyScalarType::Float => "float", PlyScalarType::Double => "double" } }
fn scalar_kind(name: &str) -> Result<PlyScalarType, String> { match name { "char" => Ok(PlyScalarType::Char), "uchar" => Ok(PlyScalarType::UChar), "short" => Ok(PlyScalarType::Short), "ushort" => Ok(PlyScalarType::UShort), "int" => Ok(PlyScalarType::Int), "uint" => Ok(PlyScalarType::UInt), "float" => Ok(PlyScalarType::Float), "double" => Ok(PlyScalarType::Double), _ => Err("unknown PLY scalar kind".into()) } }
fn scalar(value: &PlyValue) -> Result<(&'static str, Cell<'static>, Cell<'static>), String> {
    let null = Cell::Null;
    match value {
        PlyValue::Char(value) => Ok(("char", Cell::Integer(i64::from(*value)), null)),
        PlyValue::UChar(value) => Ok(("uchar", Cell::Integer(i64::from(*value)), null)),
        PlyValue::Short(value) => Ok(("short", Cell::Integer(i64::from(*value)), null)),
        PlyValue::UShort(value) => Ok(("ushort", Cell::Integer(i64::from(*value)), null)),
        PlyValue::Int(value) => Ok(("int", Cell::Integer(i64::from(*value)), null)),
        PlyValue::UInt(value) => Ok(("uint", Cell::Integer(i64::from(*value)), null)),
        PlyValue::Float(value) => Ok(("float", null, Cell::Float32(*value))),
        PlyValue::Double(value) => Ok(("double", null, Cell::Real(*value))),
        PlyValue::List(_) => Err("PLY list items must be scalar".into()),
    }
}



fn account(control: &mut SqliteSnapshotControl<'_>, budget: &mut (usize, usize), bytes: usize) -> Result<(), String> {
    budget.0 = budget.0.checked_add(1).ok_or("PLY entity count overflow")?;
    budget.1 = budget.1.checked_add(bytes).ok_or("PLY value size overflow")?;
    control.check_rows(budget.0)?;
    control.check_value_bytes(budget.1)?;
    if budget.0 % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, budget.0)?; }
    Ok(())
}

fn append(projection: &mut Projection<'_,'_>, name: &str, values: Vec<Cell<'_>>) -> Result<i64, String> {
    let columns=if matches!(name,"ply_cell"|"ply_list_item"){if matches!(values.get(2),Some(Cell::Text("float"))){BINARY32}else{BINARY64}}else{&[]};
    insert_ieee754(projection,name,&values,columns)
}

fn ordinal(value: usize) -> Result<Cell<'static>, String> { i64::try_from(value).map(Cell::Integer).map_err(|error| error.to_string()) }
fn text(value: &str) -> Cell<'_> { Cell::Text(value) }
fn ids(database: &SqliteDatabase, table: &str) -> Result<BTreeSet<i64>, String> {
    let mut ids = BTreeSet::new();
    for row in &database.table(table)?.rows { let id = row.integer(0)?; if id < 1 || id != row.rowid || !ids.insert(id) { return Err(format!("{table} requires distinct positive identifiers")); } }
    Ok(ids)
}

fn children<'a>(database: &'a SqliteDatabase, name: &str, owners: &BTreeSet<i64>, ordered: bool, control: &mut SqliteSnapshotControl<'_>) -> Result<BTreeMap<i64, Vec<&'a SqliteRow>>, String> {
    ids(database, name)?;
    let table = database.table(name)?;
    let mut result = BTreeMap::<i64, Vec<&SqliteRow>>::new();
    for (ordinal, row) in table.rows.iter().enumerate() {
        if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, table.rows.len())?; }
        let owner = row.integer(1)?;
        if !owners.contains(&owner) { return Err(format!("{name} has an unknown owner")); }
        result.entry(owner).or_default().push(row);
    }
    if ordered {
        for rows in result.values_mut() {
            rows.sort_by_key(|row| row.integer(2).unwrap_or(i64::MIN));
            for (ordinal, row) in rows.iter().enumerate() { if row.integer(2)? != ordinal as i64 { return Err(format!("{name} requires contiguous ordinals")); } }
        }
    }
    Ok(result)
}

fn read_scalar(row: &SqliteRow) -> Result<PlyValue, String> {
    let kind = scalar_kind(row.text(3)?)?;
    let row=FloatRow::new(row,if kind==PlyScalarType::Float{BINARY32}else{BINARY64})?;
    if matches!(kind, PlyScalarType::Float | PlyScalarType::Double) {
        if row.values.get(4) != Some(&SqliteValue::Null) { return Err("PLY real scalar must not have an integer payload".into()); }
        if kind == PlyScalarType::Double { return Ok(PlyValue::Double(row.real(5)?)); }
        return Ok(PlyValue::Float(row.binary32(5)?));
    }
    if !row.is_null(5)? { return Err("PLY integer scalar must not have a real payload".into()); }
    let value = row.integer(4)?;
    match kind {
        PlyScalarType::Char => i8::try_from(value).map(PlyValue::Char).map_err(|error| error.to_string()),
        PlyScalarType::UChar => u8::try_from(value).map(PlyValue::UChar).map_err(|error| error.to_string()),
        PlyScalarType::Short => i16::try_from(value).map(PlyValue::Short).map_err(|error| error.to_string()),
        PlyScalarType::UShort => u16::try_from(value).map(PlyValue::UShort).map_err(|error| error.to_string()),
        PlyScalarType::Int => i32::try_from(value).map(PlyValue::Int).map_err(|error| error.to_string()),
        PlyScalarType::UInt => u32::try_from(value).map(PlyValue::UInt).map_err(|error| error.to_string()),
        _ => unreachable!(),
    }
}

impl ArtifactSqliteSnapshot for PlySnapshot {
    fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{let mut bound=NativeEncodingBound::new(control)?;bound.add(1024)?;bound.repeated(self.schema.len(),6)?;for comment in &self.comments{bound.add(64)?;bound.repeated(comment.len(),6)?;}for element in &self.elements{bound.add(256)?;bound.repeated(element.name.len(),6)?;for property in &element.properties{bound.add(256)?;bound.repeated(property.name().len(),6)?;}for row in &element.rows{bound.add(64)?;for value in &row.values{native_value_bound(value,&mut bound)?;}}}bound.finish()}

    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
        if dialect.artifact_kind!="s.stdio.ply"||dialect.standard!="1.0"||dialect.subset!="*"{return Err(String::from("geometry owned SQLite dialect differs").into());}
        let row=database.table("ply_document")?.single_row()?;
        if row.rowid!=1||row.text(1)?!=self.schema{return Err(String::from("geometry document identity differs from its semantic projection").into());}
        Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))
    }
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?;
        let format = match self.format { PlyFormat::Ascii => "ascii", PlyFormat::BinaryLittleEndian => "binary_little_endian", PlyFormat::BinaryBigEndian => "binary_big_endian" };
        let mut budget = (0usize, 0usize);
        account(control, &mut budget, 8usize.checked_add(self.schema.len()).and_then(|count| count.checked_add(format.len())).ok_or("PLY value size overflow")?)?;
        for comment in &self.comments { account(control, &mut budget, 24usize.checked_add(comment.len()).ok_or("PLY value size overflow")?)?; }
        for element in &self.elements {
            account(control, &mut budget, 40usize.checked_add(element.name.len()).ok_or("PLY value size overflow")?)?;
            for property in &element.properties {
                let bytes = match property { PlyProperty::Scalar { name, kind } => 30usize.checked_add(name.len()).and_then(|count| count.checked_add(kind_name(*kind).len())), PlyProperty::List { name, count_kind, value_kind } => { 28usize.checked_add(name.len()).and_then(|count| count.checked_add(kind_name(*count_kind).len())).and_then(|count| count.checked_add(kind_name(*value_kind).len())) } }.ok_or("PLY value size overflow")?;
                account(control, &mut budget, bytes)?;
            }
            for row in &element.rows {
                if row.values.len() != element.properties.len() { return Err("PLY row must have one value for every property".into()); }
                account(control, &mut budget, 24)?;
                for (property, value) in element.properties.iter().zip(&row.values) {
                    match (property, value) {
                        (PlyProperty::Scalar { kind, .. }, value) => { let (name, _, _) = scalar(value)?; if name != kind_name(*kind) { return Err("PLY cell kind does not match its property".into()); } account(control, &mut budget, 24 + name.len()+scalar_bytes(value))?; },
                        (PlyProperty::List { count_kind, value_kind, .. }, PlyValue::List(items)) => {
                            account(control, &mut budget, 28)?;
                            for item in items { let (name, _, _) = scalar(item)?; if name != kind_name(*value_kind) { return Err("PLY list item kind does not match its property".into()); } account(control, &mut budget, 24 + name.len()+scalar_bytes(item))?; }
                        },
                        _ => return Err("PLY list property requires a list cell".into()),
                    }
                }
            }
        }
                let mut database = Projection::new(Self::SQLITE_SCHEMA,control)?;
        append(&mut database, "ply_document", vec![text(&self.schema), text(format)])?;
        for (index, comment) in self.comments.iter().enumerate() { append(&mut database, "ply_comment", vec![Cell::Integer(1), ordinal(index)?, text(comment)])?; }
        for (index, element) in self.elements.iter().enumerate() {
            let element_id = append(&mut database, "ply_element", vec![Cell::Integer(1), ordinal(index)?, text(&element.name), Cell::Integer((element.count>>32) as i64),Cell::Integer((element.count&0xffffffff) as i64)])?;
            let mut property_ids = Vec::new();
            for (index, property) in element.properties.iter().enumerate() {
                let values = match property {
                    PlyProperty::Scalar { name, kind } => vec![Cell::Integer(element_id), ordinal(index)?, text(name), text("scalar"), text(kind_name(*kind)), Cell::Null, Cell::Null],
                    PlyProperty::List { name, count_kind, value_kind } => vec![Cell::Integer(element_id), ordinal(index)?, text(name), text("list"), Cell::Null, text(kind_name(*count_kind)), text(kind_name(*value_kind))],
                };
                property_ids.push(append(&mut database, "ply_property", values)?);
            }
            for (index, row) in element.rows.iter().enumerate() {
                let row_id = append(&mut database, "ply_row", vec![Cell::Integer(element_id), ordinal(index)?])?;
                for (property_id, value) in property_ids.iter().zip(&row.values) {
                    match value {
                        PlyValue::List(items) => {
                            let cell_id = append(&mut database, "ply_cell", vec![Cell::Integer(row_id), Cell::Integer(*property_id), text("list"), Cell::Null, Cell::Null])?;
                            for (index, item) in items.iter().enumerate() { let (name, integer, real) = scalar(item)?; append(&mut database, "ply_list_item", vec![Cell::Integer(cell_id), ordinal(index)?, text(name), integer, real])?; }
                        },
                        value => { let (name, integer, real) = scalar(value)?; append(&mut database, "ply_cell", vec![Cell::Integer(row_id), Cell::Integer(*property_id), text(name), integer, real])?; },
                    }
                }
            }
        }
        database.finish()
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits()).map_err(|error| error.to_string())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let document = database.table("ply_document")?.single_row()?;
        if document.integer(0)? != 1 || document.rowid != 1 { return Err("PLY document identifier must be 1".into()); }
        let format = match document.text(2)? { "ascii" => PlyFormat::Ascii, "binary_little_endian" => PlyFormat::BinaryLittleEndian, "binary_big_endian" => PlyFormat::BinaryBigEndian, _ => return Err("unknown PLY format".into()) };
        let element_ids = ids(database, "ply_element")?;
        let mut comments = children(database, "ply_comment", &BTreeSet::from([1]), true, control)?.remove(&1).unwrap_or_default();
        let mut properties = children(database, "ply_property", &element_ids, true, control)?;
        let mut rows = children(database, "ply_row", &element_ids, true, control)?;
        let mut cells = children(database, "ply_cell", &ids(database, "ply_row")?, false, control)?;
        let mut list_items = children(database, "ply_list_item", &ids(database, "ply_cell")?, true, control)?;
        let mut snapshot = Self { schema: document.text(1)?.into(), format, comments: Vec::new(), elements: Vec::new() };
        let total = database.tables.iter().map(|table| table.rows.len()).sum();
        let mut completed = 0;
        for row in comments.drain(..) { if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1; snapshot.comments.push(row.text(3)?.into()); }
        for element in database.table("ply_element")?.ordered_rows(2)? {
            if element.integer(1)? != 1 { return Err("PLY element has an unknown document".into()); }
            let element_id = element.integer(0)?;
            let property_rows = properties.remove(&element_id).unwrap_or_default();
            let mut element_properties = Vec::new();
            let mut property_indices = BTreeMap::new();
            for property in property_rows {
                if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1;
                property_indices.insert(property.integer(0)?, element_properties.len());
                let decoded = match property.text(4)? {
                    "scalar" => { if property.optional_text(6)?.is_some() || property.optional_text(7)?.is_some() { return Err("PLY scalar property must not have list kind declarations".into()); } PlyProperty::Scalar { name: property.text(3)?.into(), kind: scalar_kind(property.text(5)?)? } },
                    "list" => { if property.optional_text(5)?.is_some() { return Err("PLY list property must not have a scalar kind declaration".into()); } let count_kind = scalar_kind(property.text(6)?)?; PlyProperty::List { name: property.text(3)?.into(), count_kind, value_kind: scalar_kind(property.text(7)?)? } },
                    _ => return Err("unknown PLY property form".into()),
                };
                element_properties.push(decoded);
            }
            let element_rows = rows.remove(&element_id).unwrap_or_default();
            let high=u32::try_from(element.integer(4)?).map_err(|_|"PLY declared count high word exceeds unsigned32")?;
            let low=u32::try_from(element.integer(5)?).map_err(|_|"PLY declared count low word exceeds unsigned32")?;
            let count=(u64::from(high)<<32)|u64::from(low);
            let mut decoded_rows = Vec::new();
            for row in element_rows {
                let mut values = vec![None; element_properties.len()];
                for cell in cells.remove(&row.integer(0)?).unwrap_or_default() {
                    if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1;
                    let property = *property_indices.get(&cell.integer(2)?).ok_or("PLY cell references a property outside its row's element")?;
                    if values[property].is_some() { return Err("PLY row has multiple cells for the same property".into()); }
                    let owned = list_items.remove(&cell.integer(0)?).unwrap_or_default();
                    let value = match &element_properties[property] {
                        PlyProperty::Scalar { kind, .. } => { if !owned.is_empty() { return Err("PLY scalar cell must not own list items".into()); } if cell.text(3)? != kind_name(*kind) { return Err("PLY scalar kind does not match its property".into()); } read_scalar(cell)? },
                        PlyProperty::List { count_kind, value_kind, .. } => {
                            if cell.text(3)? != "list" || cell.values.get(4) != Some(&SqliteValue::Null) || !FloatRow::new(cell,BINARY64)?.is_null(5)? { return Err("PLY list cell must have no scalar payload".into()); }
                            let mut items = Vec::new();
                            for item in owned { if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1; if item.text(3)? != kind_name(*value_kind) { return Err("PLY list item kind does not match its property".into()); } items.push(read_scalar(item)?); }
                            PlyValue::List(items)
                        },
                    };
                    values[property] = Some(value);
                }
                decoded_rows.push(PlyRow { values: values.into_iter().map(|value| value.ok_or_else(|| "PLY row is missing a property cell".into())).collect::<Result<_, String>>()? });
            }
            snapshot.elements.push(PlyElement { name: element.text(3)?.into(), count, properties: element_properties, rows: decoded_rows });
        }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?;
        Ok(snapshot)
    }
}
