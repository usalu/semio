//! 📑️ Retained scalar record ordering over immutable source projections.
use super::*;
use super::token::BorrowedTextToken;
use std::fmt::Write;

#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Validate,Keyword,FindLast,Positions,Named,FieldKey,FieldEquals,Value,NumberNext,ListNext,ListClose,BytesNext,BytesClose,BlockNewline,BlockKey,BlockOpen,BlockValueStart,BlockCloseLine,BlockCloseIndent,StatementFind,StatementNewline,TableNewline,TableKey,TableBegin,TableHeaderKey,TableHeaderColon,TableHeaderType,TableBlockOpen,TableRowStart,TableValidate,TableCellSelect,TableCloseLine,TableCloseIndent,TextClose,ChildWait,Finish,Complete,Closed}
#[derive(Clone,Copy)]
enum Separator{Space,Comma}
#[derive(Clone,Copy,PartialEq,Eq)]
enum SequenceKind{List,Tuple,Coord,Direction,Statements}
#[derive(Clone,Copy)]
struct ScalarList{length:usize,index:usize,shape:H,kind:SequenceKind}
#[derive(Clone,Copy)]
struct ScalarTable{spec:R,length:usize,row:usize,column:usize,header:usize,validate:usize}

/// 🧵️ Retains scalar record field order, formatting and byte frontier without copying the source.
pub struct BorrowedScalarRecordCursor{
    phase:Phase,identity:Option<usize>,spec:Option<R>,index:usize,rank:u8,position:u16,last:Option<(u8,usize)>,
    field:usize,keyed:bool,line_open:bool,glued:bool,indent:usize,indent_pending:usize,document:bool,block:bool,block_shape:Option<H>,table:Option<ScalarTable>,child:Option<R>,child_braced:bool,child_tight:bool,enum_index:usize,statement_variant:usize,statement_offset:usize,sequence:Option<ScalarList>,literal:Option<&'static str>,literal_offset:usize,
    separator:Option<Separator>,number_len:usize,number_offset:usize,number_suffix:Option<&'static str>,byte_offset:usize,byte_unit:usize,byte_len:usize,byte_intrinsic:bool,token:Option<BorrowedTextToken>,written:usize,
}
/// 🧾️ Carries the initialized prefix from a single record turn.
#[derive(Default,Debug)]
pub struct BorrowedScalarRecordProgress{pub written_bytes:usize,pub complete:bool}
/// ⚠️ Carries the original refusal and output already initialized in its turn.
#[derive(Debug)]
pub struct BorrowedScalarRecordFailure{pub reason:ValueError,pub written_bytes:usize,pub settlement:Option<ValueError>}
struct ScalarBuffer{length:usize,target:usize,selected:Option<u8>,fractional:bool}
impl Write for ScalarBuffer{
    fn write_str(&mut self,text:&str)->std::fmt::Result{
        let end=self.length.checked_add(text.len()).ok_or(std::fmt::Error)?;
        if self.target>=self.length&&self.target<end{self.selected=Some(text.as_bytes()[self.target-self.length])}
        self.fractional|=text.as_bytes().iter().any(|byte|matches!(*byte,b'.'|b'e'|b'E'));self.length=end;Ok(())
    }
}
impl Default for BorrowedScalarRecordCursor{
    fn default()->Self{Self{phase:Phase::Validate,identity:None,spec:None,index:0,rank:0,position:0,last:None,field:0,keyed:false,line_open:false,glued:false,indent:0,indent_pending:0,document:true,block:false,block_shape:None,table:None,child:None,child_braced:false,child_tight:false,enum_index:0,statement_variant:0,statement_offset:0,sequence:None,literal:None,literal_offset:0,separator:None,number_len:0,number_offset:0,number_suffix:None,byte_offset:0,byte_unit:0,byte_len:0,byte_intrinsic:false,token:None,written:0}}
}
impl BorrowedScalarRecordCursor{
    pub(super) fn nested(line_open:bool,glued:bool,indent:usize)->Self{Self{document:false,line_open,glued,indent,..Default::default()}}
    pub(super) fn child_request(&self)->Option<(R,[usize;5],usize,bool,bool,usize)>{if self.phase==Phase::ChildWait&&!self.queued(){let(path,length)=self.field_path(true);self.child.map(|spec|(spec,path,length,self.line_open,self.glued,self.indent))}else{None}}
    pub(super) fn format_context(&self)->(bool,bool,usize){(self.line_open,self.glued,self.indent)}
    pub(super) fn finish_child(&mut self,line_open:bool,glued:bool,indent:usize){self.line_open=line_open;self.glued=glued;self.indent=indent;if self.child_braced{if self.child_tight{self.glued=true}self.atom("}")}self.child=None;self.child_braced=false;self.child_tight=false;self.after_field()}
    /// 📍️ Reports the initialized output prefix.
    pub fn emitted_bytes(&self)->usize{self.written}
    /// 📏️ Quotes the next initialized output byte before the caller lends its physical span.
    pub fn next_output_byte_demand(&self)->usize{if self.indent_pending!=0||self.separator.is_some()||self.literal.is_some_and(|text|self.literal_offset<text.len())||self.phase==Phase::NumberNext&&self.number_offset<self.number_len||self.phase==Phase::BytesNext&&self.byte_offset<self.byte_len{1}else if self.phase==Phase::Value{self.token.as_ref().map_or(0,BorrowedTextToken::next_output_byte_demand)}else{0}}
    /// ✅️ Reports that the final document newline has been emitted.
    pub fn is_complete(&self)->bool{self.phase==Phase::Complete&&!self.queued()}
    /// 🍂️ Reports that inline state has been retired.
    pub fn terminal_is_empty(&self)->bool{self.phase==Phase::Closed}
    fn failure(reason:ValueError,written_bytes:usize)->BorrowedScalarRecordFailure{BorrowedScalarRecordFailure{reason,written_bytes,settlement:None}}
    fn invalid(reason:&'static str)->BorrowedScalarRecordFailure{Self::failure(ValueError::literal(ValueRefusalKind::InvalidValue,reason),0)}
    fn queued(&self)->bool{self.literal.is_some()||self.separator.is_some()||self.indent_pending!=0}
    fn atom(&mut self,literal:&'static str){if !self.line_open{self.indent_pending=self.indent*2}else if !self.glued{self.separator=Some(Separator::Space);}self.line_open=true;self.glued=false;self.literal=Some(literal);self.literal_offset=0;}
    fn after_field(&mut self){if let Some(sequence)=self.sequence.as_mut(){sequence.index+=1;self.phase=Phase::ListNext}else if self.block{self.phase=Phase::BlockCloseLine}else if let Some(table)=self.table.as_mut(){table.column+=1;self.phase=Phase::TableCellSelect}else{self.phase=if self.keyed{Phase::Named}else{Phase::Positions};self.index+=1;}}
    fn field_path(&self,include_index:bool)->([usize;5],usize){let mut path=[0;5];path[0]=self.field;let mut length=1;if let Some(table)=self.table{path[length]=table.row;path[length+1]=table.column;length+=2}if self.block{path[length]=0;length+=1}if include_index{if let Some(sequence)=self.sequence{path[length]=sequence.index;length+=1}}(path,length)}
    /// ⏱️ Performs one admitted ordering, formatting, or byte-output action.
    pub fn advance(&mut self,source:&dyn FieldProjectionSource,spec:R,output:&mut[u8],control:&mut NativeEncodeControl<'_>)->Result<BorrowedScalarRecordProgress,BorrowedScalarRecordFailure>{
        if self.phase==Phase::Closed{return Err(Self::invalid("borrowed scalar record is closed"))}
        if self.is_complete(){return Ok(BorrowedScalarRecordProgress{complete:true,..Default::default()})}
        if self.identity.is_some_and(|old|old!=source.projection_identity())||self.spec.is_some_and(|old|old.fields.as_ptr()!=spec.fields.as_ptr()||old.fields.len()!=spec.fields.len()||old.keyword!=spec.keyword||old.layout!=spec.layout){return Err(Self::invalid("borrowed scalar record source or schema changed"))}
        if self.next_output_byte_demand()!=0&&output.is_empty(){return Ok(Default::default())}
        if self.phase==Phase::Value{
            if let Some(token)=self.token.as_mut(){
                control.checkpoint().map_err(|error|Self::failure(error,0))?;if !control.prepare_work(1).map_err(|error|Self::failure(error,0))?{return Ok(Default::default())}
                let(path,length)=self.field_path(true);let projection=source.projection_view(&path[..length]);let settlement=control.commit_work(1);
                let projection=match projection{Ok(value)=>{settlement.map_err(|error|Self::failure(error,0))?;value},Err(error)=>{let mut failure=Self::failure(error,0);failure.settlement=settlement.err();return Err(failure)}};
                let intrinsic=matches!(self.sequence.map_or_else(||self.block_shape.unwrap_or_else(||self.table.map_or(spec.fields[self.field].shape,|table|table.spec.fields[table.column].shape)),|sequence|sequence.shape),H::Value);
                let text=match(projection,intrinsic){(V::Text(text),false)|(V::IntrinsicText(text),true)=>text,_=>return Err(Self::invalid("borrowed scalar text projection changed"))};
                let turn=match token.advance(text,output,control){Ok(turn)=>turn,Err(failure)=>{self.written+=failure.written_bytes;let mut outer=Self::failure(failure.reason,failure.written_bytes);outer.settlement=failure.settlement;return Err(outer)}};
                self.written+=turn.written_bytes;if turn.complete{self.phase=Phase::TextClose}
                return Ok(BorrowedScalarRecordProgress{written_bytes:turn.written_bytes,complete:false})
            }
        }
        if self.phase==Phase::TextClose{
            let token=self.token.as_mut().unwrap();
            if !token.terminal_is_empty(){token.close(control).map_err(|error|Self::failure(error,0))?;return Ok(Default::default())}
            control.checkpoint().map_err(|error|Self::failure(error,0))?;
            if !control.prepare_work(1).map_err(|error|Self::failure(error,0))?{return Ok(Default::default())}
            self.token=None;self.after_field();control.commit_work(1).map_err(|error|Self::failure(error,0))?;
            return Ok(Default::default())
        }
        control.checkpoint().map_err(|error|Self::failure(error,0))?;
        if !control.prepare_work(1).map_err(|error|Self::failure(error,0))?{return Ok(Default::default())}
        let operation=(||->Result<usize,BorrowedScalarRecordFailure>{
        let view=if self.queued(){None}else{match self.phase{
            Phase::Validate=>Some(source.projection_view(&[]).map_err(|error|Self::failure(error,0))?),
            Phase::FindLast if self.index<spec.fields.len()=>Some(source.projection_view(&[self.index]).map_err(|error|Self::failure(error,0))?),
            Phase::Positions if self.index<spec.fields.len()=>Some(source.projection_view(&[self.index]).map_err(|error|Self::failure(error,0))?),
            Phase::Named if self.index<spec.fields.len()=>Some(source.projection_view(&[self.index]).map_err(|error|Self::failure(error,0))?),
            Phase::BlockOpen=>Some(source.projection_view(&[self.field]).map_err(|error|Self::failure(error,0))?),
            Phase::TableBegin=>Some(source.projection_view(&[self.field]).map_err(|error|Self::failure(error,0))?),
            Phase::TableValidate=>{let table=self.table.ok_or_else(||Self::invalid("borrowed table frame is absent"))?;Some(source.projection_view(&[self.field,table.row]).map_err(|error|Self::failure(error,0))?)},
            Phase::TableCellSelect if self.table.is_some_and(|table|table.column<table.spec.fields.len())=>Some({let(path,length)=self.field_path(false);source.projection_view(&path[..length])}.map_err(|error|Self::failure(error,0))?),
            Phase::Value|Phase::NumberNext=>Some({let(path,length)=self.field_path(true);source.projection_view(&path[..length])}.map_err(|error|Self::failure(error,0))?),
            Phase::BytesNext if self.byte_offset<self.byte_len=>Some({let(path,length)=self.field_path(true);source.projection_view(&path[..length])}.map_err(|error|Self::failure(error,0))?),
            _=>None,
        }};
        let mut formatted=ScalarBuffer{length:0,target:if self.phase==Phase::NumberNext{self.number_offset}else{usize::MAX},selected:None,fractional:false};let mut static_literal:Option<&'static str>=None;let mut enum_scanning=false;let mut new_sequence=None;let mut new_record=None;let mut new_bytes=None;let mut base64_byte=None;let mut new_block_shape=None;let mut inline_block=None;let mut new_table=None;
        if matches!(self.phase,Phase::Value|Phase::NumberNext)&&!self.queued(){
            let shape=self.sequence.map_or_else(||self.block_shape.unwrap_or_else(||self.table.map_or(spec.fields[self.field].shape,|table|table.spec.fields[table.column].shape)),|sequence|sequence.shape);
            let result=match(view.as_ref().unwrap(),shape){
                (V::Bool(value),H::Bool)=>write!(formatted,"{}",value),
                (V::Int(value),H::Int)=>write!(formatted,"{}",value),
                (V::UInt(value),H::UInt)=>write!(formatted,"{}",value),
                (V::UInt(value),H::Count)=>write!(formatted,"x{}",value),
                (V::Float(value),H::Float)=>if value.is_nan(){write!(formatted,"nan64_{:016x}",value.to_bits())}else{write!(formatted,"{}",value)},
                (V::Float(value),H::Quantity(unit)|H::Angle(unit))=>{static_literal=Some(unit.symbol);if value.is_nan(){write!(formatted,"nan64_{:016x}",value.to_bits())}else{write!(formatted,"{}",value)}},
                (V::Enum(value),H::Enum(variants))=>match variants.get(self.enum_index){Some((name,ordinal))if ordinal==value=>{static_literal=Some(*name);Ok(())},Some(_)=>{enum_scanning=true;Ok(())},None=>return Err(Self::invalid("borrowed scalar enum ordinal is invalid"))},
                (V::Text(_),H::Text|H::Ref(_))|(V::IntrinsicText(_),H::Value)=>Ok(()),
                (V::Record(_),H::Record(make)) if self.sequence.is_none_or(|sequence|sequence.kind==SequenceKind::List)=>{new_record=Some(make());Ok(())},
                (V::Block,H::Block(make)) if !self.block=>{inline_block=Some(make());Ok(())},
                (V::IntrinsicNull,H::Value)=>{static_literal=Some("null");Ok(())},
                (V::IntrinsicBool(value),H::Value)=>write!(formatted,"{}",value),
                (V::IntrinsicNumber(Number::UInt(value)),H::Value)=>write!(formatted,"{}",value),
                (V::IntrinsicNumber(Number::Int(value)),H::Value) if *value>=0=>write!(formatted,"int64({})",value),
                (V::IntrinsicNumber(Number::Int(value)),H::Value)=>write!(formatted,"{}",value),
                (V::IntrinsicNumber(Number::Float(value)),H::Value)=>{let result=if value.is_nan(){write!(formatted,"nan64_{:016x}",value.to_bits())}else{write!(formatted,"{}",value)};if result.is_ok()&&value.is_finite()&&!formatted.fractional{formatted.write_str(".0")}else{result}},
                (V::Bytes(bytes),H::Bytes64)=>{new_bytes=Some((bytes.len(),false));Ok(())},
                (V::BytesSource(bytes),H::Bytes64)=>{new_bytes=Some((bytes.byte_len(),false));Ok(())},
                (V::IntrinsicBytes(bytes),H::Value)=>{new_bytes=Some((bytes.len(),true));Ok(())},
                (V::IntrinsicBytesSource(bytes),H::Value)=>{new_bytes=Some((bytes.byte_len(),true));Ok(())},
                (V::List(length),H::List(make)) if self.sequence.is_none()=>{new_sequence=Some(ScalarList{length:*length,index:0,shape:make(),kind:SequenceKind::List});Ok(())},
                (V::Statements(length),H::Statements(variants)) if self.sequence.is_none()=>{new_sequence=Some(ScalarList{length:*length,index:0,shape:H::Statements(variants),kind:SequenceKind::Statements});Ok(())},
                (V::Tuple(length),H::Tuple(make,expected)) if self.sequence.is_none()&&!expected.is_some_and(|expected|expected!=*length)=>{new_sequence=Some(ScalarList{length:*length,index:0,shape:make(),kind:SequenceKind::Tuple});Ok(())},
                (V::Tuple(length),H::Coord(dimensions)) if self.sequence.is_none()&&*length==usize::from(dimensions)=>{new_sequence=Some(ScalarList{length:*length,index:0,shape:H::Float,kind:SequenceKind::Coord});Ok(())},
                (V::Tuple(length),H::Dir) if self.sequence.is_none()&&*length==3=>{new_sequence=Some(ScalarList{length:*length,index:0,shape:H::Float,kind:SequenceKind::Direction});Ok(())},
                _=>return Err(Self::invalid("borrowed scalar record requires scalar field projections")),
            };
            result.map_err(|_|Self::invalid("borrowed scalar format length overflow"))?;
        }
        let mut statement_event=None;
        if self.phase==Phase::StatementFind&&!self.queued(){
            let sequence=self.sequence.ok_or_else(||Self::invalid("borrowed statements frame is absent"))?;
            let H::Statements(variants)=sequence.shape else{return Err(Self::invalid("borrowed statements schema changed"))};
            let(name,make)=variants.get(self.statement_variant).ok_or_else(||Self::invalid("borrowed statement keyword is invalid"))?;
            let(path,length)=self.field_path(false);let key=source.projection_key(&path[..length],sequence.index).map_err(|error|Self::failure(error,0))?;
            statement_event=Some(match(key.as_bytes().get(self.statement_offset),name.as_bytes().get(self.statement_offset)){(Some(left),Some(right))if left==right=>(0,None),(None,None)=>(1,Some(make())),_=>(2,None)});
        }
        if self.phase==Phase::TableBegin&&!self.queued(){match(view.as_ref(),spec.fields[self.field].shape){(Some(V::List(length)),H::Table(make))=>new_table=Some(ScalarTable{spec:make(),length:*length,row:0,column:0,header:0,validate:0}),_=>return Err(Self::invalid("borrowed table projection changed"))}}
        if self.phase==Phase::TableValidate&&!self.queued(){let table=self.table.ok_or_else(||Self::invalid("borrowed table frame is absent"))?;let Some(V::Record(ids))=view.as_ref() else{return Err(Self::invalid("borrowed table row is not a record"))};if ids.len()!=table.spec.fields.len()||table.validate<table.spec.fields.len()&&ids[table.validate]!=table.spec.fields[table.validate].id{return Err(Self::invalid("borrowed table row schema changed"))}}
        if self.phase==Phase::BlockOpen&&!self.queued(){match(view.as_ref(),spec.fields[self.field].shape){(Some(V::Block),H::Block(make))=>new_block_shape=Some(make()),_=>return Err(Self::invalid("borrowed block projection changed"))}}
        if self.phase==Phase::BytesNext&&!self.queued()&&self.byte_offset<self.byte_len{
            let count=(self.byte_len-self.byte_offset).min(3);
            let byte_at=|index:usize|->Option<u8>{match(view.as_ref().unwrap(),self.byte_intrinsic){
                (V::Bytes(bytes),false)|(V::IntrinsicBytes(bytes),true) if bytes.len()==self.byte_len=>bytes.get(index).copied(),
                (V::BytesSource(bytes),false)|(V::IntrinsicBytesSource(bytes),true) if bytes.byte_len()==self.byte_len=>bytes.byte_at(index),
                _=>None,
            }};
            let first=byte_at(self.byte_offset).ok_or_else(||Self::invalid("borrowed byte source changed"))?;
            let second=if count>1{byte_at(self.byte_offset+1).ok_or_else(||Self::invalid("borrowed byte source changed"))?}else{0};
            let third=if count>2{byte_at(self.byte_offset+2).ok_or_else(||Self::invalid("borrowed byte source changed"))?}else{0};
            const ALPHABET:&[u8;64]=b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
            let value=match self.byte_unit{0=>ALPHABET[(first>>2) as usize],1=>ALPHABET[((first&3)<<4|second>>4) as usize],2 if count>1=>ALPHABET[((second&15)<<2|third>>6) as usize],3 if count>2=>ALPHABET[(third&63) as usize],_=>b'='};
            base64_byte=Some((value,count));
        }
        if self.phase==Phase::Validate{
            let Some(V::Record(ids))=view.as_ref() else{return Err(Self::invalid("borrowed scalar record projection is absent"))};
            if ids.len()!=spec.fields.len()||self.index<spec.fields.len()&&ids[self.index]!=spec.fields[self.index].id{return Err(Self::invalid("borrowed scalar record schema changed"))}
            if spec.layout==RecordLayout::Call{return Err(Self::invalid("borrowed scalar record component requires non-call layout"))}
        }
        self.identity=Some(source.projection_identity());self.spec=Some(spec);let mut written=0;
        if self.indent_pending!=0{
            let next=self.written.checked_add(1).ok_or_else(||Self::invalid("borrowed scalar output length overflow"))?;output[0]=b' ';self.indent_pending-=1;self.written=next;written=1;
        }else if let Some(separator)=self.separator{
            let next=self.written.checked_add(1).ok_or_else(||Self::invalid("borrowed scalar output length overflow"))?;output[0]=match separator{Separator::Space=>b' ',Separator::Comma=>b','};self.separator=None;self.written=next;written=1;
        }else if let Some(text)=self.literal{
            if self.literal_offset<text.len(){let next=self.written.checked_add(1).ok_or_else(||Self::invalid("borrowed scalar output length overflow"))?;output[0]=text.as_bytes()[self.literal_offset];self.literal_offset+=1;self.written=next;written=1;}
            if self.literal_offset==text.len(){self.literal=None;self.literal_offset=0;}
        }else{
            match self.phase{
                Phase::Validate=>{if self.index<spec.fields.len(){self.index+=1}else{self.index=0;self.phase=Phase::Keyword}},
                Phase::Keyword=>{if let Some(keyword)=spec.keyword{self.atom(keyword)}self.phase=Phase::FindLast},
                Phase::FindLast=>{if let Some(field)=spec.fields.get(self.index){if let Some(position)=field.position.filter(|_|!field.is_call_name){if !matches!(view,Some(V::Absent)){self.last=Some(self.last.map_or((position,self.index),|last|last.max((position,self.index))))}}self.index+=1}else{self.index=0;self.position=0;self.phase=Phase::Positions}},
                Phase::Positions=>{
                    if self.last.is_none_or(|last|self.position>last.0 as u16){self.index=0;self.rank=0;self.phase=Phase::Named}
                    else if self.index==spec.fields.len(){self.index=0;self.position+=1}
                    else{let field=spec.fields[self.index];if field.position==Some(self.position as u8)&&!field.is_call_name&&(field.position.unwrap(),self.index)<=self.last.unwrap(){self.field=self.index;self.keyed=false;if matches!(view,Some(V::Absent)){self.atom("_");self.index+=1}else{if !self.line_open{self.indent_pending=self.indent*2}else if !self.glued{self.separator=Some(Separator::Space);}self.line_open=true;self.glued=false;self.phase=Phase::Value}}else{self.index+=1}}
                },
                Phase::Named=>{if self.index==spec.fields.len(){if self.rank==4{self.phase=Phase::Finish}else{self.rank+=1;self.index=0}}else{let field=spec.fields[self.index];if field.position.is_none()&&!field.key.is_empty()&&!field.is_call_name&&rank(field.shape)==self.rank&&!matches!(view,Some(V::Absent)){self.field=self.index;self.keyed=true;self.phase=if matches!(field.shape,H::Block(_)){Phase::BlockNewline}else if matches!(field.shape,H::Table(_)){Phase::TableNewline}else if matches!(field.shape,H::Statements(_)){Phase::Value}else{Phase::FieldKey}}else{self.index+=1}}},
                Phase::TableNewline=>{if self.line_open{self.literal=Some("\n");self.literal_offset=0;self.line_open=false}self.phase=Phase::TableKey},
                Phase::TableKey=>{self.atom(spec.fields[self.field].key);self.phase=Phase::TableBegin},
                Phase::TableBegin=>{self.table=new_table;self.atom("[");self.glued=true;self.phase=Phase::TableHeaderKey},
                Phase::TableHeaderKey=>{let table=self.table.unwrap();if let Some(field)=table.spec.fields.get(table.header){self.atom(field.key);self.phase=Phase::TableHeaderColon}else{self.glued=true;self.atom("]");self.phase=Phase::TableBlockOpen}},
                Phase::TableHeaderColon=>{self.literal=Some(":");self.literal_offset=0;self.phase=Phase::TableHeaderType},
                Phase::TableHeaderType=>{let table=self.table.as_mut().unwrap();self.literal=Some(borrowed_type_name(table.spec.fields[table.header].shape));self.literal_offset=0;table.header+=1;self.phase=Phase::TableHeaderKey},
                Phase::TableBlockOpen=>{self.literal=Some(" {\n");self.literal_offset=0;self.indent+=1;self.line_open=false;self.glued=false;self.phase=Phase::TableRowStart},
                Phase::TableRowStart=>{let table=self.table.as_mut().unwrap();if table.row==table.length{self.phase=Phase::TableCloseLine}else{if self.line_open{self.literal=Some("\n");self.literal_offset=0;self.line_open=false}table.validate=0;self.phase=Phase::TableValidate}},
                Phase::TableValidate=>{let table=self.table.as_mut().unwrap();if table.validate<table.spec.fields.len(){table.validate+=1}else{table.column=0;self.phase=Phase::TableCellSelect}},
                Phase::TableCellSelect=>{let table=self.table.unwrap();if table.column==table.spec.fields.len(){self.table.as_mut().unwrap().row+=1;self.phase=Phase::TableRowStart}else if matches!(view,Some(V::Absent)){self.atom("_");self.table.as_mut().unwrap().column+=1}else{if !matches!(table.spec.fields[table.column].shape,H::Block(_)){if !self.line_open{self.indent_pending=self.indent*2}else if !self.glued{self.separator=Some(Separator::Space)}self.line_open=true;self.glued=false}self.phase=Phase::Value}},
                Phase::TableCloseLine=>{if self.line_open{self.literal=Some("\n");self.literal_offset=0;self.line_open=false}self.phase=Phase::TableCloseIndent},
                Phase::TableCloseIndent=>{if self.indent==0{return Err(Self::invalid("borrowed table indent underflow"))}self.indent-=1;self.indent_pending=self.indent*2;self.literal=Some("}\n");self.literal_offset=0;self.line_open=false;self.glued=false;self.table=None;self.after_field()},
                Phase::BlockNewline=>{if self.line_open{self.literal=Some("\n");self.literal_offset=0;self.line_open=false}self.phase=Phase::BlockKey},
                Phase::BlockKey=>{self.atom(spec.fields[self.field].key);self.phase=Phase::BlockOpen},
                Phase::BlockOpen=>{let shape=new_block_shape.unwrap();self.literal=Some(" {\n");self.literal_offset=0;self.indent+=1;self.line_open=false;self.glued=false;self.block=true;self.block_shape=Some(shape);self.phase=Phase::BlockValueStart},
                Phase::BlockValueStart=>{let shape=self.block_shape.unwrap();if !matches!(shape,H::Record(_)|H::Statements(_)|H::Block(_)){self.indent_pending=self.indent*2;self.line_open=true}self.phase=Phase::Value},
                Phase::BlockCloseLine=>{if self.line_open{self.literal=Some("\n");self.literal_offset=0;self.line_open=false}self.phase=Phase::BlockCloseIndent},
                Phase::BlockCloseIndent=>{if self.indent==0{return Err(Self::invalid("borrowed block indent underflow"))}self.indent-=1;self.indent_pending=self.indent*2;self.literal=Some("}\n");self.literal_offset=0;self.line_open=false;self.glued=false;self.block=false;self.block_shape=None;self.after_field()},
                Phase::FieldKey=>{self.atom(spec.fields[self.field].key);self.phase=Phase::FieldEquals},
                Phase::FieldEquals=>{self.literal=Some("=");self.literal_offset=0;self.glued=true;self.phase=Phase::Value},
                Phase::Value=>{if let Some(shape)=inline_block{self.literal=Some(" {\n");self.literal_offset=0;self.indent+=1;self.line_open=false;self.glued=false;self.block=true;self.block_shape=Some(shape);self.phase=Phase::BlockValueStart}else if let Some(spec)=new_record{self.child=Some(spec);self.child_tight=self.table.is_some()&&!self.block&&self.sequence.is_none();self.child_braced=self.sequence.is_some()||self.child_tight;if self.child_braced{self.literal=Some("{");self.literal_offset=0;self.line_open=true;self.glued=self.child_tight}else if !self.keyed{self.glued=true}self.phase=Phase::ChildWait}else{self.glued=false;if let Some((length,intrinsic))=new_bytes{self.byte_len=length;self.byte_offset=0;self.byte_unit=0;self.byte_intrinsic=intrinsic;self.literal=Some(if intrinsic{"bytes64(\""}else{"\""});self.literal_offset=0;self.phase=Phase::BytesNext}else if let Some(sequence)=new_sequence{self.sequence=Some(sequence);self.literal=Some(match sequence.kind{SequenceKind::List=>"[",SequenceKind::Tuple=>"",SequenceKind::Coord=>"@",SequenceKind::Direction=>"^",SequenceKind::Statements=>""});self.literal_offset=0;self.phase=Phase::ListNext}else if enum_scanning{self.enum_index+=1}else if matches!(view.as_ref(),Some(V::Text(_)|V::IntrinsicText(_))){self.token=Some(BorrowedTextToken::new(matches!(view.as_ref(),Some(V::IntrinsicText(_)))))}else{self.enum_index=0;if formatted.length==0{if let Some(literal)=static_literal{self.literal=Some(literal);self.literal_offset=0}self.after_field()}else{self.number_len=formatted.length;self.number_offset=0;self.number_suffix=static_literal;self.phase=Phase::NumberNext}}}},
                Phase::NumberNext=>{if formatted.length!=self.number_len{return Err(Self::invalid("borrowed scalar formatting changed"))}let next=self.written.checked_add(1).ok_or_else(||Self::invalid("borrowed scalar output length overflow"))?;let selected=formatted.selected.ok_or_else(||Self::invalid("borrowed scalar format byte is absent"))?;output[0]=selected;self.number_offset+=1;self.written=next;written=1;if self.number_offset==self.number_len{self.number_len=0;self.number_offset=0;self.literal=self.number_suffix.take();self.literal_offset=0;self.after_field()}},
                Phase::ListNext=>{let sequence=self.sequence.unwrap();if sequence.index<sequence.length{if sequence.kind==SequenceKind::Statements{self.statement_variant=0;self.statement_offset=0;self.phase=Phase::StatementFind}else{if sequence.kind==SequenceKind::List||sequence.index>0{self.separator=Some(if sequence.kind==SequenceKind::List{Separator::Space}else{Separator::Comma});}self.line_open=true;self.glued=false;self.phase=Phase::Value}}else{self.phase=Phase::ListClose}},
                Phase::StatementFind=>{let(event,child)=statement_event.unwrap();match event{0=>self.statement_offset+=1,1=>{self.child=child;self.child_braced=false;self.child_tight=false;self.phase=Phase::StatementNewline},_=>{self.statement_variant+=1;self.statement_offset=0}}},
                Phase::StatementNewline=>{if self.line_open{self.literal=Some("\n");self.literal_offset=0;self.line_open=false}self.glued=false;self.phase=Phase::ChildWait},
                Phase::ListClose=>{if self.sequence.unwrap().kind==SequenceKind::List{self.atom("]")}self.sequence=None;self.after_field()},
                Phase::BytesNext=>{if let Some((byte,count))=base64_byte{let next=self.written.checked_add(1).ok_or_else(||Self::invalid("borrowed byte output length overflow"))?;output[0]=byte;self.written=next;written=1;self.byte_unit+=1;if self.byte_unit==4{self.byte_unit=0;self.byte_offset+=count}}else{self.phase=Phase::BytesClose}},
                Phase::BytesClose=>{self.literal=Some(if self.byte_intrinsic{"\")"}else{"\""});self.literal_offset=0;self.byte_offset=0;self.byte_unit=0;self.byte_len=0;self.byte_intrinsic=false;self.after_field()},
                Phase::Finish=>{if self.document&&self.line_open{self.literal=Some("\n");self.literal_offset=0;}self.phase=Phase::Complete},
                Phase::TextClose|Phase::ChildWait|Phase::Complete|Phase::Closed=>unreachable!(),
            }
        }
        Ok(written)})();
        let settlement=control.commit_work(1);
        match operation{Ok(written)=>{settlement.map_err(|error|Self::failure(error,written))?;Ok(BorrowedScalarRecordProgress{written_bytes:written,complete:self.is_complete()})},Err(mut failure)=>{failure.settlement=settlement.err();Err(failure)}}
    }
    /// 🍂️ Retires the token and inline record cursor with paid caller actions.
    pub fn close(&mut self,control:&mut NativeEncodeControl<'_>)->Result<bool,ValueError>{
        if self.terminal_is_empty(){return Ok(true)}
        if let Some(token)=self.token.as_mut(){if !token.terminal_is_empty(){token.close(control)?;return Ok(false)}}
        control.checkpoint()?;if !control.prepare_work(1)?{return Ok(false)}
        self.token=None;self.sequence=None;self.child=None;self.phase=Phase::Closed;self.identity=None;self.spec=None;self.literal=None;self.separator=None;self.indent_pending=0;self.block=false;self.block_shape=None;self.table=None;self.child_tight=false;self.number_len=0;self.number_offset=0;self.number_suffix=None;self.byte_offset=0;self.byte_len=0;self.byte_intrinsic=false;control.commit_work(1)?;Ok(true)
    }
}

#[cfg(test)]
mod tests{
 use super::*;
 use semio_framework_value::native_encoding::NativeEncodeWorkRecipient;
 struct Loan(u64);
 impl NativeEncodeWorkRecipient for Loan{
  fn admits(&mut self,_:u64,work:u64)->Result<bool,ValueError>{Ok(self.0>=work)}
  fn committed(&mut self,work:u64)->Result<(),ValueError>{self.0-=work;Ok(())}
 }
 struct Source<'a>(&'a str);
 impl FieldProjectionSource for Source<'_>{
  fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{[]=>Ok(V::Record(&[0])),[0]=>Ok(V::Text(self.0)),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unexpected test path"))}}
 }
 #[test]
 fn scalar_record_corpus_emits_one_byte_and_retains_denied_frontier(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();let mut denied_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec;
  const FIELDS:&[BorrowedFieldSpec]=&[BorrowedFieldSpec::new(0,"name",H::Text)];
  let spec=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🔣️.json")).unwrap();
  for case in fixture["cases"].as_array().unwrap(){
   let source=Source(case["value"].as_str().unwrap());let expected=case["document"].as_str().unwrap().as_bytes();
   let mut cursor=BorrowedScalarRecordCursor::default();let mut callback=|_|true;let mut control=NativeEncodeControl::new(0,&mut callback);
   let mut byte=[0u8;1];let mut result=Vec::new();let mut loan=Loan(10000);
   control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
    for _ in 0..10000{let demand=cursor.next_output_byte_demand();let step=cursor.advance(&source,spec,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(step.written_bytes,demand);result.extend_from_slice(&byte[..step.written_bytes]);if !result.is_empty(){break}}assert!(!result.is_empty());
    Ok::<_,ValueError>(())
   }).unwrap();
   let before=cursor.emitted_bytes();byte[0]=0xaa;
   let replacement=Source(source.0);
   control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
    let work=control.actual_work();let failure=cursor.advance(&replacement,spec,&mut byte,control).unwrap_err();
    assert_eq!(failure.reason.kind,ValueRefusalKind::InvalidValue);assert_eq!(failure.written_bytes,0);assert_eq!(cursor.emitted_bytes(),before);assert_eq!(control.actual_work(),work);Ok::<_,ValueError>(())
   }).unwrap();
   let mut denied=Loan(0);
   control.with_work_recipient(&mut denied_issuer.loan(&mut denied),|control|{
    let step=cursor.advance(&source,spec,&mut byte,control).map_err(|failure|failure.reason)?;
    assert_eq!(step.written_bytes,0);assert_eq!(cursor.emitted_bytes(),before);assert_eq!(byte[0],0xaa);
    assert!(!cursor.close(control)?);assert!(!cursor.terminal_is_empty());assert_eq!(cursor.emitted_bytes(),before);Ok::<_,ValueError>(())
   }).unwrap();
   control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
    for _ in 0..10000{let demand=cursor.next_output_byte_demand();let step=cursor.advance(&source,spec,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(step.written_bytes,demand);result.extend_from_slice(&byte[..step.written_bytes]);if step.complete{break}}
    assert!(cursor.is_complete());assert_eq!(result,expected);
    for _ in 0..10000{if cursor.close(control)?{break}}assert!(cursor.terminal_is_empty());
    assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
   }).unwrap();
  }
 }
 struct OrderedSource;
 impl FieldProjectionSource for OrderedSource{
  fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{[]=>Ok(V::Record(&[0,1,2,3,4])),[0]=>Ok(V::Int(9)),[1]=>Ok(V::Text("a b")),[2]=>Ok(V::Int(7)),[3]=>Ok(V::Absent),[4]=>Ok(V::Bool(true)),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unexpected test path"))}}
 }
 #[test]
 fn scalar_record_positional_and_named_order_matches_corpus(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[
   F{position:Some(2),..F::new(0,"late",H::Int)},F::new(1,"label",H::Text),
   F{position:Some(0),..F::new(2,"first",H::Int)},F{position:Some(1),optional:true,..F::new(3,"missing",H::Int)},
   F::new(4,"active",H::Bool),
  ];
  let spec=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🔢️scalar-record.json")).unwrap();
  let oracle=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const db=new Database(":memory:");const result=db.query("SELECT printf('%d _ %d label=\"%s\" active=%s',7,9,'a b','true') AS document").get();process.stdout.write(result.document+"\n");db.close();"#]).output().unwrap();
  assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(oracle.stdout,fixture["document"].as_str().unwrap().as_bytes());
  let owned_spec=RecordSpec::new(None,RecordLayout::Inline,vec![
   FieldSpec::new(0,"late",Shape::Int).positional(2),FieldSpec::new(1,"label",Shape::Text),
   FieldSpec::new(2,"first",Shape::Int).positional(0),FieldSpec::new(3,"missing",Shape::Int).positional(1).optional(),
   FieldSpec::new(4,"active",Shape::Bool),
  ]);
  let mut owned=RecordValue::default();owned.fields.insert(0,FieldValue::Int(9));owned.fields.insert(1,FieldValue::Text("a b".into()));owned.fields.insert(2,FieldValue::Int(7));owned.fields.insert(4,FieldValue::Bool(true));
  assert_eq!(crate::print(&owned,&owned_spec,JoinMode::Document),fixture["document"].as_str().unwrap());
  let source=OrderedSource;let mut cursor=BorrowedScalarRecordCursor::default();let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(10000);let mut output=Vec::new();let mut byte=[0u8;1];
  control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
   for _ in 0..10000{let turn=cursor.advance(&source,spec,&mut byte,control).map_err(|failure|failure.reason)?;output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
   assert!(cursor.is_complete());assert_eq!(output,fixture["document"].as_str().unwrap().as_bytes());for _ in 0..10000{if cursor.close(control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
  }).unwrap();
 }

 struct CountingSource(std::cell::Cell<usize>);
 impl FieldProjectionSource for CountingSource{
  fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{self.0.set(self.0.get()+1);match path{[]=>Ok(V::Record(&[0])),[0]=>Ok(V::Text("name")),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unexpected test path"))}}
 }
 #[test]
 fn scalar_record_zero_work_does_not_project_source(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[F::new(0,"name",H::Text)];
  let spec=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};let source=CountingSource(std::cell::Cell::new(0));
  let mut cursor=BorrowedScalarRecordCursor::default();let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(0);let mut byte=[0u8;1];
  control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
   let step=cursor.advance(&source,spec,&mut byte,control).map_err(|failure|failure.reason)?;
   assert_eq!(step.written_bytes,0);assert_eq!(source.0.get(),0);assert_eq!(control.actual_work(),0);Ok::<_,ValueError>(())
  }).unwrap();
 }

 #[test]
 fn scalar_record_cancellation_retains_prefix_and_resumes(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[F::new(0,"name",H::Text)];
  let spec=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};let source=Source("é😀");
  let enabled=std::cell::Cell::new(true);let mut callback=|_|enabled.get();let mut control=NativeEncodeControl::new(0,&mut callback);
  let mut cursor=BorrowedScalarRecordCursor::default();let mut loan=Loan(10000);let mut result=Vec::new();let mut byte=[0u8;1];
  control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
   for _ in 0..10000{let turn=cursor.advance(&source,spec,&mut byte,control).map_err(|failure|failure.reason)?;result.extend_from_slice(&byte[..turn.written_bytes]);if result.len()>=6{break}}assert_eq!(result.len(),6);
   let before=control.actual_work();let emitted=cursor.emitted_bytes();enabled.set(false);
   let failure=cursor.advance(&source,spec,&mut byte,control).unwrap_err();
   assert_eq!(failure.reason.kind,ValueRefusalKind::Canceled);assert_eq!(failure.written_bytes,0);assert_eq!(cursor.emitted_bytes(),emitted);assert_eq!(control.actual_work(),before);
   enabled.set(true);
   for _ in 0..10000{let turn=cursor.advance(&source,spec,&mut byte,control).map_err(|failure|failure.reason)?;result.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
   assert!(cursor.is_complete());assert_eq!(result,"name=\"é😀\"\n".as_bytes());for _ in 0..10000{if cursor.close(control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
  }).unwrap();
 }

 struct RefusingSource;
 impl FieldProjectionSource for RefusingSource{
  fn projection_view(&self,_:&[usize])->Result<V<'_>,ValueError>{Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original source refusal"))}
 }
 struct RefusingSettlement;
 impl NativeEncodeWorkRecipient for RefusingSettlement{
  fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(true)}
  fn committed(&mut self,_:u64)->Result<(),ValueError>{Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original work settlement refusal"))}
 }
 #[test]
 fn scalar_record_preserves_source_and_settlement_refusals(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[F::new(0,"name",H::Text)];
  let spec=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};let mut cursor=BorrowedScalarRecordCursor::default();
  let mut callback=|_|true;let mut control=NativeEncodeControl::new(0,&mut callback);let mut loan=RefusingSettlement;let mut byte=[0u8;1];
  control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
   let failure=cursor.advance(&RefusingSource,spec,&mut byte,control).unwrap_err();
   assert_eq!(failure.reason.kind,ValueRefusalKind::InvalidValue);assert_eq!(failure.settlement.unwrap().kind,ValueRefusalKind::OwnershipLimit);
   assert_eq!(failure.written_bytes,0);assert_eq!(control.actual_work(),1);Ok::<_,ValueError>(())
  }).unwrap();
 }

 struct ScalarSource;
 impl FieldProjectionSource for ScalarSource{
  fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{[]=>Ok(V::Record(&[0,1,2])),[0]=>Ok(V::Float(1.5)),[1]=>Ok(V::Enum(4)),[2]=>Ok(V::UInt(7)),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unexpected test path"))}}
 }
 #[test]
 fn scalar_record_float_enum_and_count_match_corpus(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[F::new(0,"value",H::Float),F::new(1,"kind",H::Enum(&[("other",3),("ready",4)])),F::new(2,"count",H::Count)];
  let spec=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🔬️scalars.json")).unwrap();
  let source=ScalarSource;let mut cursor=BorrowedScalarRecordCursor::default();let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(10000);let mut output=Vec::new();let mut byte=[0u8;1];
  control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
   for _ in 0..10000{let demand=cursor.next_output_byte_demand();let turn=cursor.advance(&source,spec,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
   assert!(cursor.is_complete());assert_eq!(output,fixture["document"].as_str().unwrap().as_bytes());for _ in 0..10000{if cursor.close(control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
  }).unwrap();
 }

 struct ListSource;
 impl FieldProjectionSource for ListSource{
  fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{[]=>Ok(V::Record(&[0,1])),[0]=>Ok(V::List(2)),[0,0]=>Ok(V::Int(7)),[0,1]=>Ok(V::Int(9)),[1]=>Ok(V::Text("nodes")),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unexpected test path"))}}
 }
 fn list_integer_shape()->H{H::Int}
 #[test]
 fn scalar_list_cursor_matches_owned_and_sqlite(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/📚️list.json")).unwrap();
  let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"items",Shape::List(Box::new(Shape::Int))),FieldSpec::new(1,"label",Shape::Text)]);
  let mut record=RecordValue::default();record.fields.insert(0,FieldValue::List(fixture["items"].as_array().unwrap().iter().map(|value|FieldValue::Int(value.as_i64().unwrap())).collect()));record.fields.insert(1,FieldValue::Text(fixture["label"].as_str().unwrap().to_owned()));
  let actual=crate::print(&record,&spec,JoinMode::Document);assert_eq!(actual,fixture["document"].as_str().unwrap());
  let oracle=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const db=new Database(":memory:");const result=db.query("SELECT printf('label=%s items=[ %d %d ]','nodes',7,9) AS document").get();process.stdout.write(result.document+"\n");db.close();"#]).output().unwrap();
  assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(oracle.stdout,actual.as_bytes());
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[F::new(0,"items",H::List(list_integer_shape)),F::new(1,"label",H::Text)];
  let spec=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};let source=ListSource;let mut cursor=BorrowedScalarRecordCursor::default();
  let enabled=std::cell::Cell::new(true);let mut accept=|_|enabled.get();let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(10000);let mut output=Vec::new();let mut byte=[0u8;1];
  control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
   for _ in 0..10000{let demand=cursor.next_output_byte_demand();let turn=cursor.advance(&source,spec,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if output.len()>=20{break}}
   assert_eq!(output.len(),20);let prefix=cursor.emitted_bytes();let work=control.actual_work();enabled.set(false);
   let failure=cursor.advance(&source,spec,&mut byte,control).unwrap_err();assert_eq!(failure.reason.kind,ValueRefusalKind::Canceled);assert_eq!(failure.written_bytes,0);assert_eq!(cursor.emitted_bytes(),prefix);assert_eq!(control.actual_work(),work);enabled.set(true);
   for _ in 0..10000{let demand=cursor.next_output_byte_demand();let turn=cursor.advance(&source,spec,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
   assert!(cursor.is_complete());assert_eq!(output,actual.as_bytes());for _ in 0..10000{if cursor.close(control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
  }).unwrap();
 }

 struct TextListSource;
 impl FieldProjectionSource for TextListSource{
  fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{[]=>Ok(V::Record(&[0])),[0]=>Ok(V::List(2)),[0,0]=>Ok(V::Text("hello")),[0,1]=>Ok(V::Text("é😀")),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unexpected test path"))}}
 }
 fn list_text_shape()->H{H::Text}
 #[test]
 fn scalar_list_borrows_and_resumes_text_tokens(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[F::new(0,"names",H::List(list_text_shape))];
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🔤️list.json")).unwrap();
  let source=TextListSource;let spec=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};let mut cursor=BorrowedScalarRecordCursor::default();
  let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(10000);let mut output=Vec::new();let mut byte=[0u8;1];
  control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
   for _ in 0..10000{let demand=cursor.next_output_byte_demand();let turn=cursor.advance(&source,spec,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
   assert!(cursor.is_complete());assert_eq!(output,fixture["document"].as_str().unwrap().as_bytes());for _ in 0..10000{if cursor.close(control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
  }).unwrap();
 }

 #[test]
 fn scalar_record_drains_pending_output_before_projection(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[F::new(0,"name",H::Text)];
  let spec=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};let source=CountingSource(std::cell::Cell::new(0));
  let mut cursor=BorrowedScalarRecordCursor::default();let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(10000);let mut byte=[0u8;1];
  control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
   for _ in 0..10000{if cursor.next_output_byte_demand()==1{break}cursor.advance(&source,spec,&mut byte,control).map_err(|failure|failure.reason)?;}
   assert_eq!(cursor.next_output_byte_demand(),1);let before=source.0.get();
   let turn=cursor.advance(&source,spec,&mut byte,control).map_err(|failure|failure.reason)?;
   assert_eq!(turn.written_bytes,1);assert_eq!(source.0.get(),before);assert_eq!(byte[0],b'n');for _ in 0..10000{if cursor.close(control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
  }).unwrap();
 }

 struct CoordinateSource;
 impl FieldProjectionSource for CoordinateSource{
  fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{
   []=>Ok(V::Record(&[0,1])),[0]=>Ok(V::Tuple(2)),[0,0]=>Ok(V::Float(1.5)),[0,1]=>Ok(V::Float(-2.0)),
   [1]=>Ok(V::Tuple(3)),[1,0]=>Ok(V::Float(0.0)),[1,1]=>Ok(V::Float(1.0)),[1,2]=>Ok(V::Float(0.0)),
   _=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unexpected test path"))}}
 }
 #[test]
 fn scalar_coordinates_match_owned_and_sqlite(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[F::new(0,"point",H::Coord(2)),F::new(1,"direction",H::Dir)];
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/↔️coordinates.json")).unwrap();
  let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"point",Shape::Coord(2)),FieldSpec::new(1,"direction",Shape::Dir)]);
  let mut owned=RecordValue::default();owned.fields.insert(0,FieldValue::Tuple(vec![FieldValue::Float(1.5),FieldValue::Float(-2.0)]));owned.fields.insert(1,FieldValue::Tuple(vec![FieldValue::Float(0.0),FieldValue::Float(1.0),FieldValue::Float(0.0)]));
  let expected=crate::print(&owned,&spec,JoinMode::Document);assert_eq!(expected,fixture["document"].as_str().unwrap());
  let oracle=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const db=new Database(":memory:");const row=db.query("SELECT printf('point=@%.1f,%d direction=^%d,%d,%d',1.5,-2,0,1,0) AS document").get();process.stdout.write(row.document+"\n");db.close();"#]).output().unwrap();
  assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(oracle.stdout,expected.as_bytes());
  let source=CoordinateSource;let schema=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};let mut cursor=BorrowedScalarRecordCursor::default();
  let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(10000);let mut output=Vec::new();let mut byte=[0u8;1];
  control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
   for _ in 0..10000{let demand=cursor.next_output_byte_demand();let turn=cursor.advance(&source,schema,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
   assert!(cursor.is_complete());assert_eq!(output,expected.as_bytes());for _ in 0..10000{if cursor.close(control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
  }).unwrap();
 }

 struct TupleSource;
 impl FieldProjectionSource for TupleSource{
  fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{[]=>Ok(V::Record(&[0])),[0]=>Ok(V::Tuple(3)),[0,0]=>Ok(V::UInt(1)),[0,1]=>Ok(V::UInt(2)),[0,2]=>Ok(V::UInt(3)),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unexpected test path"))}}
 }
 fn tuple_unsigned_shape()->H{H::UInt}
 #[test]
 fn scalar_tuple_matches_owned_and_sqlite(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[F::new(0,"rgb",H::Tuple(tuple_unsigned_shape,Some(3)))];
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🔢️tuple.json")).unwrap();
  let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"rgb",Shape::Tuple(Box::new(Shape::UInt),Some(3)))]);
  let mut owned=RecordValue::default();owned.fields.insert(0,FieldValue::Tuple(vec![FieldValue::UInt(1),FieldValue::UInt(2),FieldValue::UInt(3)]));
  let expected=crate::print(&owned,&spec,JoinMode::Document);assert_eq!(expected,fixture["document"].as_str().unwrap());
  let oracle=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const db=new Database(":memory:");const row=db.query("SELECT printf('rgb=%d,%d,%d',1,2,3) AS document").get();process.stdout.write(row.document+"\n");db.close();"#]).output().unwrap();
  assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(oracle.stdout,expected.as_bytes());
  let source=TupleSource;let schema=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};let mut cursor=BorrowedScalarRecordCursor::default();
  let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(10000);let mut output=Vec::new();let mut byte=[0u8;1];
  control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
   for _ in 0..10000{let demand=cursor.next_output_byte_demand();let turn=cursor.advance(&source,schema,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
   assert!(cursor.is_complete());assert_eq!(output,expected.as_bytes());for _ in 0..10000{if cursor.close(control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
  }).unwrap();
 }

 struct BytesSource{bytes:Vec<u8>,paged:bool}
 impl FieldProjectionSource for BytesSource{
  fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{[]=>Ok(V::Record(&[0])),[0] if self.paged=>Ok(V::BytesSource(&self.bytes)),[0]=>Ok(V::Bytes(&self.bytes)),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unexpected test path"))}}
 }
 #[test]
 fn scalar_bytes_match_owned_and_independent_base64(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[F::new(0,"data",H::Bytes64)];
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🔡️bytes.json")).unwrap();
  let bytes:Vec<u8>=fixture["bytes"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect();
  let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"data",Shape::Bytes64)]);
  let mut owned=RecordValue::default();owned.fields.insert(0,FieldValue::Bytes64(crate::RecordBytes::try_from_slice(&bytes).unwrap()));
  let expected=crate::print(&owned,&spec,JoinMode::Document);assert_eq!(expected,fixture["document"].as_str().unwrap());
  let oracle=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const bytes=Uint8Array.from(JSON.parse(process.argv[1]));const db=new Database(":memory:");const row=db.query("SELECT hex(?) AS hex").get(bytes);if(row.hex!=="010203FF")throw Error("byte oracle");process.stdout.write('data="'+Buffer.from(bytes).toString("base64")+'"\n');db.close();"#]).arg(fixture["bytes"].to_string()).output().unwrap();
  assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(oracle.stdout,expected.as_bytes());
  for paged in [false,true]{
   let source=BytesSource{bytes:bytes.clone(),paged};let schema=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};let mut cursor=BorrowedScalarRecordCursor::default();
   let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(10000);let mut output=Vec::new();let mut byte=[0u8;1];
   control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
    for _ in 0..10000{let demand=cursor.next_output_byte_demand();let turn=cursor.advance(&source,schema,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
    assert!(cursor.is_complete());assert_eq!(output,expected.as_bytes());for _ in 0..10000{if cursor.close(control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
   }).unwrap();
  }
 }

 #[test]
 fn scalar_bytes_preserve_empty_and_partial_groups(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[F::new(0,"data",H::Bytes64)];
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🔡️bytes.json")).unwrap();
  let schema=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};
  for case in fixture["cases"].as_array().unwrap(){for paged in [false,true]{
   let source=BytesSource{bytes:case["bytes"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect(),paged};
   let mut cursor=BorrowedScalarRecordCursor::default();let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(10000);let mut output=Vec::new();let mut byte=[0u8;1];
   control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
    for _ in 0..10000{let demand=cursor.next_output_byte_demand();let turn=cursor.advance(&source,schema,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
    assert!(cursor.is_complete());assert_eq!(output,case["document"].as_str().unwrap().as_bytes());for _ in 0..10000{if cursor.close(control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
   }).unwrap();
  }}
 }

 struct IntrinsicSource;
 impl FieldProjectionSource for IntrinsicSource{
  fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{
   []=>Ok(V::Record(&[0,1,2,3])),[0]=>Ok(V::IntrinsicText("hello")),[1]=>Ok(V::IntrinsicNumber(Number::Int(7))),
   [2]=>Ok(V::IntrinsicNumber(Number::Float(1.0))),[3]=>Ok(V::IntrinsicNull),
   _=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unexpected test path"))}}
 }
 #[test]
 fn scalar_intrinsics_match_owned_and_sqlite(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[F::new(0,"text",H::Value),F::new(1,"positive",H::Value),F::new(2,"whole",H::Value),F::new(3,"missing",H::Value)];
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🔮️intrinsics.json")).unwrap();
  let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"text",Shape::Value),FieldSpec::new(1,"positive",Shape::Value),FieldSpec::new(2,"whole",Shape::Value),FieldSpec::new(3,"missing",Shape::Value)]);
  let mut owned=RecordValue::default();owned.fields.insert(0,FieldValue::Value(DslValue::String("hello".into())));owned.fields.insert(1,FieldValue::Value(DslValue::Number(Number::Int(7))));owned.fields.insert(2,FieldValue::Value(DslValue::Number(Number::Float(1.0))));owned.fields.insert(3,FieldValue::Value(DslValue::Null));
  let expected=crate::print(&owned,&spec,JoinMode::Document);assert_eq!(expected,fixture["document"].as_str().unwrap());
  let oracle=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const db=new Database(":memory:");const row=db.query("SELECT printf('text=\"%s\" positive=int64(%d) whole=%.1f missing=null','hello',7,1.0) AS document").get();process.stdout.write(row.document+"\n");db.close();"#]).output().unwrap();
  assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(oracle.stdout,expected.as_bytes());
  let source=IntrinsicSource;let schema=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};let mut cursor=BorrowedScalarRecordCursor::default();
  let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(10000);let mut output=Vec::new();let mut byte=[0u8;1];
  control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
   for _ in 0..10000{let demand=cursor.next_output_byte_demand();let turn=cursor.advance(&source,schema,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
   assert!(cursor.is_complete());assert_eq!(output,expected.as_bytes());for _ in 0..10000{if cursor.close(control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
  }).unwrap();
 }

 struct IntrinsicBytesSource{bytes:Vec<u8>,paged:bool}
 impl FieldProjectionSource for IntrinsicBytesSource{
  fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{[]=>Ok(V::Record(&[0])),[0] if self.paged=>Ok(V::IntrinsicBytesSource(&self.bytes)),[0]=>Ok(V::IntrinsicBytes(&self.bytes)),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unexpected test path"))}}
 }
 #[test]
 fn scalar_intrinsic_bytes_match_owned_and_base64(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[F::new(0,"data",H::Value)];
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🔮️bytes.json")).unwrap();
  let bytes:Vec<u8>=fixture["bytes"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect();
  let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"data",Shape::Value)]);let mut owned=RecordValue::default();owned.fields.insert(0,FieldValue::Value(DslValue::Bytes(bytes.clone())));
  let expected=crate::print(&owned,&spec,JoinMode::Document);assert_eq!(expected,fixture["document"].as_str().unwrap());
  let oracle=std::process::Command::new("bun").args(["-e",r#"const bytes=Uint8Array.from(JSON.parse(process.argv[1]));process.stdout.write('data=bytes64("'+Buffer.from(bytes).toString("base64")+'")\n');"#]).arg(fixture["bytes"].to_string()).output().unwrap();
  assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(oracle.stdout,expected.as_bytes());
  for paged in [false,true]{
   let source=IntrinsicBytesSource{bytes:bytes.clone(),paged};let schema=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};let mut cursor=BorrowedScalarRecordCursor::default();
   let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(10000);let mut output=Vec::new();let mut byte=[0u8;1];
   control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
    for _ in 0..10000{let demand=cursor.next_output_byte_demand();let turn=cursor.advance(&source,schema,&mut byte,control).map_err(|failure|failure.reason)?;assert_eq!(turn.written_bytes,demand);output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
    assert!(cursor.is_complete());assert_eq!(output,expected.as_bytes());for _ in 0..10000{if cursor.close(control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
   }).unwrap();
  }
 }

 #[test]
 fn scalar_bytes_cancel_inside_base64_and_resume(){let mut loan_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();
  use crate::BorrowedFieldSpec as F;
  const FIELDS:&[F]=&[F::new(0,"data",H::Bytes64)];
  let source=BytesSource{bytes:vec![1,2,3,255],paged:true};let schema=R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};
  let enabled=std::cell::Cell::new(true);let mut accept=|_|enabled.get();let mut control=NativeEncodeControl::new(0,&mut accept);let mut loan=Loan(10000);let mut cursor=BorrowedScalarRecordCursor::default();let mut output=Vec::new();let mut byte=[0u8;1];
  control.with_work_recipient(&mut loan_issuer.loan(&mut loan),|control|{
   for _ in 0..10000{let turn=cursor.advance(&source,schema,&mut byte,control).map_err(|failure|failure.reason)?;output.extend_from_slice(&byte[..turn.written_bytes]);if output.len()>=8{break}}
   assert_eq!(output.as_slice(),b"data=\"AQ");let before=cursor.emitted_bytes();let work=control.actual_work();enabled.set(false);
   let failure=cursor.advance(&source,schema,&mut byte,control).unwrap_err();assert_eq!(failure.reason.kind,ValueRefusalKind::Canceled);assert_eq!(failure.written_bytes,0);assert_eq!(cursor.emitted_bytes(),before);assert_eq!(control.actual_work(),work);enabled.set(true);
   for _ in 0..10000{let turn=cursor.advance(&source,schema,&mut byte,control).map_err(|failure|failure.reason)?;output.extend_from_slice(&byte[..turn.written_bytes]);if turn.complete{break}}
   assert!(cursor.is_complete());assert_eq!(output.as_slice(),b"data=\"AQID/w==\"\n");for _ in 0..10000{if cursor.close(control)?{break}}assert!(cursor.terminal_is_empty());Ok::<_,ValueError>(())
  }).unwrap();
 }

 fn owned_node_schema()->RecordSpec{RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"id",Shape::Text),FieldSpec::new(1,"x",Shape::Float)])}
 fn controlled_node_schema<C:crate::NativeSchemaControl>(control:&mut C)->Result<RecordSpec,ValueError>{
  let mut fields=control.allocate_vec(2)?;fields.push(crate::producer::field(0,"id",Shape::Text,control)?);fields.push(crate::producer::field(1,"x",Shape::Float,control)?);crate::producer::record(None,RecordLayout::Inline,fields,control)
 }
 fn decode_node_schema(control:&mut semio_framework_value::native_decoding::NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{controlled_node_schema(control)}
 fn encode_node_schema(control:&mut NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{controlled_node_schema(control)}
 struct NodeListSource{nodes:Vec<(String,f64)>}
 impl FieldProjectionSource for NodeListSource{
  fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{
   []=>Ok(V::Record(&[0])),[0]=>Ok(V::List(self.nodes.len())),[0,index] if *index<self.nodes.len()=>Ok(V::Record(&[0,1])),
   [0,index,0]=>self.nodes.get(*index).map(|node|V::Text(node.0.as_str())).ok_or_else(crate::native_encoding::projection_path_error),
   [0,index,1]=>self.nodes.get(*index).map(|node|V::Float(node.1)).ok_or_else(crate::native_encoding::projection_path_error),
   _=>Err(crate::native_encoding::projection_path_error())}}
 }
 #[test]
 fn compound_record_list_fixture_matches_owned_sqlite_and_source_views(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🫳️borrowed-text/🪆️records.json")).unwrap();
  let nodes:Vec<(String,f64)>=fixture["nodes"].as_array().unwrap().iter().map(|node|(node["id"].as_str().unwrap().to_owned(),node["x"].as_f64().unwrap())).collect();
  let producer=crate::RecordSpecProducer{ordinary:owned_node_schema,encoding:encode_node_schema,decoding:decode_node_schema};
  let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"nodes",Shape::List(Box::new(Shape::Record(producer))))]);
  let mut owned=RecordValue::default();owned.fields.insert(0,FieldValue::List(nodes.iter().map(|(id,x)|{let mut row=RecordValue::default();row.fields.insert(0,FieldValue::Text(id.clone()));row.fields.insert(1,FieldValue::Float(*x));FieldValue::Record(row)}).collect()));
  let expected=crate::print(&owned,&spec,JoinMode::Document);assert_eq!(expected,fixture["document"].as_str().unwrap());
  let oracle=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const db=new Database(":memory:");const row=db.query("SELECT printf('nodes=[ { id=%s x=%.1f } { id=%s x=%d } ]','A',1.5,'B',2) AS document").get();process.stdout.write(row.document+"\n");db.close();"#]).output().unwrap();
  assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(oracle.stdout,expected.as_bytes());
  let source=NodeListSource{nodes};assert!(matches!(source.projection_view(&[0]).unwrap(),V::List(2)));for index in 0..2{assert!(matches!(source.projection_view(&[0,index]).unwrap(),V::Record(ids)if ids==[0,1]));let V::Text(text)=source.projection_view(&[0,index,0]).unwrap()else{panic!("node text")};assert_eq!(text.as_ptr(),source.nodes[index].0.as_ptr());}
 }

}
