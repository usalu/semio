//! 🧩️ Relational row controls for explicitly authored artifact projections.
use super::{SqliteDatabase, SqliteRow, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase, ValueError, ValueRefusalKind};

/// 📏️ Accumulates an owner's explicit native allocation and encoded output upper bounds.
pub struct NativeEncodingBound<'c, 'p> { control: &'c mut SqliteSnapshotControl<'p>, bytes: usize, units: usize, semantic:bool }

impl<'c, 'p> NativeEncodingBound<'c, 'p> {

    /// 🗃️ Pays actual borrowed forecast storage independently from the unpaid output bound.
    pub fn allocate_frontier<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{self.checkpoint()?;super::transfer::reserve(count,self.control)}
    /// ➕️ Admits a forecast walk's actual replacement buffer before moving its references.
    pub fn push_frontier<T>(&mut self,frontier:&mut Vec<T>,value:T)->Result<(),ValueError>{super::transfer::grow(frontier,SqliteSnapshotPhase::EncodeNative,self.control)?;frontier.push(value);Ok(())}

    /// 🏁️ Checks cancellation before the owner starts its borrowed field walk.
    pub fn new(control: &'c mut SqliteSnapshotControl<'p>) -> Result<Self, ValueError> {
        control.checkpoint(SqliteSnapshotPhase::EncodeNative, 0, 0)?;
        Ok(Self { control, bytes: 0, units: 0, semantic:true })
    }
    /// 📁️ Checks a separate file forecast after the owner admits its actual semantic cells.
    pub fn file_only(control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,ValueError>{let mut bound=Self::new(control)?;bound.semantic=false;Ok(bound)}
    /// ➕️ Bounds an explicitly calculated allocation before any encoding ownership is created.
    pub fn add(&mut self, bytes: usize) -> Result<(), ValueError> {
        let total = self.bytes.checked_add(bytes).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native encoding byte bound overflow"))?;
        if self.semantic{self.control.check_value_bytes(total)?;}
        if total > self.control.limits().max_file_bytes { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "native encoding exceeds file byte limit")); }
        let units = self.units.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "native encoding work count overflow"))?;
        if units % 256 == 0 || bytes > 65_536 { self.control.checkpoint(SqliteSnapshotPhase::EncodeNative, units, 0)?; }
        self.bytes = total; self.units = units; Ok(())
    }
    /// 🧮️ Checks a caller-authored repeated field estimate with overflow-safe multiplication.
    pub fn repeated(&mut self, count: usize, bytes: usize) -> Result<(), ValueError> {
        self.add(count.checked_mul(bytes).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native encoding repeated byte bound overflow"))?)
    }
    /// 🎚️ Exposes caller limits before allocating an owner's traversal frontier.
    pub fn limits(&self) -> super::SqliteDatabaseLimits { self.control.limits() }
    /// 🔢️ Bounds an owner's explicitly predicted entity count before traversal allocation.
    pub fn check_rows(&self, count: usize) -> Result<(), ValueError> { self.control.check_rows(count) }
    /// ⏱️ Checks cancellation during a borrowed walk before its next emitted estimate.
    pub fn checkpoint(&mut self) -> Result<(), ValueError> { self.control.checkpoint(SqliteSnapshotPhase::EncodeNative, self.units, 0) }
    /// ⏱️ Checks cancellation immediately before returning admission to the encoder.
    pub fn finish(self) -> Result<(), ValueError> { self.control.checkpoint(SqliteSnapshotPhase::EncodeNative, self.units, self.units) }
}

