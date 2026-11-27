/** 📸️ Contract: the real native-build driver compiles the checkout snapshot it is rooted at. A live source rewritten while the Cargo child runs changes neither the linked component nor the sources the build consumed, and the driver still stages every component. */
import { registeredToolExecutableV1 } from "../../../../../🧰️tools/🏃️run/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../🏗️build/🎟️capacity/🟦️.ts";
import { registerIsolatedBuildTestV1, withBuildTestInvocationV1 } from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import { captureCheckoutSnapshotV1, verifyCheckoutSnapshotV1 } from "../../../../../🏗️build/📸️checkout-snapshot/🟦️.ts";
import { expect } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { lawSourceBoundaryV1, lawStoragePolicyV1, schedulerBinaries, withLawStorageV1 } from "../../../🧪️tests/🦀️cargo/🟦️.ts";
import { nativeBuildComponentStagePathV1, runNativeBuildDriverV1 } from "../../🟦️.ts";
import { type NativeBuildPlanV1 } from "../../📋️contract/🟦️.ts";
import { artifacts } from "../🎭️stub/🟦️.ts";

registerIsolatedBuildTestV1(import.meta.path, "the real driver builds the snapshot generation although the live sources are rewritten while Cargo runs", { bodyMilliseconds: 600_000, scope: { kind: "body" } }, async () => {
  const root = mkdtempSync(join(artifacts, "s-")), workspace = join(root, "w"), component = fixture.real.components[0]!, profile = "release";
  mkdirSync(join(workspace, "base", "src"), { recursive: true });
  mkdirSync(join(workspace, component, "src"), { recursive: true });
  writeFileSync(join(workspace, "Cargo.toml"), `[workspace]\nmembers = ["base", "${component}"]\nresolver = "2"\n\n[profile.release]\nopt-level = "s"\n`);
  writeFileSync(join(workspace, "base", "Cargo.toml"), `[package]\nname = "base"\nversion = "0.1.0"\nedition = "2021"\nbuild = "build.rs"\n`);
  writeFileSync(join(workspace, "base", "build.rs"), `fn main(){let d=std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());std::fs::write(d.join("started"),"1").unwrap();for _ in 0..2400{if d.join("release").exists(){break;}std::thread::sleep(std::time::Duration::from_millis(50));}}\n`);
  writeFileSync(join(workspace, "base", "src", "lib.rs"), `pub static TAG:&str="snapshot-original-marker";\n`);
  writeFileSync(join(workspace, component, "Cargo.toml"), `[package]\nname = "${component}"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\ncrate-type = ["cdylib"]\n\n[dependencies]\nbase = { path = "../base" }\n`);
  writeFileSync(join(workspace, component, "src", "lib.rs"), `#[unsafe(no_mangle)]\npub extern "C" fn run()->usize{ base::TAG.as_ptr() as usize + base::TAG.len() }\n`);
  expect(spawnSync(registeredToolExecutableV1("cargo"), ["generate-lockfile", "--offline", "--manifest-path", join(workspace, "Cargo.toml")], { encoding: "utf8" }).status).toBe(0);
  const snapshot = await captureCheckoutSnapshotV1({ workspace, state: join(root, "st"), environment: { SEMIO_LANGUAGE: "en" } });
  const plan: NativeBuildPlanV1 = { version: 1, passes: [{ id: `${profile}-00`, manifest: join(snapshot.root, "Cargo.toml"), packages: [{ name: component, manifest: join(snapshot.root, component, "Cargo.toml") }], target: "wasm32-wasip2", profile }] };
  const scheduler = (await schedulerBinaries()).get("semio-build-scheduler")!;
  await withBuildTestInvocationV1(original => withLawStorageV1(scriptCompilerAdmissionControlV1(original, "test:component-pass-snapshot", 600_000), lawStoragePolicyV1.realImageBytes, async (held, control) => {
    const building = runNativeBuildDriverV1({ plan, storage: held.storage, sourceBoundary: lawSourceBoundaryV1, cwd: snapshot.root, originCwd: workspace, cacheRoot: join(held.root, "cache"), shared: { target: join(held.root, "t"), build: join(held.root, "b") }, generation: "fresh", leaseDirectory: artifacts, environment: { ...process.env, ...held.environment, CARGO_TARGET_DIR: join(held.root, "t") }, control, tokens: fixture.real.tokens, storageReserveBytes: lawStoragePolicyV1.reserveBytes, scheduler, progressMs: 1000 });
    for (let waited = 0; !existsSync(join(snapshot.root, "base", "started")) && waited < 2400; waited++) await new Promise(resolve => setTimeout(resolve, 100));
    expect(existsSync(join(snapshot.root, "base", "started"))).toBe(true);
    writeFileSync(join(workspace, "base", "src", "lib.rs"), `pub static TAG:&str="live-edited-marker";\n`);
    writeFileSync(join(workspace, component, "src", "lib.rs"), `#[unsafe(no_mangle)]\npub extern "C" fn run()->usize{ 7 }\n`);
    writeFileSync(join(snapshot.root, "base", "release"), "1");
    const outcome = await building;
    expect(outcome.failed).toEqual([]);
    expect(outcome.staged).toHaveLength(1);
    const wasm = readFileSync(nativeBuildComponentStagePathV1(outcome.generation, plan.passes[0]!, plan.passes[0]!.packages[0]!));
    expect(wasm.includes(Buffer.from("snapshot-original-marker"))).toBe(true);
    expect(wasm.includes(Buffer.from("live-edited-marker"))).toBe(false);
    expect((await verifyCheckoutSnapshotV1(snapshot.root)).changed).toEqual([]);
    expect(readFileSync(join(workspace, "base", "src", "lib.rs"), "utf8")).toContain("live-edited-marker");
  }, original));
});
