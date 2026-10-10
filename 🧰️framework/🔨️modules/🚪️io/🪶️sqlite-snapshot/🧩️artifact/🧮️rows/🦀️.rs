//! 🧮️ One authored cell writer supports owned SQL projection and borrowed native semantic admission.
use super::{Cell,FloatColumn,Projection,ProjectionStorage,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind};

pub enum RowWriter<'c,'p>{Original{storage:&'c mut ProjectionStorage,control:&'c mut SqliteSnapshotControl<'p>,census:bool},Owned(Projection<'c,'p>),Borrowed{control:&'c mut SqliteSnapshotControl<'p>,phase:SqliteSnapshotPhase,rows:usize,bytes:usize}}
impl<'c,'p> RowWriter<'c,'p>{
    /// 🏛️ Constructs the owner's real SQL projection through the supplied control.
    pub fn new(sql:&str,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,ValueError>{Projection::new(sql,control).map(Self::Owned)}
    /// 🔢️ Visits the same authored traversal with original table-local census identities.
    pub fn census(storage:&'c mut ProjectionStorage,sql:&str,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,ValueError>{storage.initialize(sql,control)?;Ok(Self::Original{storage,control,census:true})}
    /// 🫙️ Borrows the same original output after its exact table-row preparation.
    pub fn new_into(storage:&'c mut ProjectionStorage,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,ValueError>{Ok(Self::Original{storage,control,census:false})}
    /// 🧭️ Loans the actual caller control to separately retained original scalar scratch.
    pub fn control(&mut self)->&mut SqliteSnapshotControl<'p>{match self{Self::Original{control,..}|Self::Borrowed{control,..}=>control,Self::Owned(projection)=>projection.control}}
    /// 🧮️ Reserves the original row vectors after completing the exact table-local census.
    pub fn finish_census(self)->Result<(),ValueError>{match self{Self::Original{storage,control,census:true}=>storage.prepare_rows(control),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original census writer required"))}}
    /// 🏁️ Orders completed original output without transferring its auxiliary prefix.
    pub fn finish_into(self)->Result<(),ValueError>{match self{Self::Original{storage,control,census:false}=>storage.finish(control),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original output writer required"))}}
    /// 🫳️ Visits actual storage-class cells without constructing output or copying literals.
    pub fn borrowed(control:&'c mut SqliteSnapshotControl<'p>,phase:SqliteSnapshotPhase)->Result<Self,ValueError>{control.checkpoint(phase,0,0)?;Ok(Self::Borrowed{control,phase,rows:0,bytes:0})}
    /// 🔢️ Checks the owner's concrete predicted row count.
    pub fn check_rows(&self,count:usize)->Result<(),ValueError>{match self{Self::Original{control,..}=>control.check_rows(count),Self::Owned(value)=>value.check_rows(count),Self::Borrowed{control,..}=>control.check_rows(count)}}
    /// ⏱️ Checks cancellation at the current authored row boundary.
    pub fn checkpoint(&mut self)->Result<(),ValueError>{match self{Self::Original{control,storage,..}=>control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,storage.row_count(),0),Self::Owned(value)=>value.checkpoint(),Self::Borrowed{control,phase,rows,..}=>control.checkpoint(*phase,*rows,0)}}
    /// 🧾️ Publishes the owner's actual row boundary with its complete known workload.
    pub fn checkpoint_total(&mut self,total:usize)->Result<(),ValueError>{match self{Self::Original{control,storage,..}=>{if storage.row_count()>total{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original projection exceeds predicted rows"))}control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,storage.row_count(),total)},Self::Owned(value)=>value.checkpoint_total(total),Self::Borrowed{control,phase,rows,..}=>{if *rows>total{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"artifact projection exceeded its predicted row count"))}control.checkpoint(*phase,*rows,total)}}}
    /// 🧵️ Admits concrete borrowed traversal storage through the same caller ledger.
    pub fn allocate_frontier<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{match self{Self::Original{..}=>Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original projection requires a retained scalar frontier")),Self::Owned(value)=>value.allocate_frontier(count),Self::Borrowed{control,phase,..}=>{control.checkpoint(*phase,0,count)?;super::super::transfer::reserve(count,control)}}}
    /// 🪜️ Pays complete replacement backing before a traversal frontier grows.
    pub fn push_frontier<T>(&mut self,frontier:&mut Vec<T>,value:T)->Result<(),ValueError>{match self{Self::Original{control,..}=>{if frontier.len()==frontier.capacity(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original projection scalar frontier extent"))}control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,frontier.len(),frontier.capacity())?;frontier.push(value);Ok(())},Self::Owned(projection)=>projection.push_frontier(frontier,value),Self::Borrowed{control,phase,..}=>{super::super::transfer::grow(frontier,*phase,control)?;frontier.push(value);Ok(())}}}
    /// 🧭️ Identifies the caller's actual semantic traversal phase.
    pub fn phase(&self)->SqliteSnapshotPhase{match self{Self::Original{..}|Self::Owned(_)=>SqliteSnapshotPhase::ProjectSnapshot,Self::Borrowed{phase,..}=>*phase}}
    /// 🗂️ Sorts a paid borrowed frontier through bounded caller work.
    pub fn sort_frontier<T>(&mut self,values:&mut[T],compare:impl FnMut(&T,&T,&mut SqliteSnapshotControl<'_>)->Result<std::cmp::Ordering,ValueError>)->Result<(),ValueError>{match self{Self::Original{control,..}=>super::super::transfer::heap_sort(values,SqliteSnapshotPhase::ProjectSnapshot,control,compare),Self::Owned(projection)=>projection.sort_frontier(values,compare),Self::Borrowed{control,phase,..}=>super::super::transfer::heap_sort(values,*phase,control,compare)}}
    /// 🔎️ Compares full literal keys with the same bounded callback authority.
    pub fn compare_text(&mut self,left:&str,right:&str)->Result<std::cmp::Ordering,ValueError>{match self{Self::Original{control,..}=>super::super::transfer::compare_text(left,right,SqliteSnapshotPhase::ProjectSnapshot,control),Self::Owned(projection)=>projection.compare_text(left,right),Self::Borrowed{control,phase,..}=>super::super::transfer::compare_text(left,right,*phase,control)}}
    /// 🔗️ Writes the owner's exact one-to-one relationship cells.
    pub fn insert_key(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),ValueError>{match self{Self::Original{storage,control,census}=>if *census{storage.count_row(table,cells,&[],control).map(|_|())}else{storage.insert_key(table,key,cells,control)},Self::Owned(value)=>value.insert_key(table,key,cells),Self::Borrowed{..}=>self.admit(cells,&[]).map(|_|())}}
    /// ➕️ Writes an entity with the actual owned table identity or a borrowed scalar identity.
    pub fn insert(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>{match self{Self::Original{storage,control,census}=>if *census{storage.count_row(table,cells,&[],control)}else{storage.insert(table,cells,control)},Self::Owned(value)=>value.insert(table,cells),Self::Borrowed{..}=>self.admit(cells,&[])}}
    /// 🧬️ Writes exact raw IEEE companions for an authored entity.
    pub fn insert_float(&mut self,table:&str,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,ValueError>{match self{Self::Original{storage,control,census}=>if *census{storage.count_row(table,cells,columns,control)}else{storage.insert_float(table,cells,columns,control)},Self::Owned(value)=>super::insert_ieee754(value,table,cells,columns),Self::Borrowed{..}=>self.admit(cells,columns)}}
    /// 🧿️ Writes exact raw IEEE companions for a one-to-one entity.
    pub fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<(),ValueError>{match self{Self::Original{storage,control,census}=>if *census{storage.count_row(table,cells,columns,control).map(|_|())}else{storage.insert_key_float(table,key,cells,columns,control)},Self::Owned(value)=>super::insert_key_ieee754(value,table,key,cells,columns),Self::Borrowed{..}=>self.admit(cells,columns).map(|_|())}}
    fn admit(&mut self,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,ValueError>{
        let Self::Borrowed{control,phase,rows,bytes}=self else{unreachable!()};
        let width=cells.len().checked_add(columns.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"artifact companion column count overflow"))?).and_then(|count|count.checked_add(1)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"artifact column count overflow"))?;
        if width>control.limits().max_columns{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"authored artifact row exceeds column limit"));}
        let mut row=8usize;
        for cell in cells{row=row.checked_add(cell.bytes()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"artifact semantic cell byte count overflow"))?;}
        for(position,column)in columns.iter().copied().enumerate(){
            let index=column.index().checked_sub(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"IEEE column cannot be a row identity"))?;
            if columns[..position].iter().any(|previous|previous.index()==column.index()){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate authored IEEE scalar column"));}
            let cell=*cells.get(index).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing authored IEEE scalar field"))?;
            let value=match(column,cell){(_,Cell::Null)=>continue,(FloatColumn::Binary64(_),Cell::Real(value))=>value,(FloatColumn::Binary32(_),Cell::Float32(value))=>f64::from(value),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"native IEEE width differs from authored scalar field"))};
            if value.is_nan(){row-=8;}
            row=row.checked_add(8+super::numeric_class(value).len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"artifact IEEE semantic byte count overflow"))?;
        }
        for(index,cell)in cells.iter().enumerate(){if match cell{Cell::Real(value)=>value.is_nan(),Cell::Float32(value)=>value.is_nan(),_=>false}&&!columns.iter().any(|column|column.index()==index+1){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"NaN requires an authored IEEE scalar companion"));}}
        let count=rows.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"artifact semantic row count overflow"))?;
        let total=bytes.checked_add(row).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"artifact semantic value byte count overflow"))?;
        control.check_rows(count)?;control.check_value_bytes(total)?;control.checkpoint(*phase,count,0)?;*rows=count;*bytes=total;
        i64::try_from(count).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"artifact semantic identity exceeds integer64"))
    }
    /// 📤️ Returns the actual owned database after its physical identity ordering.
    pub fn finish(self)->Result<SqliteDatabase,ValueError>{match self{Self::Original{..}=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original projection requires retained output handoff")),Self::Owned(value)=>value.finish(),Self::Borrowed{..}=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"borrowed semantic admission has no owned database"))}}
    /// ✅️ Completes only the borrowed row walk and publishes its actual census.
    pub fn finish_borrowed(self)->Result<(),ValueError>{match self{Self::Borrowed{control,phase,rows,..}=>control.checkpoint(phase,rows,rows),Self::Original{..}|Self::Owned(_)=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"owned projection requires its database result"))}}
}
