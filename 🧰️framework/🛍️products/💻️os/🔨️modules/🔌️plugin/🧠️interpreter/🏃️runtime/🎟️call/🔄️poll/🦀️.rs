//! 📨️ Original poll framing borrows the existing event and page sources before controlled serialization.
use crate::{Budget,Event,OwnedPollInput,OwnedRuntime};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind};

impl OwnedRuntime {
    /// 📥️ Uses the same poll envelope as the guest; its single ordinary event stays borrowed on the stack.
    pub fn encode_original_poll_input_into(events:&[Event],budget:Budget,control:&mut NativeEncodeControl<'_>,output:&mut Option<Vec<u8>>)->Result<(),ValueError>{
        let mut ordinary=None;
        let mut command_page=None;
        let mut cold_pair_page=None;
        for event in events {
            match event {
                Event::CommandIngressPage{cursor,bytes} if command_page.is_none()&&bytes.len()<=semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES&&(!bytes.is_empty()||(cursor.kind==28&&cursor.item_count==0))=>command_page=Some((cursor,bytes)),
                Event::CommandIngressPage{..}=>return Err(invalid("original poll has duplicate or invalid command pages")),
                Event::ColdDocumentPairPage(page) if cold_pair_page.is_none()=>{semio_framework_plugin_guest_contract::validate_cold_page(page).map_err(|_|invalid("original poll has an invalid cold pair page"))?;cold_pair_page=Some(page);},
                Event::ColdDocumentPairPage(_)=>return Err(invalid("original poll has duplicate cold pair pages")),
                event if ordinary.is_none()=>ordinary=Some(event),
                _=>return Err(invalid("original poll admits at most one ordinary event")),
            }
        }
        let events=ordinary.as_ref().map_or(&[][..],std::slice::from_ref);
        crate::operation::input::encode_json_input_into(&OwnedPollInput{events,command_page,cold_pair_page,budget},control,output)
    }
}

fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}

#[cfg(test)]
mod tests{
use super::*;

#[test]
fn original_poll_uses_the_same_borrowed_json_envelope_as_the_independent_serializer(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let budget:Budget=serde_json::from_value(fixture["budget"].clone()).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let events:Vec<Event>=serde_json::from_value(row["events"].clone()).unwrap();
        let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;
        let mut control=NativeEncodeControl::new(8192,&mut observer);
        let mut output=None;let result=OwnedRuntime::encode_original_poll_input_into(&events,budget,&mut control,&mut output);
        assert_eq!(result.is_ok(),row["valid"].as_bool().unwrap());
        if result.is_ok(){let bytes=output.take().unwrap();
            let actual:serde_json::Value=serde_json::from_slice(&bytes).unwrap();
            assert_eq!(actual,serde_json::json!({"events":row["events"],"command_page":null,"cold_pair_page":null,"budget":fixture["budget"]}));
            assert_eq!(control.owned_bytes(),bytes.capacity());
        }else{assert_eq!(control.owned_bytes(),0);}
    }
}

}
