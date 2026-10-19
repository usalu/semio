use super::*;
use semio_framework_value::serde_json;

fn generous() -> DenseSolveBudget { DenseSolveBudget { items:1, copy_bytes:64, depth:1, capacity_bytes:0 } }

#[test]
fn retained_dense_matches_portable_oracle_with_exact_resume_and_refusal() {
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){
  let mut matrix:Vec<f64>=case["matrix"].as_array().unwrap().iter().map(|x|x.as_f64().unwrap()).collect();
  let mut rhs:Vec<f64>=case["rhs"].as_array().unwrap().iter().map(|x|x.as_f64().unwrap()).collect();
  let pointers=(matrix.as_ptr(),rhs.as_ptr());let mut cursor=DenseSolveCursor::new(case["order"].as_u64().unwrap() as usize,matrix.len(),rhs.len());let mut turns=0;let expected:[u64;12]=std::array::from_fn(|i|u64::from_str_radix(case["initialSnapshot"][i].as_str().unwrap(),16).unwrap());assert_eq!(cursor.snapshot(),expected);
  loop {
   let before=cursor;let a=matrix.clone();let b=rhs.clone();let demand=cursor.demand();
   if demand.items>0 {
    let mut budgets=vec![DenseSolveBudget {items:0,..generous()},DenseSolveBudget {depth:0,..generous()}];
    if demand.copy_bytes>0 {budgets.push(DenseSolveBudget {copy_bytes:demand.copy_bytes-1,..generous()});}
    for budget in budgets {assert!(matches!(cursor.step(&mut matrix,&mut rhs,budget).status,DenseSolveStatus::Refused(_)));assert_eq!(cursor,before);assert_eq!(matrix,a);assert_eq!(rhs,b);}
   }
   let turn=cursor.step(&mut matrix,&mut rhs,generous());assert!(turn.receipt.items<=1);assert!(turn.receipt.copy_bytes<=64);assert_eq!(turn.receipt.capacity_bytes,0);
   assert_eq!(pointers,(matrix.as_ptr(),rhs.as_ptr()));let restored=DenseSolveCursor::from_snapshot(cursor.snapshot()).unwrap();assert_eq!(restored,cursor);cursor=restored;turns+=1;
   match turn.status {
    DenseSolveStatus::Complete=>{for (value,expected) in rhs.iter().zip(case["solution"].as_array().unwrap()){let expected=expected.as_f64().unwrap();assert!((value-expected).abs()<=1e-10*expected.abs().max(1.0),"{}",case["id"]);}break;},
    DenseSolveStatus::Fault(fault)=>{assert_eq!(fault.name(),case["fault"].as_str().unwrap(),"{}",case["id"]);break;},
    DenseSolveStatus::Pending=>{},DenseSolveStatus::Refused(_)=>panic!("funded action refused")
   }assert!(turns<10000);
  }
  println!("[DEBUG] shared dense original {} completed in {} bounded turns",case["id"],turns);
 }
}

#[test]
fn retained_dense_reports_nonfinite_and_overflow_without_substituting_physics() {
 for bad in [f64::NAN,f64::INFINITY,f64::NEG_INFINITY]{let mut a=[bad];let mut b=[2.0];let mut cursor=DenseSolveCursor::new(1,1,1);assert_eq!(cursor.step(&mut a,&mut b,generous()).status,DenseSolveStatus::Fault(DenseSolveFault::NonFiniteInput));assert_eq!(a[0].to_bits(),bad.to_bits());assert_eq!(b,[2.0]);}
 let mut a=[1e-308];let mut b=[1e308];let mut cursor=DenseSolveCursor::new(1,1,1);
 loop {match cursor.step(&mut a,&mut b,generous()).status {DenseSolveStatus::Pending=>{},status=>{assert_eq!(status,DenseSolveStatus::Fault(DenseSolveFault::NonFiniteArithmetic));break;}}}
 assert_eq!(b,[1e308]);assert!(DenseSolveCursor::from_snapshot([0;12]).is_err());
 println!("[DEBUG] shared dense typed faults preserve original nonfinite input and reject arithmetic overflow");
}
