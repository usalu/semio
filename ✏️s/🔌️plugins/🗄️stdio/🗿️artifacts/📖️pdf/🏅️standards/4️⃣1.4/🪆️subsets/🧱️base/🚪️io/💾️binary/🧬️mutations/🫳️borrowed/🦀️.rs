//! 🤲️ Original PDF 1.4 operation recipes stream their own fields into admitted byte backing.
use crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation;
use crate::standards::v1_4::subsets::base::io::binary::mutations::{insert_page,remove_page,move_page,resize_page,replace_page_text};
use protocol::{ProtocolError,PackRefusal,codec::PackEncodeOptions,operation_bytes::{OperationByteOutput,OperationByteLimitedOutput}};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind};

fn index(value:usize,output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),ProtocolError>{
    let word=u64::try_from(value).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"PDF operation index"})?;
    output.write_bytes(&word.to_le_bytes(),control)?;Ok(())
}
fn text(value:&str,output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),ProtocolError>{index(value.len(),output,control)?;output.write_bytes(value.as_bytes(),control)?;Ok(())}
fn number(value:f64,output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),ProtocolError>{
    if !value.is_finite(){return Err(ProtocolError::Malformed{what:"PDF 1.4 mutation",offset:0,detail:"Non-finite geometry".into()});}
    output.write_bytes(&value.to_bits().to_le_bytes(),control)?;Ok(())
}

impl PdfMutation{
    /// ✍️ Emits the original five registered recipes with the caller's full frame and ownership policy.
    pub fn encode_op_into(&self,options:&PackEncodeOptions,output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),ProtocolError>{
        protocol::os_spr::operation_bytes::with_operation_encode_policy(options,control,|control|{
            control.checkpoint().map_err(PackRefusal::from)?;
            let mut limited=OperationByteLimitedOutput::new(output,options.limits.max_file_len);
            let output:&mut dyn OperationByteOutput=&mut limited;
            let tag=match self{Self::InsertPage(_)=>insert_page::TAG,Self::RemovePage(_)=>remove_page::TAG,Self::MovePage(_)=>move_page::TAG,Self::ResizePage(_)=>resize_page::TAG,Self::ReplacePageText(_)=>replace_page_text::TAG};
            output.write_bytes(&[store::pack_rt::OP_BINARY_FORMAT,tag],control)?;
            match self{
                Self::InsertPage(payload)=>{index(payload.index,output,control)?;if !payload.page.width.is_finite()||!payload.page.height.is_finite(){return Err(ProtocolError::Malformed{what:"PDF 1.4 mutation",offset:0,detail:"Non-finite geometry".into()});}number(payload.page.width,output,control)?;number(payload.page.height,output,control)?;text(&payload.page.text,output,control)?;},
                Self::RemovePage(payload)=>index(payload.index,output,control)?,
                Self::MovePage(payload)=>{index(payload.from,output,control)?;index(payload.to,output,control)?;},
                Self::ResizePage(payload)=>{index(payload.index,output,control)?;number(payload.width,output,control)?;number(payload.height,output,control)?;},
                Self::ReplacePageText(payload)=>{index(payload.index,output,control)?;text(&payload.text,output,control)?;},
            }
            Ok(())
        })
    }
}