/// 🫳️ Borrowed storage-class cells are bounded before creating owned copies.
#[derive(Clone, Copy)]
pub enum Cell<'a> { Null, Integer(i64), Real(f64), Float32(f32), Text(&'a str), Blob(&'a [u8]) }

#[path="🧮️rows/🦀️.rs"]
mod row_writer;
pub use row_writer::RowWriter;

impl Cell<'_> {
    fn bytes(self) -> usize { match self { Self::Null => 0, Self::Integer(_) | Self::Real(_) | Self::Float32(_) => 8, Self::Text(value) => value.len(), Self::Blob(value) => value.len() } }
    fn owned(self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteValue, ValueError> { Ok(match self { Self::Null => SqliteValue::Null, Self::Integer(value) => SqliteValue::Integer(value), Self::Real(value) => SqliteValue::Real(value), Self::Float32(value) => SqliteValue::Real(f64::from(value)), Self::Text(value) => SqliteValue::Text(copy_text(value, control, SqliteSnapshotPhase::ProjectSnapshot)?), Self::Blob(value) => SqliteValue::Blob(copy_blob(value, control, SqliteSnapshotPhase::ProjectSnapshot)?) }) }
}

fn copy_text(text:&str,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<String,ValueError>{
 super::transfer::copy_text(text,phase,control)
}

fn copy_blob(blob:&[u8],control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<Vec<u8>,ValueError>{
 control.allocation_stage(phase,|remaining,progress|{
  let mut callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|progress(event.completed,event.total);
  let mut native=semio_framework_value::NativeEncodeControl::new(remaining,&mut callback);let result=native.copy_bytes(blob);(result,native.owned_bytes())
 })?
}

/// 🔤️ Copies an explicitly pre-admitted native projection field in bounded UTF-8 chunks.
pub fn project_text(control: &mut SqliteSnapshotControl<'_>, text: &str) -> Result<String, ValueError> { control.check_value_bytes(text.len())?; copy_text(text, control, SqliteSnapshotPhase::ProjectSnapshot) }

/// 🏗️ Inserts only the domain rows explicitly supplied by an artifact implementation.
pub struct Projection<'c, 'p> { database: SqliteDatabase, control: &'c mut SqliteSnapshotControl<'p>, rows: usize, bytes: usize }

impl<'c, 'p> Projection<'c, 'p> {

    /// 🗂️ Admits an owner's concrete borrowed frontier on the shared backing ledger.
    pub fn allocate_frontier<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,count)?;super::transfer::reserve(count,self.control)}
    /// ➕️ Pays complete replacement backing before growing an owner's traversal frontier.
    pub fn push_frontier<T>(&mut self,frontier:&mut Vec<T>,value:T)->Result<(),ValueError>{super::transfer::grow(frontier,SqliteSnapshotPhase::ProjectSnapshot,self.control)?;frontier.push(value);Ok(())}

    /// 🔀️ Orders a paid frontier with the same bounded physical comparator authority.
    pub fn sort_frontier<T>(&mut self,values:&mut[T],compare:impl FnMut(&T,&T,&mut SqliteSnapshotControl<'_>)->Result<std::cmp::Ordering,ValueError>)->Result<(),ValueError>{super::transfer::heap_sort(values,SqliteSnapshotPhase::ProjectSnapshot,self.control,compare)}

    /// 🔎️ Probes an owner's ordered borrowed frontier through its existing caller control.
    pub fn search_frontier<T>(&mut self,values:&[T],mut compare:impl FnMut(&T,&mut SqliteSnapshotControl<'_>)->Result<std::cmp::Ordering,ValueError>)->Result<Option<usize>,ValueError>{
        let(mut low,mut high)=(0,values.len());
        while low<high{self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,low,values.len())?;let middle=low+(high-low)/2;match compare(&values[middle],self.control)?{std::cmp::Ordering::Less=>low=middle+1,std::cmp::Ordering::Greater=>high=middle,std::cmp::Ordering::Equal=>return Ok(Some(middle))}}
        Ok(None)
    }

    /// 🏛️ Loads the artifact's authored table declarations under its schema limits.
    pub fn new(sql:&str,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,ValueError>{
  let database=super::transfer::schema_controlled(sql,SqliteSnapshotPhase::ProjectSnapshot,control)?;
  Ok(Self{database,control,rows:0,bytes:0})
 }

    /// 🔢️ Allocates the next surrogate identity in one declared entity table.
    pub fn insert(&mut self, table: &str, cells: &[Cell<'_>]) -> Result<i64, ValueError> {
        let key = i64::try_from(self.database.table(table)?.rows.len() + 1).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "artifact row identity overflow"))?;
        self.insert_key(table, key, cells)?;
        Ok(key)
    }

    /// 🔗️ Uses an existing entity identity for a declared one-to-one relationship.
    pub fn insert_key(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),ValueError>{
  self.database.table(table)?;let rows=self.rows.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"artifact row count overflow"))?;self.control.check_rows(rows)?;
  let mut bytes=self.bytes.checked_add(8).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"artifact value byte count overflow"))?;
  for cell in cells{bytes=bytes.checked_add(cell.bytes()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"artifact value byte count overflow"))?;self.control.check_value_bytes(bytes)?;if match cell{Cell::Real(value)=>value.is_nan(),Cell::Float32(value)=>value.is_nan(),_=>false}{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"NaN requires an authored IEEE scalar companion"));}}
  self.control.check_value_bytes(bytes)?;self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,rows,0)?;
  let count=cells.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"artifact cell backing overflow"))?;
  let mut values=super::transfer::reserve(count,self.control)?;values.push(SqliteValue::Integer(key));for cell in cells{values.push(cell.owned(self.control)?);}
  let table=self.database.table_mut(table)?;super::transfer::grow(&mut table.rows,SqliteSnapshotPhase::ProjectSnapshot,self.control)?;table.rows.push(SqliteRow{rowid:key,values});
  self.rows=rows;self.bytes=bytes;Ok(())
 }

    /// 📏️ Exposes the caller's declared resource bounds before domain traversal allocation.
    pub fn limits(&self) -> super::SqliteDatabaseLimits { self.control.limits() }

    /// 🔢️ Bounds the domain rows this projection actually materializes.
    pub fn check_rows(&self, count: usize) -> Result<(), ValueError> { self.control.check_rows(count) }

    /// 📦️ Bounds predicted aggregate storage-class bytes before allocation.
    pub fn check_value_bytes(&self, count: usize) -> Result<(), ValueError> { self.control.check_value_bytes(count) }

    /// 🔍️ Compares full borrowed literal keys through this projection's caller control.
    pub fn compare_text(&mut self,left:&str,right:&str)->Result<std::cmp::Ordering,ValueError>{super::transfer::compare_text(left,right,SqliteSnapshotPhase::ProjectSnapshot,self.control)}

    /// 🕸️ Publishes a bounded domain traversal independently of materialized row count.
    pub fn checkpoint_work(&mut self,completed:usize,total:usize)->Result<(),ValueError>{self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,completed,total)}

    /// ⏱️ Checks cancellation around domain loops that do not emit a row immediately.
    pub fn checkpoint(&mut self) -> Result<(), ValueError> { self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, self.rows, 0) }

    /// 🔢️ Reports actual inserted rows against the owner's borrowed preflight count.
    pub fn checkpoint_total(&mut self, total_rows: usize) -> Result<(), ValueError> { if self.rows > total_rows { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "artifact projection exceeded its predicted row count")); } self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, self.rows, total_rows) }

    /// 📤️ Orders physical identities without copying rows and checks cancellation during ordering.
    pub fn finish(mut self) -> Result<SqliteDatabase, ValueError> {
        self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?;
        let mut work = RowOrdering { control: self.control, units: 0 };
        for table in &mut self.database.tables { order_rows(&mut table.rows, &mut work)?; }
        work.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, work.units, work.units)?;
        Ok(self.database)
    }
}

