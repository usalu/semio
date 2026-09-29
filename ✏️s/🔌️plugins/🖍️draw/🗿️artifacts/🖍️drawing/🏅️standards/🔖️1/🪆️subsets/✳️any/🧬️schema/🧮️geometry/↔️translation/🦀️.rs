//! ↔️ Convert world-space dragging to a layer's parent coordinates.
pub fn translate(transform: [f64;5], parent: [f64;6], delta: [f64;2]) -> Option<[f64;5]> {
    if !transform.iter().chain(delta.iter()).all(|value| value.is_finite()) { return None; }
    let inverse = super::inverse(parent)?;
    let mut result = transform;
    result[0] += inverse[0]*delta[0]+inverse[2]*delta[1];
    result[1] += inverse[1]*delta[0]+inverse[3]*delta[1];
    result.iter().all(|value| value.is_finite()).then_some(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parent_translation_fixtures() {
        let cases: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
        for case in cases.as_array().unwrap() {
            let result = super::translate(serde_json::from_value(case["transform"].clone()).unwrap(),serde_json::from_value(case["parent"].clone()).unwrap(),serde_json::from_value(case["delta"].clone()).unwrap());
            let expected: Option<[f64;5]> = serde_json::from_value(case["after"].clone()).unwrap();
            assert_eq!(result,expected,"{}",case["name"]);
        }
        eprintln!("[TRACE] world translation agrees with all nested-parent fixtures");
    }
}

/// 🗂️ A selected ancestor owns its descendants' world movement.
pub fn path_contains(parent: &[u16], child: &[u16]) -> bool {
    child.starts_with(parent)
}

#[cfg(test)]
mod selection_tests {
    #[test]
    fn selected_ancestors_move_once() {
        let cases:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🗂️selection/🔣️.json")).unwrap();
        for case in cases.as_array().unwrap() {
            let paths:Vec<Vec<u16>>=serde_json::from_value(case["paths"].clone()).unwrap();
            let roots:Vec<_>=paths.iter().filter(|path|!paths.iter().any(|parent|parent.len()<path.len() && super::path_contains(parent,path))).cloned().collect();
            assert_eq!(roots,serde_json::from_value::<Vec<Vec<u16>>>(case["roots"].clone()).unwrap());
        }
    }
}
