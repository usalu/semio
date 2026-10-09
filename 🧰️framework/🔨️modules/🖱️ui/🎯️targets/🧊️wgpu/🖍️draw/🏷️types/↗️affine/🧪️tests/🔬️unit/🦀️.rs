use super::*;

#[test]
fn authored_glyph_and_image_corners_retain_their_affine_axes() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for sample in fixture["cases"].as_array().unwrap() {
        let rect=std::array::from_fn(|index|sample["rect"][index].as_f64().unwrap() as f32);
        let matrix=std::array::from_fn(|index|sample["matrix"][index].as_f64().unwrap() as f32);
        let glyph=UiInstance::glyph_affine(rect,Rgba::new(1.0,1.0,1.0,1.0),[0.0,0.0,1.0,1.0],matrix);
        let raster=UiInstance::raster_affine(rect,[0.0,0.0,1.0,1.0],0.5,matrix);
        for (index,[x,y]) in [[0.0,0.0],[1.0,0.0],[1.0,1.0],[0.0,1.0]].into_iter().enumerate() {
            let expected=std::array::from_fn(|axis|sample["corners"][index][axis].as_f64().unwrap() as f32);
            for instance in [glyph,raster] {
                let actual=[instance.rect[0]+x*instance.rect[2]+y*instance.params[0],instance.rect[1]+x*instance.rect[3]+y*instance.params[1]];
                assert_eq!(actual,expected,"{}",sample["name"]);
                assert_eq!(instance.uv_rect,[0.0,0.0,1.0,1.0]);
            }
        }
        assert_eq!(glyph.params[2],KIND_AFFINE_GLYPH);
        assert_eq!(raster.params[2],KIND_AFFINE_RASTER);
        assert_eq!(raster.color[3],0.5);
    }
}