struct RowOrdering<'c, 'p> { control: &'c mut SqliteSnapshotControl<'p>, units: usize }
impl RowOrdering<'_, '_> {
    fn step(&mut self) -> Result<(), ValueError> { self.units = self.units.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "artifact row ordering work overflow"))?; if self.units % 256 == 0 { self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, self.units, 0)?; } Ok(()) }
}
fn sift_rows(rows: &mut [SqliteRow], mut root: usize, end: usize, work: &mut RowOrdering<'_, '_>) -> Result<(), ValueError> {
    while root < end / 2 {
        work.step()?;
        let mut child = root * 2 + 1;
        if child + 1 < end && rows[child].rowid < rows[child + 1].rowid { child += 1; }
        if rows[root].rowid >= rows[child].rowid { break; }
        rows.swap(root, child); root = child;
    }
    Ok(())
}
fn order_rows(rows: &mut [SqliteRow], work: &mut RowOrdering<'_, '_>) -> Result<(), ValueError> {
    let count = rows.len();
    let mut ordered = true;
    for index in 1..count { work.step()?; if rows[index - 1].rowid > rows[index].rowid { ordered = false; } }
    if !ordered {
        for root in (0..count / 2).rev() { sift_rows(rows, root, count, work)?; }
        for end in (1..count).rev() { work.step()?; rows.swap(0, end); sift_rows(rows, 0, end, work)?; }
    }
    for index in 1..count { work.step()?; if rows[index - 1].rowid == rows[index].rowid { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "artifact row identities must be unique")); } }
    Ok(())
}

