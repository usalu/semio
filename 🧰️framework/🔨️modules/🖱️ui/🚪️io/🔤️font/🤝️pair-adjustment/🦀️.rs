//! 🤝️ Reads bounded OpenType pair positioning directly from the caller's borrowed font.
//! https://learn.microsoft.com/en-us/typography/opentype/spec/gpos

#[path="🧬️schema/🦀️.rs"]
mod schema;
pub use schema::{FontPairAdjustment,FontPairQuery};
use semio_framework_value::{ValueError,ValueRefusalKind};

fn invalid()->ValueError {ValueError::literal(ValueRefusalKind::InvalidValue,"font pair table is malformed")}
fn unsupported()->ValueError {ValueError::literal(ValueRefusalKind::UnsupportedOwner,"font pair table exceeds the bounded reader contract")}

#[derive(Clone,Copy)]
struct Table<'a>(&'a[u8]);
impl<'a> Table<'a> {
 fn u16(self,offset:usize)->Result<u16,ValueError> {let bytes=self.0.get(offset..offset.checked_add(2).ok_or_else(invalid)?).ok_or_else(invalid)?;Ok(u16::from_be_bytes([bytes[0],bytes[1]]))}
 fn u32(self,offset:usize)->Result<u32,ValueError> {let bytes=self.0.get(offset..offset.checked_add(4).ok_or_else(invalid)?).ok_or_else(invalid)?;Ok(u32::from_be_bytes([bytes[0],bytes[1],bytes[2],bytes[3]]))}
 fn sub(self,offset:usize)->Result<Self,ValueError> {self.0.get(offset..).map(Self).ok_or_else(invalid)}
 fn relative(self,offset:usize)->Result<Self,ValueError> {self.sub(usize::from(self.u16(offset)?))}
 fn array(self,offset:usize,count:usize,stride:usize)->Result<(),ValueError> {self.0.get(offset..offset.checked_add(count.checked_mul(stride).ok_or_else(invalid)?).ok_or_else(invalid)?).map(|_|()).ok_or_else(invalid)}
 fn tag(self,tag:u32,maximum:usize)->Result<Option<Self>,ValueError> {
  let count=usize::from(self.u16(0)?);if count>maximum{return Err(unsupported());}self.array(2,count,6)?;
  for index in 0..count {let offset=2+index*6;if self.u32(offset)?==tag {return self.relative(offset+4).map(Some);}}
  Ok(None)
 }
 fn coverage(self,glyph:u16)->Result<Option<usize>,ValueError> {
  let count=usize::from(self.u16(2)?);let format=self.u16(0)?;let stride=match format{1=>2,2=>6,_=>return Err(unsupported())};self.array(4,count,stride)?;
  let(mut low,mut high)=(0,count);
  while low<high {let index=(low+high)/2;let offset=4+index*stride;let first=self.u16(offset)?;let last=if format==1{first}else{self.u16(offset+2)?};if glyph<first{high=index;}else if glyph>last{low=index+1;}else{return Ok(Some(if format==1{index}else{usize::from(self.u16(offset+4)?)+usize::from(glyph-first)}));}}
  Ok(None)
 }
 fn class(self,glyph:u16)->Result<u16,ValueError> {
  match self.u16(0)? {
   1=>{let first=self.u16(2)?;let count=usize::from(self.u16(4)?);self.array(6,count,2)?;let Some(index)=glyph.checked_sub(first).map(usize::from).filter(|index|*index<count)else{return Ok(0)};self.u16(6+index*2)}
   2=>{let count=usize::from(self.u16(2)?);self.array(4,count,6)?;let(mut low,mut high)=(0,count);while low<high{let index=(low+high)/2;let offset=4+index*6;if glyph<self.u16(offset)?{high=index;}else if glyph>self.u16(offset+2)?{low=index+1;}else{return self.u16(offset+4);}}Ok(0)}
   _=>Err(unsupported())
  }
 }
 fn advance(self,offset:usize,format:u16)->Result<i32,ValueError> {
  if format&!15!=0{return Err(unsupported());}self.array(offset,format.count_ones()as usize,2)?;
  if format&4==0 {Ok(0)}else{Ok(i32::from(self.u16(offset+2*(format&3).count_ones()as usize)?as i16))}
 }
 fn pair(self,query:FontPairQuery)->Result<Option<i32>,ValueError> {
  let Some(coverage)=self.relative(2)?.coverage(query.left_glyph)?else{return Ok(None)};
  let first=self.u16(4)?;let second=self.u16(6)?;if (first|second)&!15!=0{return Err(unsupported());}
  let first_size=2*first.count_ones()as usize;let stride=first_size+2*second.count_ones()as usize;
  let (table,offset)=match self.u16(0)? {
   1=>{let count=usize::from(self.u16(8)?);self.array(10,count,2)?;if coverage>=count{return Err(invalid());}let set=self.relative(10+coverage*2)?;let count=usize::from(set.u16(0)?);set.array(2,count,stride+2)?;let(mut low,mut high)=(0,count);let mut found=None;while low<high{let index=(low+high)/2;let offset=2+index*(stride+2);let glyph=set.u16(offset)?;if query.right_glyph<glyph{high=index;}else if query.right_glyph>glyph{low=index+1;}else{found=Some(offset+2);break;}}let Some(offset)=found else{return Ok(None)};(set,offset)}
   2=>{let left=usize::from(self.relative(8)?.class(query.left_glyph)?);let right=usize::from(self.relative(10)?.class(query.right_glyph)?);let rows=usize::from(self.u16(12)?);let columns=usize::from(self.u16(14)?);if left>=rows||right>=columns{return Err(invalid());}self.array(16,rows.checked_mul(columns).ok_or_else(invalid)?,stride)?;(self,16+(left*columns+right)*stride)}
   _=>return Err(unsupported())
  };
  Ok(Some(table.advance(offset,first)?+table.advance(offset+first_size,second)?))
 }
}

