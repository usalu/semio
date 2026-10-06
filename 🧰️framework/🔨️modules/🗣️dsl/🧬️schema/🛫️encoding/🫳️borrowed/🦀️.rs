//! 🫳️ Prospective Text bytes use the canonical emitter and immutable ordinal field views.
use super::*;
use crate::{BorrowedRecordSpec as R,BorrowedShape as H,native_encoding::{FieldProjectionSource,FieldProjectionView as V}};

pub(crate) fn measure<T:FieldProjectionSource>(source:&T,spec:&R,maximum:usize,control:&mut NativeEncodeControl<'_>)->Result<usize,ValueError>{
 control.scoped_stage(|control|{control.begin_stage(0)?;control.checkpoint()?;let mut writer=Emitter::new(JoinMode::Document,None,maximum);writer.prospective=true;writer.borrowed_record(source,spec,&mut[0;64],0,control)?;writer.newline(control)?;Ok(writer.bytes)})
}

fn rank(shape:H)->u8{match shape{H::List(_)|H::Map(_)|H::Record(_)|H::Block(_)|H::Value|H::Wire=>1,H::Table(_)=>2,H::Statements(_)=>3,H::Embed(_)|H::EmbedFrom(_)=>4,_=>0}}
fn absent<T:FieldProjectionSource>(source:&T,path:&[usize])->Result<bool,ValueError>{Ok(matches!(source.projection_view(path)?,V::Absent))}
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"borrowed field view disagrees with declared schema")}


fn borrowed_type_name(shape:H)->&'static str{match shape{
 H::Bool=>"BOOL",H::Int=>"INT",H::UInt=>"UINT",H::Float=>"NUM",H::Text=>"TEXT",H::Bytes64=>"BYTES",H::Enum(_)=>"ENUM",
 H::Tuple(_,_)=>"TUPLE",H::List(_)=>"LIST",H::Record(_)=>"REC",H::Block(_)=>"BLOCK",H::Statements(_)=>"STMT",H::Map(_)=>"MAP",
 H::Value=>"VAL",H::Table(_)=>"TABLE",H::Wire=>"WIRE",H::Quantity(_)=>"QTY",H::Angle(_)=>"ANG",H::Ref(_)=>"REF",
 H::Coord(_)=>"CRD",H::Dir=>"DIR",H::Dim(_)=>"DIM",H::Range=>"RNG",H::Count=>"CNT",H::Expr=>"EXPR",H::Embed(_)|H::EmbedFrom(_)=>"EMBED",
}}

impl Emitter{
 fn borrowed_table<T:FieldProjectionSource>(&mut self,source:&T,spec:&R,path:&mut[usize;64],depth:usize,length:usize,columnar:bool,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  if depth+1>=path.len(){return Err(ValueError::new(ValueRefusalKind::DepthLimit,"borrowed table exceeds fixed path frontier"))}
  self.atom("[",control)?;
  if columnar{
   self.glued=true;
   for field in spec.fields{control.checkpoint()?;self.begin_atom(control)?;self.raw(field.key,control)?;self.raw(":",control)?;self.raw(borrowed_type_name(field.shape),control)?;}
   self.glued=true;self.atom("]",control)?;self.open(control)?;
  }
  control.scoped_stage(|control|{
   control.begin_stage(length)?;
   for index in 0..length{
    path[depth]=index;
    let V::Record(ids)=source.projection_view(&path[..depth+1])?else{return Err(invalid())};
    if ids.len()!=spec.fields.len()||ids.iter().zip(spec.fields).any(|(id,field)|*id!=field.id){return Err(invalid())}
    if columnar{
     self.newline(control)?;
     for(field_index,field)in spec.fields.iter().enumerate(){
      control.checkpoint()?;path[depth+1]=field_index;
      if absent(source,&path[..depth+2])?{self.atom("_",control)?;}
      else if matches!(field.shape,H::Record(_)){self.atom("{",control)?;self.glued=true;self.borrowed_shape(source,field.shape,path,depth+2,control)?;self.glued=true;self.atom("}",control)?;}
      else{self.borrowed_shape(source,field.shape,path,depth+2,control)?;}
     }
    }else{
     self.atom("{",control)?;self.glued=true;self.borrowed_record(source,spec,path,depth+1,control)?;self.glued=true;self.atom("}",control)?;
    }
    control.step()?;
   }Ok::<_,ValueError>(())
  })?;
  if columnar{self.close(control)}else{self.atom("]",control)}
 }

