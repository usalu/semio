//! 🧩️ Relational row controls for explicitly authored artifact projections.
use super::{SqliteDatabase, SqliteRow, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase, ValueError, ValueRefusalKind};
#[path="🫴️receiving/🦀️.rs"]
pub mod receiving;

/// 📏️ Accumulates an owner's explicit native allocation and encoded output upper bounds.
#[doc(hidden)] pub struct NativeEncodingBound<'c, 'p> { control: &'c mut SqliteSnapshotControl<'p>, #[doc(hidden)] pub bytes: usize, units: usize, semantic:bool }

impl<'c, 'p> NativeEncodingBound<'c, 'p> {

    /// 🗃️ Pays actual borrowed forecast storage independently from the unpaid output bound.
    pub fn allocate_frontier<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{self.checkpoint()?;super::transfer::reserve(count,self.control)}
    /// 🔏️ Admits a forecast walk's actual replacement buffer before moving its references.
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
    /// ✅️ Bounds an owner's explicitly predicted entity count before traversal allocation.
    pub fn check_rows(&self, count: usize) -> Result<(), ValueError> { self.control.check_rows(count) }
    /// ⏱️ Checks cancellation during a borrowed walk before its next emitted estimate.
    pub fn checkpoint(&mut self) -> Result<(), ValueError> { self.control.checkpoint(SqliteSnapshotPhase::EncodeNative, self.units, 0) }
    /// ✔️ Checks cancellation immediately before returning admission to the encoder.
    pub fn finish(self) -> Result<(), ValueError> { self.control.checkpoint(SqliteSnapshotPhase::EncodeNative, self.units, self.units) }
}

/// 🫳️ Borrowed storage-class cells are bounded before creating owned copies.
#[derive(Clone, Copy)]
pub enum Cell<'a> { Null, Integer(i64), Real(f64), Float32(f32), Text(&'a str), PagedText(&'a dyn semio_framework_value::paged::Utf8Text), Blob(&'a [u8]) }

#[path="🧮️rows/🦀️.rs"]
mod row_writer;
pub use row_writer::RowWriter;

#[path="🫙️projection/🦀️.rs"]
pub mod projection;
pub use projection::{ProjectionStorage,project_owned,owned_workspace};

#[path="🧵️paged-text/🦀️.rs"]
mod paged_text;

/// 🪝️ Test hook: the private row writer, public only behind the `testkit` feature.
#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub mod row_writer {
        pub use super::super::row_writer::*;
    }
}

impl Cell<'_> {
    fn bytes(self) -> usize { match self { Self::Null => 0, Self::Integer(_) | Self::Real(_) | Self::Float32(_) => 8, Self::Text(value) => value.len(), Self::PagedText(value) => value.text_bytes(), Self::Blob(value) => value.len() } }
    #[doc(hidden)] pub fn owned(self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteValue, ValueError> { Ok(match self { Self::Null => SqliteValue::Null, Self::Integer(value) => SqliteValue::Integer(value), Self::Real(value) => SqliteValue::Real(value), Self::Float32(value) => SqliteValue::Real(f64::from(value)), Self::Text(value) => SqliteValue::Text(copy_text(value, control, SqliteSnapshotPhase::ProjectSnapshot)?), Self::PagedText(value) => SqliteValue::Text(paged_text::copy_text(value, control, SqliteSnapshotPhase::ProjectSnapshot)?), Self::Blob(value) => SqliteValue::Blob(copy_blob(value, control, SqliteSnapshotPhase::ProjectSnapshot)?) }) }
}

