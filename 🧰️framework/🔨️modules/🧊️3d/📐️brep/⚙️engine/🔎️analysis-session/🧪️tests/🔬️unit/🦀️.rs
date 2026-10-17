use super::*;
use semio_framework_3d_brep_analysis::Watertightness;


#[test]
fn validate_judges_the_selected_shape_not_the_whole_session() {
    let mut kernel = Brep::new();
    let sound = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
    let broken = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
    let face = kernel.body.solid_faces(kernel.solid_id(&broken).unwrap())[0];
    kernel.body.faces.get_mut(face).unwrap().flipped ^= true;
    let ok = |json: &str| json.contains("\"ok\":true");
    assert!(ok(&kernel.validate_sync(&sound).unwrap()), "{}", kernel.validate_sync(&sound).unwrap());
    assert!(!ok(&kernel.validate_sync(&broken).unwrap()));
    assert!(kernel.validity_sync(&sound).unwrap().ok);
    let verdict = kernel.watertightness_sync(&broken).unwrap();
    assert_eq!((verdict.verdict, verdict.misoriented_edges), (Watertightness::Misoriented, 4));
    assert_eq!(kernel.watertightness_sync(&sound).unwrap().verdict, Watertightness::Watertight);
    let shells = kernel.solid_shells_sync(&sound).unwrap();
    assert!(kernel.validate_sync(&shells[0]).unwrap().contains("\"ok\":true"), "validate also answers for a shell");
}
