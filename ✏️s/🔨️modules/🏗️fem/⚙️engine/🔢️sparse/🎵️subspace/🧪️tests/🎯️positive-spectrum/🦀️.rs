//! 🎯️ Original generalized eigenpairs certify positive spectrum and the elastic equation residual.
use super::*;
use semio_s_fem_sparse::Coo;
use semio_s_fem_sparse_ldlt::ldlt_factor;
use semio_s_fem_test_support::OriginalNumericalFixture;
use semio_s_fem_test_support::close_grant;
use semio_s_fem_test_support::job_close_demand;
use semio_s_fem_test_support::observe_original_numerical_allocations;
use semio_s_fem_test_support::TEST_GRANT;
fn matrix_case(row:&serde_json::Value,dense:&str,diagonal:&str)->Coo{if let Some(rows)=row[dense].as_array(){let mut coo=Coo::new(rows.len());for(i,values)in rows.iter().enumerate(){for(j,value)in values.as_array().unwrap().iter().enumerate(){let value=value.as_f64().unwrap();if value!=0.0{coo.add(i,j,value);}}}coo}else{let rows=row[diagonal].as_array().unwrap();let mut coo=Coo::new(rows.len());for(i,value)in rows.iter().enumerate(){let value=value.as_f64().unwrap();if value!=0.0{coo.add(i,i,value);}}coo}}
#[test]
fn mathematical_subspace_certifies_the_same_neutral_positive_spectrum(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🎯️positive-spectrum/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  let k=matrix_case(row,"stiffness","stiffnessDiagonal");let b=matrix_case(row,"right","rightDiagonal");let n=k.n;let requested=row["requested"].as_u64().unwrap()as usize;
  let factor=ldlt_factor(&k.to_csc_sym_upper()).unwrap();let pairs=subspace_iteration(&factor,&b.to_csr(),n,requested,1000);
  let expected=row["values"].as_array().unwrap();assert_eq!(pairs.values.len(),expected.len(),"{}",row["name"]);assert_eq!(pairs.vectors.len(),expected.len());
  for(mode,value)in pairs.values.iter().enumerate(){let expected=expected[mode].as_f64().unwrap();assert!((value-expected).abs()/expected<1e-6,"{} {value} {expected}",row["name"]);let x=&pairs.vectors[mode];let kx=k.to_csr().mul_vec(x);let bx=b.to_csr().mul_vec(x);let residual=(0..n).map(|i|(kx.get(i)-value*bx.get(i)).powi(2)).sum::<f64>().sqrt();let scale=kx.dot(&kx).sqrt()+value.abs()*bx.dot(&bx).sqrt();assert!(residual/scale.max(f64::MIN_POSITIVE)<corpus["residualTolerance"].as_f64().unwrap(),"mathematical pencil residual");}
 }
}
#[test]
fn generalized_subspace_targets_positive_spectrum_and_certifies_original_residual(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🎯️positive-spectrum/🔣️.json")).unwrap();
 for(index,row)in corpus["cases"].as_array().unwrap().iter().enumerate(){
  let k=matrix_case(row,"stiffness","stiffnessDiagonal");let b=matrix_case(row,"right","rightDiagonal");let n=k.n;let requested=row["requested"].as_u64().unwrap()as usize;
  let factor=ldlt_factor(&k.to_csc_sym_upper()).unwrap();let operation=Operation::new(semio_framework_job::allocate_operation_id(),semio_framework_job::RevisionId(7),semio_framework_job::Generation(9),index as u64);
  let second=b.to_csr();let(mut job,physical)=observe_original_numerical_allocations(||SubspaceIterationJob::new(operation,factor,second,n,requested,1000));assert_eq!(physical,(0,0,0),"constructor transfers original owners without allocating");let mut original=OriginalNumericalFixture::new(operation);let mut turns=0;
  loop{let(outcome,physical)=observe_original_numerical_allocations(||original.turn(RetainedCloneGrant{maximum_items:1,..TEST_GRANT},1,u64::MAX,|cx|job.step(cx).map(|outcome|match outcome{Some(JobOutcomeBorrow::Complete{..})=>1,Some(JobOutcomeBorrow::Fault{..}|JobOutcomeBorrow::Cancelled{..})=>2,_=>0})));let receipt=original.receipt();assert!(physical.2<=NUMERICAL_OWNER_PAGE_BYTES,"{} allocated one oversized original backing: {}",row["name"],physical.2);assert_eq!(physical.0,receipt.retained_capacity_bytes,"{} original allocation receipt",row["name"]);assert_eq!(physical.1,receipt.released_bytes,"{} original release receipt",row["name"]);match outcome.expect("original numerical turn"){1=>break,0=>{},_=>panic!("{}: refused generalized eigenpair",row["name"])}turns+=1;assert!(turns<20000000,"retained frontier made no completion");}
  let pairs=job.solution();let expected=row["values"].as_array().unwrap();assert_eq!(pairs.values.len(),expected.len(),"{}",row["name"]);assert_eq!(pairs.vectors.len(),expected.len());
  for(mode,value)in pairs.values.iter().enumerate(){let expected=expected[mode].as_f64().unwrap();assert!((value-expected).abs()/expected<1e-6,"{} {value} {expected}",row["name"]);let x=&pairs.vectors[mode];let kx=k.to_csr().mul_vec(x);let bx=b.to_csr().mul_vec(x);let residual=(0..n).map(|i|(kx.get(i)-value*bx.get(i)).powi(2)).sum::<f64>().sqrt();let scale=kx.dot(&kx).sqrt()+value.abs()*bx.dot(&bx).sqrt();assert!(residual/scale.max(f64::MIN_POSITIVE)<corpus["residualTolerance"].as_f64().unwrap(),"original pencil residual");}
  if expected.is_empty(){assert_eq!(job.spectrum(),SubspaceSpectrum::NoPositive);assert!(job.visual_mode_scalar(0,0).is_none());}
  assert!(job.visual_progress().3);job.begin_close();while !job.terminal_is_empty(){let grant=close_grant(job_close_demand(&job));let(step,physical)=observe_original_numerical_allocations(||original.close_turn(&mut job,grant));let receipt=match step.expect("original numerical close"){InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>progress,outcome=>panic!("unadmitted original close: {outcome:?}")};assert!(physical.2<=NUMERICAL_OWNER_PAGE_BYTES);assert_eq!(physical.0,receipt.retained_capacity_bytes,"original close allocation receipt");assert_eq!(physical.1,receipt.released_bytes,"original close release receipt");}assert!(job.terminal_is_empty());original.finish();
 }
 println!("[DEBUG] retained generalized solver selects hidden positive modes, returns explicit no-positive spectrum, certifies Kx-lambdaBx and exactly funds every original allocation/release at most 4096 bytes");
}
