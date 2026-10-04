//! 🖋️ DXF header, table, block, entity and typed group-code relations.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::*;
use store::ArtifactSqliteSnapshot;
use store::sqlite_snapshot::{self, SqliteDatabase as Db, SqliteValue as V, SqliteSnapshotControl as Control, SqliteSnapshotPhase as Phase, artifact::{Cell as C}};
use std::collections::{BTreeMap, BTreeSet};
#[path="🔢️number/🦀️.rs"]
mod number;
use number::{Row,Projection};

fn value_cells(value: &DxfValue) -> [C<'_>; 7] {
    match value { DxfValue::Str { value } => [C::Text("str"), C::Text(value), C::Null, C::Null, C::Null, C::Null, C::Null], DxfValue::Int { value } => [C::Text("int"), C::Null, C::Integer(*value), C::Null, C::Null, C::Null, C::Null], DxfValue::Double { value } => [C::Text("double"), C::Null, C::Null, C::Real(*value), C::Null, C::Null, C::Null], DxfValue::Point { value } => [C::Text("point"), C::Null, C::Null, C::Null, C::Real(value[0]), C::Real(value[1]), C::Real(value[2])] }
}

fn codes(out: &mut Projection<'_, '_>, owner: usize, key: i64, codes: &[(i32, DxfValue)]) -> Result<(), ValueError> {
    for (ordinal, (code, value)) in codes.iter().enumerate() { let mut cells = [C::Null; 16]; cells[owner] = C::Integer(key); cells[7] = C::Integer(i64::try_from(ordinal).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"DXF code ordinal exceeds i64"))?); cells[8] = C::Integer(i64::from(*code)); cells[9..].copy_from_slice(&value_cells(value)); out.insert("dxf_group_code", &cells)?; }
    Ok(())
}

fn point(out: &mut Projection<'_, '_>, entity: i64, ordinal: usize, role: &str, point: &[f64; 3]) -> Result<(), ValueError> { out.insert("dxf_entity_point", &[C::Integer(entity), C::Integer(ordinal as i64), C::Text(role), C::Real(point[0]), C::Real(point[1]), C::Real(point[2])])?; Ok(()) }

