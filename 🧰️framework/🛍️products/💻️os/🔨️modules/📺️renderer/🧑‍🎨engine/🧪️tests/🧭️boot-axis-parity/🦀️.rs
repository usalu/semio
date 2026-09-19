//! 🧭️ Boot-axis parity law — the four boot doors must speak ONE vocabulary.
//!
//! React has a single door (`FrameworkOsBootOptions`, `🧱️elements/🐚️Shell/🟦️.tsx`); wgpu has three
//! (the trunk page's descriptor `🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts`, the embeddable
//! `FrameworkOsWgpuBootOptions` in `🎬️renderer-boot/🟦️.ts`, and the native CLI flags in
//! `⌨️native-entrypoint/🦀️.rs`). They had drifted into three different subsets of the same axes, so
//! `?plugin=cad&example=concrete-forest` and `--plugin cad` opened different things. This parses all
//! four field sets out of their own sources and asserts they name the same axes, modulo an explicit,
//! reasoned allowlist — a standing law rather than three one-off fixes that can drift apart again
//! (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1d, audit packet 10).

use std::collections::BTreeSet;
use std::path::PathBuf;

/// 🌳️ `…/🧑‍🎨engine`, the common root of all four sources.
fn engine_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..").canonicalize().expect("engine root")
}

fn read(relative: &str) -> String {
    let path = engine_root().join(relative);
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

/// 🧾️ The `readonly <name>` keys of ONE exported TypeScript object type, in declaration order. The
/// body is taken from `export type <name> = {` to the first line that is exactly `};`, so a nested
/// inline object (indented) cannot end it and a sibling type cannot leak into it.
fn typescript_type_fields(source: &str, type_name: &str) -> Vec<String> {
    let header = format!("export type {type_name} = {{");
    let start = source.find(&header).unwrap_or_else(|| panic!("`{type_name}` is declared")) + header.len();
    let body = &source[start..];
    let end = body.find("\n};").unwrap_or_else(|| panic!("`{type_name}` closes at column 0"));
    body[..end]
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            let rest = trimmed.strip_prefix("readonly ")?;
            let name = rest.split(['?', ':']).next()?.trim();
            (!name.is_empty() && name.chars().all(|character| character.is_ascii_alphanumeric())).then(|| name.to_string())
        })
        .collect()
}

/// 🧾️ The `pub <name>:` fields of ONE Rust struct, camelCased the way its `rename_all` serde
/// attribute renames them — so the Rust twin is compared in the wire spelling, not the snake one.
fn rust_struct_fields(source: &str, struct_name: &str) -> Vec<String> {
    let header = format!("pub struct {struct_name} {{");
    let start = source.find(&header).unwrap_or_else(|| panic!("`{struct_name}` is declared")) + header.len();
    let body = &source[start..];
    let end = body.find("\n}").unwrap_or_else(|| panic!("`{struct_name}` closes at column 0"));
    body[..end]
        .lines()
        .filter_map(|line| {
            let name = line.trim_start().strip_prefix("pub ")?.split(':').next()?.trim();
            (!name.is_empty() && name.chars().all(|character| character.is_ascii_lowercase() || character == '_')).then(|| camel_case(name))
        })
        .collect()
}

fn camel_case(snake: &str) -> String {
    let mut out = String::with_capacity(snake.len());
    let mut upper_next = false;
    for character in snake.chars() {
        if character == '_' {
            upper_next = true;
        } else if upper_next {
            out.extend(character.to_uppercase());
            upper_next = false;
        } else {
            out.push(character);
        }
    }
    out
}

/// 🧾️ Every `--flag` the native entrypoint reads through `arg_value`, as a `kebab` name.
fn native_arg_flags(source: &str) -> Vec<String> {
    source
        .match_indices("arg_value(\"--")
        .filter_map(|(index, needle)| {
            let rest = &source[index + needle.len()..];
            rest.find('"').map(|end| rest[..end].to_string())
        })
        .collect()
}

