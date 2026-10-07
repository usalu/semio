//! 🧪️ Neutral paint samples and immutable preparation boundaries.
use super::*;

#[test]
fn shared_fill_samples() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let mut count=0;
    for case in cases.as_array().unwrap() {
        let fill:FillStyle=serde_json::from_value(case["fill"].clone()).unwrap();
        let prepared=PreparedFill::new(&fill);
        if case["error"]==true {assert!(prepared.is_err(),"{}",case["name"]);continue;}
        let paint=prepared.unwrap();
        for sample in case["samples"].as_array().unwrap() {
            let point=serde_json::from_value(sample["point"].clone()).unwrap();
            let expected:[f64;4]=serde_json::from_value(sample["color"].clone()).unwrap();
            let actual=paint.sample(point).unwrap();
            for index in 0..4 {assert!((actual[index]-expected[index]).abs()<1e-12,"{}: {actual:?} != {expected:?}",case["name"]);}
            count+=1;
        }
    }
}

#[test]
fn preparation_owns_stops_and_rejects_invalid_numbers() {
    let mut fill=FillStyle::LinearGradient {x1:0.0,y1:0.0,x2:100.0,y2:0.0,stops:vec![GradientStop {offset:0.0,color:[1.0,0.0,0.0,1.0]},GradientStop {offset:1.0,color:[0.0,0.0,1.0,1.0]}].into()};
    let prepared=PreparedFill::new(&fill).unwrap();
    if let FillStyle::LinearGradient {stops,..}=&mut fill {stops[0].color[0]=0.0;}
    assert_eq!(prepared.sample([0.0,0.0]).unwrap(),[1.0,0.0,0.0,1.0]);
    for value in [f64::NAN,f64::INFINITY,f64::NEG_INFINITY] {
        assert!(prepared.sample([value,0.0]).is_err());
        assert!(PreparedFill::new(&FillStyle::Solid {color:[0.0,0.0,0.0,value]}).is_err());
        assert!(PreparedFill::new(&FillStyle::RadialGradient {cx:value,cy:0.0,r:1.0,stops:vec![].into()}).is_err());
    }
}

#[test]
fn prepared_paint_retirement_drains_actual_stop_owners_and_composes_the_real_ramp() {
    let sources:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let rows:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap(){for grant in [1,7,4096]{
        let source=sources.as_array().unwrap().iter().find(|source|source["name"]==row["source"]).unwrap();let mut fill:FillStyle=serde_json::from_value(source["fill"].clone()).unwrap();
        if let Some(repeat)=row["repeat"].as_u64(){match &mut fill {FillStyle::LinearGradient{stops,..}|FillStyle::RadialGradient{stops,..}=>*stops=vec![stops[0].clone();repeat as usize].into(),_=>panic!("gradient required")}}
        let paint=PreparedFill::new(&fill).unwrap();let samples:Vec<[f64;4]>=source["samples"].as_array().unwrap().iter().map(|sample|paint.sample(serde_json::from_value(sample["point"].clone()).unwrap()).unwrap()).collect();
        let mut owner=paint.into_retirement();assert!(owner.advance(0).is_err());if usize::BITS>53{assert!(owner.advance(usize::MAX).is_err());}let mut work=0;
        for _ in 0..5000{let p=owner.advance(grant).unwrap();assert!(p.work>work&&p.work-work<=grant as u64);assert_eq!(p.phase,if p.done{"complete"}else{"closing"});work=p.work;if p.done{break;}}
        assert_eq!(work,row["work"]["prepared"]["native"].as_u64().unwrap());assert!(owner.terminal_is_empty());assert!(owner.fill.is_none()&&owner.ramp.is_none());assert_eq!(owner.advance(1).unwrap().work,work);
        for (sample,actual) in source["samples"].as_array().unwrap().iter().zip(samples){let expected:[f64;4]=serde_json::from_value(sample["color"].clone()).unwrap();for c in 0..4{assert!((actual[c]-expected[c]).abs()<1e-12);}}
        if let FillStyle::LinearGradient{stops,..}|FillStyle::RadialGradient{stops,..}=&fill{
            let ramp=GradientRamp::new(stops).unwrap();let pointer=ramp.stops.as_ptr();let mut retired=ramp.into_retirement();assert_eq!(retired.ramp.as_ref().unwrap().stops.as_ptr(),pointer);let mut work=0;
            for _ in 0..5000{let p=retired.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done{break;}}assert_eq!(work,row["work"]["ramp"]["native"].as_u64().unwrap());assert!(retired.terminal_is_empty());assert!(retired.ramp.is_none());
        }
        eprintln!("[DEBUG] Actual native paint retirement {:?}: grant={grant}, composed_work={work}, samples unchanged",row["source"]);
    }}
}
