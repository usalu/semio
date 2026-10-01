//! 🔢️ Exact native binary64 fields of the Note document and its owned entities.
use super::*;
use sqlite_snapshot::artifact::{FloatColumn,FloatRow,insert_ieee754,insert_key_ieee754};
pub(super) type Row<'a>=FloatRow<'a>;
pub(super) fn columns(table:&str)->&'static[FloatColumn]{
    use FloatColumn::Binary64 as F;
    match table{
        "note_document"=>&[F(5),F(6),F(7),F(9),F(10),F(11)],
        "note_asset"=>&[F(5),F(6)],
        "note_block"=>&[F(7),F(8),F(9),F(10),F(11)],
        "note_text"=>&[F(6)],
        "note_ink"=>&[F(1),F(2),F(3),F(4),F(5)],
        "note_point"=>&[F(3),F(4)],
        _=>&[],
    }
}
pub(super) struct Projection<'c,'p>{inner:sqlite_snapshot::artifact::Projection<'c,'p>}
impl<'c,'p> Projection<'c,'p>{
    pub fn new(sql:&str,control:&'c mut Control<'p>)->Result<Self,String>{Ok(Self{inner:sqlite_snapshot::artifact::Projection::new(sql,control)?})}
    pub fn insert(&mut self,table:&str,cells:&[C<'_>])->Result<i64,String>{insert_ieee754(&mut self.inner,table,cells,columns(table))}
    pub fn insert_key(&mut self,table:&str,key:i64,cells:&[C<'_>])->Result<(),String>{insert_key_ieee754(&mut self.inner,table,key,cells,columns(table))}
    pub fn check_rows(&self,count:usize)->Result<(),String>{self.inner.check_rows(count)}
    pub fn check_value_bytes(&self,count:usize)->Result<(),String>{self.inner.check_value_bytes(count)}
    pub fn checkpoint(&mut self)->Result<(),String>{self.inner.checkpoint()}
    pub fn finish(self)->Result<Db,String>{self.inner.finish()}
}