 fn borrowed_record<T:FieldProjectionSource>(&mut self,source:&T,spec:&R,path:&mut[usize;64],depth:usize,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  if depth>=path.len(){return Err(ValueError::new(ValueRefusalKind::DepthLimit,"borrowed Text record exceeds path depth"))}
  control.scoped_stage(|control|{control.begin_stage(0)?;control.scoped_depth(64,|control|{
   control.checkpoint()?;let V::Record(ids)=source.projection_view(&path[..depth])?else{return Err(invalid())};
   if ids.len()!=spec.fields.len()||ids.iter().zip(spec.fields).any(|(id,field)|*id!=field.id){return Err(invalid())}
   if spec.layout==RecordLayout::Call{
    let index=spec.fields.iter().position(|field|field.is_call_name).ok_or_else(invalid)?;path[depth]=index;let V::Text(text)=source.projection_view(&path[..depth+1])?else{return Err(invalid())};self.begin_atom(control)?;self.text(text,false,control)?;self.atom("=",control)?;if let Some(keyword)=spec.keyword{self.atom(keyword,control)?;}self.raw("(",control)?;self.inline(control,|writer,control|writer.borrowed_fields(source,spec,path,depth,control))?;self.raw(")",control)
   }else{if let Some(keyword)=spec.keyword{self.atom(keyword,control)?;}self.borrowed_fields(source,spec,path,depth,control)}
  })})
 }
 fn borrowed_fields<T:FieldProjectionSource>(&mut self,source:&T,spec:&R,path:&mut[usize;64],depth:usize,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  let mut last=None;
  for(index,field)in spec.fields.iter().enumerate(){control.step()?;if let Some(position)=field.position.filter(|_|!field.is_call_name){path[depth]=index;if !absent(source,&path[..depth+1])?{last=Some(last.map_or((position,index),|old:(u8,usize)|old.max((position,index))));}}}
  if let Some(last)=last{for position in 0..=last.0{for(index,field)in spec.fields.iter().enumerate(){control.step()?;if field.position==Some(position)&&!field.is_call_name&&(position,index)<=last{path[depth]=index;if absent(source,&path[..depth+1])?{self.atom("_",control)?;}else{self.borrowed_shape(source,field.shape,path,depth+1,control)?;}}}}}
  for order in 0..5{for(index,field)in spec.fields.iter().enumerate(){control.step()?;if field.position.is_some()||field.key.is_empty()||field.is_call_name||rank(field.shape)!=order{continue}path[depth]=index;if absent(source,&path[..depth+1])?{continue}
   match field.shape{
    H::Statements(_)=>self.borrowed_shape(source,field.shape,path,depth+1,control)?,
    H::Block(_)=>{self.newline(control)?;self.atom(field.key,control)?;self.borrowed_shape(source,field.shape,path,depth+1,control)?;},
    H::Table(make)=>{self.newline(control)?;self.atom(field.key,control)?;let V::List(length)=source.projection_view(&path[..depth+1])?else{return Err(invalid())};self.borrowed_table(source,&make(),path,depth+1,length,true,control)?;},
    H::EmbedFrom(key)=>{let mut language="plaintext";if let Some(index)=spec.fields.iter().position(|field|field.key==key){path[depth]=index;if let V::Text(text)=source.projection_view(&path[..depth+1])?{language=text;}}path[depth]=index;let V::Text(text)=source.projection_view(&path[..depth+1])?else{return Err(invalid())};self.newline(control)?;self.field_key(field.key,control)?;self.verbatim(language,text,control)?;},
    _=>{self.field_key(field.key,control)?;self.borrowed_shape(source,field.shape,path,depth+1,control)?;}
   }
  }}Ok(())
 }
 fn borrowed_shape<T:FieldProjectionSource>(&mut self,source:&T,shape:H,path:&mut[usize;64],depth:usize,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  if depth>=path.len(){return Err(ValueError::new(ValueRefusalKind::DepthLimit,"borrowed Text field exceeds path depth"))}
  if matches!(shape,H::Value){return self.borrowed_intrinsic(source,path,depth,control)}
  control.scoped_stage(|control|{control.begin_stage(0)?;control.scoped_depth(64,|control|{control.checkpoint()?;match(source.projection_view(&path[..depth])?,shape){
   (V::Bool(value),H::Bool)=>self.atom(if value{"true"}else{"false"},control),
   (V::Int(value),H::Int)=>{self.begin_atom(control)?;self.number(value,control)},
   (V::UInt(value),H::UInt|H::Count)=>{self.begin_atom(control)?;if matches!(shape,H::Count){self.raw("x",control)?;}self.number(value,control)},
   (V::Float(value),H::Float|H::Quantity(_)|H::Angle(_))=>{self.begin_atom(control)?;self.float(value,control)?;if let H::Quantity(unit)|H::Angle(unit)=shape{self.raw(unit.symbol,control)?;}Ok(())},
   (V::Text(text),H::Text|H::Ref(_))=>{self.begin_atom(control)?;self.text(text,false,control)},
   (V::Text(text),H::Embed(language))=>self.verbatim(language,text,control),
   (V::Bytes(bytes),H::Bytes64)=>{self.begin_atom(control)?;self.octets(bytes,control)},
   (V::Enum(value),H::Enum(variants))=>{let(keyword,_)=variants.iter().find(|(_,ordinal)|*ordinal==value).ok_or_else(invalid)?;self.atom(keyword,control)},
   (V::Record(_),H::Record(make))=>self.borrowed_record(source,&make(),path,depth,control),
   (V::List(length),H::List(make))=>{self.atom("[",control)?;control.scoped_stage(|control|{control.begin_stage(length)?;for index in 0..length{path[depth]=index;control.scoped_stage(|control|{if matches!(make(),H::Record(_)){self.atom("{",control)?;self.borrowed_shape(source,make(),path,depth+1,control)?;self.atom("}",control)}else{self.borrowed_shape(source,make(),path,depth+1,control)}})?;control.step()?;}Ok::<_,ValueError>(())})?;self.atom("]",control)},
   (V::List(length),H::Table(make))=>self.borrowed_table(source,&make(),path,depth,length,false,control),
   (V::Statements(length),H::Statements(variants))=>control.scoped_stage(|control|{
    control.begin_stage(length)?;
    for index in 0..length{
     control.checkpoint()?;let keyword=source.projection_key(&path[..depth],index)?;
     let(_,make)=variants.iter().find(|(name,_)|*name==keyword).ok_or_else(invalid)?;
     self.newline(control)?;path[depth]=index;
     control.scoped_stage(|control|self.borrowed_record(source,&make(),path,depth+1,control))?;
     control.step()?;
    }Ok::<_,ValueError>(())
   }),
   (V::Tuple(length),H::Coord(_)|H::Dir)=>{
    let expected=match shape{H::Coord(dimensions)=>usize::from(dimensions),H::Dir=>3,_=>unreachable!()};
    if length!=expected{return Err(invalid())}
    self.begin_atom(control)?;self.raw(if matches!(shape,H::Dir){"^"}else{"@"},control)?;
    control.scoped_stage(|control|{
     control.begin_stage(length)?;
     for index in 0..length{
      path[depth]=index;if index>0{self.raw(",",control)?;}
      control.scoped_depth(64,|control|{control.checkpoint()?;let V::Float(value)=source.projection_view(&path[..depth+1])?else{return Err(invalid())};self.float(value,control)})?;
      control.step()?;
     }Ok::<_,ValueError>(())
    })
   },
   (V::Tuple(length),H::Tuple(make,expected))=>{if expected.is_some_and(|expected|expected!=length){return Err(invalid())}self.begin_atom(control)?;for index in 0..length{control.step()?;path[depth]=index;if index>0{self.raw(",",control)?;}self.inline(control,|writer,control|writer.borrowed_shape(source,make(),path,depth+1,control))?;}Ok(())},
   (V::Block,H::Block(make))=>{self.open(control)?;path[depth]=0;self.borrowed_shape(source,make(),path,depth+1,control)?;self.close(control)},
   _=>Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"borrowed Text shape has no immutable emitter"))
  }})})
 }
 fn borrowed_intrinsic<T:FieldProjectionSource>(&mut self,source:&T,path:&mut[usize;64],depth:usize,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  if depth>=path.len(){return Err(ValueError::new(ValueRefusalKind::DepthLimit,"borrowed intrinsic Text exceeds path frontier"))}
  control.scoped_depth(64,|control|{control.checkpoint()?;match source.projection_view(&path[..depth])?{
   V::IntrinsicNull=>self.intrinsic(&DslValue::Null,control),V::IntrinsicBool(value)=>self.intrinsic(&DslValue::Bool(value),control),V::IntrinsicNumber(value)=>self.intrinsic(&DslValue::Number(value),control),
   V::IntrinsicText(text)=>{self.begin_atom(control)?;self.quoted(text,control)},
   V::IntrinsicBytes(bytes)=>{self.begin_atom(control)?;self.raw("bytes64(",control)?;self.octets(bytes,control)?;self.raw(")",control)},
   V::IntrinsicArray(length)=>{self.atom("[",control)?;control.scoped_stage(|control|{control.begin_stage(length)?;for index in 0..length{path[depth]=index;self.borrowed_intrinsic(source,path,depth+1,control)?;control.step()?;}Ok::<_,ValueError>(())})?;self.atom("]",control)},
   V::IntrinsicObject(length)=>{self.open(control)?;control.scoped_stage(|control|{control.begin_stage(length)?;for index in 0..length{let key=source.projection_key(&path[..depth],index)?;self.key(key,control)?;path[depth]=index;self.borrowed_intrinsic(source,path,depth+1,control)?;control.step()?;}Ok::<_,ValueError>(())})?;self.close(control)},
   _=>Err(invalid())
  }})
 }
}
