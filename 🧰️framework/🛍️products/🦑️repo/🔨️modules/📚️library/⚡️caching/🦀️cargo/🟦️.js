"use strict";
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.wasmBindgenVersion = wasmBindgenVersion;
exports.cargoDirectories = cargoDirectories;
exports.cargoTargetDirectory = cargoTargetDirectory;
exports.cargoBuildDirectory = cargoBuildDirectory;
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
/** 🔒️ The binding generator must have the same identity as the locked Rust crate. */
function wasmBindgenVersion(lock) {
    var versions = __spreadArray([], lock.matchAll(/^name = "wasm-bindgen"\r?\nversion = "([^"]+)"$/gm), true).map(function (match) { return match[1]; });
    if (versions.length !== 1)
        throw new Error("Cargo.lock must resolve exactly one wasm-bindgen version");
    return versions[0];
}
var resolved = new Map();
/**
 * 🧭️ Resolves Cargo's effective directories with Cargo's own precedence (environment over the repository
 * `.cargo/config.toml`), so every script reads deliverables exactly where Cargo wrote them.
 * https://doc.rust-lang.org/cargo/reference/config.html#buildtarget-dir
 * https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-dir
 */
function cargoDirectories(repoRoot, env) {
    var _a, _b, _c;
    if (env === void 0) { env = process.env; }
    var overrideTarget = (_a = env.CARGO_TARGET_DIR) !== null && _a !== void 0 ? _a : env.CARGO_BUILD_TARGET_DIR, overrideBuild = env.CARGO_BUILD_BUILD_DIR;
    var key = "".concat((0, node_path_1.resolve)(repoRoot), "\0").concat(overrideTarget !== null && overrideTarget !== void 0 ? overrideTarget : "", "\0").concat(overrideBuild !== null && overrideBuild !== void 0 ? overrideBuild : "");
    var cached = resolved.get(key);
    if (cached)
        return cached;
    var path = (0, node_path_1.join)(repoRoot, ".cargo", "config.toml");
    var build = (_b = ((0, node_fs_1.existsSync)(path) ? Bun.TOML.parse((0, node_fs_1.readFileSync)(path, "utf8")) : {}).build) !== null && _b !== void 0 ? _b : {};
    var target = (0, node_path_1.resolve)(repoRoot, (_c = overrideTarget !== null && overrideTarget !== void 0 ? overrideTarget : build["target-dir"]) !== null && _c !== void 0 ? _c : "target");
    var directories = { target: target, build: overrideBuild ? (0, node_path_1.resolve)(repoRoot, overrideBuild) : build["build-dir"] ? (0, node_path_1.resolve)(repoRoot, build["build-dir"]) : target };
    resolved.set(key, directories);
    return directories;
}
/** 🎯️ Cargo's uplifted deliverable root (`<target>/<triple?>/<profile>/…`). */
function cargoTargetDirectory(repoRoot, env) {
    if (env === void 0) { env = process.env; }
    return cargoDirectories(repoRoot, env).target;
}
/** 🏗️ Cargo's shared intermediate root (`<build>/<triple?>/<profile>/build/<package>/<hash>/…`). */
function cargoBuildDirectory(repoRoot, env) {
    if (env === void 0) { env = process.env; }
    return cargoDirectories(repoRoot, env).build;
}
