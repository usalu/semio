//! 🤲️ Original PDF 1.4 operation recipes stream their own fields into admitted byte backing.
use super::super::{PdfMutation,insert_page,remove_page,move_page,resize_page,replace_page_text,set_snapshot,patch_snapshot};
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
    /// ✍️ Emits the original seven registered recipes with the caller's full frame and ownership policy.
    pub fn encode_op_into(&self,options:&PackEncodeOptions,output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),ProtocolError>{
        protocol::io::binary::operation_bytes::with_operation_encode_policy(options,control,|control|{
            control.checkpoint().map_err(PackRefusal::from)?;
            let mut limited=OperationByteLimitedOutput::new(output,options.limits.max_file_len);
            let output:&mut dyn OperationByteOutput=&mut limited;
            let tag=match self{Self::InsertPage(_)=>insert_page::binary::TAG,Self::RemovePage(_)=>remove_page::binary::TAG,Self::MovePage(_)=>move_page::binary::TAG,Self::ResizePage(_)=>resize_page::binary::TAG,Self::ReplacePageText(_)=>replace_page_text::binary::TAG,Self::SetSnapshot(_)=>set_snapshot::binary::TAG,Self::PatchSnapshot(_)=>patch_snapshot::binary::TAG};
            output.write_bytes(&[store::pack_rt::OP_BINARY_FORMAT,tag],control)?;
            match self{
                Self::InsertPage(payload)=>{index(payload.index,output,control)?;if !payload.page.width.is_finite()||!payload.page.height.is_finite(){return Err(ProtocolError::Malformed{what:"PDF 1.4 mutation",offset:0,detail:"Non-finite geometry".into()});}number(payload.page.width,output,control)?;number(payload.page.height,output,control)?;text(&payload.page.text,output,control)?;},
                Self::RemovePage(payload)=>index(payload.index,output,control)?,
                Self::MovePage(payload)=>{index(payload.from,output,control)?;index(payload.to,output,control)?;},
                Self::ResizePage(payload)=>{index(payload.index,output,control)?;number(payload.width,output,control)?;number(payload.height,output,control)?;},
                Self::ReplacePageText(payload)=>{index(payload.index,output,control)?;text(&payload.text,output,control)?;},
                Self::SetSnapshot(payload)=>{
                    control.scoped_depth(usize::from(options.limits.max_depth),|control|Ok::<_,ValueError>((||{
                        let pages=&payload.snapshot.pages;
                        if pages.len()as u64>options.limits.max_items{return Err(ProtocolError::from(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"PDF replacement page extent"}));}
                        text(&payload.snapshot.schema,output,control)?;index(pages.len(),output,control)?;
                        control.scoped_stage(|control|{control.begin_stage(pages.len()).map_err(PackRefusal::from)?;for page in pages{control.checkpoint().map_err(PackRefusal::from)?;output.write_bytes(&page.width.to_bits().to_le_bytes(),control)?;output.write_bytes(&page.height.to_bits().to_le_bytes(),control)?;text(&page.text,output,control)?;control.step().map_err(PackRefusal::from)?;}Ok::<_,ProtocolError>(())})
                    })())).map_err(PackRefusal::from)??;
                },
                Self::PatchSnapshot(payload)=>payload.patch.encode_op_into(options,output,control)?,
            }
            Ok(())
        })
    }
}