fn add_native_rows(rows:&mut usize,count:usize,control:&mut Control<'_>)->Result<(),ValueError>{*rows=rows.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"DXF entity count overflow"))?;control.check_rows(*rows)?;control.checkpoint(Phase::EncodeNative,*rows,0)}
fn native_entity_rows(entities:&[DxfEntity],rows:&mut usize,control:&mut Control<'_>)->Result<(),ValueError>{
    for entity in entities{
        add_native_rows(rows,1,control)?;
        let codes=match entity{
            DxfEntity::Line{unknown_group_codes,..}=>{add_native_rows(rows,2,control)?;unknown_group_codes},
            DxfEntity::Circle{unknown_group_codes,..}|DxfEntity::Arc{unknown_group_codes,..}|DxfEntity::Text{unknown_group_codes,..}|DxfEntity::Insert{unknown_group_codes,..}=>{add_native_rows(rows,1,control)?;unknown_group_codes},
            DxfEntity::Solid{unknown_group_codes,..}=>{add_native_rows(rows,4,control)?;unknown_group_codes},
            DxfEntity::Polyline{vertices,unknown_group_codes,..}=>{for vertex in vertices{add_native_rows(rows,1,control)?;add_native_rows(rows,vertex.unknown_group_codes.len(),control)?;}unknown_group_codes},
            DxfEntity::Other{group_codes,..}=>group_codes,
        };add_native_rows(rows,codes.len(),control)?;
    }Ok(())
}
fn native_rows(snapshot:&DxfSnapshot,control:&mut Control<'_>)->Result<(),ValueError>{
    let mut rows=0;add_native_rows(&mut rows,1,control)?;
    for header in &snapshot.header_vars{add_native_rows(&mut rows,1,control)?;add_native_rows(&mut rows,header.extra_group_codes.len(),control)?;}
    for layer in &snapshot.tables.layers{add_native_rows(&mut rows,1,control)?;add_native_rows(&mut rows,layer.unknown_group_codes.len(),control)?;}
    for style in &snapshot.tables.styles{add_native_rows(&mut rows,1,control)?;add_native_rows(&mut rows,style.unknown_group_codes.len(),control)?;}
    for linetype in &snapshot.tables.linetypes{add_native_rows(&mut rows,1,control)?;add_native_rows(&mut rows,linetype.unknown_group_codes.len(),control)?;}
    for table in &snapshot.other_tables{add_native_rows(&mut rows,1,control)?;add_native_rows(&mut rows,table.tags.len(),control)?;}
    for block in &snapshot.blocks{add_native_rows(&mut rows,1,control)?;add_native_rows(&mut rows,block.unknown_group_codes.len(),control)?;native_entity_rows(&block.entities,&mut rows,control)?;}
    native_entity_rows(&snapshot.entities,&mut rows,control)
}

fn entities(out: &mut Projection<'_, '_>, block: Option<i64>, entities: &[DxfEntity]) -> Result<(), ValueError> {
    for (ordinal, entity) in entities.iter().enumerate() {
        let mut fields = [C::Null; 14];
        let (kind, layer, extra) = match entity {
            DxfEntity::Line { layer, unknown_group_codes, .. } => ("line", Some(layer.as_str()), unknown_group_codes),
            DxfEntity::Circle { radius, layer, unknown_group_codes, .. } => { fields[2] = C::Real(*radius); ("circle", Some(layer.as_str()), unknown_group_codes) },
            DxfEntity::Arc { radius, start_angle, end_angle, layer, unknown_group_codes, .. } => { fields[2] = C::Real(*radius); fields[3] = C::Real(*start_angle); fields[4] = C::Real(*end_angle); ("arc", Some(layer.as_str()), unknown_group_codes) },
            DxfEntity::Polyline { closed, layer, unknown_group_codes, .. } => { fields[12] = C::Integer(i64::from(*closed)); ("polyline", Some(layer.as_str()), unknown_group_codes) },
            DxfEntity::Text { height, value, layer, unknown_group_codes, .. } => { fields[5] = C::Real(*height); fields[6] = C::Text(value); ("text", Some(layer.as_str()), unknown_group_codes) },
            DxfEntity::Solid { layer, unknown_group_codes, .. } => ("solid", Some(layer.as_str()), unknown_group_codes),
            DxfEntity::Insert { block_name, scale, rotation, layer, unknown_group_codes, .. } => { fields[7] = C::Text(block_name); fields[8] = C::Real(scale[0]); fields[9] = C::Real(scale[1]); fields[10] = C::Real(scale[2]); fields[11] = C::Real(*rotation); ("insert", Some(layer.as_str()), unknown_group_codes) },
            DxfEntity::Other { kind, group_codes } => { fields[13] = C::Text(kind); ("other", None, group_codes) },
        };
        fields[0] = C::Text(kind); fields[1] = layer.map_or(C::Null, C::Text);
        let mut cells = [C::Null; 17]; cells[..3].copy_from_slice(&[C::Integer(1), block.map_or(C::Null, C::Integer), C::Integer(i64::try_from(ordinal).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"DXF entity ordinal exceeds i64"))?)]); cells[3..].copy_from_slice(&fields);
        let key = out.insert("dxf_entity", &cells)?;
        match entity {
            DxfEntity::Line { start, end, .. } => { point(out, key, 0, "start", start)?; point(out, key, 1, "end", end)?; },
            DxfEntity::Circle { center, .. } | DxfEntity::Arc { center, .. } => point(out, key, 0, "center", center)?,
            DxfEntity::Text { position, .. } | DxfEntity::Insert { position, .. } => point(out, key, 0, "position", position)?,
            DxfEntity::Solid { points, .. } => { for (ordinal, corner) in points.iter().enumerate() { point(out, key, ordinal, "corner", corner)?; } },
            DxfEntity::Polyline { vertices, .. } => { for (ordinal, vertex) in vertices.iter().enumerate() { let vertex_key = out.insert("dxf_vertex", &[C::Integer(key), C::Integer(ordinal as i64), C::Real(vertex.x), C::Real(vertex.y), C::Real(vertex.z), C::Real(vertex.bulge)])?; codes(out, 6, vertex_key, &vertex.unknown_group_codes)?; } },
            DxfEntity::Other { .. } => {},
        }
        codes(out, 5, key, extra)?;
    }
    Ok(())
}

impl DxfSnapshot{
fn preflight_sqlite_encoding(&self, _: sqlite_snapshot::SnapshotEncoding, control: &mut Control<'_>) -> Result<(), ValueError>{
        control.checkpoint(Phase::EncodeNative, 0, 0)?;
        let database = self.project_sqlite_database(control)?;
        let mut bound = sqlite_snapshot::artifact::NativeEncodingBound::new(control)?;
        bound.add(16_384)?;
        for table in &database.tables { for row in &table.rows {
            bound.add(8192)?;
            for value in &row.values { match value {
                V::Text(value) => { bound.repeated(value.len(), 16)?; bound.add(256)?; },
                V::Blob(value) => { bound.repeated(value.len(), 64)?; bound.add(256)?; },
                V::Integer(_) | V::Real(_) | V::Null => bound.add(2048)?,
            } }
        } }
        bound.finish()
    }
fn project_sqlite_database(&self, control: &mut Control<'_>) -> Result<Db, ValueError>{
        let mut out = Projection::new(Self::SQLITE_SCHEMA, control)?;
        self.write_sqlite_rows(&mut out)?;out.finish()
    }
fn write_sqlite_rows(&self,out:&mut Projection<'_,'_>)->Result<(),ValueError>{
        out.insert("dxf_document", &[C::Text(&self.schema)])?;
        for (ordinal, header) in self.header_vars.iter().enumerate() { let mut cells = [C::Null; 11]; cells[..4].copy_from_slice(&[C::Integer(1), C::Integer(i64::try_from(ordinal).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"DXF header ordinal exceeds i64"))?), C::Text(&header.name), C::Integer(i64::from(header.group_code))]); cells[4..].copy_from_slice(&value_cells(&header.value)); let key = out.insert("dxf_header", &cells)?; codes(out, 0, key, &header.extra_group_codes)?; }
        for (ordinal, layer) in self.tables.layers.iter().enumerate() { let key = out.insert("dxf_layer", &[C::Integer(1), C::Integer(ordinal as i64), C::Text(&layer.name), C::Integer(i64::from(layer.color)), C::Text(&layer.linetype), C::Integer(i64::from(layer.flags))])?; codes(out, 1, key, &layer.unknown_group_codes)?; }
        for (ordinal, style) in self.tables.styles.iter().enumerate() { let key = out.insert("dxf_style", &[C::Integer(1), C::Integer(ordinal as i64), C::Text(&style.name), C::Integer(i64::from(style.flags)), C::Text(&style.font_name)])?; codes(out, 2, key, &style.unknown_group_codes)?; }
        for (ordinal, linetype) in self.tables.linetypes.iter().enumerate() { let key = out.insert("dxf_linetype", &[C::Integer(1), C::Integer(ordinal as i64), C::Text(&linetype.name), C::Integer(i64::from(linetype.flags)), C::Text(&linetype.description)])?; codes(out, 3, key, &linetype.unknown_group_codes)?; }
        for (ordinal, table) in self.other_tables.iter().enumerate() { let key = out.insert("dxf_other_table", &[C::Integer(1), C::Integer(ordinal as i64), C::Text(&table.name)])?; for (ordinal, tag) in table.tags.iter().enumerate() { out.insert("dxf_raw_tag", &[C::Integer(key), C::Integer(ordinal as i64), C::Integer(i64::from(tag.code)), C::Text(&tag.value)])?; } }
        for (ordinal, block) in self.blocks.iter().enumerate() { let key = out.insert("dxf_block", &[C::Integer(1), C::Integer(ordinal as i64), C::Text(&block.name), C::Real(block.base_point[0]), C::Real(block.base_point[1]), C::Real(block.base_point[2])])?; codes(out, 4, key, &block.unknown_group_codes)?; entities(out, Some(key), &block.entities)?; }
        entities(out, None, &self.entities)?;
        Ok(())
    }
