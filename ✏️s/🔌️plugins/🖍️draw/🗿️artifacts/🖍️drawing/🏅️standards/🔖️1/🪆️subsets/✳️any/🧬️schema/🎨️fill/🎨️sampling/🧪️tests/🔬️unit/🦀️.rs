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
    eprintln!("[DEBUG] prepared fill sampler matched {count} neutral samples");
}

#[test]
fn preparation_owns_stops_and_rejects_invalid_numbers() {
    let mut fill=FillStyle::LinearGradient {x1:0.0,y1:0.0,x2:100.0,y2:0.0,stops:vec![GradientStop {offset:0.0,color:[1.0,0.0,0.0,1.0]},GradientStop {offset:1.0,color:[0.0,0.0,1.0,1.0]}]};
    let prepared=PreparedFill::new(&fill).unwrap();
    if let FillStyle::LinearGradient {stops,..}=&mut fill {stops[0].color[0]=0.0;}
    assert_eq!(prepared.sample([0.0,0.0]).unwrap(),[1.0,0.0,0.0,1.0]);
    for value in [f64::NAN,f64::INFINITY,f64::NEG_INFINITY] {
        assert!(prepared.sample([value,0.0]).is_err());
        assert!(PreparedFill::new(&FillStyle::Solid {color:[0.0,0.0,0.0,value]}).is_err());
        assert!(PreparedFill::new(&FillStyle::RadialGradient {cx:value,cy:0.0,r:1.0,stops:vec![]}).is_err());
    }
}