fn copy_text(text:&str,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<String,ValueError>{
 super::transfer::copy_text(text,phase,control)
}

fn copy_blob(blob:&[u8],control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<Vec<u8>,ValueError>{
 control.allocation_stage(phase,|remaining,progress,allocation|{
  let mut callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|progress(event.completed,event.total);
  let mut native_allocation=|request:semio_framework_value::native_encoding::NativeEncodeAllocation|allocation(request.bytes);let mut native=semio_framework_value::NativeEncodeControl::new_forwarded(remaining,&mut callback,&mut native_allocation);let result=native.copy_bytes(blob);(result,native.owned_bytes())
 })?
}

/// 🔤️ Copies an explicitly pre-admitted native projection field in bounded UTF-8 chunks.
pub fn project_text(control: &mut SqliteSnapshotControl<'_>, text: &str) -> Result<String, ValueError> { control.check_value_bytes(text.len())?; copy_text(text, control, SqliteSnapshotPhase::ProjectSnapshot) }

/// 🏗️ Inserts only the domain rows explicitly supplied by an artifact implementation.
#[doc(hidden)] pub struct Projection<'c, 'p> { #[doc(hidden)] pub database: SqliteDatabase, #[doc(hidden)] pub control: &'c mut SqliteSnapshotControl<'p>, rows: usize, bytes: usize }

impl<'c, 'p> Projection<'c, 'p> {

    /// 🟢️ Admits an owner's concrete borrowed frontier on the shared backing ledger.
    pub fn allocate_frontier<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,count)?;super::transfer::reserve(count,self.control)}
    /// ⏪️ Pays complete replacement backing before growing an owner's traversal frontier.
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

    /// ⏭️ Allocates the next surrogate identity in one declared entity table.
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

    /// 🚧️ Exposes the caller's declared resource bounds before domain traversal allocation.
    pub fn limits(&self) -> super::SqliteDatabaseLimits { self.control.limits() }

    /// ☑️ Bounds the domain rows this projection actually materializes.
    pub fn check_rows(&self, count: usize) -> Result<(), ValueError> { self.control.check_rows(count) }

    /// 📦️ Bounds predicted aggregate storage-class bytes before allocation.
    pub fn check_value_bytes(&self, count: usize) -> Result<(), ValueError> { self.control.check_value_bytes(count) }

    /// 🔍️ Compares full borrowed literal keys through this projection's caller control.
    pub fn compare_text(&mut self,left:&str,right:&str)->Result<std::cmp::Ordering,ValueError>{super::transfer::compare_text(left,right,SqliteSnapshotPhase::ProjectSnapshot,self.control)}

    /// 🕸️ Publishes a bounded domain traversal independently of materialized row count.
    pub fn checkpoint_work(&mut self,completed:usize,total:usize)->Result<(),ValueError>{self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,completed,total)}

    /// 🔁️ Checks cancellation around domain loops that do not emit a row immediately.
    pub fn checkpoint(&mut self) -> Result<(), ValueError> { self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, self.rows, 0) }

    /// 🧾️ Reports actual inserted rows against the owner's borrowed preflight count.
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

/// 📝️ An explicitly authored scalar column and its native IEEE width.
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
/// 🖊️ Inserts authored scalar fields with named numeric class and signed INTEGER bit companions.
pub fn insert_ieee754(projection:&mut Projection<'_,'_>,table:&str,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,ValueError>{
 float_cell_limit(projection,cells.len(),columns.len())?;let cells=float_cells(cells,columns,projection.control)?;projection.insert(table,&cells)
}
/// ⌨️ Inserts authored one-to-one scalar fields with their exact native IEEE companions.
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
/// 🖋️ Restores binary64, interpreting signed INTEGER bit storage as its exact unsigned bit pattern.
pub fn read_binary64(row:&SqliteRow,index:usize,columns:&[FloatColumn])->Result<f64,ValueError>{let(slot,column)=float_slot(row,index,columns)?;if !matches!(column,FloatColumn::Binary64(_)){return Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected an authored binary64 scalar"));}let value=f64::from_bits(row.integer(slot)? as u64);check_float(row,index,slot,value)?;Ok(value)}
/// 🔈️ Restores binary32 directly from its own bits without quieting signaling NaNs through a binary64 cast.
pub fn read_binary32(row:&SqliteRow,index:usize,columns:&[FloatColumn])->Result<f32,ValueError>{let(slot,column)=float_slot(row,index,columns)?;if !matches!(column,FloatColumn::Binary32(_)){return Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected an authored binary32 scalar"));}let value=f32::from_bits(u32::try_from(row.integer(slot)?).map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, error.to_string()))?);check_float(row,index,slot,f64::from(value))?;Ok(value)}
/// 🛡️ Validates authored scalar companions before restoring any owned native fields.
pub fn validate_ieee754_row(row:&SqliteRow,original_columns:usize,columns:&[FloatColumn])->Result<(),ValueError>{if row.values.len()!=original_columns.checked_add(columns.len().checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "IEEE column count overflow"))?).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "IEEE column count overflow"))?{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "authored IEEE row column count differs"));}for column in columns{if ieee754_is_null(row,column.index(),columns)?{continue;}match column{FloatColumn::Binary64(index)=>{read_binary64(row,*index,columns)?;},FloatColumn::Binary32(index)=>{read_binary32(row,*index,columns)?;}}}Ok(())}