/// 🔢️ An explicitly authored scalar column and its native IEEE width.
#[derive(Clone,Copy)]
pub enum FloatColumn { Binary64(usize), Binary32(usize) }
impl FloatColumn { fn index(self)->usize{match self{Self::Binary64(i)|Self::Binary32(i)=>i}} }
fn numeric_class(value:f64)->&'static str{if value.is_nan(){"nan"}else if value==f64::INFINITY{"positiveInfinity"}else if value==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"}}
fn float_cells<'a>(cells:&[Cell<'a>],columns:&[FloatColumn],control:&mut SqliteSnapshotControl<'_>)->Result<Vec<Cell<'a>>,ValueError>{
 let phase=SqliteSnapshotPhase::ProjectSnapshot;control.checkpoint(phase,0,columns.len())?;
 let count=cells.len().checked_add(columns.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"IEEE cell backing overflow"))?).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"IEEE cell backing overflow"))?;
 let mut indices=super::transfer::reserve(columns.len(),control)?;
 for column in columns{let index=column.index().checked_sub(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"IEEE column cannot be a row identity"))?;if index>=cells.len(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"missing authored IEEE scalar field"));}indices.push(index);}
 super::transfer::heap_sort(&mut indices,phase,control,|a,b,_|Ok(a.cmp(b)))?;
 if indices.windows(2).any(|pair|pair[0]==pair[1]){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate authored IEEE scalar column"));}
 let mut result=super::transfer::reserve(count,control)?;result.extend_from_slice(cells);
 for(count,column)in columns.iter().enumerate(){let index=column.index()-1;let cell=cells[index];
  let(value,bits)=match(column,cell){(_,Cell::Null)=>{result.extend([Cell::Null,Cell::Null]);continue;},(FloatColumn::Binary64(_),Cell::Real(value))=>(value,value.to_bits()as i64),(FloatColumn::Binary32(_),Cell::Float32(value))=>(f64::from(value),i64::from(value.to_bits())),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"native IEEE width differs from authored scalar field"))};
  result[index]=if value.is_nan(){Cell::Null}else{Cell::Real(value)};result.extend([Cell::Integer(bits),Cell::Text(numeric_class(value))]);if(count+1)%256==0{control.checkpoint(phase,count+1,columns.len())?;}
 }Ok(result)
}
/// 🧮️ Inserts authored scalar fields with named numeric class and signed INTEGER bit companions.
pub fn insert_ieee754(projection:&mut Projection<'_,'_>,table:&str,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,ValueError>{
 float_cell_limit(projection,cells.len(),columns.len())?;let cells=float_cells(cells,columns,projection.control)?;projection.insert(table,&cells)
}
/// 🔗️ Inserts authored one-to-one scalar fields with their exact native IEEE companions.
pub fn insert_key_ieee754(projection:&mut Projection<'_,'_>,table:&str,key:i64,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<(),ValueError>{
 float_cell_limit(projection,cells.len(),columns.len())?;let cells=float_cells(cells,columns,projection.control)?;projection.insert_key(table,key,&cells)
}
fn float_cell_limit(projection:&Projection<'_,'_>,cells:usize,columns:usize)->Result<(),ValueError>{let fields=cells.checked_add(columns.checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "IEEE cell count overflow"))?).and_then(|n|n.checked_add(1)).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "IEEE cell count overflow"))?;if fields>projection.limits().max_columns{Err(ValueError::new(ValueRefusalKind::WorkLimit, "authored IEEE row exceeds column limit"))}else{Ok(())}}
fn float_slot(row:&SqliteRow,index:usize,columns:&[FloatColumn])->Result<(usize,FloatColumn),ValueError>{let original=row.values.len().checked_sub(columns.len().checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "IEEE column count overflow"))?).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "missing IEEE scalar columns"))?;let(slot,column)=columns.iter().copied().enumerate().find(|(_,column)|column.index()==index).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "scalar field has no authored IEEE column"))?;if index>=original{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "IEEE scalar field is outside authored columns"));}Ok((original+slot*2,column))}
/// 🫥️ Distinguishes an absent optional scalar from a present NaN with a NULL query value.
pub fn ieee754_is_null(row:&SqliteRow,index:usize,columns:&[FloatColumn])->Result<bool,ValueError>{let(slot,_)=float_slot(row,index,columns)?;Ok(row.values.get(index)==Some(&SqliteValue::Null)&&row.values.get(slot)==Some(&SqliteValue::Null)&&row.values.get(slot+1)==Some(&SqliteValue::Null))}
fn check_float(row:&SqliteRow,index:usize,slot:usize,value:f64)->Result<(),ValueError>{
    if row.text(slot+1)?!=numeric_class(value){return Err(ValueError::new(ValueRefusalKind::InvalidValue, "IEEE numeric class disagrees with native bits"));}
    if value.is_nan(){if row.values.get(index)!=Some(&SqliteValue::Null){return Err(ValueError::new(ValueRefusalKind::InvalidValue, "NaN query value must be NULL"));}return Ok(())}
    let exact=match row.values.get(index){Some(SqliteValue::Real(query))=>*query==value,Some(SqliteValue::Integer(query))=>value>=i64::MIN as f64&&value < -(i64::MIN as f64)&&value.fract()==0.0&&value as i64==*query,_=>false};
    if !exact{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "query scalar disagrees with exact native IEEE value"));}Ok(())
}
/// 🧮️ Restores binary64, interpreting signed INTEGER bit storage as its exact unsigned bit pattern.
pub fn read_binary64(row:&SqliteRow,index:usize,columns:&[FloatColumn])->Result<f64,ValueError>{let(slot,column)=float_slot(row,index,columns)?;if !matches!(column,FloatColumn::Binary64(_)){return Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected an authored binary64 scalar"));}let value=f64::from_bits(row.integer(slot)? as u64);check_float(row,index,slot,value)?;Ok(value)}
/// 🔢️ Restores binary32 directly from its own bits without quieting signaling NaNs through a binary64 cast.
pub fn read_binary32(row:&SqliteRow,index:usize,columns:&[FloatColumn])->Result<f32,ValueError>{let(slot,column)=float_slot(row,index,columns)?;if !matches!(column,FloatColumn::Binary32(_)){return Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected an authored binary32 scalar"));}let value=f32::from_bits(u32::try_from(row.integer(slot)?).map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, error.to_string()))?);check_float(row,index,slot,f64::from(value))?;Ok(value)}
/// 🛡️ Validates authored scalar companions before restoring any owned native fields.
pub fn validate_ieee754_row(row:&SqliteRow,original_columns:usize,columns:&[FloatColumn])->Result<(),ValueError>{if row.values.len()!=original_columns.checked_add(columns.len().checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "IEEE column count overflow"))?).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "IEEE column count overflow"))?{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "authored IEEE row column count differs"));}for column in columns{if ieee754_is_null(row,column.index(),columns)?{continue;}match column{FloatColumn::Binary64(index)=>{read_binary64(row,*index,columns)?;},FloatColumn::Binary32(index)=>{read_binary32(row,*index,columns)?;}}}Ok(())}

