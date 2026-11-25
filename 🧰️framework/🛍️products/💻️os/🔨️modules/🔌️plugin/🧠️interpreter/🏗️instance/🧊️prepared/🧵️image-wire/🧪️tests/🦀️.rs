//! 🧪️ The exact neutral Wasm image round-trips through sealed state and refuses malformed resources.

use super::*;
use serde_json::Value as Json;

const STORAGE: usize = 1 << 20;

fn hex(value: &Json) -> Vec<u8> {
 let text = value.as_str().unwrap();
 (0..text.len()).step_by(2).map(|offset| u8::from_str_radix(&text[offset..offset + 2], 16).unwrap()).collect()
}

fn fixture() -> (Json, Arc<CoreModule>, [u8; 32], Vec<u8>) {
 let row: Json = serde_json::from_str(include_str!("prepared-image.json")).unwrap();
 let module = Arc::new(CoreModule::parse(&hex(&row["coreHex"])).unwrap());
 let digest = hex(&row["componentSha256"]).try_into().unwrap();
 let image = module.prepare_actor_image(128, STORAGE, |_| true).unwrap().unwrap();
 let mut bytes = Vec::new();
 image.write_image(digest, STORAGE, |chunk| {
  assert!(chunk.len() <= CHUNK_BYTES);
  bytes.extend_from_slice(chunk);
  Ok(())
 }).unwrap();
 (row, module, digest, bytes)
}

#[test]
fn prepared_image_round_trips_exact_module_resources() {
 let (row, module, digest, bytes) = fixture();
 let fresh = Arc::new(CoreModule::parse(&hex(&row["coreHex"])).unwrap());
 let image = PreparedCoreInstance::read_image(Arc::clone(&fresh), digest, &bytes, bytes.len(), STORAGE, || true).unwrap();
 assert!(image.matches_module(&fresh));
 assert!(!image.matches_module(&module));
 assert_eq!(bytes.len(), row["wireBytes"].as_u64().unwrap() as usize);
 assert!(bytes.starts_with(&hex(&row["prefix"])));
 assert_eq!(&bytes[HEADER.len()..HEADER.len() + digest.len()], &digest);
 let prefix: Vec<_> = row["memoryPrefix"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect();
 assert_eq!(&image.memories[0].bytes[..prefix.len()], prefix);
 assert_eq!(image.memories[0].bytes.len(), WASM_PAGE_BYTES);
 assert!(image.memories[0].bytes[prefix.len()..].iter().all(|byte| *byte == 0));
 let table: Vec<_> = row["table"].as_array().unwrap().iter().map(Json::as_u64).collect();
 assert_eq!(image.tables[0].values, table);
 let globals: Vec<_> = image.globals.iter().map(|row| row.value).collect();
 assert_eq!(globals, [Value::I32(42), Value::F32(0x8000_0000), Value::ExternRef(None)]);
 assert_eq!(image.data, [None, Some(vec![4, 5, 6]), None]);
 assert_eq!(image.elements, [None, Some(vec![Some(0)]), None]);
 let mut again = Vec::new();
 image.write_image(digest, bytes.len(), |chunk| { again.extend_from_slice(chunk); Ok(()) }).unwrap();
 assert_eq!(bytes, again);
}

#[test]
fn prepared_image_rejects_forged_bounds_references_and_extents() {
 let (row, module, digest, wire) = fixture();
 let read = |input: &[u8]| PreparedCoreInstance::read_image(Arc::clone(&module), digest, input, STORAGE, STORAGE, || true);
 for prefix in [0, HEADER.len(), HEADER.len() + digest.len(), wire.len() - 1] {
  assert!(read(&wire[..prefix]).is_err());
 }
 let mut extra = wire.clone();
 extra.push(0);
 assert!(read(&extra).is_err());
 let mut wrong_sha = digest;
 wrong_sha[31] ^= 0x80;
 assert!(PreparedCoreInstance::read_image(Arc::clone(&module), wrong_sha, &wire, STORAGE, STORAGE, || true).is_err());
 let mut different_core = hex(&row["coreHex"]);
 different_core.extend([0, 1, 0]);
 let different = Arc::new(CoreModule::parse(&different_core).unwrap());
 assert!(PreparedCoreInstance::read_image(different, digest, &wire, STORAGE, STORAGE, || true).is_err());
 let shape = HEADER.len() + digest.len() + 8;
 let table_refs = shape + 4 * 8 + WASM_PAGE_BYTES;
 let data_flags = wire.len() - 3 - 8 - 3;
 for (offset, bits) in [
  (shape, u64::MAX),
  (shape + 8, 0),
  (shape + 8, u64::MAX),
  (table_refs - 8, u64::MAX),
  (table_refs, 1),
 ] {
  let mut forged = wire.clone();
  forged[offset..offset + 8].copy_from_slice(&bits.to_le_bytes());
  assert!(read(&forged).is_err(), "accepted forged extent at {offset}");
 }
 for (offset, value) in [(8, 2), (data_flags, 1), (data_flags + 1, 2)] {
  let mut forged = wire.clone();
  forged[offset] = value;
  assert!(read(&forged).is_err(), "accepted forged property at {offset}");
 }
 assert!(PreparedCoreInstance::read_image(Arc::clone(&module), digest, &wire, wire.len() - 1, STORAGE, || true).is_err());
 assert!(PreparedCoreInstance::read_image(Arc::clone(&module), digest, &wire, STORAGE, WASM_PAGE_BYTES - 1, || true).is_err());
 let mut checks = 0;
 let error = PreparedCoreInstance::read_image(Arc::clone(&module), digest, &wire, STORAGE, STORAGE, || { checks += 1; checks < 5 });
 assert!(matches!(error, Err(CoreError::State(message)) if message.contains("cancelled")));
 let image = read(&wire).unwrap();
 assert!(image.write_image(digest, wire.len() - 1, |_| Ok(())).is_err());
 assert!(image.write_image(digest, wire.len(), |chunk| {
  if chunk.len() == WASM_PAGE_BYTES { Err(state("cancelled output sink")) } else { Ok(()) }
 }).is_err());
}
