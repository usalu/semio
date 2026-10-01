//! 🔢️ Explicit DXF binary64 field companions and logical row access.
use super::*;
use sqlite_snapshot::artifact::{FloatColumn::Binary64 as F,FloatRow,insert_ieee754};
pub(super) type Row<'a>=FloatRow<'a>;
pub(super) fn columns(table:&str)->&'static[sqlite_snapshot::artifact::FloatColumn]{match table{
    "dxf_header"=>&[F(8),F(9),F(10),F(11)],
    "dxf_block"=>&[F(4),F(5),F(6)],
    "dxf_entity"=>&[F(6),F(7),F(8),F(9),F(12),F(13),F(14),F(15)],
    "dxf_entity_point"=>&[F(4),F(5),F(6)],
    "dxf_vertex"=>&[F(3),F(4),F(5),F(6)],
    "dxf_group_code"=>&[F(13),F(14),F(15),F(16)],
    _=>&[],
}}
pub(super) struct Projection<'c,'p>{inner:sqlite_snapshot::artifact::Projection<'c,'p>}
impl<'c,'p> Projection<'c,'p>{
    pub fn new(sql:&str,control:&'c mut Control<'p>)->Result<Self,String>{Ok(Self{inner:sqlite_snapshot::artifact::Projection::new(sql,control)?})}
    pub fn insert(&mut self,table:&str,cells:&[C<'_>])->Result<i64,String>{insert_ieee754(&mut self.inner,table,cells,columns(table))}
    pub fn finish(self)->Result<Db,String>{self.inner.finish()}
}