/// 🔤️ One canonical axis name for the four spellings the same axis carries.
fn canonical(name: &str) -> &'static str {
    match name {
        "plugin" | "pluginVariant" => "plugin",
        "plugins" => "plugins",
        "rootId" => "rootId",
        "appId" | "app" => "appId",
        "appRole" | "role" => "appRole",
        "appMode" | "mode" => "appMode",
        "appExample" | "example" => "appExample",
        "brand" | "brandId" => "brand",
        "locks" => "locks",
        "defaults" => "defaults",
        "hub" => "hub",
        "user" => "hubUser",
        "dataDir" | "data-dir" => "hubDataDir",
        "brokerProof" => "brokerProof",
        "rendererModuleUrl" => "rendererModuleUrl",
        "surfaceSessionFactories" => "surfaceSessionFactories",
        other => Box::leak(other.to_string().into_boxed_str()),
    }
}

fn axes(names: impl IntoIterator<Item = String>) -> BTreeSet<&'static str> {
    names.into_iter().map(|name| canonical(&name)).collect()
}

/// 🙈️ Axes one door legitimately does not carry, each with the reason it cannot. Nothing else may be
/// missing: a NEW axis added to one door and not the others fails this law until it is either wired
/// or listed here with a reason.
fn allowed_absences(door: &str) -> BTreeSet<&'static str> {
    match door {
        // 🧭️ The descriptor is the RESOLVED axis set the renderer receives, not a caller's options:
        // `rootId`/`plugins`/`rendererModuleUrl` address the mount and the module, never the session,
        // and `surfaceSessionFactories` is a JS closure that cannot cross into wasm at all.
        "descriptor" => ["rootId", "plugins", "rendererModuleUrl", "surfaceSessionFactories"].into_iter().collect(),
        // ⚛️ React's door: `appMode` has no React reader yet (the mode axis is `?mode=`, wgpu-only);
        // `hub`/`hubUser`/`hubDataDir` reach React through ShellHost's backbone worker rather than
        // through boot options; `brokerProof` is read from `location.hash` at module scope
        // (`🏛️ShellHost/🟦️.tsx`), never passed in; `appExample` is React's `defaults.exampleId`, which
        // `defaults` already covers; `rendererModuleUrl` has no React analogue (no wasm renderer
        // module to point at).
        "react" => ["appMode", "appExample", "hub", "hubUser", "hubDataDir", "brokerProof", "rendererModuleUrl"].into_iter().collect(),
        // 🧊️ The embeddable wgpu door: `surfaceSessionFactories` is React-only (see above);
        // `brokerProof` is read from the page's own `location.hash`, like React's, never passed in.
        "wgpu-library" => ["surfaceSessionFactories", "brokerProof"].into_iter().collect(),
        // ⌨️ The native CLI: `rootId`/`plugins`/`rendererModuleUrl`/`surfaceSessionFactories` are
        // browser-mount concepts a winit window has no counterpart for; `brokerProof` is a browser
        // hash hand-off (native claims its local credential through the inherited fd instead);
        // `locks`/`defaults` stay process-env axes (`SEMIO_LOCKED_*`/`SEMIO_DEFAULT_EXAMPLE`), the
        // same per-server level React reads them at (`VITE_SEMIO_LOCKED_*`), so they are deliberately
        // NOT per-invocation flags.
        "native" => ["rootId", "plugins", "rendererModuleUrl", "surfaceSessionFactories", "brokerProof", "locks", "defaults"].into_iter().collect(),
        other => panic!("unknown door {other}"),
    }
}

/// ⚙️ Native flags that are not boot axes at all — ops probes and the scale-bench harness.
const NATIVE_NON_AXIS_FLAGS: [&str; 4] = ["scale", "scale-wasm", "report", "shards"];

