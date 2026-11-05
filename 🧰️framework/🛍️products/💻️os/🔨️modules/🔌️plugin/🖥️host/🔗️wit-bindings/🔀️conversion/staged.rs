//! 📨️ A checked actor donor transfers its original backing to WIT without cloning command bytes.
use super::{Event,TurnFault,wit_events};
pub(super) fn staged_form_event_to_wit(event:&mut Event,instance_id:u32)->Result<wit_events::Event,TurnFault>{
    let Event::StagedFormRequested{instance,surface,body_key,request}=event else{return Err(TurnFault::Trapped("staging.original-donor".into()));};
    let maximum=semio_framework::action_staging::StagedFormLimits::FRAMEWORK.wire_bytes().map_err(|error|TurnFault::Trapped(error.to_string()))?;
    if *instance!=instance_id||!surface.split_once(':').is_some_and(|(owner,name)|owner.parse::<u32>()==Ok(*instance)&&!name.is_empty())||body_key.is_empty()||body_key.len()>semio_framework::MAX_SURFACE_BODY_KEY_BYTES||request.is_empty()||request.len()>maximum{return Err(TurnFault::Trapped("staging.original-donor".into()));}
    Ok(wit_events::Event::StagedFormRequested(wit_events::StagedFormRequestedEvent{instance:*instance,surface:std::mem::take(surface),body_key:std::mem::take(body_key),request:std::mem::take(request)}))
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn original_staged_event_transfers_the_exact_backing_once_and_refuses_foreign_actors(){
        let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎠️kernel/🧪️tests/📝️staging/🔣️.json")).unwrap();
        let mut event:Event=serde_json::from_value(law["event"].clone()).unwrap();
        let Event::StagedFormRequested{request,surface,body_key,..}=&event else{unreachable!()};
        let pointers=(request.as_ptr(),surface.as_ptr(),body_key.as_ptr());
        assert!(staged_form_event_to_wit(&mut event,8).is_err());
        assert_eq!(serde_json::to_value(&event).unwrap(),law["event"]);
        let wit_events::Event::StagedFormRequested(taken)=staged_form_event_to_wit(&mut event,7).unwrap()else{unreachable!()};
        assert_eq!((taken.request.as_ptr(),taken.surface.as_ptr(),taken.body_key.as_ptr()),pointers);
        assert_eq!(taken.request,vec![0,1,127,255]);
        assert_eq!(taken.surface,"7:form");
        assert!(staged_form_event_to_wit(&mut event,7).is_err());
        let Event::StagedFormRequested{request,surface,body_key,..}=event else{unreachable!()};
        assert!(request.is_empty()&&surface.is_empty()&&body_key.is_empty());
    }
}
