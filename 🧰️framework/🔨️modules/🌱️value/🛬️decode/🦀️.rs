//! 🛬️ Cumulative ownership admission shared by native parsers and typed field construction.

/// ⏱️ Decoder work units and admitted owned bytes at one cancellation boundary.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeDecodeProgress { pub completed:usize, pub total:usize, pub owned_bytes:usize }

/// 🧮️ One caller-owned budget persists from input scanning through final typed construction.
pub struct NativeDecodeControl<'a> { maximum_bytes:usize, owned_bytes:usize, completed:usize, total:usize, started:bool, stage:u64, depth:usize, callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool }

impl<'a> NativeDecodeControl<'a> {
    /// 🚦️ Binds the explicit allocation ceiling and cancellation callback.
    pub fn new(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Self { Self{maximum_bytes,owned_bytes:0,completed:0,total:0,started:false,stage:0,depth:0,callback} }
    /// 🪆️ Bounds recursive typed construction independently of physical input parsing.
    pub fn scoped_depth<T,E:From<String>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{
        if self.depth>=maximum{return Err(E::from("native typed construction exceeds depth limit".into()))}
        self.depth+=1;let result=operation(self);self.depth-=1;result
    }
    /// 📊️ Returns the cumulative ownership already admitted for this operation.
    pub fn owned_bytes(&self)->usize { self.owned_bytes }
    /// 📏️ Returns the caller's complete native allocation allowance.
    pub fn maximum_bytes(&self)->usize { self.maximum_bytes }
    /// 🛑️ Checks cancellation before an explicitly owned expensive operation.
    pub fn checkpoint(&mut self)->Result<(),String> { if(self.callback)(NativeDecodeProgress{completed:self.completed,total:self.total,owned_bytes:self.owned_bytes}){self.started=true;Ok(())}else{Err("native decoding canceled".into())} }
    /// 🧭️ Begins a known stage workload while retaining every previously admitted byte.
    pub fn begin_stage(&mut self,total:usize)->Result<(),String>{self.stage=self.stage.checked_add(1).ok_or("native decoding stage overflow")?;self.completed=0;self.total=total;self.started=false;self.checkpoint()}
    /// 🪆️ Restores a parent workload after a child begins its own stage, preserving cumulative ownership.
    pub fn scoped_stage<T,E>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{
        let parent=(self.completed,self.total,self.started,self.stage);let result=operation(self);
        if self.stage!=parent.3{self.completed=parent.0;self.total=parent.1;self.started=parent.2;self.stage=parent.3;}
        result
    }
    /// 📐️ Restricts one owned decoder stage to its domain ceiling, retaining cumulative charges afterward.
    pub fn scoped_maximum<T>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,String>)->Result<T,String>{let parent=self.maximum_bytes;self.maximum_bytes=parent.min(maximum);if self.owned_bytes>self.maximum_bytes{self.maximum_bytes=parent;return Err("native decoding ownership exceeds stage limit".into());}let result=operation(self);self.maximum_bytes=parent;result}
    /// 📍️ Advances consumed stage units and checks cancellation across256-unit boundaries.
    pub fn advance(&mut self,units:usize)->Result<(),String>{if !self.started{self.checkpoint()?;}let previous=self.completed;self.completed=self.completed.checked_add(units).ok_or("native decoding work overflow")?;if self.total!=0&&self.completed>self.total{return Err("native decoding exceeded declared stage workload".into());}if previous/256!=self.completed/256||(self.total!=0&&self.completed==self.total){self.checkpoint()?;}Ok(())}
    /// 🔢️ Advances decoder work and publishes a checkpoint every256units.
    pub fn step(&mut self)->Result<(),String> {self.advance(1)}
    /// 📦️ Admits cumulative storage before a caller copies or reserves it.
    pub fn charge(&mut self,bytes:usize)->Result<(),String> { let next=self.owned_bytes.checked_add(bytes).filter(|next|*next<=self.maximum_bytes).ok_or("native decoding ownership exceeds caller limit")?;if !self.started||bytes>65536 {self.checkpoint()?;}self.owned_bytes=next;Ok(()) }
    /// 🗂️ Reserves typed collection slots after overflow and caller-bound admission.
    pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,String> { let bytes=count.checked_mul(std::mem::size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or("native decoding collection size overflow")?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_|"native decoding collection allocation failed")?;Ok(output) }
    fn copy_checkpoint(&mut self,completed:usize,total:usize)->Result<(),String>{if(self.callback)(NativeDecodeProgress{completed,total,owned_bytes:self.owned_bytes}){Ok(())}else{Err("native decoding canceled".into())}}
    /// 🔎️ Validates borrowed UTF-8 in bounded spans without owning a second text buffer.
    pub fn borrow_text<'text>(&mut self,bytes:&'text[u8])->Result<&'text str,String>{
        self.copy_checkpoint(0,bytes.len())?;let mut position=0;
        while position<bytes.len(){
            let mut end=position.saturating_add(65536).min(bytes.len());
            loop{match std::str::from_utf8(&bytes[position..end]){Ok(_)=>break,Err(error) if error.error_len().is_none()&&end<bytes.len()=>{end+=1;},Err(_)=>return Err("invalid native UTF-8".into())}}
            position=end;self.copy_checkpoint(position,bytes.len())?;
        }
        Ok(unsafe{std::str::from_utf8_unchecked(bytes)})
    }
    /// 🔤️ Copies owned UTF-8 in cancellable64KiB spans, preserving the outer work stage.
    pub fn copy_text(&mut self,text:&str)->Result<String,String> {
        self.charge(text.len())?;self.copy_checkpoint(0,text.len())?;
        let mut output=String::new();output.try_reserve_exact(text.len()).map_err(|_|"native decoding text allocation failed")?;
        let mut position=0;while position<text.len(){let mut end=position.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[position..end]);position=end;self.copy_checkpoint(position,text.len())?;}Ok(output)
    }
    /// 🧬️ Copies intrinsic octets in cancellable64KiB spans after complete ownership admission.
    pub fn copy_bytes(&mut self,bytes:&[u8])->Result<Vec<u8>,String> {
        self.charge(bytes.len())?;self.copy_checkpoint(0,bytes.len())?;
        let mut output=Vec::new();output.try_reserve_exact(bytes.len()).map_err(|_|"native decoding collection allocation failed")?;
        for chunk in bytes.chunks(65536){output.extend_from_slice(chunk);self.copy_checkpoint(output.len(),bytes.len())?;}Ok(output)
    }
}
