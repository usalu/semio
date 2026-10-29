//! 📖️ Borrowed octet observation is independent of contiguous storage and original custody remains with the caller.
pub use semio_framework_value_control_contract::{ByteReadSource,ByteReadRange};
impl<const N:usize> ByteReadSource for super::PagedBytes<N>{fn byte_len(&self)->usize{self.len()}fn byte_at(&self,index:usize)->Option<u8>{self.retained_bytes().get(index).copied()}fn read_bytes(&self,offset:usize,output:&mut[u8])->usize{let count=self.len().saturating_sub(offset).min(output.len());let mut written=0;while written<count{let Some(page)=self.retained_bytes().contiguous_at(offset+written)else{break};let length=page.len().min(count-written);output[written..written+length].copy_from_slice(&page[..length]);written+=length;}written}}