/// 🪟️ A borrowed view of explicitly authored scalar columns retains logical field positions.
#[derive(Clone,Copy)]
pub struct FloatRow<'a> { pub rowid: i64, pub values: &'a [SqliteValue], row: &'a SqliteRow, columns: &'static [FloatColumn] }

impl<'a> FloatRow<'a> {
    /// 🧲️ Binds only the caller's declared native scalar columns and validates their companions.
    pub fn new(row:&'a SqliteRow,columns:&'static [FloatColumn])->Result<Self,ValueError>{let original=row.values.len().checked_sub(columns.len().checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "IEEE column count overflow"))?).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "missing IEEE companion columns"))?;validate_ieee754_row(row,original,columns)?;Ok(Self{rowid:row.rowid,values:&row.values[..original],row,columns})}
    /// 🔢️ Reads a native INTEGER field from the declared logical row.
    pub fn integer(self,index:usize)->Result<i64,ValueError>{self.row.integer(index)}
    /// 💎️ Reads a native binary64 field from exact bits and validates its query value.
    pub fn real(self,index:usize)->Result<f64,ValueError>{if self.columns.iter().any(|c|c.index()==index){read_binary64(self.row,index,self.columns)}else{self.row.real(index)}}
    /// 💻️ Reads a native binary32 field without intermediate NaN conversion.
    pub fn binary32(self,index:usize)->Result<f32,ValueError>{read_binary32(self.row,index,self.columns)}
    /// 💬️ Borrows one native text field without creating ownership.
    pub fn text(self,index:usize)->Result<&'a str,ValueError>{self.row.text(index)}
    /// 🆕️ Borrows one native binary field without creating ownership.
    pub fn blob(self,index:usize)->Result<&'a [u8],ValueError>{self.row.blob(index)}
    /// ❓️ Borrows an optional native text field with exact presence.
    pub fn optional_text(self,index:usize)->Result<Option<&'a str>,ValueError>{self.row.optional_text(index)}
    /// 🧪️ Tests native optional presence independently of a NaN's NULL query scalar.
    pub fn is_null(self,index:usize)->Result<bool,ValueError>{if self.columns.iter().any(|c|c.index()==index){ieee754_is_null(self.row,index,self.columns)}else{Ok(self.values.get(index)==Some(&SqliteValue::Null))}}
}

/// 🪪️ Declares the actual authored surrogate identity domain.
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum RowIdentity{Signed,Positive}

/// 🗂️ Pays borrowed row positions and explicit consumption marks before relational reconstruction.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct RowIndexStorage{indices:Vec<usize>,used:Vec<u8>,remaining:usize}
impl RowIndexStorage{pub fn empty()->Self{Self{indices:Vec::new(),used:Vec::new(),remaining:0}}}