/// 🪟️ A borrowed view of explicitly authored scalar columns retains logical field positions.
#[derive(Clone,Copy)]
pub struct FloatRow<'a> { pub rowid: i64, pub values: &'a [SqliteValue], row: &'a SqliteRow, columns: &'static [FloatColumn] }

impl<'a> FloatRow<'a> {
    /// 🧮️ Binds only the caller's declared native scalar columns and validates their companions.
    pub fn new(row:&'a SqliteRow,columns:&'static [FloatColumn])->Result<Self,ValueError>{let original=row.values.len().checked_sub(columns.len().checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "IEEE column count overflow"))?).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "missing IEEE companion columns"))?;validate_ieee754_row(row,original,columns)?;Ok(Self{rowid:row.rowid,values:&row.values[..original],row,columns})}
    /// 🔢️ Reads a native INTEGER field from the declared logical row.
    pub fn integer(self,index:usize)->Result<i64,ValueError>{self.row.integer(index)}
    /// 🧮️ Reads a native binary64 field from exact bits and validates its query value.
    pub fn real(self,index:usize)->Result<f64,ValueError>{if self.columns.iter().any(|c|c.index()==index){read_binary64(self.row,index,self.columns)}else{self.row.real(index)}}
    /// 🔢️ Reads a native binary32 field without intermediate NaN conversion.
    pub fn binary32(self,index:usize)->Result<f32,ValueError>{read_binary32(self.row,index,self.columns)}
    /// 🔤️ Borrows one native text field without creating ownership.
    pub fn text(self,index:usize)->Result<&'a str,ValueError>{self.row.text(index)}
    /// 📦️ Borrows one native binary field without creating ownership.
    pub fn blob(self,index:usize)->Result<&'a [u8],ValueError>{self.row.blob(index)}
    /// 🫥️ Borrows an optional native text field with exact presence.
    pub fn optional_text(self,index:usize)->Result<Option<&'a str>,ValueError>{self.row.optional_text(index)}
    /// 🫥️ Tests native optional presence independently of a NaN's NULL query scalar.
    pub fn is_null(self,index:usize)->Result<bool,ValueError>{if self.columns.iter().any(|c|c.index()==index){ieee754_is_null(self.row,index,self.columns)}else{Ok(self.values.get(index)==Some(&SqliteValue::Null))}}
}

/// 🫴️ Bounds explicitly restored native fields before creating owned copies.
pub struct Reconstruction<'c, 'p> { control: &'c mut SqliteSnapshotControl<'p>, bytes: usize, units: usize }

/// 🔤️ Copies one explicitly selected native text field within the transfer's ownership budget.
pub fn reconstruct_text(control: &mut SqliteSnapshotControl<'_>, text: &str) -> Result<String, ValueError> {
    Reconstruction { control, bytes: 0, units: 0 }.text(text)
}

/// 📦️ Copies one explicitly selected native binary field within the transfer's ownership budget.
pub fn reconstruct_blob(control: &mut SqliteSnapshotControl<'_>, blob: &[u8]) -> Result<Vec<u8>, ValueError> {
    Reconstruction { control, bytes: 0, units: 0 }.blob(blob)
}

/// 🔢️ Validates and orders authored ordinal relationships with bounded cancellation checkpoints.
pub fn ordered_row_refs<'a>(table:&'a super::SqliteTable,ordinal:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
 let phase=SqliteSnapshotPhase::ReconstructSnapshot;let total=table.rows.len();control.check_rows(total)?;control.checkpoint(phase,0,total)?;let mut rows=super::transfer::reserve(total,control)?;
 for(count,row)in table.rows.iter().enumerate(){let index=usize::try_from(row.integer(ordinal)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;if index>=total{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"relationship ordinals must be contiguous and unique"));}rows.push(row);if(count+1)%256==0{control.checkpoint(phase,count+1,total)?;}}
 super::transfer::heap_sort(&mut rows,phase,control,|a,b,_|Ok(a.integer(ordinal)?.cmp(&b.integer(ordinal)?)))?;
 for(expected,row)in rows.iter().enumerate(){if row.integer(ordinal)?!=expected as i64{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"relationship ordinals must be contiguous and unique"));}if(expected+1)%256==0{control.checkpoint(phase,expected+1,total)?;}}
 control.checkpoint(phase,total,total)?;Ok(rows)
}

