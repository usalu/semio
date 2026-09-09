
use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap()
}

fn hex(value: &str) -> Vec<u8> {
    value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

async fn actual_bytes(fixture: &serde_json::Value) -> Vec<u8> {
    let options = super::super::WriteOptions { required_flags: crate::REQUIRED_HASH_CHAIN, optional_flags: crate::OPTIONAL_CANONICAL };
    let mut writer = super::super::SprWriter::begin(Vec::new(), &options).await.unwrap();
    for commit in fixture["commits"].as_array().unwrap() {
        for record in commit["records"].as_array().unwrap() {
            writer.write_record(record["kind"].as_u64().unwrap() as u8, true, &hex(record["payloadHex"].as_str().unwrap()), crate::codec::ids::CodecId(0)).await.unwrap();
        }
        assert_eq!(writer.commit().await.unwrap(), commit["offset"].as_u64().unwrap());
        assert_eq!(writer.position().await, commit["end"].as_u64().unwrap());
    }
    writer.into_sink().await
}

fn scan(bytes: &[u8], grant: usize, limits: RetainedSprLimits) -> Result<VerifiedSprSpan, RetainedSprDiagnostic> {
    let mut scan = RetainedSprVerification::new(bytes.len() as u64, limits)?;
    while scan.consumed() < bytes.len() as u64 {
        let start = scan.consumed() as usize;
        let mut fuel = grant;
        let read = scan.push(&bytes[start..], &mut fuel)?;
        assert!(read > 0 && read <= grant);
        assert_eq!(read + fuel, grant);
    }
    scan.finish()
}

#[semio_framework_async_macros::async_test]
async fn retained_spr_resume_preserves_exact_prefix_and_commit_chain() {
    let fixture = fixture();
    let bytes = actual_bytes(&fixture).await;
    let resume = &fixture["resume"];
    let record = &resume["record"];
    for cut in resume["cuts"].as_array().unwrap() {
        let cut = cut.as_u64().unwrap() as usize;
        let span = scan(&bytes[..cut], 7, RetainedSprLimits::default()).unwrap();
        let end = span.end() as usize;
        let sequence = span.sequence();
        let frames = span.frames();
        let previous_offset = span.commit_offset();
        let previous_chain = *span.chain();
        let mut writer = super::super::SprWriter::resume_verified(bytes[..end].to_vec(), span).await.unwrap();
        assert_eq!(writer.position().await, end as u64);
        writer.write_record(record["kind"].as_u64().unwrap() as u8, true, &hex(record["payloadHex"].as_str().unwrap()), crate::codec::ids::CodecId(0)).await.unwrap();
        let offset = writer.commit().await.unwrap() as usize;
        let resumed = writer.into_sink().await;
        assert_eq!(&resumed[..end], &bytes[..end]);
        assert_eq!(resumed.len() - end, resume["addedBytes"].as_u64().unwrap() as usize);
        let next = scan(&resumed, 1, RetainedSprLimits::default()).unwrap();
        assert_eq!(next.sequence(), sequence + 1);
        assert_eq!(next.frames(), frames + 2);
        assert_eq!(next.end(), resumed.len() as u64);
        assert_eq!(next.tail(), 0);
        assert_eq!(u64::from_le_bytes(resumed[offset + 11..offset + 19].try_into().unwrap()), previous_offset);
        let mut independent = blake3::Hasher::new();
        independent.update(&previous_chain);
        independent.update(blake3::hash(&resumed[end..offset]).as_bytes());
        assert_eq!(&resumed[offset + 35..offset + 67], independent.finalize().as_bytes());
        for delta in resume["wrongSinkOffsets"].as_array().unwrap() {
            let span = scan(&bytes[..cut], 7, RetainedSprLimits::default()).unwrap();
            let wrong_len = (span.end() as i64 + delta.as_i64().unwrap()) as usize;
            assert!(super::super::SprWriter::resume_verified(vec![0; wrong_len], span).await.is_err());
        }
    }
    let span = scan(&bytes, 7, RetainedSprLimits::default()).unwrap();
    let mut writer = super::super::SprWriter::resume_verified(bytes.clone(), span).await.unwrap();
    writer.next_commit_seq = resume["exhaustedSequence"].as_str().unwrap().parse().unwrap();
    assert!(writer.commit().await.is_err());
    assert_eq!(writer.into_sink().await, bytes);
    eprintln!("[DEBUG] SPR resume: 6 verified prefixes survive byte-exactly; chain/sequence/offset continue; 12 wrong sink lengths and exhausted sequence denied");
}

async fn verify_compressed_fixture(fixture: &serde_json::Value) {
    let header = hex(fixture["headerHex"].as_str().unwrap());
    let frame = |kind, flags, payload: &[u8]| {
        let mut body = vec![kind, flags];
        body.extend_from_slice(payload);
        let mut bytes = Vec::new();
        crate::codec::write_varint_u64(&mut bytes, body.len() as u64);
        bytes.extend_from_slice(&body);
        bytes.extend_from_slice(&crate::codec::crc32c(&body).to_le_bytes());
        let length = bytes.len() as u32 + 4;
        bytes.extend_from_slice(&length.to_le_bytes());
        bytes
    };
    for row in fixture["compressed"].as_array().unwrap() {
        let kind = row["kind"].as_u64().unwrap() as u8;
        let flags = row["flags"].as_u64().unwrap() as u8;
        let mut payload = hex(row["rawLengthHex"].as_str().unwrap());
        let stored = hex(row["storedHex"].as_str().unwrap());
        payload.extend_from_slice(&stored);
        let record = frame(kind, flags, &payload);
        let mut chain = blake3::Hasher::new();
        chain.update(blake3::hash(&header).as_bytes());
        chain.update(blake3::hash(&record).as_bytes());
        let mut commit = [0u8; 64];
        commit[..8].copy_from_slice(&1u64.to_le_bytes());
        commit[16..24].copy_from_slice(&(record.len() as u64).to_le_bytes());
        commit[24..28].copy_from_slice(&1u32.to_le_bytes());
        commit[32..].copy_from_slice(chain.finalize().as_bytes());
        let mut bytes = header.clone();
        bytes.extend_from_slice(&record);
        bytes.extend_from_slice(&frame(crate::REC_COMMIT, crate::wire::FRAME_FLAG_CRITICAL, &commit));
        for grant in [1, 7, 4096] {
            let result = scan(&bytes, grant, RetainedSprLimits::default());
            match row["error"].as_str() {
                None => {
                    let span = result.unwrap();
                    assert_eq!(span.end(), bytes.len() as u64);
                    assert_eq!(span.sequence(), 1);
                    assert_eq!(span.frames(), 2);
                    assert_eq!(span.tail(), 0);
                }
                Some("frame") => assert_eq!(result, Err(RetainedSprDiagnostic::Frame), "{}", row["id"]),
                Some("commit") => assert_eq!(result, Err(RetainedSprDiagnostic::Commit), "{}", row["id"]),
                _ => unreachable!(),
            }
        }
        if row["error"].is_null() {
            let raw = hex(row["rawHex"].as_str().unwrap());
            let mut original = Vec::new();
            super::super::write_frame_retained(&mut original, kind, flags, Some(raw.len() as u64), &stored).await.unwrap();
            assert_eq!(original, record, "compressed framing differs from the production retained writer");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_spr_verification_matches_neutral_commits_and_torn_prefixes() {
    let fixture = fixture();
    let bytes = actual_bytes(&fixture).await;
    assert_eq!(&bytes[..32], hex(fixture["headerHex"].as_str().unwrap()));
    for grant in fixture["fuelGrants"].as_array().unwrap() {
        let grant = grant.as_u64().unwrap() as usize;
        for end in 32..=bytes.len() {
            let span = scan(&bytes[..end], grant, RetainedSprLimits::default()).unwrap();
            let commit = fixture["commits"].as_array().unwrap().iter().rev().find(|row| row["end"].as_u64().unwrap() <= end as u64);
            let committed = commit.map_or(32, |row| row["end"].as_u64().unwrap());
            assert_eq!(span.end(), committed);
            assert_eq!(span.tail(), end as u64 - committed);
            assert_eq!(span.sequence(), commit.map_or(0, |row| row["sequence"].as_u64().unwrap()));
            assert_eq!(span.frames(), commit.map_or(0, |row| row["recoveredFrames"].as_u64().unwrap()));
        }
    }
    let mut scan = RetainedSprVerification::new(bytes.len() as u64, RetainedSprLimits::default()).unwrap();
    assert_eq!(scan.push(&bytes, &mut 0), Ok(0));
    assert_eq!(scan.consumed(), 0);
    assert_eq!(scan.finish(), Err(RetainedSprDiagnostic::State));
    let mut fuel = bytes.len();
    scan.push(&bytes, &mut fuel).unwrap();
    let span = scan.finish().unwrap();
    assert_eq!(span.end(), bytes.len() as u64);
    assert_eq!(scan.finish(), Err(RetainedSprDiagnostic::State));
    let mut chain = *blake3::hash(&bytes[..32]).as_bytes();
    for (start, record_ranges) in [(91, vec![(32, 75), (75, 91)]), (180, vec![(166, 180)])] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&chain);
        for (from, to) in record_ranges {
            hasher.update(blake3::hash(&bytes[from..to]).as_bytes());
        }
        chain = *hasher.finalize().as_bytes();
        assert_eq!(&bytes[start + 35..start + 67], &chain);
    }
    assert_eq!(span.chain(), &chain);
    eprintln!("[DEBUG] retained SPR: 2 real writer commits, 224 LastCommit prefixes at 3 fuel grants; exact EOF required; one span handoff; no typed records");
}

#[semio_framework_async_macros::async_test]
async fn retained_spr_verification_rejects_hostile_frames_without_publication() {
    let fixture = fixture();
    let bytes = actual_bytes(&fixture).await;
    verify_compressed_fixture(&fixture).await;
    for row in fixture["negative"].as_array().unwrap() {
        let mut mutated = bytes.clone();
        let mut limits = RetainedSprLimits::default();
        match row["operation"].as_str().unwrap() {
            "replace-first-length" => {
                mutated.splice(32..33, hex(row["hex"].as_str().unwrap()));
            }
            "record-limit" => limits.records = row["value"].as_u64().unwrap(),
            "file-limit" => limits.file_bytes = row["value"].as_u64().unwrap(),
            operation => {
                let offset = row["offset"].as_u64().unwrap() as usize
                    + match operation {
                        "commit-xor" => 94,
                        "second-commit-xor" => 183,
                        _ => 0,
                    };
                mutated[offset] ^= row["value"].as_u64().unwrap() as u8;
                if row["repairCrc"].as_bool().unwrap() {
                    let (start, end) = match operation {
                        "header-xor" => (0, 20),
                        "second-commit-xor" => (181, 247),
                        _ => (92, 158),
                    };
                    let crc = crate::codec::crc32c(&mutated[start..end]);
                    mutated[end..end + 4].copy_from_slice(&crc.to_le_bytes());
                }
            }
        }
        let expected = match row["error"].as_str().unwrap() {
            "header" => RetainedSprDiagnostic::Header,
            "frame" => RetainedSprDiagnostic::Frame,
            "commit" => RetainedSprDiagnostic::Commit,
            "capacity" => RetainedSprDiagnostic::Capacity,
            _ => unreachable!(),
        };
        for grant in [1, 7, 4096] {
            assert_eq!(scan(&mutated, grant, limits), Err(expected), "{}", row["id"]);
        }
    }
    for boundary in 0..=bytes.len() {
        let mut scan = RetainedSprVerification::new(bytes.len() as u64, RetainedSprLimits::default()).unwrap();
        let mut fuel = boundary;
        scan.push(&bytes[..boundary], &mut fuel).unwrap();
        scan.cancel();
        let position = scan.consumed();
        let mut fuel = 4096;
        assert_eq!(scan.push(&bytes[boundary..], &mut fuel), Err(RetainedSprDiagnostic::Cancelled));
        assert_eq!(fuel, 4096);
        assert_eq!(scan.consumed(), position);
        assert_eq!(scan.finish(), Err(RetainedSprDiagnostic::Cancelled));
    }
    for end in 0..32 {
        assert_eq!(scan(&bytes[..end], 1, RetainedSprLimits::default()), Err(RetainedSprDiagnostic::Header));
    }
    eprintln!("[DEBUG] retained SPR: 26 hostile header/frame/commit/limit denials and 10 compressed grammar cases at 3 grants; cancellation at all 256 byte boundaries; no input authority or semantic publication");
}
