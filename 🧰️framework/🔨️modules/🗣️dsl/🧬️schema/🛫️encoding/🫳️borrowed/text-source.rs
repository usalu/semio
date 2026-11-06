//! 🪟️ Caller-owned source-view slot for a separately admitted projection action.
use super::*;

/// 🧵️ Retains a borrowed text view from the same original source until paid release.
pub struct BorrowedTextSourceSlot<'a>{identity:usize,value:Option<&'a str>}
impl<'a> BorrowedTextSourceSlot<'a>{
    /// 🌱️ Binds the original immutable source identity without copying its content.
    pub fn new(source:&'a dyn FieldProjectionSource)->Self{Self{identity:source.projection_identity(),value:None}}
    /// 👁️ Lends the parked view without transferring custody from the slot.
    pub fn view(&self)->Option<&str>{self.value}
    /// 📍️ Reports whether no source view is retained.
    pub fn is_empty(&self)->bool{self.value.is_none()}
    /// 🎟️ Performs one externally admitted original-source projection and marks the attempted work.
    pub fn borrow_effect(&mut self,source:&'a dyn FieldProjectionSource,path:&[usize],intrinsic:bool,performed:&mut bool)->Result<(),ValueError>{
        if self.value.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed text source slot is occupied"))}
        if source.projection_identity()!=self.identity{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed text source identity changed"))}
        *performed=true;
        let view=source.projection_view(path)?;
        self.value=Some(match(view,intrinsic){(V::Text(text),false)|(V::IntrinsicText(text),true)=>text,_=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"borrowed text projection changed"))});
        Ok(())
    }
    /// 🍂️ Releases one parked source view through a caller-owned work marker.
    pub fn release_effect(&mut self,performed:&mut bool)->bool{
        if self.value.is_none(){return true}
        self.value=None;*performed=true;true
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    struct Source<'a>{panic:bool,text:&'a str}
    impl FieldProjectionSource for Source<'_>{
        fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{if self.panic{panic!("fixture source callback")};match path{[0]=>Ok(V::Text(self.text)),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unexpected test path"))}}
    }
    #[test]
    fn token_text_source_slot_marks_projection_and_release(){
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🔣️.json")).unwrap();let text=fixture["cases"].as_array().unwrap().iter().find(|case|case["name"]=="unicode").unwrap()["value"].as_str().unwrap();
        let source=Source{panic:false,text};let other=Source{panic:false,text};let mut slot=BorrowedTextSourceSlot::new(&source);
        let mut performed=false;assert!(slot.borrow_effect(&other,&[0],false,&mut performed).is_err());assert!(!performed);assert!(slot.is_empty());
        slot.borrow_effect(&source,&[0],false,&mut performed).unwrap();assert!(performed);assert_eq!(slot.view(),Some(text));
        performed=false;assert!(slot.borrow_effect(&source,&[0],false,&mut performed).is_err());assert!(!performed);
        assert!(slot.release_effect(&mut performed));assert!(performed);assert!(slot.is_empty());performed=false;assert!(slot.release_effect(&mut performed));assert!(!performed);
    }
    #[test]
    fn token_text_source_slot_marks_panicking_projection(){
        let source=Source{panic:true,text:"ignored"};let mut slot=BorrowedTextSourceSlot::new(&source);let mut performed=false;
        let outcome=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||slot.borrow_effect(&source,&[0],false,&mut performed)));
        assert!(outcome.is_err());assert!(performed);assert!(slot.is_empty());
    }
    struct FixtureLoan{remaining:u64,committed:u64,refuse_post:bool}
    impl semio_framework_value::native_encoding::NativeEncodeWorkRecipient for FixtureLoan{
        fn admits(&mut self,_:u64,work:u64)->Result<bool,ValueError>{Ok(self.remaining>=work)}
        fn committed(&mut self,work:u64)->Result<(),ValueError>{self.remaining-=work;self.committed+=work;if self.refuse_post{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"fixture work post refusal"))}else{Ok(())}}
    }
    #[test]
    fn token_text_source_slot_runs_under_work_quote_fixture(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
        use semio_framework_io_control::NativeSnapshotEncodeWorkQuote;
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🔣️.json")).unwrap();let text=fixture["cases"].as_array().unwrap().iter().find(|case|case["name"]=="unicode").unwrap()["value"].as_str().unwrap();
        let source=Source{panic:false,text};let mut slot=BorrowedTextSourceSlot::new(&source);let mut callback=|_|true;let mut control=NativeEncodeControl::new(0,&mut callback);let mut loan=FixtureLoan{remaining:4,committed:0,refuse_post:false};
        control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
            let mut performed=false;let mut failure=None;
            let quote=NativeSnapshotEncodeWorkQuote::reserve(control).unwrap();
            let outcome=quote.run(&mut performed,&mut failure,&mut |mark|slot.borrow_effect(&source,&[0],false,mark));
            assert!(outcome.result.is_ok());assert_eq!(outcome.work,1);assert!(performed);assert!(failure.is_none());assert_eq!(slot.view(),Some(text));
            performed=false;
            let quote=NativeSnapshotEncodeWorkQuote::reserve(control).unwrap();
            let outcome=quote.run(&mut performed,&mut failure,&mut |mark|{slot.release_effect(mark);Ok(())});
            assert!(outcome.result.is_ok());assert_eq!(outcome.work,1);assert!(performed);assert!(slot.is_empty());Ok::<_,ValueError>(())
        }).unwrap();
        assert_eq!(loan.committed,2);
    }
    #[test]
    fn token_text_source_slot_work_quote_parks_unwind_refusal(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
        use semio_framework_io_control::NativeSnapshotEncodeWorkQuote;
        let source=Source{panic:true,text:"ignored"};let mut slot=BorrowedTextSourceSlot::new(&source);let mut callback=|_|true;let mut control=NativeEncodeControl::new(0,&mut callback);let mut loan=FixtureLoan{remaining:1,committed:0,refuse_post:true};let mut performed=false;let mut failure=None;
        let caught=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
            let _=control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
                let quote=NativeSnapshotEncodeWorkQuote::reserve(control).unwrap();
                let _=quote.run(&mut performed,&mut failure,&mut |mark|slot.borrow_effect(&source,&[0],false,mark));Ok::<_,ValueError>(())
            });
        }));
        assert!(caught.is_err());assert!(performed);assert!(failure.is_some());assert!(slot.is_empty());assert_eq!(loan.committed,1);assert_eq!(control.actual_work(),1);
    }

}
