//! 🫳️ Typed DWG row ownership, unsigned word pairs and bounded reconstruction.
use semio_framework_os_kernel::sqlite_snapshot::{validate_sqlite_database_schema,SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase};
use std::collections::{BTreeMap,BTreeSet};
use super::number::Row;
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell;

pub(super) struct Reader<'db,'c,'p> {
    database: &'db SqliteDatabase,
    control: &'c mut SqliteSnapshotControl<'p>,
    used: BTreeMap<&'static str,BTreeSet<i64>>,
    groups: BTreeMap<(&'static str,usize,usize),BTreeMap<i64,Vec<&'db SqliteRow>>>,
    completed: usize,
    scanned: usize,
    total: usize,
}
impl<'db,'c,'p> Reader<'db,'c,'p> {
    pub(super) fn new(database: &'db SqliteDatabase,sql: &str,control: &'c mut SqliteSnapshotControl<'p>) -> Result<Self,String> {
        validate_sqlite_database_schema(database,sql,control.limits()).map_err(|error|error.to_string())?;
        control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
        let total=database.tables.iter().map(|table|table.rows.len()).sum();
        Ok(Self{database,control,used:BTreeMap::new(),groups:BTreeMap::new(),completed:0,scanned:0,total})
    }
    fn consume(&mut self,table: &'static str,row: &'db SqliteRow) -> Result<(),String> {
        if row.integer(0)?!=row.rowid||!self.used.entry(table).or_default().insert(row.rowid){return Err(format!("DWG {table} identity is mismatched or multiply owned"));}
        self.completed+=1;
        if self.completed%256==0{self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.completed,self.total)?;}
        Ok(())
    }
    pub(super) fn one(&mut self,table: &'static str) -> Result<Row<'db>,String> {
        let row=self.database.table(table)?.single_row()?;
        if row.rowid!=1{return Err(format!("DWG {table} requires identity 1"));}
        self.consume(table,row)?;
        Ok(Row::new(table,row))
    }
    pub(super) fn component(&mut self,table: &'static str,key: i64) -> Result<Row<'db>,String> {
        let row=self.database.table(table)?.rows.iter().find(|row|row.rowid==key).ok_or_else(||format!("DWG {table} typed component is missing"))?;
        self.consume(table,row)?;
        Ok(Row::new(table,row))
    }
    pub(super) fn list(&mut self,table: &'static str,owner_column: usize,owner: i64,ordinal_column: usize) -> Result<Vec<Row<'db>>,String> {
        let key=(table,owner_column,ordinal_column);
        if !self.groups.contains_key(&key) {
            let mut groups=BTreeMap::<i64,Vec<&SqliteRow>>::new();
            for row in &self.database.table(table)?.rows {
                let typed=Row::new(table,row);
                if !matches!(typed.value(owner_column)?,Cell::Null){groups.entry(typed.integer(owner_column)?).or_default().push(row);}
                self.scanned+=1;
                if self.scanned%256==0{self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.completed,self.total)?;}
            }
            self.groups.insert(key,groups);
        }
        let source=self.groups.get_mut(&key).unwrap().remove(&owner).unwrap_or_default();
        let mut ordered=vec![None;source.len()];
        for row in source {
            let ordinal=usize::try_from(Row::new(table,row).integer(ordinal_column)?).map_err(|_|"DWG ordinal must be nonnegative")?;
            if ordinal>=ordered.len()||ordered[ordinal].replace(Row::new(table,row)).is_some(){return Err(format!("DWG {table} ordinals must be contiguous"));}
            self.consume(table,row)?;
        }
        ordered.into_iter().map(|row|row.ok_or_else(||format!("DWG {table} ordinal is missing"))).collect()
    }
    pub(super) fn finish(self) -> Result<(),String> {
        for table in &self.database.tables {
            if self.used.iter().find(|(name,_)|name.eq_ignore_ascii_case(&table.name)).map_or(0,|(_,rows)|rows.len())!=table.rows.len(){return Err(format!("DWG {} has unowned rows",table.name));}
        }
        self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.total,self.total)
    }
}
pub(super) fn document(row: Row<'_>) -> Result<(),String> {if row.integer(1)?!=1{Err("DWG typed component has an unknown document".into())}else{Ok(())}}
pub(super) fn optional_text(row: Row<'_>,column: usize) -> Result<Option<String>,String> {match row.value(column)?{Cell::Null=>Ok(None),Cell::Text(value)=>Ok(Some(value.into())),_=>Err("DWG optional text must be TEXT or NULL".into())}}
pub(super) fn byte(row: Row<'_>,column: usize) -> Result<u8,String> {u8::try_from(row.integer(column)?).map_err(|error|error.to_string())}
pub(super) fn word(row: Row<'_>,column: usize) -> Result<u16,String> {u16::try_from(row.integer(column)?).map_err(|error|error.to_string())}
pub(super) fn signed_word(row: Row<'_>,column: usize) -> Result<i16,String> {i16::try_from(row.integer(column)?).map_err(|error|error.to_string())}
pub(super) fn signed_integer(row: Row<'_>,column: usize) -> Result<i32,String> {i32::try_from(row.integer(column)?).map_err(|error|error.to_string())}
pub(super) fn unsigned(row: Row<'_>,column: usize) -> Result<u32,String> {u32::try_from(row.integer(column)?).map_err(|error|error.to_string())}
pub(super) fn full_unsigned(row: Row<'_>,high: usize,low: usize) -> Result<u64,String> {Ok((u64::from(unsigned(row,high)?)<<32)|u64::from(unsigned(row,low)?))}
pub(super) fn optional_unsigned(row: Row<'_>,high: usize,low: usize) -> Result<Option<u64>,String> {match (row.value(high)?,row.value(low)?){(Cell::Null,Cell::Null)=>Ok(None),(Cell::Null,_)|(_,Cell::Null)=>Err("DWG optional unsigned handle requires two NULLs or two words".into()),_=>full_unsigned(row,high,low).map(Some)}}
pub(super) fn boolean(row: Row<'_>,column: usize) -> Result<bool,String> {match row.integer(column)?{0=>Ok(false),1=>Ok(true),_=>Err("DWG boolean must be 0 or 1".into())}}
pub(super) fn real(row: Row<'_>,column: usize) -> Result<f64,String> {row.real(column)}
pub(super) fn optional_real(row: Row<'_>,column: usize) -> Result<Option<f64>,String> {match row.value(column)?{Cell::Null=>Ok(None),_=>real(row,column).map(Some)}}
pub(super) fn optional_integer(row: Row<'_>,column: usize) -> Result<Option<u32>,String> {match row.value(column)?{Cell::Null=>Ok(None),_=>unsigned(row,column).map(Some)}}
pub(super) fn ordinal(value: usize) -> Result<i64,String> {i64::try_from(value).map_err(|error|error.to_string())}
pub(super) fn high(value: u64) -> i64 {i64::from((value>>32) as u32)}
pub(super) fn low(value: u64) -> i64 {i64::from(value as u32)}