fn reconstruct_sqlite_database(db: &Db, control: &mut Control<'_>) -> Result<Self, ValueError>{
        control.checkpoint(Phase::ReconstructSnapshot, 0, 0)?;
        sqlite_snapshot::validate_sqlite_database_schema(db, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(db, Phase::ReconstructSnapshot)?;
        Reader::new(db, control)?.snapshot()
    }
}

impl store::ArtifactSqliteSnapshot for DxfSnapshot {
    fn encode_sqlite_snapshot_native(&self,encoding:sqlite_snapshot::SnapshotEncoding,control:&mut Control<'_>)->Result<store::io_schema::IoPayload,ValueError>{(|| -> Result<(),ValueError>{
        control.checkpoint(Phase::EncodeNative,0,0)?;native_rows(self,control)?;let mut out=Projection::forecast(control,Phase::EncodeNative)?;self.write_sqlite_rows(&mut out)?;out.finish_forecast()?;
        Ok(())})()?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),snapshot_text::spec_producer(),|native|snapshot_text::to_record_controlled(self,native),control)
    }
    fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut Control<'_>)->Result<Self,ValueError>{
        let value:Self=store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),snapshot_text::spec_producer(),snapshot_text::from_record_controlled,control)?;let mut out=Projection::forecast(control,Phase::DecodeNative)?;value.write_sqlite_rows(&mut out)?;out.finish_forecast()?;Ok(value)
    }

    fn preflight_sqlite_snapshot_encoding(&self, encoding: sqlite_snapshot::SnapshotEncoding, control: &mut Control<'_>) -> Result<(),ValueError> {self.preflight_sqlite_encoding(encoding,control)}
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,database:&Db,control:&mut Control<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{( || -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
        control.checkpoint(Phase::ProjectSnapshot,0,0)?;
        if dialect.artifact_kind!="s.stdio.dxf"||dialect.standard!="r12"||dialect.subset!="*"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"snapshot has no owned semantic validator for this exact dialect"));}
        let row=database.table("dxf_document")?.single_row()?;if row.rowid!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"snapshot document identity differs from semantic projection"));}
        control.checkpoint(Phase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics:Vec::new()})
    })().map_err(|error|semio_framework_os_kernel::io_schema::IoError::from_value_error(error))}
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn to_sqlite_database(&self, control: &mut Control<'_>) -> Result<Db,ValueError> {self.project_sqlite_database(control)}

    fn from_sqlite_database(db: &Db, control: &mut Control<'_>) -> Result<Self,ValueError> {Self::reconstruct_sqlite_database(db,control)}
}

fn i32_at(row:Row<'_>, column: usize) -> Result<i32, ValueError> { i32::try_from(row.integer(column)?).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"DXF integer exceeds i32")) }
fn optional_integer(row:Row<'_>, column: usize) -> Result<Option<i64>, ValueError> { if row.values.get(column) == Some(&V::Null) { Ok(None) } else { row.integer(column).map(Some) } }
fn value(row:Row<'_>, at: usize, control: &mut Control<'_>) -> Result<DxfValue, ValueError> {
    let (result, present): (DxfValue, &[usize]) = match row.text(at)? { "str" => (DxfValue::Str { value: sqlite_snapshot::artifact::Reconstruction::new(control)?.text(row.text(at + 1)?)? }, &[1]), "int" => (DxfValue::Int { value: row.integer(at + 2)? }, &[2]), "double" => (DxfValue::Double { value: row.real(at + 3)? }, &[3]), "point" => (DxfValue::Point { value: [row.real(at + 4)?, row.real(at + 5)?, row.real(at + 6)?] }, &[4,5,6]), _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown DXF group value kind")) };
    for index in 1..7 { if !present.contains(&index) && !row.is_null(at + index)? { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DXF group value has fields belonging to another kind")); } }
    Ok(result)
}

const TABLES: [(&str, usize); 12] = [("dxf_document",2),("dxf_header",12),("dxf_layer",7),("dxf_style",6),("dxf_linetype",6),("dxf_other_table",4),("dxf_raw_tag",5),("dxf_block",7),("dxf_entity",18),("dxf_entity_point",7),("dxf_vertex",7),("dxf_group_code",17)];
const CODE_OWNERS: [&str; 7] = ["codes_header","codes_layer","codes_style","codes_linetype","codes_block","codes_entity","codes_vertex"];

struct Reader<'a, 'c, 'p> { rows: BTreeMap<(&'static str,i64), Row<'a>>, groups: BTreeMap<(&'static str,Option<i64>), Vec<Row<'a>>>, used: BTreeSet<(&'static str,i64)>, control: &'c mut Control<'p>, total: usize }

impl<'a, 'c, 'p> Reader<'a, 'c, 'p> {
    fn new(db: &'a Db, control: &'c mut Control<'p>) -> Result<Self,ValueError> {
        let total = db.tables.iter().map(|table| table.rows.len()).sum();
        let mut reader = Self { rows: BTreeMap::new(), groups: BTreeMap::new(), used: BTreeSet::new(), control, total };
        for (name,width) in TABLES { for row in &db.table(name)?.rows {
            let row=sqlite_snapshot::artifact::FloatRow::new(row,number::columns(name))?;
            if row.values.len() != width || row.rowid <= 0 || row.integer(0)? != row.rowid || reader.rows.insert((name,row.rowid),row).is_some() { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DXF entity has a duplicate or invalid identity or row width")); }
            let group = match name {
                "dxf_document" => None,
                "dxf_group_code" => { let owners = (1..8).filter_map(|column| match optional_integer(row,column) { Ok(Some(key)) => Some(Ok((column-1,key))), Ok(None) => None, Err(error) => Some(Err(error)) }).collect::<Result<Vec<_>,ValueError>>()?; if owners.len() != 1 { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DXF group code requires exactly one typed owner")); } Some((CODE_OWNERS[owners[0].0],Some(owners[0].1),8)) },
                "dxf_entity" => { if row.integer(1)? != 1 { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DXF entity document reference must be one")); } Some((name,optional_integer(row,2)?,3)) },
                _ => Some((name,Some(row.integer(1)?),2)),
            };
            if let Some((group,parent,_)) = group { reader.groups.entry((group,parent)).or_default().push(row); }
            if reader.rows.len() % 256 == 0 { reader.control.checkpoint(Phase::ReconstructSnapshot,0,total)?; }
        } }
        for ((name,_),rows) in &mut reader.groups {
            let ordinal = if name.starts_with("codes_") {8} else if *name == "dxf_entity" {3} else {2};
            let mut ticks=0; let mut cancelled=None;
            rows.sort_by_key(|row| { ticks+=1; if ticks%1024==0 && cancelled.is_none() {cancelled=reader.control.checkpoint(Phase::ReconstructSnapshot,0,total).err();} row.integer(ordinal).unwrap_or(i64::MIN) });
            if let Some(error)=cancelled {return Err(error);}
            for (index,row) in rows.iter().enumerate() { if row.integer(ordinal)? != index as i64 { return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("{name} order must be contiguous and unique"))); } }
        }
        Ok(reader)
    }
    fn text(&mut self,row:Row<'_>,column:usize)->Result<String,ValueError>{sqlite_snapshot::artifact::Reconstruction::new(self.control)?.text(row.text(column)?)}
    fn take(&mut self,table: &'static str,key:i64)->Result<Row<'a>,ValueError> { let row=self.rows.get(&(table,key)).copied().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,format!("missing {table} entity {key}")))?; if !self.used.insert((table,key)) {return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DXF row was consumed more than once"));} if self.used.len()%256==0 {self.control.checkpoint(Phase::ReconstructSnapshot,self.used.len(),self.total)?;} Ok(row) }
    fn ordered(&mut self,table:&'static str,parent:Option<i64>)->Vec<Row<'a>> {self.groups.remove(&(table,parent)).unwrap_or_default()}
    fn codes(&mut self,owner:usize,key:i64)->Result<Vec<(i32,DxfValue)>,ValueError> { let mut codes=Vec::new(); for row in self.ordered(CODE_OWNERS[owner],Some(key)) {self.take("dxf_group_code",row.rowid)?; codes.push((i32_at(row,9)?,value(row,10,self.control)?));} Ok(codes) }

    fn snapshot(&mut self)->Result<DxfSnapshot,ValueError> {
        let document=self.take("dxf_document",1)?;
        let mut snapshot=DxfSnapshot {schema:self.text(document,1)?,header_vars:Vec::new(),tables:DxfTables::default(),other_tables:Vec::new(),blocks:Vec::new(),entities:Vec::new()};
        for row in self.ordered("dxf_header",Some(1)) {self.take("dxf_header",row.rowid)?;snapshot.header_vars.push(DxfHeaderVar{name:self.text(row,3)?,group_code:i32_at(row,4)?,value:value(row,5,self.control)?,extra_group_codes:self.codes(0,row.rowid)?});}
        for row in self.ordered("dxf_layer",Some(1)) {self.take("dxf_layer",row.rowid)?;snapshot.tables.layers.push(DxfLayer{name:self.text(row,3)?,color:i32_at(row,4)?,linetype:self.text(row,5)?,flags:i32_at(row,6)?,unknown_group_codes:self.codes(1,row.rowid)?});}
        for row in self.ordered("dxf_style",Some(1)) {self.take("dxf_style",row.rowid)?;snapshot.tables.styles.push(DxfStyle{name:self.text(row,3)?,flags:i32_at(row,4)?,font_name:self.text(row,5)?,unknown_group_codes:self.codes(2,row.rowid)?});}
        for row in self.ordered("dxf_linetype",Some(1)) {self.take("dxf_linetype",row.rowid)?;snapshot.tables.linetypes.push(DxfLinetype{name:self.text(row,3)?,flags:i32_at(row,4)?,description:self.text(row,5)?,unknown_group_codes:self.codes(3,row.rowid)?});}
        for row in self.ordered("dxf_other_table",Some(1)) {self.take("dxf_other_table",row.rowid)?;let mut tags=Vec::new();for tag in self.ordered("dxf_raw_tag",Some(row.rowid)) {self.take("dxf_raw_tag",tag.rowid)?;tags.push(DxfTag{code:i32_at(tag,3)?,value:self.text(tag,4)?});}snapshot.other_tables.push(DxfOtherTable{name:self.text(row,3)?,tags});}
        for row in self.ordered("dxf_block",Some(1)) {self.take("dxf_block",row.rowid)?;snapshot.blocks.push(DxfBlock{name:self.text(row,3)?,base_point:[row.real(4)?,row.real(5)?,row.real(6)?],entities:self.entities(Some(row.rowid))?,unknown_group_codes:self.codes(4,row.rowid)?});}
        snapshot.entities=self.entities(None)?;
        if self.used.len()!=self.total {return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DXF SQLite contains orphaned or mismatched domain entities"));}
        self.control.checkpoint(Phase::ReconstructSnapshot,self.total,self.total)?;
        Ok(snapshot)
    }

    fn points(&mut self,key:i64,roles:&[&str])->Result<Vec<[f64;3]>,ValueError> {let rows=self.ordered("dxf_entity_point",Some(key));if rows.len()!=roles.len(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DXF entity point count disagrees with its kind"));}let mut points=Vec::new();for(row,role)in rows.into_iter().zip(roles){self.take("dxf_entity_point",row.rowid)?;if row.text(3)?!=*role{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DXF entity point role disagrees with its kind"));}points.push([row.real(4)?,row.real(5)?,row.real(6)?]);}Ok(points)}

    fn entities(&mut self,block:Option<i64>)->Result<Vec<DxfEntity>,ValueError> {
        let mut entities=Vec::new();
        for row in self.ordered("dxf_entity",block) {
            self.take("dxf_entity",row.rowid)?;let key=row.rowid;let extra=self.codes(5,key)?;
            let kind=row.text(4)?;
            let used:&[usize]=match kind {"line"|"solid"=>&[],"circle"=>&[6],"arc"=>&[6,7,8],"polyline"=>&[16],"text"=>&[9,10],"insert"=>&[11,12,13,14,15],"other"=>&[17],_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown DXF entity kind"))};
            for column in 6..18 {if !used.contains(&column)&&!row.is_null(column)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DXF entity carries fields belonging to another kind"));}}
            let layer=if kind=="other" {if row.values[5]!=V::Null{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DXF other entity has an undeclared layer field"));}String::new()}else{self.text(row,5)?};
            entities.push(match kind {
                "line"=>{let points=self.points(key,&["start","end"])?;DxfEntity::Line{start:points[0],end:points[1],layer,unknown_group_codes:extra}},
                "circle"=>DxfEntity::Circle{center:self.points(key,&["center"])?[0],radius:row.real(6)?,layer,unknown_group_codes:extra},
                "arc"=>DxfEntity::Arc{center:self.points(key,&["center"])?[0],radius:row.real(6)?,start_angle:row.real(7)?,end_angle:row.real(8)?,layer,unknown_group_codes:extra},
                "polyline"=>{let closed=match row.integer(16)?{0=>false,1=>true,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DXF polyline closed flag must be boolean"))};let mut vertices=Vec::new();for vertex in self.ordered("dxf_vertex",Some(key)){self.take("dxf_vertex",vertex.rowid)?;vertices.push(DxfVertex{x:vertex.real(3)?,y:vertex.real(4)?,z:vertex.real(5)?,bulge:vertex.real(6)?,unknown_group_codes:self.codes(6,vertex.rowid)?});}DxfEntity::Polyline{vertices,closed,layer,unknown_group_codes:extra}},
                "text"=>DxfEntity::Text{position:self.points(key,&["position"])?[0],height:row.real(9)?,value:self.text(row,10)?,layer,unknown_group_codes:extra},
                "solid"=>{let points=self.points(key,&["corner","corner","corner","corner"])?;DxfEntity::Solid{points:[points[0],points[1],points[2],points[3]],layer,unknown_group_codes:extra}},
                "insert"=>DxfEntity::Insert{block_name:self.text(row,11)?,position:self.points(key,&["position"])?[0],scale:[row.real(12)?,row.real(13)?,row.real(14)?],rotation:row.real(15)?,layer,unknown_group_codes:extra},
                "other"=>DxfEntity::Other{kind:self.text(row,17)?,group_codes:extra},
                _=>unreachable!(),
            });
        }
        Ok(entities)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sqlite_snapshot_dxf_controlled_output_retains_complete_state_and_interior_admission(){
        let plan:serde_json::Value=serde_json::from_str(include_str!("../📝️text/🧫️fixtures/🛫️encoding/🔣️.json")).unwrap();let mut snapshot=fixture();snapshot.schema=plan["unit"].as_str().unwrap().repeat(plan["repetitions"].as_u64().unwrap() as usize);let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        for encoding in [sqlite_snapshot::SnapshotEncoding::Binary,sqlite_snapshot::SnapshotEncoding::Text]{
            let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |_|true,limits)).unwrap();assert_eq!(DxfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,limits)).unwrap(),snapshot);
            let mut reached=false;let result=snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |event|{if event.phase==Phase::EncodeNative&&event.total==snapshot.schema.len()&&event.completed>=plan["cancelAfter"].as_u64().unwrap() as usize&&event.completed<event.total{reached=true;false}else{true}},limits));assert!(reached);assert!(result.is_err());
            assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |_|true,sqlite_snapshot::SqliteDatabaseLimits{max_value_bytes:plan["tinyMaximumBytes"].as_u64().unwrap() as usize,..limits})).is_err());
        }
    }
    #[test]
    fn sqlite_snapshot_dxf_controlled_native_owner_preserves_full_fixture_and_stops_inside_text(){
        use store::{ArtifactDsl,ArtifactPack};use semio_framework_os_kernel::io::IoPayload;
        let mut snapshot=fixture();snapshot.schema="DXF 世界\0".repeat(16384);let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        for payload in [IoPayload::Binary(snapshot.encode_pack()),IoPayload::Text(snapshot.print_dsl())]{
            assert_eq!(DxfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,limits)).unwrap(),snapshot);
            let mut reached=false;let result=DxfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |event|{if event.phase==Phase::DecodeNative&&event.completed>=65536&&event.completed<event.total{reached=true;false}else{true}},limits));assert!(result.is_err());assert!(reached,"cancel inside long native text");
            let mut small=limits;small.max_value_bytes=128;assert!(DxfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,small)).is_err());
            assert!(DxfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|false,limits)).is_err());
        }
    }

    #[test]
    fn sqlite_snapshot_dxf_owned_guard_checks_exact_identity_and_projection(){
        let plan:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🎯️dialect.json")).unwrap();let snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let database=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();let dialect=|value:&serde_json::Value|semio_framework_os_kernel::io_schema::ArtifactDialect{artifact_kind:value["artifactKind"].as_str().unwrap().into(),standard:value["standard"].as_str().unwrap().into(),subset:value["subset"].as_str().unwrap().into()};let valid=dialect(&plan["valid"]);assert!(snapshot.validate_sqlite_snapshot_subset(&valid,&database,&mut Control::new(&mut |_|true,limits)).unwrap().diagnostics.is_empty());for invalid in plan["invalid"].as_array().unwrap(){assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(invalid),&database,&mut Control::new(&mut |_|true,limits)).is_err());}let mut mismatched=database.clone();mismatched.table_mut("dxf_document").unwrap().rows[0].values[1]=V::Text("other.schema".into());assert!(snapshot.validate_sqlite_snapshot_subset(&valid,&mismatched,&mut Control::new(&mut |_|true,limits)).is_err());assert!(snapshot.validate_sqlite_snapshot_subset(&valid,&database,&mut Control::new(&mut |_|false,limits)).is_err());
    }
    use super::*;
    use store::ArtifactSqliteSnapshot;
