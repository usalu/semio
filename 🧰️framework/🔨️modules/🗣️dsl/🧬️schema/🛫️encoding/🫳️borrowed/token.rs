//! 🔤 Incremental canonical text token emission over an immutable borrowed string.
use super::*;
use semio_framework_value::RetainedCloneProgress;
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Scan,Open,Body,Close,Done,Closed}
/// 🧵️ Retains text validation and byte frontier without an owned text copy.
pub struct BorrowedTextToken{phase:Phase,identity:Option<(usize,usize)>,offset:usize,unit_offset:usize,quoted:bool,written:usize}
/// 🧾️ Carries the bytes initialized in a single turn.
#[derive(Default,Debug)]
pub struct BorrowedTextTokenProgress{pub written_bytes:usize,pub complete:bool}
/// ⚠️ Preserves the original refusal, initialized prefix and settlement refusal.
#[derive(Debug)]
pub struct BorrowedTextTokenFailure{pub reason:ValueError,pub written_bytes:usize,pub settlement:Option<ValueError>}
impl BorrowedTextToken{
    /// 🌱️ Starts a token; intrinsic strings require quotation.
    pub fn new(force_quote:bool)->Self{Self{phase:if force_quote{Phase::Open}else{Phase::Scan},identity:None,offset:0,unit_offset:0,quoted:force_quote,written:0}}
    /// 📍️ Reports the cumulative initialized output.
    pub fn emitted_bytes(&self)->usize{self.written}
    /// 📏️ Quotes the next output span without visiting the source.
    pub fn next_output_byte_demand(&self)->usize{usize::from(match self.phase{Phase::Open|Phase::Close=>self.quoted,Phase::Body=>self.identity.is_some_and(|(_,length)|self.offset<length),_=>false})}
    /// ✅️ Reports completion after the final output byte.
    pub fn is_complete(&self)->bool{self.phase==Phase::Done}
    /// 🍂️ Reports retirement of the inline cursor state.
    pub fn terminal_is_empty(&self)->bool{self.phase==Phase::Closed}
    fn failure(reason:ValueError,written_bytes:usize,settlement:Option<ValueError>)->BorrowedTextTokenFailure{BorrowedTextTokenFailure{reason,written_bytes,settlement}}
    fn word_byte(ch:char,quoted:bool,index:usize)->(u8,usize){
        if quoted{
            let escape=match ch{'\\'=>Some(b'\\'),'"'=>Some(b'"'),'\n'=>Some(b'n'),'\r'=>Some(b'r'),'\t'=>Some(b't'),_=>None};
            if let Some(escape)=escape{return(if index==0{b'\\'}else{escape},2)}
            let n=ch as usize;
            if n<32{const HEX:&[u8]=b"0123456789abcdef";let length=if n>=16{6}else{5};let byte=match index{0=>b'\\',1=>b'u',2=>b'{',3=>if n>=16{HEX[n>>4]}else{HEX[n]},4=>if n>=16{HEX[n&15]}else{b'}'},_=>b'}'};return(byte,length)}
        }
        let value=ch as u32;
        if value<0x80{return(value as u8,1)}
        if value<0x800{return(if index==0{0xc0|(value>>6) as u8}else{0x80|(value as u8&0x3f)},2)}
        if value<0x10000{return(match index{0=>0xe0|(value>>12) as u8,1=>0x80|((value>>6) as u8&0x3f),_=>0x80|(value as u8&0x3f)},3)}
        (match index{0=>0xf0|(value>>18) as u8,1=>0x80|((value>>12) as u8&0x3f),2=>0x80|((value>>6) as u8&0x3f),_=>0x80|(value as u8&0x3f)},4)
    }
    /// 🎟️ Performs one externally quoted transition and marks each actual effect.
    pub fn advance_effect(&mut self,text:&str,output:&mut[u8],performed:&mut bool,receipt:&mut RetainedCloneProgress)->Result<BorrowedTextTokenProgress,ValueError>{
        if self.phase==Phase::Closed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed text token is closed"))}
        if self.is_complete(){return Ok(BorrowedTextTokenProgress{complete:true,..Default::default()})}
        let identity=(text.as_ptr() as usize,text.len());
        if self.identity.is_some_and(|old|old!=identity){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed text token source changed"))}
        if self.next_output_byte_demand()!=0&&output.is_empty(){return Ok(Default::default())}
        let mut byte=None;let mut next_phase=self.phase;let mut next_offset=self.offset;let mut next_unit=self.unit_offset;let mut quoted=self.quoted;
        match self.phase{
            Phase::Scan=>{
                if self.offset==0&&!bare_ident_prefix(text){quoted=true;next_phase=Phase::Open}
                else if let Some(ch)=text[self.offset..].chars().next(){
                    let next=text[self.offset+ch.len_utf8()..].chars().next();
                    let valid=if self.offset==0{ch.is_alphabetic()||ch=='_'}else{(ch.is_alphanumeric()||matches!(ch,'_'|'-'|'.'|'/'))&&!(ch=='-'&&next.is_some_and(|value|matches!(value,'>'|'-')))};
                    if valid{next_offset+=ch.len_utf8()}else{next_offset=0;quoted=true;next_phase=Phase::Open}
                }else{quoted=self.offset==0;next_offset=0;next_phase=Phase::Open}
            },
            Phase::Open=>{if quoted{byte=Some(b'"')}next_phase=Phase::Body},
            Phase::Body=>{if let Some(ch)=text[self.offset..].chars().next(){let(value,length)=Self::word_byte(ch,quoted,self.unit_offset);byte=Some(value);next_unit+=1;if next_unit==length{next_unit=0;next_offset+=ch.len_utf8()}}else{next_phase=Phase::Close}},
            Phase::Close=>{if quoted{byte=Some(b'"')}next_phase=Phase::Done},
            Phase::Done|Phase::Closed=>unreachable!(),
        }
        let next_written=self.written.checked_add(usize::from(byte.is_some())).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"borrowed text output length overflow"))?;
        if let Some(byte)=byte{output[0]=byte}
        self.identity=Some(identity);self.offset=next_offset;self.unit_offset=next_unit;self.quoted=quoted;self.phase=next_phase;self.written=next_written;
        *receipt=RetainedCloneProgress{copied_bytes:usize::from(byte.is_some()),..Default::default()};*performed=true;
        Ok(BorrowedTextTokenProgress{written_bytes:usize::from(byte.is_some()),complete:self.is_complete()})
    }
    /// ⏱️ Performs one transition using the actual borrowed encoder work loan.
    pub fn advance(&mut self,text:&str,output:&mut[u8],control:&mut NativeEncodeControl<'_>)->Result<BorrowedTextTokenProgress,BorrowedTextTokenFailure>{
        if self.is_complete(){return Ok(BorrowedTextTokenProgress{complete:true,..Default::default()})}
        if self.terminal_is_empty(){return Err(Self::failure(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed text token is closed"),0,None))}
        if self.identity.is_some_and(|old|old!=(text.as_ptr() as usize,text.len())){return Err(Self::failure(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed text token source changed"),0,None))}
        if self.next_output_byte_demand()!=0&&output.is_empty(){return Ok(Default::default())}
        control.checkpoint().map_err(|error|Self::failure(error,0,None))?;
        if !control.prepare_work(1).map_err(|error|Self::failure(error,0,None))?{return Ok(Default::default())}
        let mut performed=false;let mut receipt=RetainedCloneProgress::default();let result=self.advance_effect(text,output,&mut performed,&mut receipt);
        let settlement=if performed{control.commit_work(1)}else{control.cancel_work()};
        match result{Ok(progress)=>{settlement.map_err(|error|Self::failure(error,progress.written_bytes,None))?;Ok(progress)},Err(reason)=>Err(Self::failure(reason,receipt.copied_bytes,settlement.err()))}
    }
    /// 🍂️ Retires inline state through a caller-owned effect marker.
    pub fn close_effect(&mut self,performed:&mut bool,receipt:&mut RetainedCloneProgress)->Result<bool,ValueError>{
        if self.terminal_is_empty(){return Ok(true)}
        self.phase=Phase::Closed;self.identity=None;self.offset=0;self.unit_offset=0;
        *receipt=RetainedCloneProgress::default();*performed=true;Ok(true)
    }
    /// 🍂️ Retires inline state under the actual caller work loan.
    pub fn close(&mut self,control:&mut NativeEncodeControl<'_>)->Result<bool,ValueError>{
        if self.terminal_is_empty(){return Ok(true)}control.checkpoint()?;if !control.prepare_work(1)?{return Ok(false)}
        let mut performed=false;let mut receipt=RetainedCloneProgress::default();let result=self.close_effect(&mut performed,&mut receipt);
        if performed{control.commit_work(1)?}else{control.cancel_work()?}result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_value::native_encoding::NativeEncodeWorkRecipient;
    struct Loan { remaining:u64, committed:u64 }
    impl NativeEncodeWorkRecipient for Loan {
        fn admits(&mut self,_:u64,additional:u64)->Result<bool,ValueError>{Ok(self.remaining>=additional)}
        fn committed(&mut self,actual:u64)->Result<(),ValueError>{self.remaining-=actual;self.committed+=actual;Ok(())}
    }
    #[test]
    fn token_corpus_emits_one_byte_and_denial_preserves_frontier(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();let mut denied_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🔣️.json")).unwrap();
        for input in fixture["invalidSurrogateJson"].as_array().unwrap(){assert!(serde_json::from_str::<String>(input.as_str().unwrap()).is_err())}
        for case in fixture["cases"].as_array().unwrap() {
            let text=case["value"].as_str().unwrap();
            let expected=case["document"].as_str().unwrap().strip_prefix("name=").unwrap().strip_suffix('\n').unwrap();
            let mut cursor=BorrowedTextToken::new(false);let mut accepted=|_|true;
            let mut control=NativeEncodeControl::new(0,&mut accepted);
            let mut denied=Loan{remaining:0,committed:0};let mut byte=[0u8;1];
            control.with_work_recipient(&mut denied_issuer.loan(&mut denied),|control|{
                let progress=cursor.advance(text,&mut byte,control).map_err(|failure|failure.reason)?;
                assert_eq!(progress.written_bytes,0);assert_eq!(cursor.emitted_bytes(),0);Ok::<_,ValueError>(())
            }).unwrap();
            assert_eq!(denied.committed,0);
            let mut loan=Loan{remaining:10000,committed:0};let mut result=Vec::new();
            control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
                for _ in 0..10000 {
                    let progress=cursor.advance(text,&mut byte,control).map_err(|failure|failure.reason)?;
                    result.extend_from_slice(&byte[..progress.written_bytes]);
                    if !result.is_empty(){break}
                }
                assert_eq!(result.len(),1);Ok::<_,ValueError>(())
            }).unwrap();
            let before=cursor.emitted_bytes();byte[0]=0xaa;
            control.with_work_recipient(&mut denied_issuer.loan(&mut denied),|control|{
                let progress=cursor.advance(text,&mut byte,control).map_err(|failure|failure.reason)?;
                assert_eq!(progress.written_bytes,0);assert_eq!(cursor.emitted_bytes(),before);assert_eq!(byte[0],0xaa);Ok::<_,ValueError>(())
            }).unwrap();
            control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
                for _ in 0..10000 {
                    let progress=cursor.advance(text,&mut byte,control).map_err(|failure|failure.reason)?;
                    result.extend_from_slice(&byte[..progress.written_bytes]);
                    if progress.complete {break}
                }
                assert!(cursor.is_complete());assert_eq!(result,expected.as_bytes());Ok::<_,ValueError>(())
            }).unwrap();
            control.with_work_recipient(&mut denied_issuer.loan(&mut denied),|control|{
                assert!(!cursor.close(control)?);assert!(!cursor.terminal_is_empty());Ok::<_,ValueError>(())
            }).unwrap();
            control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
                assert!(cursor.close(control)?);assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
            }).unwrap();
            assert!(loan.committed>result.len() as u64);
        }
    }
    #[test]
    fn token_cancellation_preserves_prefix_and_resumes(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
        let enabled=std::cell::Cell::new(true);let mut accept=|_|enabled.get();
        let mut control=NativeEncodeControl::new(0,&mut accept);
        let mut loan=Loan{remaining:1000,committed:0};
        let mut cursor=BorrowedTextToken::new(false);let text="é😀";let mut output=Vec::new();let mut byte=[0u8;1];
        control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
            for _ in 0..10000 {
                let turn=cursor.advance(text,&mut byte,control).map_err(|failure|failure.reason)?;
                output.extend_from_slice(&byte[..turn.written_bytes]);
                if !output.is_empty(){break}
            }
            assert_eq!(output.len(),1);
            let before=control.actual_work();enabled.set(false);
            let failure=cursor.advance(text,&mut byte,control).unwrap_err();
            assert_eq!(failure.reason.kind,ValueRefusalKind::Canceled);
            assert_eq!(failure.written_bytes,0);assert_eq!(cursor.emitted_bytes(),1);assert_eq!(control.actual_work(),before);
            enabled.set(true);
            for _ in 0..10000{
                if cursor.is_complete(){break}
                let turn=cursor.advance(text,&mut byte,control).map_err(|failure|failure.reason)?;
                output.extend_from_slice(&byte[..turn.written_bytes]);
            }
            assert!(cursor.is_complete());
            assert_eq!(output,"\"é😀\"".as_bytes());assert!(cursor.close(control)?);Ok::<_,ValueError>(())
        }).unwrap();
    }

    #[test]
    fn token_effect_marks_exact_output_for_caller_quote(){
        let text="a\n😀";let mut cursor=BorrowedTextToken::new(false);let mut output=Vec::new();let mut byte=[0u8;1];let mut denied_once=false;
        for _ in 0..10000{
            let demand=cursor.next_output_byte_demand();let mut performed=false;let mut receipt=RetainedCloneProgress::default();
            if demand==1&&!denied_once{let before=cursor.emitted_bytes();let turn=cursor.advance_effect(text,&mut[],&mut performed,&mut receipt).unwrap();assert_eq!(turn.written_bytes,0);assert!(!performed);assert_eq!(receipt,RetainedCloneProgress::default());assert_eq!(cursor.emitted_bytes(),before);denied_once=true;}
            let progress=cursor.advance_effect(text,&mut byte,&mut performed,&mut receipt).unwrap();
            assert!(performed);assert_eq!(receipt.copied_items,0);assert_eq!(receipt.copied_bytes,demand);assert_eq!(progress.written_bytes,demand);
            output.extend_from_slice(&byte[..progress.written_bytes]);if progress.complete{break}
        }
        assert!(cursor.is_complete());assert_eq!(output,"\"a\\n😀\"".as_bytes());
        let mut performed=false;let mut receipt=RetainedCloneProgress::default();assert!(cursor.close_effect(&mut performed,&mut receipt).unwrap());assert!(performed);assert_eq!(receipt.copied_items,0);assert_eq!(receipt.copied_bytes,0);assert!(cursor.terminal_is_empty());
    }

    struct FixturePhysical{allowance:semio_framework_value::RetainedCloneGrant,submitted:RetainedCloneProgress}
    impl semio_framework_value::native_encoding::NativeEncodeConstructionRecipient for FixturePhysical{
        fn submitted(&self)->RetainedCloneProgress{self.submitted}
        fn grant(&mut self,demand:semio_framework_value::RetirementDemand)->Result<semio_framework_value::RetainedCloneGrant,ValueError>{
            let remaining=self.allowance.maximum_copy_bytes.saturating_sub(self.submitted.copied_bytes);
            if demand.copy_bytes>remaining{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixture copy grant exhausted"))}
            Ok(semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:remaining,..self.allowance})
        }
        fn accept(&mut self,grant:semio_framework_value::RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{
            if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixture receipt exceeds quote"))}
            self.submitted=self.submitted.checked_add(progress)?;Ok(())
        }
    }
    #[test]
    fn token_effect_runs_under_physical_quote_fixture(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();let mut physical_issuer=semio_framework_value::native_encoding::NativeEncodeConstructionIssuer::new();
        use semio_framework_io_control::{NativeSnapshotBodyWallet,NativeSnapshotEncodeEffectQuote,NativeSnapshotEffectProgress};
        use semio_framework_value::RetainedCloneGrant;
        let text="a\n😀";let expected="\"a\\n😀\"".as_bytes();let mut cursor=BorrowedTextToken::new(false);let mut output=Vec::new();let mut byte=[0u8;1];
        let allowance=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:expected.len(),maximum_depth:1,..Default::default()};
        let mut body=NativeSnapshotBodyWallet::new(allowance);let mut physical=FixturePhysical{allowance,submitted:Default::default()};let mut loan=Loan{remaining:10000,committed:0};
        let mut accepted=|_|true;let mut control=NativeEncodeControl::new(0,&mut accepted);
        control.with_construction_recipient(&mut physical_issuer.loan(&mut physical),|control|control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
            for _ in 0..10000{
                let demand=RetainedCloneGrant{maximum_copy_bytes:cursor.next_output_byte_demand(),maximum_depth:1,..Default::default()};
                let quote=NativeSnapshotEncodeEffectQuote::encode(&mut body,control,demand).unwrap();let mut marker=NativeSnapshotEffectProgress::default();let mut failure=None;let mut turn=None;
                let outcome=quote.run_native(&mut marker,&mut failure,&mut |_,performed,receipt|{turn=Some(cursor.advance_effect(text,&mut byte,performed,receipt)?);Ok(())});
                assert!(outcome.result.is_ok());assert_eq!(outcome.work,1);assert!(failure.is_none());let turn=turn.unwrap();assert_eq!(turn.written_bytes,demand.maximum_copy_bytes);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}
            }
            assert!(cursor.is_complete());assert_eq!(output,expected);
            let quote=NativeSnapshotEncodeEffectQuote::encode(&mut body,control,RetainedCloneGrant{maximum_depth:1,..Default::default()}).unwrap();let mut marker=NativeSnapshotEffectProgress::default();let mut failure=None;
            let outcome=quote.run_native(&mut marker,&mut failure,&mut |_,performed,receipt|{cursor.close_effect(performed,receipt)?;Ok(())});assert!(outcome.result.is_ok());assert_eq!(outcome.work,1);assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
        })).unwrap();
        assert_eq!(physical.submitted.copied_bytes,expected.len());assert!(loan.committed>expected.len() as u64);
    }

}
