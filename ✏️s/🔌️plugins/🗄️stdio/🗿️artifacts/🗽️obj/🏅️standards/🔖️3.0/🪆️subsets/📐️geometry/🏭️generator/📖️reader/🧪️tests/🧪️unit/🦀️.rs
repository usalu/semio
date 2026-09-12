
use super::*;

#[test]
fn every_recipe_id_resolves_and_before_admits() {
    for recipe in RECIPES {
        assert!(find_recipe(recipe.id).is_some());
        admit_or_panic(recipe.id, recipe.before);
        if let Some(after) = recipe.after {
            admit_or_panic(recipe.id, after);
        }
    }
}
