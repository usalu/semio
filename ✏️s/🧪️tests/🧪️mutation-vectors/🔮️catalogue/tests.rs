//! 🧪️ Structured mutation catalogue membership uses the same neutral cases as the independent JSON oracle.
use super::*;
#[test]
fn native_mutation_catalogue_matches_structured_neutral_cases(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("cases.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let kinds=row["kinds"].as_array().unwrap().iter().map(|kind|kind.as_str().unwrap()).collect::<Vec<_>>();
        let input=serde_json::to_string(&row["manifest"]).unwrap();
        let result=mutation_catalogue_missing(&kinds,&input);
        if let Some(error)=row["error"].as_str(){assert!(result.unwrap_err().starts_with(error),"{}",row["name"]);}else{
            let actual=result.unwrap();let expected=serde_json::from_value::<Vec<String>>(row["missing"].clone()).unwrap();assert_eq!(actual,expected,"{}",row["name"]);
            let registered=row["manifest"]["mutationCatalogs"].as_array().unwrap().iter().flat_map(|catalogue|catalogue["vectors"].as_array().unwrap()).map(|vector|vector["mutationId"].as_str().unwrap()).collect::<std::collections::BTreeSet<_>>();
            let oracle=kinds.iter().filter(|kind|!registered.contains(**kind)).map(|kind|kind.to_string()).collect::<Vec<_>>();assert_eq!(actual,oracle);
        }
    }
}