impl<'c, 'p> Reconstruction<'c, 'p> {
    /// 🏁️ Starts native ownership accounting independently of database storage accounting.
    pub fn new(control: &'c mut SqliteSnapshotControl<'p>) -> Result<Self, ValueError> {
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, 0)?;
        Ok(Self { control, bytes: 0, units: 0 })
    }
    fn reserve(&mut self, count: usize) -> Result<(), ValueError> {
        let bytes = self.control.reconstruction_bytes.checked_add(count).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native reconstruction value byte count overflow"))?;
        self.control.check_value_bytes(bytes.checked_add(self.control.reconstruction_scalar_bytes).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native reconstruction aggregate byte count overflow"))?)?;
        let units = self.control.reconstruction_units.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "native reconstruction unit count overflow"))?;
        if units % 256 == 0 || count > 65_536 { self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, units, 0)?; }
        self.control.reconstruction_bytes = bytes; self.control.reconstruction_units = units; self.bytes = bytes; self.units = units; Ok(())
    }
    /// 🔤️ Checks the aggregate native byte bound before copying one borrowed text field.
    pub fn text(&mut self, text: &str) -> Result<String, ValueError> { self.reserve(text.len())?; copy_text(text, self.control, SqliteSnapshotPhase::ReconstructSnapshot) }
    /// 📦️ Checks the aggregate native byte bound before copying one borrowed binary field.
    pub fn blob(&mut self, blob: &[u8]) -> Result<Vec<u8>, ValueError> { self.reserve(blob.len())?; copy_blob(blob, self.control, SqliteSnapshotPhase::ReconstructSnapshot) }
    /// 🔢️ Accounts for one explicitly restored numeric or boolean field.
    pub fn scalar(&mut self) -> Result<(), ValueError> { self.reserve(8) }
    /// ⏱️ Checks cancellation around relationship traversal and prior to returning native ownership.
    pub fn checkpoint(&mut self) -> Result<(), ValueError> { self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, self.units, 0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_encoding_bounds_reject_overflow_and_cancel_before_large_admission() {
        let mut limits = super::super::SqliteDatabaseLimits::default(); limits.max_value_bytes = 32;
        let mut callback = |_| true; let mut control = SqliteSnapshotControl::new(&mut callback, limits);
        let mut bound = NativeEncodingBound::new(&mut control).unwrap();
        bound.repeated(4, 8).unwrap(); assert!(bound.add(1).is_err()); assert_eq!(bound.bytes, 32);
        assert!(bound.repeated(usize::MAX, 2).is_err());
        let mut calls = 0; let mut callback = |_| { calls += 1; calls == 1 };
        let mut control = SqliteSnapshotControl::new(&mut callback, super::super::SqliteDatabaseLimits::default());
        let mut bound = NativeEncodingBound::new(&mut control).unwrap();
        assert!(bound.add(65_537).is_err()); assert_eq!(bound.bytes, 0);
        let mut limits = super::super::SqliteDatabaseLimits::default(); limits.max_file_bytes = 16;
        let mut callback = |_| true; let mut control = SqliteSnapshotControl::new(&mut callback, limits);
        assert!(NativeEncodingBound::new(&mut control).unwrap().add(17).is_err());
    }
    #[test]
    fn native_file_forecast_separates_exact_semantic_cells_and_preserves_file_refusals(){
        let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/📏️file-bound/🔣️.json")).unwrap();
        let limits=super::super::SqliteDatabaseLimits{max_value_bytes:fixture["maxSemanticBytes"].as_u64().unwrap()as usize,max_file_bytes:fixture["maxFileBytes"].as_u64().unwrap()as usize,..super::super::SqliteDatabaseLimits::default()};
        let mut accepted=|_|true;let mut control=SqliteSnapshotControl::new(&mut accepted,limits);let mut bound=NativeEncodingBound::file_only(&mut control).unwrap();
        for chunk in fixture["chunks"].as_array().unwrap(){bound.add(chunk.as_u64().unwrap()as usize).unwrap();}
        assert_eq!(bound.bytes,fixture["expectedBytes"].as_u64().unwrap()as usize);assert_eq!(bound.add(1).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(bound.bytes,limits.max_file_bytes);assert!(bound.repeated(usize::MAX,2).is_err());bound.finish().unwrap();
        let mut control=SqliteSnapshotControl::new(&mut accepted,super::super::SqliteDatabaseLimits{max_file_bytes:limits.max_file_bytes-1,..limits});let mut short=NativeEncodingBound::file_only(&mut control).unwrap();assert!(short.add(limits.max_file_bytes).is_err());assert_eq!(short.bytes,0);
        let mut denied=|_|false;let mut control=SqliteSnapshotControl::new(&mut denied,limits);assert!(NativeEncodingBound::file_only(&mut control).is_err());
        let mut calls=0;let mut cancellation=|_|{calls+=1;calls==1};let mut control=SqliteSnapshotControl::new(&mut cancellation,super::super::SqliteDatabaseLimits::default());let mut canceled=NativeEncodingBound::file_only(&mut control).unwrap();assert!(canceled.add(65_537).is_err());assert_eq!(canceled.bytes,0);
    }
    #[test]
    fn reconstruction_bounds_repeated_foreign_key_copies_and_large_copy_cancellation() {
        let mut limits = super::super::SqliteDatabaseLimits::default(); limits.max_value_bytes = 8;
        let mut callback = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut callback, limits);
        let mut restore = Reconstruction::new(&mut control).unwrap();
        assert_eq!(restore.text("same").unwrap(), "same");
        assert_eq!(restore.text("same").unwrap(), "same");
        assert!(restore.text("same").is_err());
        let bytes = vec![0; 100_000]; let mut first = true;
        let mut callback = |_| { let result = first; first = false; result };
        let mut control = SqliteSnapshotControl::new(&mut callback, super::super::SqliteDatabaseLimits::default());
        let mut restore = Reconstruction::new(&mut control).unwrap();
        assert!(restore.blob(&bytes).is_err()); assert_eq!(restore.bytes, 0);
    }
    #[test]
    fn borrowed_projection_bounds_values_before_copy_and_checks_cancellation() {
        let sql = "CREATE TABLE explicit_entity (id INTEGER PRIMARY KEY, label TEXT NOT NULL)";
        let mut limits = super::super::SqliteDatabaseLimits::default(); limits.max_value_bytes = 9;
        let mut callback = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut callback, limits);
        let mut projection = Projection::new(sql, &mut control).unwrap();
        assert!(projection.insert("explicit_entity", &[Cell::Text("too long")]).is_err());
        let mut control = SqliteSnapshotControl::new(&mut callback, super::super::SqliteDatabaseLimits::default());
        let mut projection = Projection::new(sql, &mut control).unwrap();
        projection.insert("explicit_entity", &[Cell::Text("a")]).unwrap();
        let db = projection.finish().unwrap(); assert_eq!(db.table("explicit_entity").unwrap().single_row().unwrap().text(1).unwrap(), "a");
        let payload = "a".repeat(100_000);
        let armed = std::cell::Cell::new(false);
        let mut progress = |_| !armed.get();
        let mut control = SqliteSnapshotControl::new(&mut progress, super::super::SqliteDatabaseLimits::default());
        let mut projection = Projection::new(sql, &mut control).unwrap();
        armed.set(true);
        assert_eq!(projection.insert("explicit_entity", &[Cell::Text(&payload)]).unwrap_err().kind, ValueRefusalKind::Canceled);
        assert!(projection.database.table("explicit_entity").unwrap().rows.is_empty());
        assert!(Projection::new(sql, &mut SqliteSnapshotControl::new(&mut |_| false, super::super::SqliteDatabaseLimits::default())).is_err());
    }
}