/// 📇️ Borrows a database and its original caller-owned scalar reconstruction storage.
pub struct RowIndex<'a>{table:&'a super::SqliteTable,columns:&'static[FloatColumn],storage:&'a mut RowIndexStorage}
impl std::ops::Deref for RowIndex<'_>{type Target=RowIndexStorage;fn deref(&self)->&RowIndexStorage{self.storage}}
impl std::ops::DerefMut for RowIndex<'_>{fn deref_mut(&mut self)->&mut RowIndexStorage{self.storage}}
impl<'a> RowIndex<'a>{
    /// 🛂️ Validates one authored row family and admits both scalar index buffers before allocation.
    pub fn new(storage:&'a mut RowIndexStorage,db:&'a SqliteDatabase,table_name:&str,logical_columns:usize,columns:&'static[FloatColumn],control:&mut SqliteSnapshotControl<'_>,message:&str)->Result<Self,ValueError>{Self::new_with_identity(storage,db,table_name,logical_columns,columns,RowIdentity::Positive,control,message)}
    /// 🔑️ Applies the caller's actual signed or positive surrogate identity declaration.
    pub fn new_with_identity(storage:&'a mut RowIndexStorage,db:&'a SqliteDatabase,table_name:&str,logical_columns:usize,columns:&'static[FloatColumn],identity:RowIdentity,control:&mut SqliteSnapshotControl<'_>,message:&str)->Result<Self,ValueError>{
        if storage.indices.capacity()!=0||storage.used.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"row index storage already owns backing"));}let table=db.table(table_name)?;let total=table.rows.len();let phase=SqliteSnapshotPhase::ReconstructSnapshot;control.check_rows(total)?;control.checkpoint(phase,0,total)?;
        storage.indices=super::transfer::reserve(total,control)?;storage.used=super::transfer::reserve(total,control)?;let(indices,used)=(&mut storage.indices,&mut storage.used);
        for(index,raw)in table.rows.iter().enumerate(){control.checkpoint(phase,index,total)?;let row=FloatRow::new(raw,columns)?;if row.values.len()!=logical_columns||(identity==RowIdentity::Positive&&row.rowid<=0)||row.integer(0)?!=row.rowid{return Err(ValueError::new(ValueRefusalKind::InvalidValue,message));}indices.push(index);used.push(0);}
        super::transfer::heap_sort(indices,phase,control,|a,b,_|Ok(table.rows[*a].rowid.cmp(&table.rows[*b].rowid)))?;
        for(index,pair)in indices.windows(2).enumerate(){control.checkpoint(phase,index,total)?;if table.rows[pair[0]].rowid==table.rows[pair[1]].rowid{return Err(ValueError::new(ValueRefusalKind::InvalidValue,message));}}
        control.checkpoint(phase,total,total)?;storage.remaining=total;Ok(Self{table,columns,storage})
    }
    /// ✏️ Reports the actual authored source row count.
    pub fn len(&self)->usize{self.indices.len()}
    /// 📊️ Reports rows not yet consumed by explicit native ownership.
    pub fn remaining(&self)->usize{self.remaining}
    /// 🧭️ Borrows paid row positions ordered by actual primary identity.
    pub fn indices(&self)->&[usize]{&self.indices}
    /// 🔬️ Wraps one actual source row without allocating or owning its fields.
    pub fn row(&self,source_index:usize)->Result<FloatRow<'a>,ValueError>{FloatRow::new(self.table.rows.get(source_index).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"row index is outside its authored table"))?,self.columns)}
    /// 📍️ Resolves one authored identity through bounded scalar binary search.
    pub fn position(&self,id:i64,control:&mut SqliteSnapshotControl<'_>)->Result<Option<usize>,ValueError>{
        let(mut low,mut high)=(0,self.indices.len());while low<high{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,low,self.len())?;let middle=low+(high-low)/2;let position=self.indices[middle];match self.table.rows[position].rowid.cmp(&id){std::cmp::Ordering::Less=>low=middle+1,std::cmp::Ordering::Greater=>high=middle,std::cmp::Ordering::Equal=>return Ok(Some(position))}}Ok(None)
    }
    /// 📖️ Borrows a matching row independently of whether ownership was consumed.
    pub fn get(&self,id:i64,control:&mut SqliteSnapshotControl<'_>)->Result<Option<FloatRow<'a>>,ValueError>{self.position(id,control)?.map(|index|self.row(index)).transpose()}
    /// 🫴️ Consumes a matching identity at most once without copying its source cells.
    pub fn take(&mut self,id:i64,control:&mut SqliteSnapshotControl<'_>)->Result<Option<FloatRow<'a>>,ValueError>{match self.position(id,control)?{Some(index)=>self.take_index(index,control),None=>Ok(None)}}
    /// 🏷️ Marks a source row consumed only after its borrowed wrapper succeeds.
    pub fn take_index(&mut self,index:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Option<FloatRow<'a>>,ValueError>{
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,self.len())?;let row=self.row(index)?;if self.used[index]!=0{return Ok(None)}self.used[index]=1;self.remaining-=1;Ok(Some(row))
    }
    /// 📋️ Admits an independent scalar position buffer before copying its entries.
    pub fn all_indices_into(&self,result:&mut Vec<usize>,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
        if result.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"row positions already own backing"));}*result=super::transfer::reserve(self.len(),control)?;for(index,position)in self.indices.iter().enumerate(){control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,self.len())?;result.push(*position);}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.len(),self.len())?;Ok(())
    }
    /// 📐️ Orders and validates an entire authored contiguous ordinal relationship.
    pub fn ordered_into(&self,result:&mut Vec<usize>,ordinal:usize,control:&mut SqliteSnapshotControl<'_>,message:&str)->Result<(),ValueError>{
        self.grouped_by_into(result,ordinal,control,message,|_|Ok((0,None)))
    }
    /// 👥️ Orders fixed scalar owner keys and validates contiguous ordinals within every group.
    pub fn grouped_by_into(&self,result:&mut Vec<usize>,ordinal:usize,control:&mut SqliteSnapshotControl<'_>,message:&str,key:impl Fn(FloatRow<'a>)->Result<(u8,Option<i64>),ValueError>)->Result<(),ValueError>{
        self.all_indices_into(result,control)?;let phase=SqliteSnapshotPhase::ReconstructSnapshot;
        super::transfer::heap_sort(result,phase,control,|a,b,_|{let left=self.row(*a)?;let right=self.row(*b)?;Ok(key(left)?.cmp(&key(right)?).then(left.integer(ordinal)?.cmp(&right.integer(ordinal)?)))})?;
        let mut previous=None;let mut expected=0i64;for(index,position)in result.iter().enumerate(){control.checkpoint(phase,index,result.len())?;let row=self.row(*position)?;let owner=key(row)?;if previous!=Some(owner){previous=Some(owner);expected=0;}if row.integer(ordinal)?!=expected{return Err(ValueError::new(ValueRefusalKind::InvalidValue,message));}expected=expected.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"relationship ordinal overflow"))?;}
        control.checkpoint(phase,result.len(),result.len())?;Ok(())
    }
    /// 🔧️ Finds a previously ordered fixed scalar owner group without allocating.
    pub fn range_by(&self,indices:&[usize],owner:(u8,Option<i64>),control:&mut SqliteSnapshotControl<'_>,key:impl Fn(FloatRow<'a>)->Result<(u8,Option<i64>),ValueError>)->Result<std::ops::Range<usize>,ValueError>{
        let(mut low,mut high)=(0,indices.len());while low<high{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,low,indices.len())?;let middle=low+(high-low)/2;if key(self.row(indices[middle])?)?<owner{low=middle+1}else{high=middle}}let start=low;high=indices.len();while low<high{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,low,indices.len())?;let middle=low+(high-low)/2;if key(self.row(indices[middle])?)?<=owner{low=middle+1}else{high=middle}}Ok(start..low)
    }
    /// 🆚️ Pays selected positions and compares full borrowed text keys through caller cancellation.
    pub fn unique_text(&self,ordered:&mut Vec<usize>,indices:&[usize],column:usize,control:&mut SqliteSnapshotControl<'_>,message:&str)->Result<(),ValueError>{
        let phase=SqliteSnapshotPhase::ReconstructSnapshot;if ordered.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"row uniqueness storage already owns backing"));}*ordered=super::transfer::reserve(indices.len(),control)?;
        for(index,position)in indices.iter().enumerate(){control.checkpoint(phase,index,indices.len())?;self.row(*position)?.text(column)?;ordered.push(*position);}
        super::transfer::heap_sort(ordered,phase,control,|a,b,c|super::transfer::compare_text(self.row(*a)?.text(column)?,self.row(*b)?.text(column)?,phase,c))?;
        for(index,pair)in ordered.windows(2).enumerate(){control.checkpoint(phase,index,ordered.len())?;if super::transfer::compare_text(self.row(pair[0])?.text(column)?,self.row(pair[1])?.text(column)?,phase,control)?.is_eq(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,message));}}
        control.checkpoint(phase,ordered.len(),ordered.len())
    }
    /// 🌳️ Pays parent links and resolves each non-null foreign identity against this row family.
    pub fn parent_positions_into(&self,parents:&mut Vec<(Option<usize>,u8)>,column:usize,control:&mut SqliteSnapshotControl<'_>,message:&str)->Result<(),ValueError>{
        if parents.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"row parent storage already owns backing"));}*parents=super::transfer::reserve(self.len(),control)?;for index in 0..self.len(){control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,self.len())?;let row=self.row(index)?;let parent=if row.is_null(column)?{None}else{Some(self.position(row.integer(column)?,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,message))?)};parents.push((parent,0));}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.len(),self.len())?;Ok(())
    }
    /// 🔄️ Checks paid scalar parent links with two bounded walks and no additional frontier.
    pub fn cycles(parents:&mut[(Option<usize>,u8)],control:&mut SqliteSnapshotControl<'_>,message:&str)->Result<(),ValueError>{
        let total=parents.len();for index in 0..total{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,total)?;if parents[index].1==2{continue}let mut next=Some(index);while let Some(position)=next{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,total)?;let entry=parents.get_mut(position).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,message))?;match entry.1{0=>{entry.1=1;next=entry.0},1=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,message)),2=>break,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,message))}}let mut next=Some(index);while let Some(position)=next{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,total)?;let entry=parents.get_mut(position).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,message))?;if entry.1!=1{break}entry.1=2;next=entry.0;}}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,total,total)
    }
}

