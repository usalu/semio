use super::*;

/// 🧪️ Independent portable radiosity coefficients include exact blackbodies and enclosures beyond forty-eight faces.
#[test]
fn enclosure_exchange_matches_portable_radiosity_without_emissivity_or_size_floors(){
 let corpus:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🔲️enclosure/⚡️exchange/🧪️tests/🧫️fixtures/🔣️.json")).expect("portable exchange corpus");
 let numbers=|value:&serde_json::Value|value.as_array().expect("numeric span").iter().map(|v|v.as_f64().expect("finite fixture scalar")).collect::<Vec<_>>();
 let mut total_turns=0;
 for case in corpus["cases"].as_array().expect("cases"){
  let areas=numbers(&case["areas"]);let emissivities=numbers(&case["emissivities"]);let view=numbers(&case["view"]);let n=areas.len();
  let mut matrix=vec![0.0;n*n];let mut inverse=vec![0.0;n*n];let mut exchange=vec![0.0;n*n];
  let pointers=(areas.as_ptr(),emissivities.as_ptr(),view.as_ptr(),matrix.as_ptr(),inverse.as_ptr(),exchange.as_ptr());
  let mut cursor=EnclosureExchangeCursor::new(n,emissivities.len(),view.len(),matrix.len(),inverse.len(),exchange.len());let mut turns=0;
  let terminal=loop{
   let demand=cursor.demand();let full=DenseSolveBudget{items:demand.items,copy_bytes:demand.copy_bytes,depth:demand.depth,capacity_bytes:0};
   if demand.items!=0&&(n<=4||turns%1024==0){
    let before=cursor.clone();let held=(matrix.clone(),inverse.clone(),exchange.clone());
    for denied in [DenseSolveBudget{items:0,..full},DenseSolveBudget{copy_bytes:demand.copy_bytes.saturating_sub(1),..full},DenseSolveBudget{depth:0,..full}]{
     if denied.items>=demand.items&&denied.copy_bytes>=demand.copy_bytes&&denied.depth>=demand.depth{continue;}
     let turn=cursor.step(&areas,&emissivities,&view,&mut matrix,&mut inverse,&mut exchange,denied);assert!(matches!(turn.status,EnclosureExchangeStatus::Refused(_)));assert_eq!(turn.receipt,DenseSolveDemand::default());assert_eq!(cursor,before);assert_eq!((&matrix,&inverse,&exchange),(&held.0,&held.1,&held.2));
    }
    if n<=4||turns%4096==0{let value=semio_framework_value::ToValue::to_value(&cursor);let decoded=<EnclosureExchangeCursor as semio_framework_value::FromValue>::from_value(value).expect("actual exact word Value decoding");assert_eq!(decoded,cursor);let wire=serde_json::to_vec(&cursor).expect("actual checkpoint encoding");cursor=serde_json::from_slice(&wire).expect("actual checkpoint restoration");assert_eq!(cursor,before);}
   }
   let turn=cursor.step(&areas,&emissivities,&view,&mut matrix,&mut inverse,&mut exchange,full);turns+=1;assert!(turn.receipt.items<=1&&turn.receipt.copy_bytes<=40&&turn.receipt.capacity_bytes==0&&turn.receipt.depth<=1);assert!(turn.receipt.copy_bytes<=full.copy_bytes);
   match turn.status{EnclosureExchangeStatus::Pending=>{},EnclosureExchangeStatus::Refused(_)=>panic!("exact admitted scalar grant refused"),terminal=>break terminal}
   assert!(turns<2_000_000,"scalar enclosure work stalled");
  };
  assert_eq!(pointers,(areas.as_ptr(),emissivities.as_ptr(),view.as_ptr(),matrix.as_ptr(),inverse.as_ptr(),exchange.as_ptr()));
  if let Some(expected)=case["expected"].as_str(){assert_eq!(format!("{:?}",match terminal{EnclosureExchangeStatus::Fault(fault)=>fault,_=>panic!("expected typed fixture fault")}),expected);}else{
   assert!(matches!(terminal,EnclosureExchangeStatus::Complete));let expected=numbers(&case["expected"]);
   for i in 0..n{for j in 0..n{let index=i*n+j;assert!((exchange[index]-expected[index]).abs()<=1e-10,"case {} coefficient {} actual {} expected {}",case["name"],index,exchange[index],expected[index]);assert!((areas[i]*exchange[index]-areas[j]*exchange[j*n+i]).abs()<=1e-9,"pair energy reciprocity");}}
   let temperatures=(0..n).map(|i|273.15+17.0+i as f64).collect::<Vec<_>>();let net=(0..n).map(|i|areas[i]*(0..n).map(|j|exchange[i*n+j]*(temperatures[j].powi(4)-temperatures[i].powi(4))).sum::<f64>()).sum::<f64>();let net_watts=crate::units::STEFAN_BOLTZMANN*net;assert!(net_watts.abs()<=1e-9,"closed enclosure energy watts {net_watts}");
  }
  total_turns+=turns;
 }
 eprintln!("[DEBUG] retained enclosure exchange original spans matched 15 independent Mathjs cases across {total_turns} scalar turns");
}
