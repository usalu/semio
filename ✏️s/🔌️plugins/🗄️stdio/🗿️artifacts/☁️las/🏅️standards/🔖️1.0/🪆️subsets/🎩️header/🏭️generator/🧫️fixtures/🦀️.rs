//! 🏭️ Generates `survey-strip.las`, a small, deliberately varied, fully deterministic LAS 1.0
//! point cloud — built entirely through the real `las` 0.11 crate's byte-exact `las::raw::{Header,
//! Vlr, Point}` types (the same crate registered as `las-1-0-any-mutate` in `../../🔣️oracle.json`),
//! never through this repository's own `encode_las`. No wall-clock, no randomness: byte-for-byte
//! reproducible on every run, which is what `test fixture reproduce` checks.
//!
//! Deliberately exercises the surface `las@1.0`'s 15 mutation kinds touch: a real system
//! identifier/generating-software pair, a fixed creation date, non-trivial scale/offset (so
//! `set-scale-and-offset` genuinely changes reconstructed coordinates), bounds that DIVERGE from
//! the real point extent (so `set-bounds`/`set-points-by-return` are checked mutations, not
//! no-ops against the real distribution — see the oracle module's own doc comment on this), two
//! VLRs with distinct payloads, and six points spanning a range of classification, intensity,
//! return-number/edge-of-flight-line flags and scan angle.
//!
//! Usage: `generate <output.las>`.

use las::raw::point::{Flags, ScanAngle};
use las::raw::vlr::RecordLength;
use las::raw::{Header, Point, Vlr, LASF};
use std::env;

fn fixed<const N: usize>(bytes: &[u8]) -> [u8; N] {
    let mut buf = [0u8; N];
    let n = bytes.len().min(N);
    buf[..n].copy_from_slice(&bytes[..n]);
    buf
}

fn main() {
    let out_path = env::args().nth(1).expect("usage: generate <output.las>");
    let bytes = build_las();
    std::fs::write(&out_path, &bytes).unwrap_or_else(|error| panic!("writing {out_path}: {error}"));
    eprintln!("wrote {} bytes to {out_path}", bytes.len());
}

fn build_las() -> Vec<u8> {
    let vlrs = vec![
        Vlr { reserved: 0, user_id: fixed(b"SEMIOFIX"), record_id: 100, record_length_after_header: RecordLength::Vlr(20), description: fixed(b"survey-strip-a"), data: b"survey-strip-vlr-one".to_vec() },
        Vlr { reserved: 0, user_id: fixed(b"SEMIOFIX"), record_id: 101, record_length_after_header: RecordLength::Vlr(20), description: fixed(b"survey-strip-b"), data: b"survey-strip-vlr-two".to_vec() },
    ];

    // 📏️ Scale/offset chosen so real-world coordinates are non-trivial: x = record*0.01 + 500000,
    // y = record*0.01 + 4000000, z = record*0.01 + 100.
    let x_scale = 0.01;
    let y_scale = 0.01;
    let z_scale = 0.01;
    let x_offset = 500_000.0;
    let y_offset = 4_000_000.0;
    let z_offset = 100.0;

    let points: Vec<Point> = (0..6i32)
        .map(|i| {
            let return_number = ((i % 3) + 1) as u8; // 1..=3
            let number_of_returns = 3u8;
            let scan_direction_flag = i % 2 == 0;
            let edge_of_flight_line = i == 5;
            let classification = 2 + (i as u8 % 4); // varies 2..=5
            let a = (return_number & 0x07) | ((number_of_returns & 0x07) << 3) | ((scan_direction_flag as u8) << 6) | ((edge_of_flight_line as u8) << 7);
            Point {
                x: 1000 * i,
                y: -500 * i,
                z: 250 + 10 * i,
                intensity: (1000 + i * 137) as u16,
                flags: Flags::TwoByte(a, classification),
                scan_angle: ScanAngle::Rank((i * 3 - 9) as i8),
                user_data: i as u8,
                point_source_id: (7000 + i) as u16,
                gps_time: None,
                color: None,
                waveform: None,
                nir: None,
                extra_bytes: Vec::new(),
            }
        })
        .collect();

    let vlr_bytes: u32 = vlrs.iter().map(|vlr| 54 + vlr.data.len() as u32).sum();
    let header = Header {
        file_signature: LASF,
        version: las::Version::new(1, 0),
        system_identifier: fixed(b"SEMIO-FIXTURE-GENERATOR"),
        generating_software: fixed(b"semio-las-1-0-any-fixture-generator 0.1.0"),
        file_creation_day_of_year: 42,
        file_creation_year: 2026,
        header_size: 227,
        offset_to_point_data: 227 + vlr_bytes,
        number_of_variable_length_records: vlrs.len() as u32,
        point_data_record_format: 0,
        point_data_record_length: 20,
        number_of_point_records: points.len() as u32,
        // 🎯️ Deliberately NOT the real min/max of the points above — non-structural, directly
        // retained content per the oracle's own doc comment, so `set-bounds`/`set-points-by-return`
        // are genuinely checked mutations rather than accidentally matching the real distribution.
        number_of_points_by_return: [2, 2, 2, 0, 0],
        x_scale_factor: x_scale,
        y_scale_factor: y_scale,
        z_scale_factor: z_scale,
        x_offset,
        y_offset,
        z_offset,
        max_x: 5100.0,
        max_y: 100.0,
        max_z: 350.0,
        min_x: -100.0,
        min_y: -2600.0,
        min_z: 245.0,
        ..Default::default()
    };

    let mut out = Vec::new();
    header.write_to(&mut out).expect("las header write");
    for vlr in &vlrs {
        vlr.write_to(&mut out).expect("las vlr write");
    }
    for point in &points {
        point.write_to(&mut out, &las::point::Format::new(0).expect("point format 0")).expect("las point write");
    }
    out
}
