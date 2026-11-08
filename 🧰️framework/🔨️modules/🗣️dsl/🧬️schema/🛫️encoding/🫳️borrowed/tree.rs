//! 🌳️ Nested borrowed records use only caller-admitted typed frame slots.
use super::*;
use super::record::BorrowedScalarRecordCursor;

/// 🧱️ One typed traversal frame, held in the caller's already-admitted storage.
pub struct BorrowedRecordFrame{cursor:BorrowedScalarRecordCursor,spec:R,segment:[usize;5],segment_len:usize,prefix_len:usize}
/// 🧵️ Retains original source identity and a frontier into caller-owned frame slots.
pub struct BorrowedRecordTree{identity:Option<usize>,frame_base:Option<usize>,root:Option<R>,depth:usize,started:bool,closed:bool,written:usize}
/// 🧾️ Reports the actual output prefix and the minimum caller-owned frame count.
#[derive(Default,Debug)]
pub struct BorrowedRecordTreeProgress{pub written_bytes:usize,pub complete:bool,pub required_frames:usize}
/// ⚠️ Preserves the refusal, initialized output prefix and a separate settlement refusal.
#[derive(Debug)]
pub struct BorrowedRecordTreeFailure{pub reason:ValueError,pub written_bytes:usize,pub settlement:Option<ValueError>}
struct PrefixSource<'a>{source:&'a dyn FieldProjectionSource,parents:&'a[Option<BorrowedRecordFrame>],segment:[usize;5],segment_len:usize,prefix_len:usize}
impl PrefixSource<'_>{
    fn full_path(&self,path:&[usize])->Result<([usize;64],usize),ValueError>{
        let length=self.prefix_len.checked_add(path.len()).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"borrowed record source path overflows"))?;
        if length>=64{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"borrowed record source path exceeds canonical depth"))}
        let mut full=[0usize;64];let mut offset=0usize;
        for parent in self.parents.iter().skip(1){let parent=parent.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed record parent frame is absent"))?;let end=offset.checked_add(parent.segment_len).filter(|end|*end<=self.prefix_len&&parent.segment_len<=5).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed record parent segment changed"))?;full[offset..end].copy_from_slice(&parent.segment[..parent.segment_len]);offset=end;}
        let end=offset.checked_add(self.segment_len).filter(|end|*end<=self.prefix_len&&self.segment_len<=5).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed record active segment changed"))?;full[offset..end].copy_from_slice(&self.segment[..self.segment_len]);offset=end;
        if offset!=self.prefix_len{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed record frame prefix changed"))}
        full[offset..length].copy_from_slice(path);Ok((full,length))
    }
}
impl FieldProjectionSource for PrefixSource<'_>{
    fn projection_identity(&self)->usize{self.source.projection_identity()}
    fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{let(full,length)=self.full_path(path)?;self.source.projection_view(&full[..length])}
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{let(full,length)=self.full_path(path)?;self.source.projection_key(&full[..length],index)}
}
impl Default for BorrowedRecordTree{
    fn default()->Self{Self{identity:None,frame_base:None,root:None,depth:0,started:false,closed:false,written:0}}
}
impl BorrowedRecordTree{
    fn failure(reason:ValueError,written_bytes:usize,settlement:Option<ValueError>)->BorrowedRecordTreeFailure{BorrowedRecordTreeFailure{reason,written_bytes,settlement}}
    fn invalid(message:&'static str)->BorrowedRecordTreeFailure{Self::failure(ValueError::literal(ValueRefusalKind::InvariantViolated,message),0,None)}
    /// 📏️ Quotes the actual typed capacity of a caller-owned frame slice.
    pub fn frame_storage_bytes(count:usize)->Result<usize,ValueError>{std::mem::size_of::<Option<BorrowedRecordFrame>>().checked_mul(count).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"borrowed record frame storage overflows"))}
    /// 📍️ Reports the total output bytes initialized by this tree.
    pub fn emitted_bytes(&self)->usize{self.written}
    /// 🍂️ Reports that every caller frame has been paid closed.
    pub fn terminal_is_empty(&self)->bool{self.closed&&!self.started}
    /// 📏️ Quotes the next initialized output span of the active frame.
    pub fn next_output_byte_demand(&self,frames:&[Option<BorrowedRecordFrame>])->Result<usize,ValueError>{if !self.started{return Ok(0)}if self.frame_base!=Some(frames.as_ptr() as usize){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed record frame storage changed"))}let frame=frames.get(self.depth).and_then(Option::as_ref).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed record active frame is absent"))?;Ok(frame.cursor.next_output_byte_demand())}
    /// ⏱️ Advances one real frame, source or output action without allocating frame storage.
    pub fn advance(&mut self,source:&dyn FieldProjectionSource,spec:R,frames:&mut[Option<BorrowedRecordFrame>],output:&mut[u8],control:&mut NativeEncodeControl<'_>)->Result<BorrowedRecordTreeProgress,BorrowedRecordTreeFailure>{
        if self.closed{return Err(Self::invalid("borrowed record tree is closed"))}
        if self.started&&(self.identity!=Some(source.projection_identity())||self.frame_base!=Some(frames.as_ptr() as usize)||self.root.is_some_and(|old|old.fields.as_ptr()!=spec.fields.as_ptr()||old.fields.len()!=spec.fields.len()||old.keyword!=spec.keyword||old.layout!=spec.layout)){return Err(Self::invalid("borrowed record tree source or schema changed"))}
        if !self.started{
            if frames.is_empty(){return Ok(BorrowedRecordTreeProgress{required_frames:1,..Default::default()})}
            if frames[0].is_some(){return Err(Self::invalid("borrowed record root slot is occupied"))}
            control.checkpoint().map_err(|error|Self::failure(error,0,None))?;
            if !control.prepare_work(1).map_err(|error|Self::failure(error,0,None))?{return Ok(BorrowedRecordTreeProgress{required_frames:1,..Default::default()})}
            frames[0]=Some(BorrowedRecordFrame{cursor:BorrowedScalarRecordCursor::default(),spec,segment:[0;5],segment_len:0,prefix_len:0});
            self.identity=Some(source.projection_identity());self.frame_base=Some(frames.as_ptr() as usize);self.root=Some(spec);self.started=true;
            control.commit_work(1).map_err(|error|Self::failure(error,0,None))?;
            return Ok(BorrowedRecordTreeProgress{required_frames:1,..Default::default()})
        }
        let frame=frames.get(self.depth).and_then(Option::as_ref).ok_or_else(||Self::invalid("borrowed record active frame is absent"))?;
        if frame.cursor.terminal_is_empty(){
            if self.depth==0{return Err(Self::invalid("borrowed record root frame closed before tree retirement"))}
            control.checkpoint().map_err(|error|Self::failure(error,0,None))?;
            if !control.prepare_work(1).map_err(|error|Self::failure(error,0,None))?{return Ok(BorrowedRecordTreeProgress{required_frames:self.depth+1,..Default::default()})}
            let child=frames[self.depth].take().unwrap();let context=child.cursor.format_context();self.depth-=1;
            frames[self.depth].as_mut().unwrap().cursor.finish_child(context.0,context.1,context.2);
            control.commit_work(1).map_err(|error|Self::failure(error,0,None))?;
            return Ok(BorrowedRecordTreeProgress{required_frames:self.depth+1,..Default::default()})
        }
        if frame.cursor.is_complete(){
            if self.depth==0{return Ok(BorrowedRecordTreeProgress{complete:true,required_frames:1,..Default::default()})}
            frames[self.depth].as_mut().unwrap().cursor.close(control).map_err(|error|Self::failure(error,0,None))?;
            return Ok(BorrowedRecordTreeProgress{required_frames:self.depth+1,..Default::default()})
        }
        if let Some((child,segment,segment_len,line_open,glued,indent))=frame.cursor.child_request(){
            let needed=self.depth+2;if frames.len()<needed{return Ok(BorrowedRecordTreeProgress{required_frames:needed,..Default::default()})}
            if frames[self.depth+1].is_some(){return Err(Self::invalid("borrowed record child slot is occupied"))}
            let prefix_len=frame.prefix_len;
            if prefix_len+segment_len>=64{return Err(Self::failure(ValueError::literal(ValueRefusalKind::DepthLimit,"borrowed record source path exceeds canonical depth"),0,None))}
            control.checkpoint().map_err(|error|Self::failure(error,0,None))?;
            if !control.prepare_work(1).map_err(|error|Self::failure(error,0,None))?{return Ok(BorrowedRecordTreeProgress{required_frames:needed,..Default::default()})}
            frames[self.depth+1]=Some(BorrowedRecordFrame{cursor:BorrowedScalarRecordCursor::nested(line_open,glued,indent),spec:child,segment,segment_len,prefix_len:prefix_len+segment_len});self.depth+=1;
            control.commit_work(1).map_err(|error|Self::failure(error,0,None))?;
            return Ok(BorrowedRecordTreeProgress{required_frames:needed,..Default::default()})
        }
        let (parents,active)=frames.split_at_mut(self.depth);let frame=active[0].as_mut().unwrap();
        if frame.cursor.next_output_byte_demand()!=0&&self.written==usize::MAX{return Err(Self::failure(ValueError::literal(ValueRefusalKind::WorkLimit,"borrowed record output length overflow"),0,None))}
        let projected=PrefixSource{source,parents,segment:frame.segment,segment_len:frame.segment_len,prefix_len:frame.prefix_len};
        let turn=match frame.cursor.advance(&projected,frame.spec,output,control){
            Ok(turn)=>turn,
            Err(failure)=>{self.written+=failure.written_bytes;return Err(Self::failure(failure.reason,failure.written_bytes,failure.settlement))}
        };
        self.written+=turn.written_bytes;
        Ok(BorrowedRecordTreeProgress{written_bytes:turn.written_bytes,complete:self.depth==0&&turn.complete,required_frames:self.depth+1})
    }
    /// 🍂️ Pays each child and slot retirement while leaving frame storage with its caller.
    pub fn close(&mut self,frames:&mut[Option<BorrowedRecordFrame>],control:&mut NativeEncodeControl<'_>)->Result<bool,ValueError>{
        if self.terminal_is_empty(){return Ok(true)}
        if !self.started{control.checkpoint()?;if !control.prepare_work(1)?{return Ok(false)}self.closed=true;control.commit_work(1)?;return Ok(true)}
        if self.frame_base!=Some(frames.as_ptr() as usize){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed record frame storage changed"))}
        let frame=frames.get_mut(self.depth).and_then(Option::as_mut).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed record close frame is absent"))?;
        if !frame.cursor.terminal_is_empty(){frame.cursor.close(control)?;return Ok(false)}
        control.checkpoint()?;if !control.prepare_work(1)?{return Ok(false)}
        frames[self.depth]=None;
        if self.depth==0{self.started=false;self.closed=true;self.identity=None;self.frame_base=None;self.root=None}else{self.depth-=1}
        control.commit_work(1)?;Ok(self.terminal_is_empty())
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::BorrowedFieldSpec as F;
    use semio_framework_value::native_encoding::{OriginalWorkAuthority,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalSourceTurnIdentity};
    struct Loan(u64);
    struct LoanDriver;
    impl OriginalWorkRecipient for LoanDriver{
        type Context<'t,'s>=Loan where 's:'t;
        fn validate_context<'t,'s>(&self,_:&Loan)->Result<(),ValueError> where 's:'t{Ok(())}
        fn source_turn<'t,'s>(&self,_:&Loan)->Option<OriginalSourceTurnIdentity> where 's:'t{None}
        fn admits<'r,'t,'s>(&mut self,c:&mut Loan,quote:OriginalWorkQuote<'r>,_:u64,work:u64)->OriginalWorkQuoteComplete<'r> where 's:'t{quote.complete(Ok(c.0>=work))}
        fn settle_partial<'r,'t,'s>(&mut self,c:&mut Loan,check:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r> where 's:'t{let work=u64::try_from(check.delta()).unwrap_or(u64::MAX);let result=(||->Result<(),ValueError>{c.0=c.0.saturating_sub(work);Ok(())})();check.complete(result)}
        fn recover_partial<'r,'t,'s>(&mut self,_:&mut Loan,check:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r> where 's:'t{check.pending()}
    }
    fn with_work<R>(control:&mut NativeEncodeControl<'_>,issuer:&mut OriginalWorkAuthority<LoanDriver>,loan:&mut Loan,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<R,ValueError>)->Result<R,ValueError>{issuer.with_loan(loan,|work|control.with_work_recipient(work,operation))?}
    struct Source{nodes:Vec<(String,f64)>}
    impl FieldProjectionSource for Source{
        fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{
            []=>Ok(V::Record(&[0])),[0]=>Ok(V::List(self.nodes.len())),[0,index] if *index<self.nodes.len()=>Ok(V::Record(&[0,1])),
            [0,index,0]=>self.nodes.get(*index).map(|node|V::Text(node.0.as_str())).ok_or_else(crate::native_encoding::projection_path_error),
            [0,index,1]=>self.nodes.get(*index).map(|node|V::Float(node.1)).ok_or_else(crate::native_encoding::projection_path_error),
            _=>Err(crate::native_encoding::projection_path_error())}}
    }
    fn node_spec()->R{const FIELDS:&[F]=&[F::new(0,"id",H::Text),F::new(1,"x",H::Float)];R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS}}
    fn node_shape()->H{H::Record(node_spec)}
    #[test]
    fn compound_record_list_uses_caller_slots_and_resumes(){let mut loan_issuer=OriginalWorkAuthority::new(LoanDriver);let mut denied_issuer=OriginalWorkAuthority::new(LoanDriver);
        const FIELDS:&[F]=&[F::new(0,"nodes",H::List(node_shape))];
        let schema=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🪆️records.json")).unwrap();
        let nodes=fixture["nodes"].as_array().unwrap().iter().map(|node|(node["id"].as_str().unwrap().to_owned(),node["x"].as_f64().unwrap())).collect();
        let source=Source{nodes};let mut frames:[Option<BorrowedRecordFrame>;2]=[const{None};2];let mut cursor=BorrowedRecordTree::default();
        assert_eq!(BorrowedRecordTree::frame_storage_bytes(2).unwrap(),2*std::mem::size_of::<Option<BorrowedRecordFrame>>());
        let enabled=std::cell::Cell::new(true);let mut callback=|_|enabled.get();let mut control=NativeEncodeControl::new(0,&mut callback);
        let mut denied=Loan(0);let mut byte=[0u8;1];let mut output=Vec::new();
        let empty=cursor.advance(&source,schema,&mut frames[..0],&mut byte,&mut control).unwrap();assert_eq!(empty.required_frames,1);assert_eq!(empty.written_bytes,0);assert_eq!(control.actual_work(),0);
        with_work(&mut control,&mut denied_issuer,&mut denied,|control|{
            let turn=cursor.advance(&source,schema,&mut frames,&mut byte,control).map_err(|failure|failure.reason)?;
            assert_eq!(turn.required_frames,1);assert_eq!(turn.written_bytes,0);assert!(frames[0].is_none());assert_eq!(control.actual_work(),0);Ok::<_,ValueError>(())
        }).unwrap();
        let mut loan=Loan(100000);
        with_work(&mut control,&mut loan_issuer,&mut loan,|control|{
            for _ in 0..100000{
                let demand=cursor.next_output_byte_demand(&frames[..1])?;
                let turn=cursor.advance(&source,schema,&mut frames[..1],&mut byte,control).map_err(|failure|failure.reason)?;
                assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.required_frames==2{break}
            }
            assert_eq!(output.as_slice(),b"nodes=[ {");let before=cursor.emitted_bytes();let work=control.actual_work();
            let other_source=Source{nodes:vec![("A".into(),1.5),("B".into(),2.0)]};
            let mismatch=cursor.advance(&other_source,schema,&mut frames,&mut byte,control).unwrap_err();assert_eq!(mismatch.reason.kind,ValueRefusalKind::InvariantViolated);assert_eq!(control.actual_work(),work);assert_eq!(cursor.emitted_bytes(),before);
            let mut other:[Option<BorrowedRecordFrame>;2]=[const{None};2];
            let mismatch=cursor.advance(&source,schema,&mut other,&mut byte,control).unwrap_err();assert_eq!(mismatch.reason.kind,ValueRefusalKind::InvariantViolated);assert_eq!(control.actual_work(),work);assert_eq!(cursor.emitted_bytes(),before);
            let denied=cursor.advance(&source,schema,&mut frames[..1],&mut byte,control).map_err(|failure|failure.reason)?;
            assert_eq!(denied.required_frames,2);assert_eq!(denied.written_bytes,0);assert_eq!(cursor.emitted_bytes(),before);assert_eq!(control.actual_work(),work);
            enabled.set(false);let failure=cursor.advance(&source,schema,&mut frames,&mut byte,control).unwrap_err();assert_eq!(failure.reason.kind,ValueRefusalKind::Canceled);assert!(frames[1].is_none());assert_eq!(control.actual_work(),work);enabled.set(true);
            for _ in 0..100000{
                let demand=cursor.next_output_byte_demand(&frames)?;
                let turn=cursor.advance(&source,schema,&mut frames,&mut byte,control).map_err(|failure|failure.reason)?;
                assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}
            }
            assert_eq!(output,fixture["document"].as_str().unwrap().as_bytes());
            for _ in 0..100000{if cursor.close(&mut frames,control)?{break}}
            assert!(cursor.terminal_is_empty());assert!(frames.iter().all(Option::is_none));Ok::<_,ValueError>(())
        }).unwrap();
    }
    struct DeepSource;
    impl FieldProjectionSource for DeepSource{
        fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{[]|[0]|[0,0]=>Ok(V::Record(&[0])),[0,0,0]=>Ok(V::Text("x")),_=>Err(crate::native_encoding::projection_path_error())}}
    }
    fn leaf_spec()->R{const FIELDS:&[F]=&[F::new(0,"value",H::Text)];R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS}}
    fn middle_spec()->R{const FIELDS:&[F]=&[F::new(0,"inner",H::Record(leaf_spec))];R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS}}
    #[test]
    fn compound_nested_records_keep_three_caller_frames(){let mut loan_issuer=OriginalWorkAuthority::new(LoanDriver);
        const FIELDS:&[F]=&[F::new(0,"outer",H::Record(middle_spec))];
        let schema=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🪆️deep.json")).unwrap();
        let oracle=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const db=new Database(":memory:");const row=db.query("SELECT printf('outer=inner=value=%s','x') AS document").get();process.stdout.write(row.document+"\n");db.close();"#]).output().unwrap();
        assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(oracle.stdout,fixture["document"].as_str().unwrap().as_bytes());
        let source=DeepSource;let mut frames:[Option<BorrowedRecordFrame>;3]=[const{None};3];let mut cursor=BorrowedRecordTree::default();let mut callback=|_|true;let mut control=NativeEncodeControl::new(0,&mut callback);let mut loan=Loan(100000);let mut byte=[0u8;1];let mut output=Vec::new();let mut maximum_frames=0usize;
        with_work(&mut control,&mut loan_issuer,&mut loan,|control|{
            for _ in 0..100000{
                let demand=cursor.next_output_byte_demand(&frames)?;
                let turn=cursor.advance(&source,schema,&mut frames,&mut byte,control).map_err(|failure|failure.reason)?;
                maximum_frames=maximum_frames.max(turn.required_frames);assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}
            }
            assert_eq!(maximum_frames,fixture["requiredFrames"].as_u64().unwrap() as usize);assert_eq!(output,fixture["document"].as_str().unwrap().as_bytes());for _ in 0..100000{if cursor.close(&mut frames,control)?{break}}assert!(cursor.terminal_is_empty());assert!(frames.iter().all(Option::is_none));Ok::<_,ValueError>(())
        }).unwrap();
    }

    fn owned_truss_schema()->crate::RecordSpec{crate::RecordSpec::new(Some("truss"),RecordLayout::Inline,vec![crate::FieldSpec::new(0,"id",crate::Shape::Text)])}
    fn controlled_truss_schema<C:crate::NativeSchemaControl>(control:&mut C)->Result<crate::RecordSpec,ValueError>{let mut fields=control.allocate_vec(1)?;fields.push(crate::producer::field(0,"id",crate::Shape::Text,control)?);crate::producer::record(Some("truss"),RecordLayout::Inline,fields,control)}
    fn decode_truss_schema(control:&mut semio_framework_value::native_decoding::NativeDecodeControl<'_>)->Result<crate::RecordSpec,ValueError>{controlled_truss_schema(control)}
    fn encode_truss_schema(control:&mut NativeEncodeControl<'_>)->Result<crate::RecordSpec,ValueError>{controlled_truss_schema(control)}
    struct StatementSource{ids:Vec<String>}
    impl FieldProjectionSource for StatementSource{
        fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{
            []=>Ok(V::Record(&[0])),[0]=>Ok(V::Block),[0,0]=>Ok(V::Statements(self.ids.len())),
            [0,0,index] if *index<self.ids.len()=>Ok(V::Record(&[0])),
            [0,0,index,0]=>self.ids.get(*index).map(|id|V::Text(id.as_str())).ok_or_else(crate::native_encoding::projection_path_error),
            _=>Err(crate::native_encoding::projection_path_error())}}
        fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path==[0,0]&&index<self.ids.len(){Ok("truss")}else{Err(crate::native_encoding::projection_path_error())}}
    }
    fn truss_spec()->R{const FIELDS:&[F]=&[F::new(0,"id",H::Text)];R{keyword:Some("truss"),layout:RecordLayout::Inline,fields:FIELDS}}
    fn statements_shape()->H{H::Statements(&[("truss",truss_spec)])}
    #[test]
    fn compound_block_statements_preserve_prefix_and_resume(){let mut loan_issuer=OriginalWorkAuthority::new(LoanDriver);
        const FIELDS:&[F]=&[F::new(0,"elements",H::Block(statements_shape))];
        let schema=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🧱️statements.json")).unwrap();
        let oracle=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const db=new Database(":memory:");const row=db.query("SELECT 'elements {'||char(10)||'  truss id=A'||char(10)||'  truss id=B'||char(10)||'}'||char(10) AS document").get();process.stdout.write(row.document);db.close();"#]).output().unwrap();
        assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(oracle.stdout,fixture["document"].as_str().unwrap().as_bytes());
        let producer=crate::RecordSpecProducer{ordinary:owned_truss_schema,encoding:encode_truss_schema,decoding:decode_truss_schema};
        let owned_schema=crate::RecordSpec::new(None,RecordLayout::Inline,vec![crate::FieldSpec::new(0,"elements",crate::Shape::Block(Box::new(crate::Shape::Statements(vec![("truss".into(),producer)]))))]);
        let entries=fixture["elements"].as_array().unwrap().iter().map(|item|{let mut row=crate::RecordValue::default();row.fields.insert(0,crate::FieldValue::Text(item["id"].as_str().unwrap().to_owned()));("truss".to_owned(),row)}).collect();
        let mut owned=crate::RecordValue::default();owned.fields.insert(0,crate::FieldValue::Block(Box::new(crate::FieldValue::Statements(entries))));assert_eq!(crate::print(&owned,&owned_schema,JoinMode::Document),fixture["document"].as_str().unwrap());
        let ids=fixture["elements"].as_array().unwrap().iter().map(|item|item["id"].as_str().unwrap().to_owned()).collect();let source=StatementSource{ids};
        let mut frames:[Option<BorrowedRecordFrame>;2]=[const{None};2];let mut cursor=BorrowedRecordTree::default();let enabled=std::cell::Cell::new(true);let mut callback=|_|enabled.get();let mut control=NativeEncodeControl::new(0,&mut callback);let mut loan=Loan(100000);let mut byte=[0u8;1];let mut output=Vec::new();
        with_work(&mut control,&mut loan_issuer,&mut loan,|control|{
            for _ in 0..100000{let demand=cursor.next_output_byte_demand(&frames)?;let turn=cursor.advance(&source,schema,&mut frames,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if output.ends_with(b"id=A"){break}}
            assert!(output.ends_with(b"id=A"));let before=cursor.emitted_bytes();let work=control.actual_work();enabled.set(false);
            let failure=cursor.advance(&source,schema,&mut frames,&mut byte,control).unwrap_err();assert_eq!(failure.reason.kind,ValueRefusalKind::Canceled);assert_eq!(failure.written_bytes,0);assert_eq!(cursor.emitted_bytes(),before);assert_eq!(control.actual_work(),work);enabled.set(true);
            for _ in 0..100000{let demand=cursor.next_output_byte_demand(&frames)?;let turn=cursor.advance(&source,schema,&mut frames,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
            assert_eq!(output,fixture["document"].as_str().unwrap().as_bytes());for _ in 0..100000{if cursor.close(&mut frames,control)?{break}}assert!(cursor.terminal_is_empty());assert!(frames.iter().all(Option::is_none));Ok::<_,ValueError>(())
        }).unwrap();
    }

    fn owned_table_node_schema()->crate::RecordSpec{crate::RecordSpec::new(None,RecordLayout::Inline,vec![crate::FieldSpec::new(0,"id",crate::Shape::Text),crate::FieldSpec::new(1,"x",crate::Shape::Float)])}
    fn controlled_table_node_schema<C:crate::NativeSchemaControl>(control:&mut C)->Result<crate::RecordSpec,ValueError>{let mut fields=control.allocate_vec(2)?;fields.push(crate::producer::field(0,"id",crate::Shape::Text,control)?);fields.push(crate::producer::field(1,"x",crate::Shape::Float,control)?);crate::producer::record(None,RecordLayout::Inline,fields,control)}
    fn decode_table_node_schema(control:&mut semio_framework_value::native_decoding::NativeDecodeControl<'_>)->Result<crate::RecordSpec,ValueError>{controlled_table_node_schema(control)}
    fn encode_table_node_schema(control:&mut NativeEncodeControl<'_>)->Result<crate::RecordSpec,ValueError>{controlled_table_node_schema(control)}
    #[test]
    fn compound_columnar_table_preserves_prefix_and_resume(){let mut loan_issuer=OriginalWorkAuthority::new(LoanDriver);
        const FIELDS:&[F]=&[F::new(0,"nodes",H::Table(node_spec))];
        let schema=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/📊️table.json")).unwrap();
        let nodes:Vec<(String,f64)>=fixture["nodes"].as_array().unwrap().iter().map(|node|(node["id"].as_str().unwrap().to_owned(),node["x"].as_f64().unwrap())).collect();
        let producer=crate::RecordSpecProducer{ordinary:owned_table_node_schema,encoding:encode_table_node_schema,decoding:decode_table_node_schema};
        let owned_schema=crate::RecordSpec::new(None,RecordLayout::Inline,vec![crate::FieldSpec::new(0,"nodes",crate::Shape::Table(producer))]);
        let mut owned=crate::RecordValue::default();owned.fields.insert(0,crate::FieldValue::List(nodes.iter().map(|(id,x)|{let mut row=crate::RecordValue::default();row.fields.insert(0,crate::FieldValue::Text(id.clone()));row.fields.insert(1,crate::FieldValue::Float(*x));crate::FieldValue::Record(row)}).collect()));
        assert_eq!(crate::print(&owned,&owned_schema,JoinMode::Document),fixture["document"].as_str().unwrap());
        let oracle=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const db=new Database(":memory:");const row=db.query("SELECT 'nodes [id:TEXT x:NUM] {'||char(10)||'  A 1.5'||char(10)||'  B 2'||char(10)||'}'||char(10) AS document").get();process.stdout.write(row.document);db.close();"#]).output().unwrap();
        assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(oracle.stdout,fixture["document"].as_str().unwrap().as_bytes());
        let source=Source{nodes};let mut frames:[Option<BorrowedRecordFrame>;1]=[const{None};1];let mut cursor=BorrowedRecordTree::default();let enabled=std::cell::Cell::new(true);let mut callback=|_|enabled.get();let mut control=NativeEncodeControl::new(0,&mut callback);let mut loan=Loan(100000);let mut byte=[0u8;1];let mut output=Vec::new();
        with_work(&mut control,&mut loan_issuer,&mut loan,|control|{
            for _ in 0..100000{let demand=cursor.next_output_byte_demand(&frames)?;let turn=cursor.advance(&source,schema,&mut frames,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if output.ends_with(b"A 1.5"){break}}
            assert!(output.ends_with(b"A 1.5"));let before=cursor.emitted_bytes();let work=control.actual_work();enabled.set(false);
            let failure=cursor.advance(&source,schema,&mut frames,&mut byte,control).unwrap_err();assert_eq!(failure.reason.kind,ValueRefusalKind::Canceled);assert_eq!(failure.written_bytes,0);assert_eq!(cursor.emitted_bytes(),before);assert_eq!(control.actual_work(),work);enabled.set(true);
            for _ in 0..100000{let demand=cursor.next_output_byte_demand(&frames)?;let turn=cursor.advance(&source,schema,&mut frames,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
            assert_eq!(output,fixture["document"].as_str().unwrap().as_bytes());for _ in 0..100000{if cursor.close(&mut frames,control)?{break}}assert!(cursor.terminal_is_empty());assert!(frames.iter().all(Option::is_none));Ok::<_,ValueError>(())
        }).unwrap();
    }

    fn owned_case_schema()->crate::RecordSpec{
        let producer=crate::RecordSpecProducer{ordinary:owned_truss_schema,encoding:encode_truss_schema,decoding:decode_truss_schema};
        crate::RecordSpec::new(None,RecordLayout::Inline,vec![crate::FieldSpec::new(0,"id",crate::Shape::Text),crate::FieldSpec::new(1,"loads",crate::Shape::Block(Box::new(crate::Shape::Statements(vec![("truss".into(),producer)])))),crate::FieldSpec::new(2,"enabled",crate::Shape::Bool)])
    }
    fn controlled_case_schema<C:crate::NativeSchemaControl>(control:&mut C)->Result<crate::RecordSpec,ValueError>{
        let mut variants=control.allocate_vec(1)?;let producer=crate::RecordSpecProducer{ordinary:owned_truss_schema,encoding:encode_truss_schema,decoding:decode_truss_schema};variants.push((control.copy_text("truss")?,producer));
        let statements=crate::Shape::Statements(variants);let block=crate::Shape::Block(crate::producer::boxed(statements,control)?);
        let mut fields=control.allocate_vec(3)?;fields.push(crate::producer::field(0,"id",crate::Shape::Text,control)?);fields.push(crate::producer::field(1,"loads",block,control)?);fields.push(crate::producer::field(2,"enabled",crate::Shape::Bool,control)?);crate::producer::record(None,RecordLayout::Inline,fields,control)
    }
    fn decode_case_schema(control:&mut semio_framework_value::native_decoding::NativeDecodeControl<'_>)->Result<crate::RecordSpec,ValueError>{controlled_case_schema(control)}
    fn encode_case_schema(control:&mut NativeEncodeControl<'_>)->Result<crate::RecordSpec,ValueError>{controlled_case_schema(control)}
    fn case_spec()->R{const FIELDS:&[F]=&[F::new(0,"id",H::Text),F::new(1,"loads",H::Block(statements_shape)),F::new(2,"enabled",H::Bool)];R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS}}
    struct CaseTableSource;
    impl FieldProjectionSource for CaseTableSource{
        fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{
            []=>Ok(V::Record(&[0])),[0]=>Ok(V::List(1)),[0,0]=>Ok(V::Record(&[0,1,2])),
            [0,0,0]=>Ok(V::Text("C")),[0,0,1]=>Ok(V::Block),[0,0,1,0]=>Ok(V::Statements(1)),
            [0,0,1,0,0]=>Ok(V::Record(&[0])),[0,0,1,0,0,0]=>Ok(V::Text("L")),[0,0,2]=>Ok(V::Bool(true)),
            _=>Err(crate::native_encoding::projection_path_error())}}
        fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path==[0,0,1,0]&&index==0{Ok("truss")}else{Err(crate::native_encoding::projection_path_error())}}
    }
    #[test]
    fn compound_table_block_statement_cell_matches_owned_and_sqlite(){let mut loan_issuer=OriginalWorkAuthority::new(LoanDriver);
        const FIELDS:&[F]=&[F::new(0,"cases",H::Table(case_spec))];let schema=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/📊️block-cell.json")).unwrap();
        let case_producer=crate::RecordSpecProducer{ordinary:owned_case_schema,encoding:encode_case_schema,decoding:decode_case_schema};
        let owned_schema=crate::RecordSpec::new(None,RecordLayout::Inline,vec![crate::FieldSpec::new(0,"cases",crate::Shape::Table(case_producer))]);
        let mut load=crate::RecordValue::default();load.fields.insert(0,crate::FieldValue::Text("L".into()));let mut row=crate::RecordValue::default();row.fields.insert(0,crate::FieldValue::Text("C".into()));row.fields.insert(1,crate::FieldValue::Block(Box::new(crate::FieldValue::Statements(vec![("truss".into(),load)]))));row.fields.insert(2,crate::FieldValue::Bool(true));let mut owned=crate::RecordValue::default();owned.fields.insert(0,crate::FieldValue::List(vec![crate::FieldValue::Record(row)]));
        assert_eq!(crate::print(&owned,&owned_schema,JoinMode::Document),fixture["document"].as_str().unwrap());
        let oracle=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const db=new Database(":memory:");const row=db.query("SELECT 'cases [id:TEXT loads:BLOCK enabled:BOOL] {'||char(10)||'  C {'||char(10)||'    truss id=L'||char(10)||'  }'||char(10)||'  true'||char(10)||'}'||char(10) AS document").get();process.stdout.write(row.document);db.close();"#]).output().unwrap();
        assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(oracle.stdout,fixture["document"].as_str().unwrap().as_bytes());
        let source=CaseTableSource;let mut frames:[Option<BorrowedRecordFrame>;2]=[const{None};2];let mut cursor=BorrowedRecordTree::default();let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(100000);let mut byte=[0u8;1];let mut output=Vec::new();
        with_work(&mut control,&mut loan_issuer,&mut loan,|control|{
            for _ in 0..100000{let demand=cursor.next_output_byte_demand(&frames)?;let turn=cursor.advance(&source,schema,&mut frames,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
            assert_eq!(output,fixture["document"].as_str().unwrap().as_bytes());for _ in 0..100000{if cursor.close(&mut frames,control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
        }).unwrap();
    }

    struct MixedModelSource;
    impl FieldProjectionSource for MixedModelSource{
        fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{
            []=>Ok(V::Record(&[0,1])),[0]=>Ok(V::List(1)),[0,0]=>Ok(V::Record(&[0,1])),[0,0,0]=>Ok(V::Text("A")),[0,0,1]=>Ok(V::Float(1.5)),
            [1]=>Ok(V::Block),[1,0]=>Ok(V::Statements(1)),[1,0,0]=>Ok(V::Record(&[0])),[1,0,0,0]=>Ok(V::Text("A")),
            _=>Err(crate::native_encoding::projection_path_error())}}
        fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path==[1,0]&&index==0{Ok("truss")}else{Err(crate::native_encoding::projection_path_error())}}
    }
    #[test]
    fn compound_model_orders_block_before_table(){let mut loan_issuer=OriginalWorkAuthority::new(LoanDriver);
        const FIELDS:&[F]=&[F::new(0,"nodes",H::Table(node_spec)),F::new(1,"elements",H::Block(statements_shape))];
        let schema=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🧩️model.json")).unwrap();
        let node_producer=crate::RecordSpecProducer{ordinary:owned_table_node_schema,encoding:encode_table_node_schema,decoding:decode_table_node_schema};let truss_producer=crate::RecordSpecProducer{ordinary:owned_truss_schema,encoding:encode_truss_schema,decoding:decode_truss_schema};
        let owned_schema=crate::RecordSpec::new(None,RecordLayout::Inline,vec![crate::FieldSpec::new(0,"nodes",crate::Shape::Table(node_producer)),crate::FieldSpec::new(1,"elements",crate::Shape::Block(Box::new(crate::Shape::Statements(vec![("truss".into(),truss_producer)]))))]);
        let mut node=crate::RecordValue::default();node.fields.insert(0,crate::FieldValue::Text("A".into()));node.fields.insert(1,crate::FieldValue::Float(1.5));let mut element=crate::RecordValue::default();element.fields.insert(0,crate::FieldValue::Text("A".into()));let mut owned=crate::RecordValue::default();owned.fields.insert(0,crate::FieldValue::List(vec![crate::FieldValue::Record(node)]));owned.fields.insert(1,crate::FieldValue::Block(Box::new(crate::FieldValue::Statements(vec![("truss".into(),element)]))));
        assert_eq!(crate::print(&owned,&owned_schema,JoinMode::Document),fixture["document"].as_str().unwrap());
        let oracle=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const db=new Database(":memory:");const row=db.query("SELECT 'elements {'||char(10)||'  truss id=A'||char(10)||'}'||char(10)||'nodes [id:TEXT x:NUM] {'||char(10)||'  A 1.5'||char(10)||'}'||char(10) AS document").get();process.stdout.write(row.document);db.close();"#]).output().unwrap();
        assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(oracle.stdout,fixture["document"].as_str().unwrap().as_bytes());
        let source=MixedModelSource;let mut frames:[Option<BorrowedRecordFrame>;2]=[const{None};2];let mut cursor=BorrowedRecordTree::default();let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(100000);let mut byte=[0u8;1];let mut output=Vec::new();
        with_work(&mut control,&mut loan_issuer,&mut loan,|control|{
            for _ in 0..100000{let demand=cursor.next_output_byte_demand(&frames)?;let turn=cursor.advance(&source,schema,&mut frames,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
            assert_eq!(output,fixture["document"].as_str().unwrap().as_bytes());for _ in 0..100000{if cursor.close(&mut frames,control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
        }).unwrap();
    }

    fn owned_meta_schema()->crate::RecordSpec{crate::RecordSpec::new(None,RecordLayout::Inline,vec![crate::FieldSpec::new(0,"name",crate::Shape::Text)])}
    fn controlled_meta_schema<C:crate::NativeSchemaControl>(control:&mut C)->Result<crate::RecordSpec,ValueError>{let mut fields=control.allocate_vec(1)?;fields.push(crate::producer::field(0,"name",crate::Shape::Text,control)?);crate::producer::record(None,RecordLayout::Inline,fields,control)}
    fn decode_meta_schema(control:&mut semio_framework_value::native_decoding::NativeDecodeControl<'_>)->Result<crate::RecordSpec,ValueError>{controlled_meta_schema(control)}
    fn encode_meta_schema(control:&mut NativeEncodeControl<'_>)->Result<crate::RecordSpec,ValueError>{controlled_meta_schema(control)}
    fn meta_producer()->crate::RecordSpecProducer{crate::RecordSpecProducer{ordinary:owned_meta_schema,encoding:encode_meta_schema,decoding:decode_meta_schema}}
    fn owned_record_cell_schema()->crate::RecordSpec{crate::RecordSpec::new(None,RecordLayout::Inline,vec![crate::FieldSpec::new(0,"id",crate::Shape::Text),crate::FieldSpec::new(1,"meta",crate::Shape::Record(meta_producer()))])}
    fn controlled_record_cell_schema<C:crate::NativeSchemaControl>(control:&mut C)->Result<crate::RecordSpec,ValueError>{let mut fields=control.allocate_vec(2)?;fields.push(crate::producer::field(0,"id",crate::Shape::Text,control)?);fields.push(crate::producer::field(1,"meta",crate::Shape::Record(meta_producer()),control)?);crate::producer::record(None,RecordLayout::Inline,fields,control)}
    fn decode_record_cell_schema(control:&mut semio_framework_value::native_decoding::NativeDecodeControl<'_>)->Result<crate::RecordSpec,ValueError>{controlled_record_cell_schema(control)}
    fn encode_record_cell_schema(control:&mut NativeEncodeControl<'_>)->Result<crate::RecordSpec,ValueError>{controlled_record_cell_schema(control)}
    fn meta_spec()->R{const FIELDS:&[F]=&[F::new(0,"name",H::Text)];R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS}}
    fn record_cell_spec()->R{const FIELDS:&[F]=&[F::new(0,"id",H::Text),F::new(1,"meta",H::Record(meta_spec))];R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS}}
    struct RecordCellSource;
    impl FieldProjectionSource for RecordCellSource{
        fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{
            []=>Ok(V::Record(&[0])),[0]=>Ok(V::List(1)),[0,0]=>Ok(V::Record(&[0,1])),[0,0,0]=>Ok(V::Text("A")),
            [0,0,1]=>Ok(V::Record(&[0])),[0,0,1,0]=>Ok(V::Text("hello")),_=>Err(crate::native_encoding::projection_path_error())}}
    }
    #[test]
    fn compound_table_record_cell_keeps_glued_braces(){let mut loan_issuer=OriginalWorkAuthority::new(LoanDriver);
        const FIELDS:&[F]=&[F::new(0,"rows",H::Table(record_cell_spec))];let schema=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/📊️record-cell.json")).unwrap();
        let row_producer=crate::RecordSpecProducer{ordinary:owned_record_cell_schema,encoding:encode_record_cell_schema,decoding:decode_record_cell_schema};
        let owned_schema=crate::RecordSpec::new(None,RecordLayout::Inline,vec![crate::FieldSpec::new(0,"rows",crate::Shape::Table(row_producer))]);
        let mut meta=crate::RecordValue::default();meta.fields.insert(0,crate::FieldValue::Text("hello".into()));let mut row=crate::RecordValue::default();row.fields.insert(0,crate::FieldValue::Text("A".into()));row.fields.insert(1,crate::FieldValue::Record(meta));let mut owned=crate::RecordValue::default();owned.fields.insert(0,crate::FieldValue::List(vec![crate::FieldValue::Record(row)]));
        assert_eq!(crate::print(&owned,&owned_schema,JoinMode::Document),fixture["document"].as_str().unwrap());
        let oracle=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const db=new Database(":memory:");const row=db.query("SELECT 'rows [id:TEXT meta:REC] {'||char(10)||'  A {name=hello}'||char(10)||'}'||char(10) AS document").get();process.stdout.write(row.document);db.close();"#]).output().unwrap();
        assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(oracle.stdout,fixture["document"].as_str().unwrap().as_bytes());
        let source=RecordCellSource;let mut frames:[Option<BorrowedRecordFrame>;2]=[const{None};2];let mut cursor=BorrowedRecordTree::default();let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(100000);let mut byte=[0u8;1];let mut output=Vec::new();
        with_work(&mut control,&mut loan_issuer,&mut loan,|control|{
            for _ in 0..100000{let demand=cursor.next_output_byte_demand(&frames)?;let turn=cursor.advance(&source,schema,&mut frames,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
            assert_eq!(output,fixture["document"].as_str().unwrap().as_bytes());for _ in 0..100000{if cursor.close(&mut frames,control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
        }).unwrap();
    }

}
