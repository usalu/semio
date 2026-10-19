//! 🔏️ The inline first-party hash state and each original source byte have explicit coroutine receiving receipts.
use super::*;

struct FoldHashBirth{control:HistoryFoldControl,copy:usize}
impl Future for FoldHashBirth{
    type Output=semio_framework_hash::Hasher;
    fn poll(self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<Self::Output>{
        let this=self.get_mut();let grant=this.control.original_grant();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<this.copy{return Poll::Pending;}
        let original=semio_framework_hash::Hasher::new();
        this.control.accept_original_progress(RetainedCloneProgress{copied_items:1,copied_bytes:this.copy,..Default::default()});this.control.0.pending_original_copy.fetch_sub(this.copy,Ordering::Relaxed);this.copy=0;Poll::Ready(original)
    }
}
impl Drop for FoldHashBirth{fn drop(&mut self){self.control.0.pending_original_copy.fetch_sub(self.copy,Ordering::Relaxed);}}

struct FoldHashBytes<'a>{control:HistoryFoldControl,hash:&'a mut semio_framework_hash::Hasher,source:&'a [u8],position:usize,copy:usize}
impl Unpin for FoldHashBytes<'_>{}
impl Future for FoldHashBytes<'_>{
    type Output=();
    fn poll(self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<Self::Output>{
        let this=self.get_mut();let grant=this.control.original_grant();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<this.copy{return Poll::Pending;}
        let mut copied=2*size_of::<Self>();
        if this.position<this.source.len(){this.hash.update(&this.source[this.position..this.position+1]);this.position+=1;copied+=1+2*size_of::<semio_framework_hash::Hasher>();}
        this.control.accept_original_progress(RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()});
        if this.position==this.source.len(){this.control.0.pending_original_copy.fetch_sub(this.copy,Ordering::Relaxed);this.copy=0;Poll::Ready(())}else{Poll::Pending}
    }
}
impl Drop for FoldHashBytes<'_>{fn drop(&mut self){self.control.0.pending_original_copy.fetch_sub(self.copy,Ordering::Relaxed);}}

struct FoldHashFinish<'a>{control:HistoryFoldControl,hash:&'a semio_framework_hash::Hasher,copy:usize}
impl Future for FoldHashFinish<'_>{
    type Output=semio_framework_hash::Hash;
    fn poll(self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<Self::Output>{
        let this=self.get_mut();let grant=this.control.original_grant();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<this.copy{return Poll::Pending;}
        let digest=this.hash.finalize();
        this.control.accept_original_progress(RetainedCloneProgress{copied_items:1,copied_bytes:this.copy,..Default::default()});this.control.0.pending_original_copy.fetch_sub(this.copy,Ordering::Relaxed);this.copy=0;Poll::Ready(digest)
    }
}
impl Drop for FoldHashFinish<'_>{fn drop(&mut self){self.control.0.pending_original_copy.fetch_sub(self.copy,Ordering::Relaxed);}}

impl HistoryFoldControl{
    /// 🌳️ Initializes the actual inline first-party hash state after its fixed byte frontier is admitted.
    pub async fn hash_start(&self)->semio_framework_hash::Hasher{
        let copy=2*(size_of::<FoldHashBirth>()+size_of::<semio_framework_hash::Hasher>());self.0.pending_original_copy.fetch_add(copy,Ordering::Relaxed);FoldHashBirth{control:self.clone(),copy}.await
    }
    /// 📥️ Absorbs one source byte per admitted original scalar frontier without temporary backing.
    pub async fn hash_bytes(&self,hash:&mut semio_framework_hash::Hasher,source:&[u8]){
        let copy=2*(size_of::<FoldHashBytes<'_>>()+size_of::<semio_framework_hash::Hasher>())+1;self.0.pending_original_copy.fetch_add(copy,Ordering::Relaxed);FoldHashBytes{control:self.clone(),hash,source,position:0,copy}.await
    }
    /// 🏁️ Produces the exact fixed digest while retaining the original stream state.
    pub async fn hash_finish(&self,hash:&semio_framework_hash::Hasher)->semio_framework_hash::Hash{
        let copy=2*(size_of::<FoldHashFinish<'_>>()+size_of::<semio_framework_hash::Hasher>()+size_of::<semio_framework_hash::Hash>());self.0.pending_original_copy.fetch_add(copy,Ordering::Relaxed);FoldHashFinish{control:self.clone(),hash,copy}.await
    }
}
