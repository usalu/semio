import json, subprocess, sys, os
here = os.path.dirname(os.path.abspath(__file__))
R = "a1" * 32
registry = "/home/a/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.229"
base = ["--edition=2021", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "incremental=/x/incr"]
vectors = [
    {"name": "registry library, dev profile", "cwd": registry, "environment": {"CARGO_MANIFEST_DIR": registry, "CARGO_PKG_NAME": "serde", "CARGO_PKG_VERSION": "1.0.229"},
     "arguments": ["--crate-name", "serde", "src/lib.rs", "--crate-type", "lib", "--cfg", "feature=\"alloc\"", "--cfg", "feature=\"derive\"", "--cfg", "feature=\"std\"", "--cfg", "no_core_cstr", *base[:1], *base[1:]]},
    {"name": "workspace member, optimised, rebuilt with another feature order", "cwd": "/w/ws", "environment": {"CARGO_MANIFEST_DIR": "/w/ws/crates/core", "CARGO_PKG_NAME": "core", "CARGO_PKG_VERSION": "0.1.0"},
     "arguments": ["--crate-name", "core", "crates/core/src/lib.rs", "--crate-type", "lib", "--cfg", "feature=\"b\"", "--cfg", "feature=\"a\"", "-C", "opt-level=3", "-C", "overflow-checks=off", "-C", "panic=abort", "-C", "codegen-units=1", "-C", "lto=thin", "--emit=dep-info,metadata,link"]},
    {"name": "proc macro for the host", "cwd": registry, "environment": {"CARGO_MANIFEST_DIR": registry, "CARGO_PKG_NAME": "serde_derive", "CARGO_PKG_VERSION": "1.0.229"},
     "arguments": ["--crate-name", "serde_derive", "src/lib.rs", "--crate-type", "proc-macro", "--emit=dep-info,link", "-Copt-level=0", "-Cdebug-assertions=on"]},
    {"name": "wasm component binary with test cfg", "cwd": "/w/ws", "environment": {"CARGO_MANIFEST_DIR": "/elsewhere/pkg", "CARGO_PKG_NAME": "plug", "CARGO_PKG_VERSION": "0.2.0"},
     "arguments": ["--crate-name", "plug", "p/src/main.rs", "--crate-type", "bin", "--target", "wasm32-wasip2", "--cfg", "test", "--emit=link", "-C", "opt-level=s"]},
]
for v in vectors:
    v["rustc"] = R
    v["host"] = "aarch64-apple-darwin"
sys.path.insert(0, os.path.join(here, "..", "🐍️oracle"))
import importlib.util
spec = importlib.util.spec_from_file_location("oracle", os.path.join(here, "..", "🐍️oracle", "🐍️.py"))
oracle = importlib.util.module_from_spec(spec); spec.loader.exec_module(oracle)
for v in vectors:
    parts = oracle.parts_of(v); v["parts"] = parts; v["slot"] = oracle.slot_of(parts)
json.dump({"version": 1, "slots": vectors}, open(os.path.join(here, "🔣️.json"), "w", encoding="utf8"), ensure_ascii=False, indent=2)
print("written", len(vectors))
