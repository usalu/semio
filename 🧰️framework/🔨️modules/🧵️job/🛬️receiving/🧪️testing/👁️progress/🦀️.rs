//! 👁️ Scoped native decoder progress observation for real receiving laws.
    pub type NativeProgressObserver=Box<dyn FnMut(semio_framework_value_native_owned::native_decoding::NativeDecodeProgress)>;
    thread_local!{static NATIVE_PROGRESS_OBSERVER:std::cell::RefCell<Option<NativeProgressObserver>>=const{std::cell::RefCell::new(None)};}
    struct NativeProgressObserverScope{previous:Option<NativeProgressObserver>}
    impl Drop for NativeProgressObserverScope{fn drop(&mut self){let displaced=NATIVE_PROGRESS_OBSERVER.with(|slot|slot.replace(self.previous.take()));drop(displaced);}}
    pub fn with_native_decode_progress<T>(observer:NativeProgressObserver,run:impl FnOnce()->T)->T{let previous=NATIVE_PROGRESS_OBSERVER.with(|slot|slot.replace(Some(observer)));let _scope=NativeProgressObserverScope{previous};run()}
    pub fn observe_native_decode_progress(progress:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress){NATIVE_PROGRESS_OBSERVER.with(|slot|{if let Some(observer)=slot.borrow_mut().as_mut(){observer(progress);}})}