/// 🪄️ Bounds explicitly restored native fields before creating owned copies.
#[doc(hidden)] pub struct Reconstruction<'c, 'p> { control: &'c mut SqliteSnapshotControl<'p>, #[doc(hidden)] pub bytes: usize, units: usize }

/// ♻️ Copies one explicitly selected native text field within the transfer's ownership budget.
pub fn reconstruct_text(control: &mut SqliteSnapshotControl<'_>, text: &str) -> Result<String, ValueError> {
    Reconstruction { control, bytes: 0, units: 0 }.text(text)
}

/// 🦜️ Copies one explicitly selected native binary field within the transfer's ownership budget.
pub fn reconstruct_blob(control: &mut SqliteSnapshotControl<'_>, blob: &[u8]) -> Result<Vec<u8>, ValueError> {
    Reconstruction { control, bytes: 0, units: 0 }.blob(blob)
}

/// 🧶️ Fills an original caller-owned text field before every interior cancellation checkpoint.
pub fn reconstruct_text_into(output:&mut String,control:&mut SqliteSnapshotControl<'_>,text:&str)->Result<(),ValueError>{
    if output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"SQLite text destination must have no existing allocation"));}
    Reconstruction::new(control)?.reserve(text.len())?;control.admit_allocation_bytes(text.len())?;
    output.try_reserve_exact(text.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"SQLite text allocation failed"))?;
    let mut position=0;while position<text.len(){let mut end=position.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end+=1;}output.push_str(&text[position..end]);position=end;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,text.len())?;}
    control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,text.len(),text.len())
}

