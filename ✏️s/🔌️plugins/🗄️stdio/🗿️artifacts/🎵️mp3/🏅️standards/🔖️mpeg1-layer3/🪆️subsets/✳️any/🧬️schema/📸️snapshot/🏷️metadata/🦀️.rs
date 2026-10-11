//! 🏷️ Admitted ID3 metadata independently of native text encoding and tag framing.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Id3Content {
    Text { values: Vec<String> },
    UserText { description: String, values: Vec<String> },
    Comment { language: String, description: String, text: String },
    Lyrics { language: String, description: String, text: String },
    Url { url: String },
    UserUrl { description: String, url: String },
    Picture { mime: String, picture_type: u8, description: String, #[dsl(base64)] payload: Vec<u8> },
    Opaque { #[dsl(base64)] bytes: Vec<u8> },
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct Id3Frame { pub id: String, #[dsl(statements)] pub content: Id3Content }
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct Id3v2Tag { pub frames: Vec<Id3Frame> }
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct Id3v1Tag {
    pub title: String, pub artist: String, pub album: String, pub year: String,
    pub comment: String, pub track: Option<u8>, pub genre: Option<u8>,
}
pub fn id3_content_kind(id: &str) -> &'static str {
    match id {
        "TXXX" => "userText", "COMM" => "comment", "USLT" => "lyrics", "WXXX" => "userUrl", "APIC" => "picture",
        "CHAP"|"CTOC"|"AENC"|"ASPI"|"COMR"|"ENCR"|"EQU2"|"EQUA"|"ETCO"|"GEOB"|"GRID"|"IPLS"|"LINK"|"MCDI"|"MLLT"|"OWNE"|"PCNT"|"POPM"|"POSS"|"PRIV"|"RBUF"|"RVAD"|"RVA2"|"RVRB"|"SEEK"|"SIGN"|"SYLT"|"SYTC"|"UFID"|"USER" => "unsupported",
        _ if id.starts_with('T') => "text", _ if id.starts_with('W') => "url", _ => "opaque",
    }
}
pub fn validate_id3_frame(frame: &Id3Frame) -> Result<(), String> {
    if frame.id.len()!=4 || !frame.id.chars().all(|c|c.is_ascii_uppercase()||c.is_ascii_digit()) { return Err("id3.frame.identifier".into()); }
    let clean=|text:&str|!text.contains('\0');
    let texts=|values:&[String]|!values.is_empty()&&values.iter().all(|v|clean(v));
    let language=|value:&str|value.len()==3&&value.chars().all(|c|c.is_ascii_lowercase());
    let (kind,valid)=match &frame.content {
        Id3Content::Text{values}=>("text",texts(values)),
        Id3Content::UserText{description,values}=>("userText",clean(description)&&texts(values)),
        Id3Content::Comment{language:l,description,text}=>("comment",language(l)&&clean(description)&&clean(text)),
        Id3Content::Lyrics{language:l,description,text}=>("lyrics",language(l)&&clean(description)&&clean(text)),
        Id3Content::Url{url}=>("url",clean(url)&&url.chars().all(|c|(c as u32)<=255)),
        Id3Content::UserUrl{description,url}=>("userUrl",clean(description)&&clean(url)&&url.chars().all(|c|(c as u32)<=255)),
        Id3Content::Picture{mime,picture_type,description,..}=>("picture",!mime.is_empty()&&mime.is_ascii()&&clean(mime)&&*picture_type<=20&&clean(description)),
        Id3Content::Opaque{..}=>("opaque",true),
    };
    if !valid||kind!=id3_content_kind(&frame.id){return Err("id3.frame.content".into());}Ok(())
}
pub fn validate_id3v1_tag(tag:&Id3v1Tag)->Result<(),String>{
    for(text,width)in [(&tag.title,30),(&tag.artist,30),(&tag.album,30),(&tag.year,4),(&tag.comment,if tag.track.is_some(){28}else{30})] {
        if text.chars().count()>width||text.ends_with(' ')||text.chars().any(|c|c=='\0'||(c as u32)>255){return Err("id3v1.text".into());}
    }
    if tag.track==Some(0)||tag.genre==Some(255){return Err("id3v1.ordinal".into());}Ok(())
}