use semio_framework_value::FromValue;
    fn fixture() -> DxfSnapshot { DxfSnapshot::from_value(serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()).unwrap() }

    #[test]
    fn sqlite_snapshot_dxf_preserves_ieee_entity_geometry_and_group_values(){
        use std::{io::Write,process::{Command,Stdio}};let cases:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔢️ieee.json")).unwrap();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        for case in cases["binary64"].as_array().unwrap(){let bits=u64::from_str_radix(case["bits"].as_str().unwrap(),16).unwrap();let value=f64::from_bits(bits);let mut snapshot=fixture();snapshot.header_vars.push(DxfHeaderVar{name:"$IEEE".into(),group_code:40,value:DxfValue::Double{value},extra_group_codes:vec![(10,DxfValue::Point{value:[value;3]})]});snapshot.blocks[0].base_point=[value;3];let DxfEntity::Circle{radius,center,..}=&mut snapshot.entities[1]else{panic!("circle");};*radius=value;*center=[value;3];let DxfEntity::Polyline{vertices,..}=&mut snapshot.entities[3]else{panic!("polyline");};vertices[0].bulge=value;let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();let file=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();let loaded=sqlite_snapshot::import_sqlite_database(&file,limits,&mut |_|true).unwrap();let restored=DxfSnapshot::from_sqlite_database(&loaded,&mut Control::new(&mut |_|true,limits)).unwrap();let DxfValue::Double{value}=restored.header_vars.last().unwrap().value else{panic!("double");};assert_eq!(value.to_bits(),bits);let DxfValue::Point{value}=restored.header_vars.last().unwrap().extra_group_codes[0].1 else{panic!("point");};assert!(value.iter().all(|value|value.to_bits()==bits));assert!(restored.blocks[0].base_point.iter().all(|value|value.to_bits()==bits));let DxfEntity::Circle{radius,center,..}=&restored.entities[1]else{panic!("circle");};assert_eq!(radius.to_bits(),bits);assert!(center.iter().all(|value|value.to_bits()==bits));let DxfEntity::Polyline{vertices,..}=&restored.entities[3]else{panic!("polyline");};assert_eq!(vertices[0].bulge.to_bits(),bits);
            let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,JSON.stringify(db.query(\"SELECT CAST(double_value_bits AS TEXT) AS bits,double_value_class AS class,double_value IS NULL AS nullQuery FROM dxf_header WHERE name='$IEEE'\").get()));db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let actual:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(actual["bits"].as_str().unwrap(),(bits as i64).to_string());assert_eq!(actual["class"],case["class"]);assert_eq!(actual["nullQuery"].as_i64().unwrap(),i64::from(case["class"]=="nan"));}
    }

    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_dxf_actual_declaration_preserves_complete_owned_model(){
        use semio_framework_os_kernel::io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}};
        crate::register_sqlite_test_declaration();
        let dialect=ArtifactDialect{artifact_kind:"s.stdio.dxf".into(),standard:"r12".into(),subset:"*".into()};let mut snapshot=fixture();snapshot.schema="complete-owned-DXF-schema".into();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut phases=Vec::new();
        let file=io_export_sqlite_snapshot(&dialect,&snapshot,sqlite_snapshot::SnapshotEncoding::Binary,limits,&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(io_import_sqlite_snapshot::<DxfSnapshot>(&dialect,&file,limits,&mut |event|{phases.push(event.phase);true}).await.unwrap().value,snapshot);assert!(!phases.iter().any(|phase|matches!(phase,Phase::EncodeNative|Phase::DecodeNative)));
    }
    async fn erased_snapshot(snapshot:&DxfSnapshot,encoding:sqlite_snapshot::SnapshotEncoding)->DxfSnapshot{
        use semio_framework_os_kernel::io::{ArtifactDialect,IoPayload,io_mechanism::{io_route,io_run}};
        use store::{ArtifactDsl,ArtifactPack};
        crate::register_sqlite_test_declaration();
        let dialect=ArtifactDialect{artifact_kind:"s.stdio.dxf".into(),standard:"r12".into(),subset:"*".into()};
        let sqlite:ArtifactDialect=semio_framework_os_kernel::io_schema::SQLITE_SNAPSHOT.into();
        let payload=match encoding{sqlite_snapshot::SnapshotEncoding::Binary=>IoPayload::Binary(snapshot.encode_pack()),sqlite_snapshot::SnapshotEncoding::Text=>IoPayload::Text(snapshot.print_dsl())};
        let export=io_route(&dialect,&sqlite,1).await.unwrap().value;
        let file=io_run(&export,payload).await.unwrap().value;
        let import=io_route(&sqlite,&dialect,1).await.unwrap().value;
        let payload=io_run(&import,file).await.unwrap().value;
        match payload{IoPayload::Binary(bytes)=>DxfSnapshot::decode_pack(&bytes).unwrap(),IoPayload::Text(text)=>DxfSnapshot::parse_dsl(&text).unwrap()}
    }
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_dxf_actual_erased_binary_preserves_complete_owned_model(){assert_eq!(erased_snapshot(&fixture(),sqlite_snapshot::SnapshotEncoding::Binary).await,fixture());}
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_dxf_actual_erased_text_preserves_complete_owned_model(){assert_eq!(erased_snapshot(&fixture(),sqlite_snapshot::SnapshotEncoding::Text).await,fixture());}
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_dxf_actual_erased_routes_preserve_exact_ieee_geometry_and_group_values(){
        let cases:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔢️ieee.json")).unwrap();
        for encoding in [sqlite_snapshot::SnapshotEncoding::Binary,sqlite_snapshot::SnapshotEncoding::Text]{for case in cases["binary64"].as_array().unwrap(){
            let bits=u64::from_str_radix(case["bits"].as_str().unwrap(),16).unwrap();let value=f64::from_bits(bits);let mut snapshot=fixture();
            snapshot.header_vars.push(DxfHeaderVar{name:"$IEEE".into(),group_code:40,value:DxfValue::Double{value},extra_group_codes:vec![(10,DxfValue::Point{value:[value;3]}),(160,DxfValue::Int{value:i64::MIN})]});
            snapshot.blocks[0].base_point=[value;3];let DxfEntity::Circle{radius,center,..}=&mut snapshot.entities[1]else{panic!("circle");};*radius=value;*center=[value;3];let DxfEntity::Polyline{vertices,..}=&mut snapshot.entities[3]else{panic!("polyline");};vertices[0].bulge=value;
            let restored=erased_snapshot(&snapshot,encoding).await;assert_eq!(restored.schema,snapshot.schema);
            let header=restored.header_vars.last().unwrap();let DxfValue::Double{value}=header.value else{panic!("double");};assert_eq!(value.to_bits(),bits);let DxfValue::Point{value}=header.extra_group_codes[0].1 else{panic!("point");};assert_eq!(value.map(f64::to_bits),[bits;3]);assert_eq!(header.extra_group_codes[1].1,DxfValue::Int{value:i64::MIN});assert_eq!(restored.blocks[0].base_point.map(f64::to_bits),[bits;3]);
            let DxfEntity::Circle{radius,center,..}=&restored.entities[1]else{panic!("circle");};assert_eq!(radius.to_bits(),bits);assert_eq!(center.map(f64::to_bits),[bits;3]);let DxfEntity::Polyline{vertices,..}=&restored.entities[3]else{panic!("polyline");};assert_eq!(vertices[0].bulge.to_bits(),bits);
        }}
    }
    #[test]
    fn sqlite_snapshot_dxf_independent_sql_entity_join_and_edit(){
        use std::{io::Write,process::{Command,Stdio}};let mut snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();let bytes=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();
        let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query(\"SELECT e.kind,p.role,p.x,p.y,p.z FROM dxf_entity e JOIN dxf_entity_point p ON p.entity_id=e.id WHERE e.kind='line' ORDER BY e.id,p.ordinal\").all();if(rows.length!==2||rows[0].role!=='start'||rows[1].role!=='end')throw Error('entity join');db.query(\"UPDATE dxf_entity SET radius=123.5,radius_bits=4638390956842811392,radius_class='finite' WHERE kind='circle'\").run();await Bun.write(Bun.stdout,db.serialize());db.close();";
        let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let loaded=sqlite_snapshot::import_sqlite_database(&output.stdout,limits,&mut |_|true).unwrap();if let DxfEntity::Circle{radius,..}=&mut snapshot.entities[1]{*radius=123.5;}else{panic!("fixture circle");}assert_eq!(DxfSnapshot::from_sqlite_database(&loaded,&mut Control::new(&mut |_|true,limits)).unwrap(),snapshot);
    }

    #[test]
    fn sqlite_snapshot_dxf_preserves_every_typed_entity_and_group_code() {
        let snapshot=fixture(); let limits=sqlite_snapshot::SqliteDatabaseLimits::default(); let mut progress=|_|true; let mut control=Control::new(&mut progress,limits);
        let db=snapshot.to_sqlite_database(&mut control).unwrap();
        assert_eq!(db.table("dxf_entity").unwrap().rows.len(),9);
        assert_eq!(db.table("dxf_group_code").unwrap().rows.len(),13);
        assert_eq!(db.table("dxf_vertex").unwrap().rows.len(),2);
        assert_eq!(DxfSnapshot::from_sqlite_database(&db,&mut control).unwrap(),snapshot);
        let file=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();
        let loaded=sqlite_snapshot::import_sqlite_database(&file,limits,&mut |_|true).unwrap();
        assert_eq!(DxfSnapshot::from_sqlite_database(&loaded,&mut control).unwrap(),snapshot);
        let mut edited=loaded;let circle=edited.table_mut("dxf_entity").unwrap().rows.iter_mut().find(|row|row.text(4).unwrap()=="circle").unwrap();circle.values[6]=V::Real(123.5);circle.values[18]=V::Integer(123.5f64.to_bits() as i64);circle.values[19]=V::Text("finite".into());
        let changed=DxfSnapshot::from_sqlite_database(&edited,&mut control).unwrap();
        assert!(matches!(changed.entities[1],DxfEntity::Circle{radius:123.5,..}));
    }

    #[test]
    fn sqlite_snapshot_dxf_rejects_broken_relations_and_controls_allocations() {
        let snapshot=fixture(); let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|false,limits)).is_err());
        let mut small=limits; small.max_rows=2; assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,small)).is_err());
        small=limits; small.max_value_bytes=1; assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,small)).is_err());
        let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();
        assert!(DxfSnapshot::from_sqlite_database(&db,&mut Control::new(&mut |_|false,limits)).is_err());
        for column in [1,2] {let mut broken=db.clone();broken.table_mut("dxf_vertex").unwrap().rows[0].values[column]=V::Integer(999);assert!(DxfSnapshot::from_sqlite_database(&broken,&mut Control::new(&mut |_|true,limits)).is_err());}
        let mut broken=db.clone(); broken.table_mut("dxf_entity").unwrap().rows.iter_mut().find(|row|row.text(4).unwrap()=="line").unwrap().values[17]=V::Text("wrong kind".into()); assert!(DxfSnapshot::from_sqlite_database(&broken,&mut Control::new(&mut |_|true,limits)).is_err());
        let mut broken=db.clone();let sql=broken.table("dxf_document").unwrap().sql.replace("schema TEXT NOT NULL","schema TEXT \"NOT NULL\"");broken.table_mut("dxf_document").unwrap().sql=sql;assert!(DxfSnapshot::from_sqlite_database(&broken,&mut Control::new(&mut |_|true,limits)).is_err());
        let mut large=snapshot;large.entities=vec![large.entities[0].clone();1000];let mut calls=0;assert!(large.to_sqlite_database(&mut Control::new(&mut |_|{calls+=1;calls<4},limits)).is_err());
    }
}
