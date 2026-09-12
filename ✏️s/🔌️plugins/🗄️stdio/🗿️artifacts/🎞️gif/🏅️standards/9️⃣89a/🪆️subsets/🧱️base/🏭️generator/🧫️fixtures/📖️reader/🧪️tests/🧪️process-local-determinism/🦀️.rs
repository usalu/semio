
use super::*;
#[test]
fn encoding_every_recipe_twice_in_one_process_is_byte_identical() {
    for id in RECIPE_IDS {
        let (b1, a1) = recipe(id).unwrap();
        let (b2, a2) = recipe(id).unwrap();
        assert_eq!(encode_gif(&b1), encode_gif(&b2), "before bytes for {id} differ across two in-process encodes");
        assert_eq!(encode_gif(&a1), encode_gif(&a2), "after bytes for {id} differ across two in-process encodes");
    }
}
