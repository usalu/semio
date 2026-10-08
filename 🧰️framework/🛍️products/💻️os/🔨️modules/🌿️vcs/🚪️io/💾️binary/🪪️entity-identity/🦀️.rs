//! 🆔️ Controlled canonical commitments for event and version-graph identities.
#[path="🎛️control/🦀️.rs"]
pub mod control;
use semio_framework_hash::Hasher;
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind};

fn add(left:usize,right:usize)->Result<usize,ValueError>{left.checked_add(right).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"identity preimage length overflow"))}

struct IdentityInput<'a,'b>{hasher:Hasher,control:&'a mut NativeEncodeControl<'b>}
impl IdentityInput<'_,'_>{
    fn span(&mut self,bytes:&[u8])->Result<(),ValueError>{for chunk in bytes.chunks(256){self.control.advance(chunk.len())?;self.hasher.update(chunk);}Ok(())}
    fn ids(&mut self,ids:&[String])->Result<(),ValueError>{for(index,id)in ids.iter().enumerate(){if index!=0{self.span(&[0])?;}self.span(id.as_bytes())?;}Ok(())}
}

fn ids_length(ids:&[String],control:&mut NativeEncodeControl<'_>)->Result<usize,ValueError>{control.scoped_stage(|control|{control.begin_stage(ids.len())?;let mut length=ids.len().saturating_sub(1);for id in ids{control.step()?;length=add(length,id.len())?;}Ok(length)})}

fn identity(prefix:&str,length:usize,control:&mut NativeEncodeControl<'_>,project:impl FnOnce(&mut IdentityInput<'_,'_>)->Result<(),ValueError>)->Result<String,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(0)?;
        let output_length=add(prefix.len(),17)?;
        let output=control.allocate_vec::<u8>(output_length)?;
        control.begin_stage(length)?;
        let mut input=IdentityInput{hasher:Hasher::new(),control};
        project(&mut input)?;
        finish(prefix,input.hasher,output,control)
    })
}

fn finish(prefix:&str,hasher:Hasher,mut output:Vec<u8>,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{
    control.begin_stage(1)?;control.advance(1)?;let digest=hasher.finalize();
    let mut digits=[0u8;16];let alphabet=b"0123456789abcdef";
    for(index,byte)in digest.as_bytes()[..8].iter().enumerate(){digits[index*2]=alphabet[(byte>>4)as usize];digits[index*2+1]=alphabet[(byte&15)as usize];}
    control.begin_stage(add(prefix.len(),17)?)?;
    for chunk in prefix.as_bytes().chunks(256){control.advance(chunk.len())?;output.extend_from_slice(chunk);}
    control.advance(17)?;output.push(b'-');output.extend_from_slice(&digits);
    String::from_utf8(output).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"identity prefix lost valid UTF-8"))
}

