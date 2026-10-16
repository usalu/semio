use semio_framework_os_renderer_wgpu_native_runtime_modules as native;
use sha2::{Digest, Sha256};
use std::{fs, io::Read, path::Path};

/// 📥️ Reads the authored Native manifest/pages implementation and independently checks component hashes and JSON descriptors.
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 3 || args.len() > 4 { std::process::exit(2); }
    let path = Path::new(&args[1]);
    let bytes = fs::read(path).unwrap();
    let manifest = match native::NativeRuntimeManifest::read(native::NativeJsonPages::new(bytes.chunks(3)), &args[2]) {
        Ok(value) => value,
        Err(_) => std::process::exit(2),
    };
    if args.get(3).is_some_and(|arg| arg == "validate") { return; }
    let mut rows = Vec::new();
    for module in manifest.modules {
        let wasm = fs::read(path.parent().unwrap().join(module.wasm_path)).unwrap();
        assert_eq!(&wasm[..8], &[0, 97, 115, 109, 13, 0, 1, 0]);
        assert_eq!(format!("{:x}", Sha256::digest(&wasm)), module.wasm_sha256);
        let bytes = fs::read(path.parent().unwrap().join(module.descriptor_path)).unwrap();
        let descriptor: serde_json::Value = serde_json::from_reader(native::NativeJsonPages::new(bytes.chunks(7))).unwrap();
        assert_eq!(descriptor["manifest"]["pluginId"], module.plugin_id);
        assert!(descriptor.get("hashes").is_none());
        rows.push(serde_json::json!({"pluginId": module.plugin_id, "wasmSha256": module.wasm_sha256}));
    }
    let pages: [&[u8]; 4] = [b"", b"first", b"", b"second"];
    let mut reader = native::NativeJsonPages::new(pages.into_iter());
    assert_eq!(reader.read(&mut []).unwrap(), 0);
    let mut joined = Vec::new();
    reader.read_to_end(&mut joined).unwrap();
    assert_eq!(joined, b"firstsecond");
    println!("{}", serde_json::to_string(&rows).unwrap());
}
