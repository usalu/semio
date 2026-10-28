//! 🔢️ Closed scalar coverage laws cannot issue a prepared source or certify a released view.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Status {Pass,Fail,NotApplicable,Incomplete}
#[derive(Clone,Copy)]
struct Evidence {applicable:Option<bool>,implemented:bool,status:Status,failed:bool,evaluated:bool}
#[derive(Default)]
struct Coverage {requested:usize,evaluated:usize,passed:usize,failed:usize,not_applicable:usize,incomplete:usize}
impl Coverage {
 fn add(&mut self,row:Evidence)->Option<()>{
  self.requested=self.requested.checked_add(1)?;
  self.evaluated=self.evaluated.checked_add(usize::from(row.evaluated))?;
  self.passed=self.passed.checked_add(usize::from(row.status==Status::Pass))?;
  self.failed=self.failed.checked_add(usize::from(row.failed))?;
  self.not_applicable=self.not_applicable.checked_add(usize::from(row.status==Status::NotApplicable))?;
  self.incomplete=self.incomplete.checked_add(usize::from(row.applicable.is_none()||row.status==Status::Incomplete||!row.implemented&&row.applicable!=Some(false)))?;
  Some(())
 }
 fn finish(&self,selected:bool)->(Status,bool,bool,bool){
  let complete=selected&&self.requested>0&&self.incomplete==0;
  let failed=self.failed>0;
  let status=if failed{Status::Fail}else if !complete{Status::Incomplete}else if self.passed>0{Status::Pass}else{Status::NotApplicable};
  (status,complete,complete&&!failed&&self.passed>0,failed)
 }
}
#[cfg(test)]
mod tests {
 use super::*;
 fn status(word:&str)->Status{match word{"pass"=>Status::Pass,"fail"=>Status::Fail,"not-applicable"=>Status::NotApplicable,"incomplete"=>Status::Incomplete,_=>panic!("closed status")}}
 #[test]
 fn native_coverage_consumes_the_same_closed_neutral_rows(){
  let cases:serde_json::Value=serde_json::from_str(include_str!("cases.json")).unwrap();
  for case in cases["coverage"].as_array().unwrap(){
   let mut coverage=Coverage::default();
   for row in case["rows"].as_array().unwrap(){let applicable=row["applicable"].as_bool();let implemented=row["implementation"]=="implemented";coverage.add(Evidence{applicable,implemented,status:status(row["status"].as_str().unwrap()),failed:row["hasFailure"].as_bool().unwrap(),evaluated:implemented&&applicable==Some(true)}).unwrap();}
   let expected=&case["expected"];
   assert_eq!(coverage.finish(case["scope"]=="selected-contracts"),(status(expected["status"].as_str().unwrap()),expected["complete"].as_bool().unwrap(),expected["valid"].as_bool().unwrap(),expected["hasFailure"].as_bool().unwrap()),"{}",case["id"]);
  }
 }
 #[test]
 fn native_coverage_never_treats_an_empty_selected_roster_as_valid(){assert_eq!(Coverage::default().finish(true),(Status::Incomplete,false,false,false));}
}