/// 🧊️ Fills an original caller-owned octet field before every interior cancellation checkpoint.
pub fn reconstruct_blob_into(output:&mut Vec<u8>,control:&mut SqliteSnapshotControl<'_>,blob:&[u8])->Result<(),ValueError>{
    if output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"SQLite blob destination must have no existing allocation"));}
    Reconstruction::new(control)?.reserve(blob.len())?;control.admit_allocation_bytes(blob.len())?;
    output.try_reserve_exact(blob.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"SQLite blob allocation failed"))?;
    for bytes in blob.chunks(65536){output.extend_from_slice(bytes);control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,output.len(),blob.len())?;}
    control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,blob.len(),blob.len())
}

/// 🔃️ Validates and orders authored ordinal relationships with bounded cancellation checkpoints.
pub fn ordered_row_refs<'a>(table:&'a super::SqliteTable,ordinal:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
 let phase=SqliteSnapshotPhase::ReconstructSnapshot;let total=table.rows.len();control.check_rows(total)?;control.checkpoint(phase,0,total)?;let mut rows=super::transfer::reserve(total,control)?;
 for(count,row)in table.rows.iter().enumerate(){let index=usize::try_from(row.integer(ordinal)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;if index>=total{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"relationship ordinals must be contiguous and unique"));}rows.push(row);if(count+1)%256==0{control.checkpoint(phase,count+1,total)?;}}
 super::transfer::heap_sort(&mut rows,phase,control,|a,b,_|Ok(a.integer(ordinal)?.cmp(&b.integer(ordinal)?)))?;
 for(expected,row)in rows.iter().enumerate(){if row.integer(ordinal)?!=expected as i64{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"relationship ordinals must be contiguous and unique"));}if(expected+1)%256==0{control.checkpoint(phase,expected+1,total)?;}}
 control.checkpoint(phase,total,total)?;Ok(rows)
}

