//! 🧾️ Incremental JSON scalar traversal over original body validation findings.

use super::BodyValidationJob;
use semio_framework_value::ValueError;

#[derive(Clone,Copy,Default)]
enum Token {
    #[default]
    Begin,Ok,CountKey,Count,Issues,Separator,EntityKey,Entity,CodeKey,Code,MessageKey,Message,EndIssue,End,Done,
}
semio_framework_value::artifact_retire_leaf!(Token);

#[derive(Default)]
pub struct ValidationReportCursor {token:Token,index:usize,offset:usize}
semio_framework_value::artifact_retire_struct!(ValidationReportCursor {token,index,offset});

pub struct ValidationReportScalar {bytes:[u8;6],source_width:usize,width:usize}

impl ValidationReportScalar {
    pub fn bytes(&self)->&[u8]{&self.bytes[..self.width]}

    pub fn width(&self)->usize{self.width}
}

fn encode(value:char,quoted:bool)->ValidationReportScalar{
    let mut bytes=[0;6];
    let width=match value {
        '"'|'\\' if quoted=>{bytes[0]=b'\\';bytes[1]=value as u8;2},
        value if quoted&&(value as u32)<32=>{
            let value=value as usize;
            let hex=b"0123456789abcdef";
            bytes=[b'\\',b'u',b'0',b'0',hex[value/16],hex[value%16]];
            6
        }
        value=>value.encode_utf8(&mut bytes[..4]).len(),
    };
    ValidationReportScalar{bytes,width,source_width:value.len_utf8()}
}

impl ValidationReportCursor {
    pub fn next(&mut self,count:usize)->bool {
        self.offset=0;
        self.token=match self.token{
            Token::Begin=>Token::Ok,
            Token::Ok=>Token::CountKey,
            Token::CountKey=>Token::Count,
            Token::Count=>Token::Issues,
            Token::Issues=>if count==0{Token::End}else{Token::EntityKey},
            Token::Separator=>Token::EntityKey,
            Token::EntityKey=>Token::Entity,
            Token::Entity=>Token::CodeKey,
            Token::CodeKey=>Token::Code,
            Token::Code=>Token::MessageKey,
            Token::MessageKey=>Token::Message,
            Token::Message=>Token::EndIssue,
            Token::EndIssue=>{
                self.index+=1;
                if self.index==count{Token::End}else{Token::Separator}
            }
            Token::End|Token::Done=>Token::Done,
        };
        matches!(self.token,Token::Done)
    }

    pub fn consume(&mut self,value:&ValidationReportScalar){
        self.offset+=value.source_width;
    }

    pub fn scalar(&self,validation:&BodyValidationJob)->Result<Option<ValidationReportScalar>,ValueError>{
        use semio_framework_3d_brep_representation::topology::history::PersistentLabel;
        let mut digits=[0;20];
        let issue=||validation.issue(self.index).ok_or_else(||ValueError::invariant("selected validation finding disappeared"));
        let (text,quoted)=match self.token{
            Token::Begin=>(r#"{"ok":"#,false),
            Token::Ok=>(if validation.issue_count()==0{"true"}else{"false"},false),
            Token::CountKey=>(r#","issueCount":"#,false),
            Token::Count=>(PersistentLabel(validation.issue_count() as u64).decimal(&mut digits),false),
            Token::Issues=>(r#","issues":["#,false),
            Token::Separator=>(",",false),
            Token::EntityKey=>(r#"{"entity":""#,false),
            Token::Entity=>(issue()?.entity.as_str(),true),
            Token::CodeKey=>(r#"","code":""#,false),
            Token::Code=>(issue()?.code,true),
            Token::MessageKey=>(r#"","message":""#,false),
            Token::Message=>(issue()?.message.as_str(),true),
            Token::EndIssue=>(r#""}"#,false),
            Token::End=>("]}",false),
            Token::Done=>return Ok(None),
        };
        Ok(text.get(self.offset..).and_then(|suffix|suffix.chars().next()).map(|value|encode(value,quoted)))
    }

    pub fn depth(&self,validation:&BodyValidationJob)->Result<usize,ValueError>{
        if matches!(self.token,Token::Entity|Token::Code|Token::Message){validation.issue_depth(self.index)}
        else{Ok(1)}
    }
}