#[test]
fn every_boot_door_names_the_same_axes() {
    let react = axes(typescript_type_fields(&read("🧱️elements/🐚️Shell/🟦️.tsx"), "FrameworkOsBootOptions"));
    let descriptor_ts = axes(typescript_type_fields(&read("🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts"), "WgpuBootDescriptor"));
    let library = axes(typescript_type_fields(&read("🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts"), "FrameworkOsWgpuBootOptions"));
    let native = axes(native_arg_flags(&read("🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs")).into_iter().filter(|flag| !NATIVE_NON_AXIS_FLAGS.contains(&flag.as_str())).collect::<Vec<_>>());

    // 🌐️ The hub trio is ONE field on the descriptor and three flags on the CLI; naming the two
    // members here keeps the union honest without pretending the descriptor forgot them.
    let mut descriptor_axes = descriptor_ts.clone();
    if descriptor_axes.contains("hub") {
        descriptor_axes.extend(["hubUser", "hubDataDir"]);
    }
    let mut library_axes = library.clone();
    if library_axes.contains("hub") {
        library_axes.extend(["hubUser", "hubDataDir"]);
    }

    let union: BTreeSet<&'static str> = react.iter().chain(descriptor_axes.iter()).chain(library_axes.iter()).chain(native.iter()).copied().collect();
    for (door, present) in [("react", &react), ("descriptor", &descriptor_axes), ("wgpu-library", &library_axes), ("native", &native)] {
        let allowed = allowed_absences(door);
        let missing: Vec<&str> = union.iter().filter(|axis| !present.contains(*axis) && !allowed.contains(*axis)).copied().collect();
        assert!(missing.is_empty(), "boot door `{door}` does not carry {missing:?} — wire it, or add it to `allowed_absences` with the reason it cannot exist there");
        let stale: Vec<&str> = allowed.iter().filter(|axis| present.contains(*axis)).copied().collect();
        assert!(stale.is_empty(), "boot door `{door}` now carries {stale:?}, which `allowed_absences` still excuses — delete the stale allowlist entries");
    }
}

/// 🧭️ The TypeScript descriptor and its Rust twin are ONE shape: a field added to one and not the
/// other would deserialize as a silent default on the renderer side, which is exactly the class of
/// drift this packet closed.
#[test]
fn the_descriptor_rust_twin_matches_the_typescript_shape() {
    let typescript: BTreeSet<String> = typescript_type_fields(&read("🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts"), "WgpuBootDescriptor").into_iter().collect();
    let renderer = read("🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
    let rust: BTreeSet<String> = rust_struct_fields(&renderer, "WgpuBootDescriptor").into_iter().collect();
    assert_eq!(typescript, rust, "`WgpuBootDescriptor` must carry the same fields in 🧭️boot-descriptor/🟦️.ts and 🧊️renderer/🦀️.rs");
    for nested in ["WgpuBootLocks", "WgpuBootDefaults", "WgpuBootHub", "WgpuBootBrand"] {
        let typescript: BTreeSet<String> = typescript_type_fields(&read("🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts"), nested).into_iter().collect();
        let rust: BTreeSet<String> = rust_struct_fields(&renderer, nested).into_iter().collect();
        assert_eq!(typescript, rust, "`{nested}` must carry the same fields on both sides");
    }
}

/// 📏️ The three doors refuse the same oversized field. A cap that drifts is a url one renderer
/// accepts and the other rejects.
#[test]
fn every_door_bounds_a_field_at_the_same_capacity() {
    let descriptor_ts = read("🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts");
    assert!(descriptor_ts.contains("export const WGPU_BOOT_FIELD_CAPACITY = 2048;"), "the TypeScript field cap is 2048");
    assert!(descriptor_ts.contains("export const WGPU_BOOT_LOCATION_CAPACITY = 8192;"), "the TypeScript location cap is 8192");
    assert!(read("🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs").contains("pub const WGPU_BOOT_FIELD_CAPACITY: usize = 2048;"), "the Rust field cap is the same 2048");
    assert!(read("../../🧑‍💻dev/🔗️boot-query/🟦️.ts").contains("export const BOOT_QUERY_CAPACITY = 8192;"), "React's own query cap is the same 8192");
}