/// 🌊️ Admits exact identity output before a native producer streams its original operation octets.
pub struct NativeIdentityPreimage<'a>{prefix:&'a str,hasher:Hasher,output:Vec<u8>}
impl<'a> NativeIdentityPreimage<'a>{
    /// 📦️ Allocates only the exact final identity and commits its canonical prefix separator.
    pub fn new(prefix:&'a str,control:&mut NativeEncodeControl<'_>)->Result<Self,ValueError>{control.begin_stage(0)?;let output=control.allocate_vec(add(prefix.len(),17)?)?;let mut value=Self{prefix,hasher:Hasher::new(),output};value.write(prefix.as_bytes(),control)?;value.write(&[0],control)?;Ok(value)}
    /// 🫳️ Hashes borrowed original octets through bounded caller cancellation boundaries.
    pub fn write(&mut self,bytes:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{for chunk in bytes.chunks(256){control.advance(chunk.len())?;self.hasher.update(chunk);}Ok(())}
    /// 🔑️ Publishes the admitted final identity after its native source completed.
    pub fn finish(self,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{finish(self.prefix,self.hasher,self.output,control)}
}

/// 🔑️ Commits prefix, a NUL separator and borrowed payload under the caller's admission.
pub fn content_addressed_entity_id(prefix:&str,payload:&[u8],control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let length=add(add(prefix.len(),1)?,payload.len())?;identity(prefix,length,control,|input|{input.span(prefix.as_bytes())?;input.span(&[0])?;input.span(payload)})}

/// 🆔️ Commits an edit id, colon and unsigned decimal ordinal without allocating its preimage.
pub fn edit_scoped_id(edit_id:&str,ordinal:u32,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let mut digits=[0u8;10];let mut remaining=ordinal;let mut start=digits.len();loop{start-=1;digits[start]=b'0'+(remaining%10)as u8;remaining/=10;if remaining==0{break;}}let decimal=&digits[start..];let length=add(add(edit_id.len(),1)?,decimal.len())?;identity("scoped",length,control,|input|{input.span(edit_id.as_bytes())?;input.span(b":")?;input.span(decimal)})}

/// ✏️ Commits the replica, sequence and borrowed forward-operation fingerprint.
pub fn mint_edit_id(replica:u64,sequence:i32,forwards_fingerprint:&[u8],control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let length=add(19,forwards_fingerprint.len())?;identity("edit",length,control,|input|{input.span(b"edit\0")?;input.span(&replica.to_le_bytes())?;input.span(&[0])?;input.span(&sequence.to_le_bytes())?;input.span(&[0])?;input.span(forwards_fingerprint)})}

/// 📦️ Commits ordered edit ids and the optional description with their canonical separators.
pub fn mint_change_id(edit_ids:&[String],description:Option<&str>,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let ids=ids_length(edit_ids,control)?;let description=description.unwrap_or("");let length=add(add(8,ids)?,description.len())?;identity("change",length,control,|input|{input.span(b"change\0")?;input.ids(edit_ids)?;input.span(&[0])?;input.span(description.as_bytes())})}

/// 🌿️ Commits the alternative name and ordered checkpoint ids through borrowed spans.
pub fn mint_alternative_id(name:&str,checkpoint_ids:&[String],control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let ids=ids_length(checkpoint_ids,control)?;let length=add(add(13,name.len())?,ids)?;identity("alternative",length,control,|input|{input.span(b"alternative\0")?;input.span(name.as_bytes())?;input.span(&[0])?;input.ids(checkpoint_ids)})}

/// ⚙️ Commits borrowed operation octets and the replica's three little-endian clock fields.
pub fn mint_mutation_id(mutation_bytes:&[u8],stamp:(u64,u64,u64),control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let length=add(33,mutation_bytes.len())?;identity("mutation",length,control,|input|{input.span(b"mutation\0")?;input.span(mutation_bytes)?;for part in[stamp.0,stamp.1,stamp.2]{input.span(&part.to_le_bytes())?;}Ok(())})}

/// 📑️ Encodes the exact existing length-prefixed operation-list fingerprint with caller admission.
pub fn operation_list_fingerprint(ops:&[Vec<u8>],control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,ValueError>{control.scoped_stage(|control|{control.begin_stage(ops.len())?;let mut length=0;for op in ops{control.step()?;length=add(add(length,8)?,op.len())?;}let mut output=control.allocate_vec(length)?;control.begin_stage(length)?;for op in ops{control.advance(8)?;output.extend_from_slice(&(op.len()as u64).to_le_bytes());for chunk in op.chunks(256){control.advance(chunk.len())?;output.extend_from_slice(chunk);}}Ok(output)})}

/// 🧒️ Commits child placement and the original parent operation fingerprint without concatenating them.
pub fn mint_child_id(parent_id:&str,slot:&str,fingerprint:&[u8],ordinal:u32,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let mut output=NativeIdentityPreimage::new("child",control)?;for part in[parent_id.as_bytes(),&[0],slot.as_bytes(),&[0],fingerprint,&[0],&ordinal.to_le_bytes()]{output.write(part,control)?;}output.finish(control)}

fn compare_identity_text(left:&str,right:&str,control:&mut NativeEncodeControl<'_>)->Result<std::cmp::Ordering,ValueError>{let length=left.len().min(right.len());for offset in(0..length).step_by(256){let end=length.min(offset+256);control.advance(end-offset)?;let order=left.as_bytes()[offset..end].cmp(&right.as_bytes()[offset..end]);if order!=std::cmp::Ordering::Equal{return Ok(order);}}Ok(left.len().cmp(&right.len()))}
fn ordered_fingerprints<'a>(values:&'a[(String,Vec<u8>)],control:&mut NativeEncodeControl<'_>)->Result<Vec<(usize,&'a(String,Vec<u8>))>,ValueError>{
    control.begin_stage(0)?;let mut ordered=control.allocate_vec(values.len())?;for(index,value)in values.iter().enumerate(){control.step()?;ordered.push((index,value));}
    fn greater(left:(usize,&(String,Vec<u8>)),right:(usize,&(String,Vec<u8>)),control:&mut NativeEncodeControl<'_>)->Result<bool,ValueError>{let order=compare_identity_text(&left.1.0,&right.1.0,control)?;Ok(order==std::cmp::Ordering::Greater||(order==std::cmp::Ordering::Equal&&left.0>right.0))}
    fn sift(values:&mut[(usize,&(String,Vec<u8>))],mut root:usize,length:usize,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{loop{let Some(mut child)=root.checked_mul(2).and_then(|value|value.checked_add(1)).filter(|child|*child<length)else{return Ok(())};if child+1<length&&greater(values[child+1],values[child],control)?{child+=1;}if !greater(values[child],values[root],control)?{return Ok(());}values.swap(root,child);root=child;}}
    let length=ordered.len();for root in(0..length/2).rev(){sift(&mut ordered,root,length,control)?;}for end in(1..length).rev(){ordered.swap(0,end);sift(&mut ordered,0,end,control)?;}Ok(ordered)
}

/// 🧩️ Commits the stable child-id order and each original operation fingerprint under caller control.
pub fn mint_invocation_id(parent_id:&str,fingerprint:&[u8],children:&[(String,Vec<u8>)],control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let ordered=ordered_fingerprints(children,control)?;let mut output=NativeIdentityPreimage::new("invocation",control)?;output.write(parent_id.as_bytes(),control)?;output.write(&[0],control)?;output.write(fingerprint,control)?;for(_, (id,fingerprint))in ordered{output.write(&[0],control)?;output.write(id.as_bytes(),control)?;output.write(&[0],control)?;output.write(fingerprint,control)?;}output.finish(control)}

/// 🪐️ Commits the Space checkpoint message and its canonically encoded pin fingerprint.
pub fn mint_space_checkpoint_id(message:&str,pins:&[u8],control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let mut output=NativeIdentityPreimage::new("space-checkpoint",control)?;output.write(message.as_bytes(),control)?;output.write(&[0],control)?;output.write(pins,control)?;output.finish(control)}

/// 🌿️ Commits the Space alternative name and its ordered logical checkpoint ids.
pub fn mint_space_alternative_id(name:&str,ids:&[String],control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let mut output=NativeIdentityPreimage::new("space-alternative",control)?;output.write(name.as_bytes(),control)?;output.write(&[0],control)?;for(index,id)in ids.iter().enumerate(){if index!=0{output.write(&[0],control)?;}output.write(id.as_bytes(),control)?;}output.finish(control)}
