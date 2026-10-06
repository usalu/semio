// 📖️ Exact child framing reads the original span into an explicitly retained caller visitor.
use semio_framework_os_kernel::os_pack::codec::{ByteSpan,ByteReader,PackDecodeOptions};
use semio_framework_os_kernel::os_pack::PackRefusal;
use semio_framework_value::{NativeDecodeControl,ValueRefusalKind};
use semio_framework_ui_locale::{Locale,Terminology};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ChildGroupText{Owner,Slot,ChildId,Schema}

/// 🫴️ The caller installs this retained owner before parsing and keeps every accepted field on any refusal.
pub trait ChildGroupDecodeVisitor{
    fn begin_groups(&mut self,count:usize,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>;
    fn begin_group(&mut self,index:usize,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>;
    fn text(&mut self,field:ChildGroupText,source:ByteSpan<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>;
    fn begin_operations(&mut self,count:usize,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>;
    fn begin_operation(&mut self,index:usize,count:usize,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>;
    fn operation_byte(&mut self,byte:u8,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>;
    fn end_operation(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>;
    fn begin_labels(&mut self,count:usize,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>;
    fn begin_label(&mut self,index:usize,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>;
    fn label_text(&mut self,terminology:Terminology,locale:Locale,source:ByteSpan<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>;
    fn end_label(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>;
    fn end_group(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>;
    fn end_groups(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>;
}

fn malformed(offset:usize,detail:&'static str)->PackRefusal{PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"retained child group wire",offset:offset as u64,detail}}
fn equal(source:ByteSpan<'_>,text:&str)->bool{source.len()==text.len()&&source.iter().eq(text.bytes())}
fn utf8(source:ByteSpan<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{
    let mut position=0;
    while position<source.len(){
        control.checkpoint()?;
        let first=source[position];let length=match first{0..=127=>1,194..=223=>2,224..=239=>3,240..=244=>4,_=>return Err(malformed(position,"invalid original UTF-8 lead octet"))};
        let mut scalar=[0;4];
        for(index,target)in scalar[..length].iter_mut().enumerate(){*target=*source.get(position+index).ok_or_else(||malformed(position,"incomplete original UTF-8 scalar"))?;}
        if std::str::from_utf8(&scalar[..length]).is_err(){return Err(malformed(position,"invalid original UTF-8 scalar"))}
        position+=length;control.advance(length)?;
    }
    Ok(())
}
struct GroupReader<'a>{reader:ByteReader<'a>,symbols:ByteReader<'a>,symbol_count:usize,options:&'a PackDecodeOptions}
impl<'a> GroupReader<'a>{
    fn depth(&self,level:u16)->Result<(),PackRefusal>{if level>self.options.limits.max_depth{return Err(PackRefusal::ValueRefusal(semio_framework_value::ValueError::new(ValueRefusalKind::DepthLimit,"actual child group shape exceeds caller depth policy")))}Ok(())}
    fn count(&mut self,control:&mut NativeDecodeControl<'_>)->Result<usize,PackRefusal>{control.checkpoint()?;let count=self.reader.read_varint_u64()?;if count>self.options.limits.max_items{return Err(malformed(self.reader.position(),"declared child collection exceeds caller item policy"))}usize::try_from(count).map_err(|_|malformed(self.reader.position(),"child collection count exceeds addressable source"))}
    fn tag(&mut self,expected:u8,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{control.checkpoint()?;let at=self.reader.position();if self.reader.read_u8()?!=expected{return Err(malformed(at,"declared child field has a different literal wire tag"))}control.step()?;Ok(())}
    fn text(&mut self,control:&mut NativeDecodeControl<'_>)->Result<ByteSpan<'a>,PackRefusal>{
        control.checkpoint()?;
        let at=self.reader.position();
        let text=match self.reader.read_u8()?{
            7=>{let length=self.reader.read_varint_u64()?;if length>self.options.limits.max_segment_len{return Err(malformed(at,"child text exceeds caller segment policy"))}let length=usize::try_from(length).map_err(|_|malformed(at,"child text exceeds addressable source"))?;self.reader.read_span(length)?},
            6=>{
                let index=self.reader.read_varint_u64()?;if index>=self.symbol_count as u64{return Err(malformed(at,"child text references an absent original symbol"))}
                let mut symbols=self.symbols.fork();let mut text=None;
                for ordinal in 0..=index{control.checkpoint()?;let length=usize::try_from(symbols.read_varint_u64()?).map_err(|_|malformed(at,"child symbol exceeds addressable source"))?;let source=symbols.read_span(length)?;if ordinal==index{text=Some(source)}control.step()?;}
                text.expect("checked original symbol ordinal")
            },
            _=>return Err(malformed(at,"child text has no declared intrinsic text tag")),
        };
        utf8(text,control)?;Ok(text)
    }
    fn key(&mut self,expected:&str,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{let at=self.reader.position();if !equal(self.text(control)?,expected){return Err(malformed(at,"child intrinsic key differs from declared original field order"))}Ok(())}
    fn fixed_count(&mut self,expected:usize,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{if self.count(control)?!=expected{return Err(malformed(self.reader.position(),"child intrinsic map does not contain its complete declared fields"))}Ok(())}
    fn operation(&mut self,index:usize,visitor:&mut dyn ChildGroupDecodeVisitor,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{
        self.depth(5)?;self.tag(12,control)?;let count=self.count(control)?;if count!=0{self.depth(6)?;}visitor.begin_operation(index,count,control)?;
        for _ in 0..count{self.tag(4,control)?;let byte=u8::try_from(self.reader.read_varint_u64()?).map_err(|_|malformed(self.reader.position(),"original child operation octet exceeds u8"))?;visitor.operation_byte(byte,control)?;control.step()?;}
        visitor.end_operation(control)
    }
    fn label(&mut self,index:usize,visitor:&mut dyn ChildGroupDecodeVisitor,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{
        self.depth(7)?;self.tag(16,control)?;self.fixed_count(Terminology::COUNT,control)?;visitor.begin_label(index,control)?;
        for terminology in Terminology::ALL{
            self.key(terminology.as_str(),control)?;self.tag(16,control)?;self.fixed_count(Locale::COUNT,control)?;
            for locale in Locale::ALL{self.key(locale.as_str(),control)?;let source=self.text(control)?;visitor.label_text(terminology,locale,source,control)?;}
        }
        visitor.end_label(control)
    }
    fn group(&mut self,index:usize,visitor:&mut dyn ChildGroupDecodeVisitor,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{
        self.depth(4)?;self.tag(16,control)?;self.fixed_count(6,control)?;visitor.begin_group(index,control)?;
        for(key,field)in[("owner",ChildGroupText::Owner),("slot",ChildGroupText::Slot),("child_id",ChildGroupText::ChildId)]{self.key(key,control)?;let text=self.text(control)?;visitor.text(field,text,control)?;}
        self.key("ops",control)?;self.tag(12,control)?;let operations=self.count(control)?;visitor.begin_operations(operations,control)?;
        for index in 0..operations{self.operation(index,visitor,control)?;}
        self.key("op_schema",control)?;let schema=self.text(control)?;visitor.text(ChildGroupText::Schema,schema,control)?;
        self.key("labels",control)?;self.tag(12,control)?;let labels=self.count(control)?;visitor.begin_labels(labels,control)?;
        for index in 0..labels{self.label(index,visitor,control)?;}
        visitor.end_group(control)
    }
}

/// 📖️ Reads the exact original source into preinstalled caller ownership with no allocated parse graph.
pub fn visit_groups_span(source:ByteSpan<'_>,options:&PackDecodeOptions,control:&mut NativeDecodeControl<'_>,visitor:&mut dyn ChildGroupDecodeVisitor)->Result<(),PackRefusal>{
    let maximum=usize::try_from(options.limits.max_total_alloc).unwrap_or(usize::MAX);
    control.scoped_maximum(maximum,|control|{
        control.checkpoint()?;
        if source.len()as u64>options.limits.max_file_len{return Err(malformed(0,"complete child group source exceeds caller file policy"))}
        if source.is_empty(){visitor.begin_groups(0,control)?;return visitor.end_groups(control)}
        let mut reader=ByteReader::from_span(source);let count=reader.read_varint_u64()?;
        if count>u64::from(options.limits.max_symbols){return Err(malformed(0,"original child symbols exceed caller policy"))}
        let symbol_count=usize::try_from(count).map_err(|_|malformed(0,"symbol count exceeds addressable source"))?;
        let symbols=reader.fork();
        for _ in 0..symbol_count{control.checkpoint()?;let length=reader.read_varint_u64()?;if length>options.limits.max_segment_len{return Err(malformed(reader.position(),"original symbol exceeds caller segment policy"))}let length=usize::try_from(length).map_err(|_|malformed(reader.position(),"original symbol exceeds addressable source"))?;utf8(reader.read_span(length)?,control)?;}
        let mut parser=GroupReader{reader,symbols,symbol_count,options};parser.fixed_count(1,control)?;
        if parser.reader.read_varint_u64()?!=1{return Err(malformed(parser.reader.position(),"child groups do not use the declared bridge field"))}
        parser.depth(2)?;parser.tag(17,control)?;parser.tag(12,control)?;let groups=parser.count(control)?;visitor.begin_groups(groups,control)?;
        for index in 0..groups{parser.group(index,visitor,control)?;}
        if parser.reader.remaining()!=0{return Err(malformed(parser.reader.position(),"child group source has trailing original octets"))}
        visitor.end_groups(control)
    })
}
