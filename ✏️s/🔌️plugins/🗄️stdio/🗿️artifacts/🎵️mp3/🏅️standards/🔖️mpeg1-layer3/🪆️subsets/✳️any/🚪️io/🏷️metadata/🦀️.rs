//! 🏷️ Native ID3 framing and text encoding admission for semantic metadata.
use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3Content,Id3Frame,Id3v1Tag,Id3v2Tag,validate_id3_frame,validate_id3v1_tag};

fn latin(bytes:&[u8])->String{bytes.iter().map(|&b|char::from(b)).collect()}
fn latin_bytes(text:&str)->Result<Vec<u8>,String>{text.chars().map(|c|u8::try_from(c as u32).map_err(|_|"id3.latin1".into())).collect()}
fn terminated(bytes:&[u8],encoding:u8)->Result<(&[u8],&[u8]),String>{
    let width=if encoding==1||encoding==2{2}else{1};
    let position=(0..bytes.len()).step_by(width).find(|&i|i+width<=bytes.len()&&bytes[i..i+width].iter().all(|&b|b==0)).ok_or("id3.text.terminator")?;
    Ok((&bytes[..position],&bytes[position+width..]))
}
fn decode_text(bytes:&[u8],encoding:u8,order:&mut Option<bool>)->Result<String,String>{
    match encoding{
        0=>Ok(latin(bytes)),3=>String::from_utf8(bytes.to_vec()).map_err(|e|e.to_string()),
        1|2=>{
            if bytes.len()%2!=0{return Err("id3.utf16.width".into());}
            let mut body=bytes;
            let little=if encoding==2{false}else if bytes.starts_with(&[255,254]){body=&bytes[2..];*order=Some(true);true}else if bytes.starts_with(&[254,255]){body=&bytes[2..];*order=Some(false);false}else{order.ok_or("id3.utf16.bom")?};
            String::from_utf16(&body.chunks_exact(2).map(|b|if little{u16::from_le_bytes([b[0],b[1]])}else{u16::from_be_bytes([b[0],b[1]])}).collect::<Vec<_>>()).map_err(|e|e.to_string())
        }
        _=>Err("id3.text.encoding".into()),
    }
}
fn decode_values(bytes:&[u8],encoding:u8,order:&mut Option<bool>)->Result<Vec<String>,String>{
    let width=if encoding==1||encoding==2{2}else{1};let mut rest=bytes;let mut values=Vec::new();
    while let Some(position)=(0..rest.len()).step_by(width).find(|&i|i+width<=rest.len()&&rest[i..i+width].iter().all(|&b|b==0)){
        values.push(decode_text(&rest[..position],encoding,order)?);rest=&rest[position+width..];
    }
    if !rest.is_empty()||values.is_empty(){values.push(decode_text(rest,encoding,order)?);}Ok(values)
}
pub fn decode_id3_frame(id:String,bytes:&[u8])->Result<Id3Frame,String>{
    use Id3Content::*;let mut order=None;
    let content=match crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::id3_content_kind(&id){
        "opaque"=>Opaque{bytes:bytes.to_vec()},"url"=>Url{url:latin(bytes).trim_end_matches('\0').into()},
        "text"=>{let(&encoding,body)=bytes.split_first().ok_or("id3.text.empty")?;Text{values:decode_values(body,encoding,&mut order)?}},
        "userText"|"userUrl"=>{let(&encoding,body)=bytes.split_first().ok_or("id3.text.empty")?;let(description,tail)=terminated(body,encoding)?;let description=decode_text(description,encoding,&mut order)?;if id=="TXXX"{UserText{description,values:decode_values(tail,encoding,&mut order)?}}else{UserUrl{description,url:latin(tail).trim_end_matches('\0').into()}}},
        "comment"|"lyrics"=>{if bytes.len()<4{return Err("id3.language.truncated".into());}let encoding=bytes[0];let language=String::from_utf8(bytes[1..4].to_vec()).map_err(|e|e.to_string())?;let(description,text)=terminated(&bytes[4..],encoding)?;let description=decode_text(description,encoding,&mut order)?;let text=decode_text(text,encoding,&mut order)?.trim_end_matches('\0').into();if id=="COMM"{Comment{language,description,text}}else{Lyrics{language,description,text}}},
        "picture"=>{let(&encoding,body)=bytes.split_first().ok_or("id3.picture.empty")?;let(mime,tail)=terminated(body,0)?;let(&picture_type,tail)=tail.split_first().ok_or("id3.picture.type")?;let(description,payload)=terminated(tail,encoding)?;Picture{mime:String::from_utf8(mime.to_vec()).map_err(|e|e.to_string())?,picture_type,description:decode_text(description,encoding,&mut order)?,payload:payload.to_vec()}},
        _=>return Err("id3.known-frame.unsupported".into()),
    };let frame=Id3Frame{id,content};validate_id3_frame(&frame)?;Ok(frame)
}
fn visit_body(frame:&Id3Frame,mut emit:impl FnMut(&[u8]))->Result<(),String>{
    validate_id3_frame(frame)?;use Id3Content::*;
    let texts=|values:&[String],emit:&mut dyn FnMut(&[u8])|{for(i,text)in values.iter().enumerate(){if i>0{emit(&[0]);}emit(text.as_bytes());}};
    match &frame.content{
        Text{values}=>{emit(&[3]);texts(values,&mut emit);},
        UserText{description,values}=>{emit(&[3]);emit(description.as_bytes());emit(&[0]);texts(values,&mut emit);},
        Comment{language,description,text}|Lyrics{language,description,text}=>{emit(&[3]);emit(language.as_bytes());emit(description.as_bytes());emit(&[0]);emit(text.as_bytes());},
        Url{url}=>{for c in url.chars(){emit(&[c as u8]);}},
        UserUrl{description,url}=>{emit(&[3]);emit(description.as_bytes());emit(&[0]);for c in url.chars(){emit(&[c as u8]);}},
        Picture{mime,picture_type,description,payload}=>{emit(&[3]);emit(mime.as_bytes());emit(&[0,*picture_type]);emit(description.as_bytes());emit(&[0]);emit(payload);},
        Opaque{bytes}=>emit(bytes),
    }Ok(())
}
pub fn id3_frame_body_len(frame:&Id3Frame)->Result<usize,String>{let mut count=0usize;let mut overflow=false;visit_body(frame,|bytes|match count.checked_add(bytes.len()){Some(value)=>count=value,None=>overflow=true})?;if overflow{return Err("id3.body.size".into());}Ok(count)}
pub fn id3_frame_body_slice(frame:&Id3Frame,offset:usize,maximum:usize)->Result<Vec<u8>,String>{let mut out=Vec::new();let mut position=0usize;visit_body(frame,|bytes|{if out.len()<maximum&&position.saturating_add(bytes.len())>offset{let start=offset.saturating_sub(position);let end=bytes.len().min(start.saturating_add(maximum-out.len()));out.extend_from_slice(&bytes[start..end]);}position=position.saturating_add(bytes.len());})?;Ok(out)}
pub fn encode_id3_frame_body(frame:&Id3Frame)->Result<Vec<u8>,String>{let mut out=Vec::new();visit_body(frame,|bytes|out.extend_from_slice(bytes))?;Ok(out)}
pub fn syncsafe(bytes:&[u8])->Result<u32,String>{if bytes.len()!=4||bytes.iter().any(|&b|b>127){return Err("id3.syncsafe".into());}Ok(bytes.iter().fold(0,|n,&b|(n<<7)|u32::from(b)))}
pub fn encode_syncsafe(mut value:u32)->[u8;4]{let mut out=[0;4];for byte in out.iter_mut().rev(){*byte=(value&127)as u8;value>>=7;}out}
pub fn decode_id3v2(bytes:&[u8])->Result<(Id3v2Tag,usize),String>{
    if bytes.len()<10||&bytes[..3]!=b"ID3"{return Err("id3.header".into());}let version=bytes[3];if !matches!(version,3|4)||bytes[4]!=0||bytes[5]!=0{return Err("id3.version-or-flags.unsupported".into());}
    let end=10usize.checked_add(syncsafe(&bytes[6..10])?as usize).ok_or("id3.size")?;if end>bytes.len(){return Err("id3.truncated".into());}let mut frames=Vec::new();let mut pos=10;
    while pos<end{
        if bytes[pos..end].iter().all(|&b|b==0){break;}if end-pos<10{return Err("id3.frame.truncated".into());}
        let id=String::from_utf8(bytes[pos..pos+4].to_vec()).map_err(|e|e.to_string())?;let size=if version==4{syncsafe(&bytes[pos+4..pos+8])?}else{u32::from_be_bytes(bytes[pos+4..pos+8].try_into().map_err(|_|"id3.frame.size")?)}as usize;
        if bytes[pos+8]!=0||bytes[pos+9]!=0{return Err("id3.frame.flags.unsupported".into());}let stop=pos.checked_add(10).and_then(|p|p.checked_add(size)).ok_or("id3.frame.size")?;if stop>end{return Err("id3.frame.truncated".into());}if version==3&&matches!(crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::id3_content_kind(&id),"text"|"userText"|"comment"|"lyrics"|"userUrl"|"picture")&&bytes.get(pos+10).is_some_and(|&b|b>1){return Err("id3v2.3.encoding.unsupported".into());}frames.push(decode_id3_frame(id,&bytes[pos+10..stop])?);pos=stop;
    }Ok((Id3v2Tag{frames},end))
}
pub fn encode_id3v2(tag:&Id3v2Tag)->Result<Vec<u8>,String>{
    let mut body=Vec::new();for frame in &tag.frames{let data=encode_id3_frame_body(frame)?;if data.len()>=1<<28{return Err("id3.frame.size".into());}body.extend_from_slice(frame.id.as_bytes());body.extend_from_slice(&encode_syncsafe(data.len()as u32));body.extend_from_slice(&[0,0]);body.extend_from_slice(&data);}if body.len()>=1<<28{return Err("id3.tag.size".into());}let mut out=b"ID3\x04\x00\x00".to_vec();out.extend_from_slice(&encode_syncsafe(body.len()as u32));out.extend_from_slice(&body);Ok(out)
}
pub fn decode_id3v1(bytes:&[u8])->Result<Id3v1Tag,String>{
    if bytes.len()!=128||&bytes[..3]!=b"TAG"{return Err("id3v1.header".into());}let read=|start,end|latin(&bytes[start..end]).trim_end_matches(['\0',' ']).to_string();let track=if bytes[125]==0&&bytes[126]!=0{Some(bytes[126])}else{None};let tag=Id3v1Tag{title:read(3,33),artist:read(33,63),album:read(63,93),year:read(93,97),comment:read(97,if track.is_some(){125}else{127}),track,genre:if bytes[127]==255{None}else{Some(bytes[127])}};validate_id3v1_tag(&tag)?;Ok(tag)
}
pub fn encode_id3v1(tag:&Id3v1Tag)->Result<[u8;128],String>{
    validate_id3v1_tag(tag)?;let mut out=[0;128];out[..3].copy_from_slice(b"TAG");for(text,start)in [(&tag.title,3),(&tag.artist,33),(&tag.album,63),(&tag.year,93),(&tag.comment,97)]{let bytes=latin_bytes(text)?;out[start..start+bytes.len()].copy_from_slice(&bytes);}if let Some(track)=tag.track{out[126]=track;}out[127]=tag.genre.unwrap_or(255);Ok(out)
}
