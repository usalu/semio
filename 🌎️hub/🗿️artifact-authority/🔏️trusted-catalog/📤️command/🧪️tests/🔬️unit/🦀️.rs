
use super::*;

#[test]
fn trusted_publication_transport_matches_neutral_arguments_and_input_bounds() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/📤️publication/📡️transport.json")).unwrap();
    let arguments = |value: &serde_json::Value| value.as_array().unwrap().iter().map(|value| OsString::from(value.as_str().unwrap())).collect::<Vec<_>>();
    assert!(selected(&arguments(&fixture["command"])).unwrap());
    assert!(!selected(&[]).unwrap());
    for row in fixture["rejectedArguments"].as_array().unwrap() {
        assert!(selected(&arguments(row)).is_err());
    }
    let command = include_bytes!("../../../🧪️fixtures/📤️publication/📬️command.json");
    assert_eq!(read_command(command.as_slice()).unwrap(), command);
    assert!(read_command(&b""[..]).is_err());
    let maximum = fixture["commandBytes"].as_u64().unwrap() as usize;
    assert_eq!(read_command(vec![b' '; maximum].as_slice()).unwrap().len(), maximum);
    assert!(read_command(vec![b' '; maximum + 1].as_slice()).is_err());
    println!("[DEBUG] publication transport: args=closed stdin=1..4096 one-shot-before-services=required");
}
