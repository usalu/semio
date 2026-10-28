//! 📖️ Neutral borrowed octet storage contracts carry no custody or decoder authority.
/// 🪟️ An immutable storage view lends bounded original byte fragments without asserting or issuing ownership authority.
pub trait ByteReadSource:Sync{
 fn byte_len(&self)->usize;
 fn byte_at(&self,index:usize)->Option<u8>;
 fn read_bytes(&self,offset:usize,output:&mut[u8])->usize{let count=self.byte_len().saturating_sub(offset).min(output.len());for(index,byte)in output[..count].iter_mut().enumerate(){let Some(value)=self.byte_at(offset+index)else{return index};*byte=value;}count}
}
impl ByteReadSource for [u8]{fn byte_len(&self)->usize{self.len()}fn byte_at(&self,index:usize)->Option<u8>{self.get(index).copied()}fn read_bytes(&self,offset:usize,output:&mut[u8])->usize{let Some(bytes)=self.get(offset..)else{return 0};let count=bytes.len().min(output.len());output[..count].copy_from_slice(&bytes[..count]);count}}
impl ByteReadSource for Vec<u8>{fn byte_len(&self)->usize{self.len()}fn byte_at(&self,index:usize)->Option<u8>{self.get(index).copied()}fn read_bytes(&self,offset:usize,output:&mut[u8])->usize{ByteReadSource::read_bytes(self.as_slice(),offset,output)}}
impl<const N:usize> ByteReadSource for [u8;N]{fn byte_len(&self)->usize{N}fn byte_at(&self,index:usize)->Option<u8>{self.get(index).copied()}fn read_bytes(&self,offset:usize,output:&mut[u8])->usize{ByteReadSource::read_bytes(self.as_slice(),offset,output)}}
impl<T:ByteReadSource+?Sized> ByteReadSource for &T{fn byte_len(&self)->usize{(**self).byte_len()}fn byte_at(&self,index:usize)->Option<u8>{(**self).byte_at(index)}fn read_bytes(&self,offset:usize,output:&mut[u8])->usize{(**self).read_bytes(offset,output)}}

/// 🪟️ A checked borrowed subrange preserves the source storage and exposes only its declared octets.
pub struct ByteReadRange<'a,S:ByteReadSource+?Sized>{source:&'a S,start:usize,length:usize}
impl<'a,S:ByteReadSource+?Sized> ByteReadRange<'a,S>{pub fn new(source:&'a S,start:usize,length:usize)->Option<Self>{start.checked_add(length).filter(|end|*end<=source.byte_len()).map(|_|Self{source,start,length})}}
impl<S:ByteReadSource+?Sized> ByteReadSource for ByteReadRange<'_,S>{fn byte_len(&self)->usize{self.length}fn byte_at(&self,index:usize)->Option<u8>{if index>=self.length{None}else{self.source.byte_at(self.start+index)}}fn read_bytes(&self,offset:usize,output:&mut[u8])->usize{let length=self.length.saturating_sub(offset).min(output.len());if length==0{return 0}self.source.read_bytes(self.start+offset,&mut output[..length])}}