fn font_table(font:Table<'_>,tag:u32)->Result<Option<Table<'_>>,ValueError> {
 let count=usize::from(font.u16(4)?);if count>32{return Err(unsupported());}font.array(12,count,16)?;
 for index in 0..count {let record=12+index*16;if font.u32(record)?==tag{let start=usize::try_from(font.u32(record+8)?).map_err(|_|invalid())?;let size=usize::try_from(font.u32(record+12)?).map_err(|_|invalid())?;let end=start.checked_add(size).ok_or_else(invalid)?;return font.0.get(start..end).map(|bytes|Some(Table(bytes))).ok_or_else(invalid);}}
 Ok(None)
}

/// 📖️ Keeps only borrowed tables and at most eight selected lookup identities.
#[derive(Clone,Copy)]
pub struct BorrowedFontPairSource<'a> {gpos:Option<Table<'a>>,classes:Option<Table<'a>>,lookups:[u16;8],count:usize}
impl<'a> BorrowedFontPairSource<'a> {
 /// 🛂️ Selects the caller's explicit script and its default language without heap ownership.
 pub fn read(bytes:&'a[u8],script:[u8;4])->Result<Self,ValueError> {
  let font=Table(bytes);let signature=font.u32(0)?;if signature!=0x00010000&&signature!=u32::from_be_bytes(*b"OTTO"){return Err(invalid());}let mut source=Self{gpos:font_table(font,u32::from_be_bytes(*b"GPOS"))?,classes:None,lookups:[0;8],count:0};
  if let Some(gdef)=font_table(font,u32::from_be_bytes(*b"GDEF"))? {if gdef.u16(4)?!=0 {source.classes=Some(gdef.relative(4)?);}}
  let Some(gpos)=source.gpos else{return Ok(source)};
  if gpos.u16(0)?!=1||gpos.u16(2)?>1{return Err(unsupported());}
  if gpos.u16(2)?==1&&gpos.u32(10)?!=0{return Err(unsupported());}
  let scripts=gpos.relative(4)?;let script=match scripts.tag(u32::from_be_bytes(script),16)?{Some(value)=>Some(value),None=>scripts.tag(u32::from_be_bytes(*b"DFLT"),16)?};
  let Some(script)=script else{return Ok(source)};if script.u16(0)?==0{return Ok(source)};let language=script.relative(0)?;
  let features=gpos.relative(6)?;let feature_count=usize::from(features.u16(0)?);if feature_count>32{return Err(unsupported());}features.array(2,feature_count,6)?;
  let count=usize::from(language.u16(4)?);if count>32{return Err(unsupported());}language.array(6,count,2)?;
  for ordinal in 0..=count {
   let index=usize::from(if ordinal==0{language.u16(2)?}else{language.u16(4+ordinal*2)?});if index==65535{continue;}if index>=feature_count{return Err(invalid());}let record=2+index*6;
   if features.u32(record)?!=u32::from_be_bytes(*b"kern"){continue;}let feature=features.relative(record+4)?;let lookups=usize::from(feature.u16(2)?);if lookups>source.lookups.len(){return Err(unsupported());}feature.array(4,lookups,2)?;
   for index in 0..lookups {let lookup=feature.u16(4+index*2)?;if source.lookups[..source.count].contains(&lookup){continue;}if source.count==source.lookups.len(){return Err(unsupported());}source.lookups[source.count]=lookup;source.count+=1;}
  }
  source.lookups[..source.count].sort_unstable();
  Ok(source)
 }
 /// 📏️ Reads each selected lookup's first matching pair and accumulates design-unit advances.
 pub fn adjustment(&self,query:FontPairQuery)->Result<FontPairAdjustment,ValueError> {
  let Some(gpos)=self.gpos else{return Ok(Default::default())};let list=gpos.relative(8)?;let count=usize::from(list.u16(0)?);list.array(2,count,2)?;let mut advance_units=0;
  for index in &self.lookups[..self.count] {
   let index=usize::from(*index);if index>=count{return Err(invalid());}let lookup=list.relative(2+index*2)?;let kind=lookup.u16(0)?;if kind!=2&&kind!=9{return Err(unsupported());}let flags=lookup.u16(2)?;if flags&!8!=0{return Err(unsupported());}
   if flags&8!=0 {if let Some(classes)=self.classes {if classes.class(query.left_glyph)?==3||classes.class(query.right_glyph)?==3{continue;}}}
   let count=usize::from(lookup.u16(4)?);if count>8{return Err(unsupported());}lookup.array(6,count,2)?;
   for index in 0..count {let mut table=lookup.relative(6+index*2)?;if kind==9 {if table.u16(0)?!=1||table.u16(2)?!=2{return Err(unsupported());}table=table.sub(usize::try_from(table.u32(4)?).map_err(|_|invalid())?)?;}if let Some(adjustment)=table.pair(query)?{advance_units+=adjustment;break;}}
  }
  Ok(FontPairAdjustment{advance_units})
 }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path="🧪️tests/🦀️.rs"]
mod tests;
