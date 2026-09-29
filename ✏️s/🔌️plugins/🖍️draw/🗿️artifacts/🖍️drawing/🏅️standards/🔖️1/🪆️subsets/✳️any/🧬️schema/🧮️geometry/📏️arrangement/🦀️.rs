//! 📏️ Document-axis alignment and equal edge spacing, preserving input order.
pub fn arrange(bounds:&[[f64;4]],operation:&str)->Option<Vec<[f64;2]>> {
    if !matches!(operation,"alignLeft"|"alignCenter"|"alignRight"|"alignTop"|"alignMiddle"|"alignBottom"|"distributeHorizontal"|"distributeVertical") {return None;}
    let distributing=operation.starts_with("distribute");
    let axis=if matches!(operation,"alignLeft"|"alignCenter"|"alignRight"|"distributeHorizontal") {0}else{1};
    if bounds.len()<(if distributing {3}else{2}) || bounds.iter().any(|b|!b.iter().all(|v|v.is_finite()) || b[2]<0.0 || b[3]<0.0 || !(b[axis]+b[axis+2]).is_finite()) {return None;}
    let min=bounds.iter().map(|b|b[axis]).fold(f64::INFINITY,f64::min);
    let max=bounds.iter().map(|b|b[axis]+b[axis+2]).fold(f64::NEG_INFINITY,f64::max);
    let mut order=(0..bounds.len()).collect::<Vec<_>>();
    if distributing {order.sort_by(|a,b|bounds[*a][axis].total_cmp(&bounds[*b][axis]));}
    let gap=(max-min-bounds.iter().map(|b|b[axis+2]).sum::<f64>())/(bounds.len()-1) as f64;
    let mut cursor=min;
    let mut result=vec![[0.0;2];bounds.len()];
    for index in order {
        let size=bounds[index][axis+2];
        let destination=if distributing {cursor}else if matches!(operation,"alignCenter"|"alignMiddle") {min/2.0+max/2.0-size/2.0}else if matches!(operation,"alignRight"|"alignBottom") {max-size}else {min};
        result[index][axis]=destination-bounds[index][axis];
        if distributing {cursor+=size+gap;}
    }
    result.iter().flatten().all(|v|v.is_finite()).then_some(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn world_arrangement_matches_shared_cases() {
        let rows:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
        for row in rows.as_array().unwrap() {
            let bounds:Vec<[f64;4]>=serde_json::from_value(row["bounds"].clone()).unwrap();
            let expected:Option<Vec<[f64;2]>>=serde_json::from_value(row["deltas"].clone()).unwrap();
            assert_eq!(super::arrange(&bounds,row["operation"].as_str().unwrap()),expected,"{}",row["name"]);
        }
        assert!(super::arrange(&[[0.0,0.0,f64::INFINITY,1.0],[1.0,0.0,1.0,1.0]],"alignLeft").is_none());
        eprintln!("[TRACE] document-axis arrangement matches shared alignment and spacing fixtures");
    }
}