#[cfg(test)]
mod ieee_tests {
    use super::*;
    use super::super::{SqliteDatabaseLimits,export_sqlite_database,import_sqlite_database};
    const SQL:&str="CREATE TABLE typed_scalar (id INTEGER PRIMARY KEY, value64 REAL, value32 REAL, value64_ieee754_bits INTEGER, value64_numeric_class TEXT, value32_ieee754_bits INTEGER, value32_numeric_class TEXT)";
    const COLUMNS:&[FloatColumn]=&[FloatColumn::Binary64(1),FloatColumn::Binary32(2)];
    fn bits()->(Vec<u64>,Vec<u32>){(vec![0,0x8000000000000000,1,0x000fffffffffffff,0x7ff0000000000000,0xfff0000000000000,0x7ff8000000000011,0x7ff0000000000007],vec![0,0x80000000,1,0x007fffff,0x7f800000,0xff800000,0x7fc00011,0x7f800007])}
    fn fixture()->SqliteDatabase{let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut projection=Projection::new(SQL,&mut control).unwrap();let(a,b)=bits();for(a,b)in a.into_iter().zip(b){insert_ieee754(&mut projection,"typed_scalar",&[Cell::Real(f64::from_bits(a)),Cell::Float32(f32::from_bits(b))],COLUMNS).unwrap();}insert_ieee754(&mut projection,"typed_scalar",&[Cell::Null,Cell::Null],COLUMNS).unwrap();projection.finish().unwrap()}
    fn law(db:&SqliteDatabase){let(a,b)=bits();for(row,(a,b))in db.table("typed_scalar").unwrap().rows.iter().zip(a.into_iter().zip(b)){let row=FloatRow::new(row,COLUMNS).unwrap();assert_eq!(row.real(1).unwrap().to_bits(),a);assert_eq!(row.binary32(2).unwrap().to_bits(),b);assert!(!row.is_null(1).unwrap());assert!(!row.is_null(2).unwrap());}let row=FloatRow::new(db.table("typed_scalar").unwrap().rows.last().unwrap(),COLUMNS).unwrap();assert!(row.is_null(1).unwrap());assert!(row.is_null(2).unwrap());}
    #[test]
    fn authored_ieee_fields_preserve_signed_zero_infinities_subnormals_and_nan_payloads(){let db=fixture();law(&db);let bytes=export_sqlite_database(&db,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();law(&import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap());}
    #[test]
    fn authored_ieee_fields_reject_inconsistent_class_bits_query_and_optional_presence(){let original=fixture();for change in 0..4{let mut db=original.clone();let row=&mut db.table_mut("typed_scalar").unwrap().rows[0];match change{0=>row.values[1]=SqliteValue::Real(1.0),1=>row.values[4]=SqliteValue::Text("nan".into()),2=>row.values[5]=SqliteValue::Integer(i64::MAX),_=>row.values[3]=SqliteValue::Null}assert!(FloatRow::new(row,COLUMNS).is_err());}}
    #[test]
    fn authored_ieee_integer_queries_reject_rounding_against_independent_sqlite(){
        use std::io::Write;
        use std::process::{Command,Stdio};
        let input=include_str!("../🔢️ieee754/🧫️fixtures/🎯️integer-query.json");
        let script="import{Database}from'bun:sqlite';const{cases}=await Bun.stdin.json(),db=new Database(':memory:',{safeIntegers:true}),view=new DataView(new ArrayBuffer(8));db.run('CREATE TABLE exact_integer(id INTEGER PRIMARY KEY,value)');for(const[i,c]of cases.entries()){const value=BigInt(c.integer);db.run('INSERT INTO exact_integer VALUES(?,?)',[i+1,value]);if(db.query('SELECT value FROM exact_integer WHERE id=?').get(i+1).value!==value)throw Error('integer identity');view.setBigUint64(0,BigInt('0x'+c.binary64Bits));if((BigInt(view.getFloat64(0))===value)!==c.accept64)throw Error('binary64 oracle');view.setUint32(0,parseInt(c.binary32Bits,16));if((BigInt(view.getFloat32(0))===value)!==c.accept32)throw Error('binary32 oracle');}db.close();await Bun.write(Bun.stdout,String(cases.length));";
        let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
        let output=child.wait_with_output().unwrap();
        assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(),"8");
        eprintln!("[DEBUG] Independent SQLite and DataView validated eight exact integer query cases");
        let fixture:serde_json::Value=serde_json::from_str(input).unwrap();
        for case in fixture["cases"].as_array().unwrap(){
            let integer=case["integer"].as_str().unwrap().parse::<i64>().unwrap();
            let word64=u64::from_str_radix(case["binary64Bits"].as_str().unwrap(),16).unwrap();
            let word32=u32::from_str_radix(case["binary32Bits"].as_str().unwrap(),16).unwrap();
            let row=SqliteRow{rowid:1,values:vec![SqliteValue::Integer(1),SqliteValue::Integer(integer),SqliteValue::Integer(integer),SqliteValue::Integer(word64 as i64),SqliteValue::Text("finite".into()),SqliteValue::Integer(i64::from(word32)),SqliteValue::Text("finite".into())]};
            assert_eq!(read_binary64(&row,1,COLUMNS).is_ok(),case["accept64"].as_bool().unwrap(),"binary64 query {integer}");
            assert_eq!(read_binary32(&row,2,COLUMNS).is_ok(),case["accept32"].as_bool().unwrap(),"binary32 query {integer}");
        }
    }
    #[test]
    fn authored_ieee_fields_are_independently_queryable_sqlite_scalars(){use std::io::Write;use std::process::{Command,Stdio};let bytes=export_sqlite_database(&fixture(),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let script="import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');if(db.query(\"SELECT count(*) AS n FROM typed_scalar WHERE value64_numeric_class='nan' AND value64 IS NULL AND typeof(value64_ieee754_bits)='integer'\").get().n!==2)throw Error('NaN scalar fields');if(db.query(\"SELECT count(*) AS n FROM typed_scalar WHERE value32_numeric_class='finite' AND typeof(value32)='real'\").get().n!==4)throw Error('finite scalars');await Bun.write(Bun.stdout,db.serialize());db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));law(&import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap());}
}

#[cfg(test)]
#[path="🧪️tests/🫳️reconstruction/🦀️.rs"]
mod reconstruction_copy_tests;

#[cfg(test)]
#[path="🧪️tests/🧮️allocation/🦀️.rs"]
mod shared_provider_allocation_tests;
