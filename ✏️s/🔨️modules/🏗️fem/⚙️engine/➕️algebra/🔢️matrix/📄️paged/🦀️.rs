//! 📄️ Dense numerical cells use the shared original paged owner independently of logical dimensions.
use semio_framework_value::list::PagedList;
#[derive(Clone,Debug,PartialEq)]
pub struct PagedMatD{pub rows:usize,pub cols:usize,pub data:PagedList<f64,{usize::MAX}>}
impl PagedMatD{
 /// ❄️ Allocates mathematical cells without issuing a native execution authority.
 pub fn zeros(rows:usize,cols:usize)->Self{let cells=rows.checked_mul(cols).expect("dense mathematical dimensions overflow");Self{rows,cols,data:(0..cells).map(|_|0.0).collect()}}
 pub fn empty()->Self{Self{rows:0,cols:0,data:PagedList::empty()}}
 pub fn get(&self,row:usize,col:usize)->f64{self.data[row*self.cols+col]}
 pub fn set(&mut self,row:usize,col:usize,value:f64){self.data[row*self.cols+col]=value;}
 pub fn add_at(&mut self,row:usize,col:usize,value:f64){self.data[row*self.cols+col]+=value;}
}
#[cfg(test)]
impl From<super::MatD> for PagedMatD{fn from(matrix:super::MatD)->Self{Self{rows:matrix.rows,cols:matrix.cols,data:matrix.data.into_iter().collect()}}}