impl<'c, 'p> Reconstruction<'c, 'p> {
    /// 📒️ Starts native ownership accounting independently of database storage accounting.
    pub fn new(control: &'c mut SqliteSnapshotControl<'p>) -> Result<Self, ValueError> {
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, 0)?;
        Ok(Self { control, bytes: 0, units: 0 })
    }
    fn reserve(&mut self, count: usize) -> Result<(), ValueError> {
        let(bytes,units)=self.control.reserve_reconstruction(count)?;self.bytes=bytes;self.units=units;Ok(())
    }
    /// 📄️ Checks the aggregate native byte bound before copying one borrowed text field.
    pub fn text(&mut self, text: &str) -> Result<String, ValueError> { self.reserve(text.len())?; copy_text(text, self.control, SqliteSnapshotPhase::ReconstructSnapshot) }
    /// 🧵️ Constructs native paged text with each actual chunk and backing page admitted independently.
    pub fn paged_text(&mut self, text: &str) -> Result<semio_framework_value::paged::PagedUtf8<{usize::MAX}>, ValueError> {
        self.reserve(text.len())?;
        self.control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot, |remaining, progress,allocation|{
            let mut callback = |event: semio_framework_value_native_owned::native_decoding::NativeDecodeProgress| progress(event.completed, event.total);
            let mut native_allocation=|request:semio_framework_value_native_owned::native_decoding::NativeDecodeAllocation|allocation(request.bytes);let mut native=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_forwarded(remaining,&mut callback,&mut native_allocation);
            let result = semio_framework_value::paged::PagedUtf8::try_from_str_controlled(text, &mut native);
            (result, native.owned_bytes())
        })?
    }
    /// 🎒️ Checks the aggregate native byte bound before copying one borrowed binary field.
    pub fn blob(&mut self, blob: &[u8]) -> Result<Vec<u8>, ValueError> { self.reserve(blob.len())?; copy_blob(blob, self.control, SqliteSnapshotPhase::ReconstructSnapshot) }
    /// 👤️ Accounts for one explicitly restored numeric or boolean field.
    pub fn scalar(&mut self) -> Result<(), ValueError> { self.reserve(8) }
    /// 🩺️ Checks cancellation around relationship traversal and prior to returning native ownership.
    pub fn checkpoint(&mut self) -> Result<(), ValueError> { self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, self.units, 0) }
}
