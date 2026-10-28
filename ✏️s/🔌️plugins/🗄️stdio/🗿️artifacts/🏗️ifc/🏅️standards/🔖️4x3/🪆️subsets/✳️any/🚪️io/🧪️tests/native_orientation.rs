use super::*;

#[test]
fn neutral_represented_determinants_remain_exact_across_underflow_and_cancellation() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("gltf-framing-cases.json")).unwrap();
    for row in cases["orientations"].as_array().unwrap() {
        let mut matrix=IDENTITY;
        for r in 0..3 {for c in 0..3 {matrix[r][c]=row["linear"][r*3+c].as_f64().unwrap();}}
        assert_eq!(orientation(matrix).unwrap() as i64,row["sign"].as_i64().unwrap(),"{}",row["name"]);
    }
}

#[test]
fn nonfinite_matrix_entries_have_no_valid_orientation() {
    for value in [f64::INFINITY,f64::NEG_INFINITY,f64::NAN] {
        for (row,column) in [(0,0),(0,3)] {
            let mut matrix=IDENTITY;
            matrix[row][column]=value;
            assert!(orientation(matrix).is_err());
        }
    }
}
