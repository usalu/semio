#[test]
fn theme_color_fields_match_owned_rows_and_independent_serde_peniko_oracles() {
    struct Fields<'a>(&'a serde_json::Value);
    impl ThemeColorFields for Fields<'_> {
        fn rgba8_field(&self, key: &str) -> Option<super::Rgba8> {
            let value = self.0.get(key)?.as_array()?;
            Some(super::Rgba8 { r: value[0].as_u64()? as u8, g: value[1].as_u64()? as u8, b: value[2].as_u64()? as u8, a: value[3].as_u64()? as u8 })
        }
    }
    fn rgba(value: &serde_json::Value) -> [u8; 4] {
        serde_json::from_value(value.clone()).expect("schema-declared RGBA bytes")
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️theme-color-fields/🔣️.json")).expect("neutral theme rows");
    let before = rgba(&fixture["before"]);
    for row in fixture["cases"].as_array().expect("theme cases") {
        let key = row["key"].as_str().expect("field name");
        let expected = rgba(&row["expected"]);
        let mut owned = Color::from_rgba8(before[0], before[1], before[2], before[3]);
        let mut interpreted = owned;
        merge_color_field(&mut owned, &Fields(&row["fields"]), key);
        merge_color_field(&mut interpreted, &row["json"], key);
        let oracle = peniko::Color::from_rgba8(expected[0], expected[1], expected[2], expected[3]).to_rgba8();
        assert_eq!(owned, interpreted, "{}", row["id"]);
        let actual = owned.to_rgba8();
        assert_eq!([actual.r, actual.g, actual.b, actual.a], expected, "{}", row["id"]);
        assert_eq!([actual.r, actual.g, actual.b, actual.a], [oracle.r, oracle.g, oracle.b, oracle.a], "{}", row["id"]);
    }
}
