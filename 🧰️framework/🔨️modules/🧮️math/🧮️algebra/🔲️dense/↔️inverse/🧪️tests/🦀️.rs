use super::*;
use semio_framework_value::serde_json;
fn funded()->DenseSolveBudget{DenseSolveBudget{items:1,copy_bytes:32,capacity_bytes:0,depth:1}}
fn relative(actual:f64,expected:f64){assert!(if expected==0.0{actual.abs()<=1e-12}else{(actual-expected).abs()<=1e-10*expected.abs()},"{actual} expected {expected}");}

#[test]
fn retained_inverse_matches_neutral_values_and_exact_original_checkpoints(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  let n=row["order"].as_u64().unwrap() as usize;let mut matrix:Vec<f64>=row["matrix"].as_array().unwrap().iter().map(|v|v.as_f64().unwrap()).collect();let mut inverse=vec![7.0;n*n];let pointers=(matrix.as_ptr(),inverse.as_ptr());let mut cursor=DenseInverseCursor::new(n,matrix.len(),inverse.len());let mut turns=0;
  loop{
   let d=cursor.demand();if d.items>0{let state=cursor;let a=matrix.clone();let b=inverse.clone();for denied in[DenseSolveBudget{items:0,..funded()},DenseSolveBudget{depth:0,..funded()},DenseSolveBudget{copy_bytes:d.copy_bytes.saturating_sub(1),..funded()}]{if denied.copy_bytes==funded().copy_bytes||d.copy_bytes>0{assert!(matches!(cursor.step(&mut matrix,&mut inverse,denied).status,DenseSolveStatus::Refused(_)));assert_eq!(state,cursor);assert_eq!(matrix,a);assert_eq!(inverse,b);}}}
   let turn=cursor.step(&mut matrix,&mut inverse,funded());assert!(turn.receipt.items<=1&&turn.receipt.copy_bytes<=32);assert_eq!(turn.receipt.capacity_bytes,0);assert_eq!(pointers,(matrix.as_ptr(),inverse.as_ptr()));cursor=DenseInverseCursor::from_snapshot(cursor.snapshot()).unwrap();turns+=1;
   match turn.status{DenseSolveStatus::Complete=>{for(value,expected)in inverse.iter().zip(row["inverse"].as_array().unwrap()){relative(*value,expected.as_f64().unwrap());}break;},DenseSolveStatus::Fault(f)=>{assert_eq!(f.name(),row["fault"].as_str().unwrap());break;},DenseSolveStatus::Pending=>{},DenseSolveStatus::Refused(_)=>panic!("funded inverse refused")}
   assert!(turns<10000);
  }
 }
 for bad in[f64::NAN,f64::INFINITY]{let mut a=[bad];let mut b=[17.0];let mut cursor=DenseInverseCursor::new(1,1,1);assert_eq!(cursor.step(&mut a,&mut b,funded()).status,DenseSolveStatus::Fault(DenseSolveFault::NonFiniteInput));assert_eq!(a[0].to_bits(),bad.to_bits());assert_eq!(b,[17.0]);}
 assert!(DenseInverseCursor::from_snapshot([0;14]).is_err());
 let mut forged=DenseInverseCursor::new(2,4,4).snapshot();forged[7]=12;assert!(DenseInverseCursor::from_snapshot(forged).is_err());
 let mut forged=DenseInverseCursor::new(2,4,4).snapshot();forged[7]=6;assert!(DenseInverseCursor::from_snapshot(forged).is_err());
}

#[test]
fn retained_inverse_order49_preserves_accuracy_and_original_workspace(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let row=&corpus["large"];let n=row["order"].as_u64().unwrap() as usize;let diagonal=row["diagonal"].as_f64().unwrap();let adjacent=row["adjacent"].as_f64().unwrap();let original:Vec<f64>=(0..n*n).map(|p|if p/n==p%n{diagonal}else if (p/n).abs_diff(p%n)==1{adjacent}else{0.0}).collect();let mut matrix=original.clone();let mut inverse=vec![0.0;n*n];let pointers=(matrix.as_ptr(),inverse.as_ptr());let mut cursor=DenseInverseCursor::new(n,matrix.len(),inverse.len());let mut turns=0;
 loop{let turn=cursor.step(&mut matrix,&mut inverse,funded());assert_eq!(pointers,(matrix.as_ptr(),inverse.as_ptr()));assert!(turn.receipt.items<=1&&turn.receipt.copy_bytes<=32);assert_eq!(turn.receipt.capacity_bytes,0);cursor=DenseInverseCursor::from_snapshot(cursor.snapshot()).unwrap();turns+=1;match turn.status{DenseSolveStatus::Complete=>break,DenseSolveStatus::Pending=>{},status=>panic!("order49 inverse {status:?}")};assert!(turns<1000000);}
 for i in 0..n{relative(inverse[i*n+i],row["samples"]["diagonal"][i].as_f64().unwrap());relative(inverse[i],row["samples"]["first"][i].as_f64().unwrap());relative(inverse[(n-1)*n+i],row["samples"]["last"][i].as_f64().unwrap());for j in 0..n{let residual=(0..n).map(|k|original[i*n+k]*inverse[k*n+j]).sum::<f64>()-if i==j{1.0}else{0.0};assert!(residual.abs()<=row["residualTolerance"].as_f64().unwrap());}}
}
