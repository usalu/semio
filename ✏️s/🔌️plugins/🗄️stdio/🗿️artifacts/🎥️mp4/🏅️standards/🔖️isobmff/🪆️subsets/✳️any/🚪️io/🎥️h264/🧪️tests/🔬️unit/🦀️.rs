
use super::*;

#[semio_framework_async_macros::async_test]
async fn avcc_round_trips_sps_pps_lists() {
    // 🧪️ A tiny, structurally valid baseline SPS RBSP (profile_idc=66) — enough to exercise
    // parse_sps_dimensions and the avcC list round trip without depending on a real fixture.
    let sps: Vec<u8> = vec![0x67, 0x42, 0x00, 0x1E, 0x8C, 0x8D, 0x40];
    let pps: Vec<u8> = vec![0x68, 0xCE, 0x3C, 0x80];
    let avcc = build_avcc(&[sps.clone()], &[pps.clone()], 4);
    // strip the write_box 8-byte header to get the raw avcC payload parse_avcc expects.
    let payload = &avcc[8..];
    let (sps_out, pps_out, nal_len) = parse_avcc(payload).expect("parse avcc");
    assert_eq!(sps_out, vec![sps]);
    assert_eq!(pps_out, vec![pps]);
    assert_eq!(nal_len, 4);
}

#[semio_framework_async_macros::async_test]
async fn strip_emulation_prevention_removes_three_byte() {
    let ebsp = [0x00, 0x00, 0x03, 0x01, 0x00, 0x00, 0x03, 0x02];
    assert_eq!(strip_emulation_prevention(&ebsp), vec![0x00, 0x00, 0x01, 0x00, 0x00, 0x02]);
}
