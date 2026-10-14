import { phaseCompilerAdmissionControlV1, type CompilerAdmissionControl } from "../../../../🏗️build/🎟️capacity/🟦️.ts";
import { captureOwnedCommandV1 } from "../../../../🎛️owned-execution/🟦️.ts";
import { expect } from "bun:test";
import { spawn, spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { checkResourceLeaseControl } from "../../../../🔒️leases/🟦️.ts";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { startAdmissionBrokerV1, type UnitRecordV1 } from "../../🎟️admission/🟦️.ts";
import { createJobserverV1, defaultJobserverStyleV1 } from "../../🎰️jobserver/🟦️.ts";
import type { BuildScriptRunV1 } from "../../📈️telemetry/🟦️.ts";
import { acquireNativeStorageV1, nativeStorageRetirementPortV1, type NativeStorageReservationV1 } from "../../💾️storage/🟦️.ts";
import { provisionNativeImageStorageV1 } from "../../💾️storage/🍎️image/🟦️.ts";
import type { CargoSourceBoundaryPolicyV1 } from "../../📥️source/🎟️admission/🟦️.ts";
import policy from "./🧫️fixtures/🔣️.json" with { type: "json" };

const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);

/** 📏️ The Source boundary every Cargo-backed law admits under: the repository's configured ceiling, kept as a language-neutral fixture. */
export const lawSourceBoundaryV1: CargoSourceBoundaryPolicyV1 = policy.sourceBoundary as CargoSourceBoundaryPolicyV1;

/** 💾️ The bounded compiler backing one Cargo-backed law holds: an original compiler-workspace reservation on a fresh fixed image, its `owned` directory and the
 * Cargo home and temporary directories every bounded child must use. */
export type LawStorageV1 = Readonly<{ storage: NativeStorageReservationV1; root: string; environment: Readonly<Record<string, string>> }>;

/** 🗄️ Runs `body` inside one original compiler reservation bound to `control`, provisioned through the canonical fixed-image owner and retired afterwards: the
 * reservation is released, the image detached and its backing file removed, whatever `body` did; a law that cancelled `control` can no longer prove the
 * reservation (cancelled or out of time), so its image is force-detached.
 * @see ../../💾️storage/🍎️image/🟦️.ts */
export async function withLawStorageV1<T>(control: CompilerAdmissionControl, filesystemBytes: number, body: (held: LawStorageV1) => Promise<T>): Promise<T> {
  const directory = join(mkdtempSync(join(artifacts, "i-")), "i"), image = await provisionNativeImageStorageV1({ directory, filesystemBytes, maximumPhysicalBytes: filesystemBytes + policy.imageSlackBytes, control }), root = join(image.root, "o");
  let storage: NativeStorageReservationV1 | undefined;
  try {
    storage = await acquireNativeStorageV1({ policy: { version: 1, root: image.root, leaseDirectory: artifacts, capacityBytes: image.maximumPhysicalBytes, maximumWorkspaceBytes: filesystemBytes, maximumArtifactBytes: filesystemBytes }, control, port: nativeStorageRetirementPortV1(image.port, async () => rmSync(root, { recursive: true, force: true })), owner: "law:" + basename(resolve(directory, "..")), kind: "compiler-workspace", bytes: filesystemBytes });
    const home = join(root, "h"), temporary = join(root, "tmp");
    for (const path of [root, home, temporary]) mkdirSync(path, { recursive: true });
    return await body({ storage, root, environment: { CARGO_HOME: home, TMPDIR: temporary, TMP: temporary, TEMP: temporary } });
  } finally {
    const live = (): boolean => { try { checkResourceLeaseControl(control); return true; } catch { return false; } };
    try { if (live()) { await storage?.retire(); await image.close(); } }
    finally {
      if (!live()) for (let attempt = 0; attempt < policy.detachAttempts && spawnSync("hdiutil", ["detach", "-force", image.root], { encoding: "utf8" }).status !== 0; attempt++) await Bun.sleep(policy.detachRetryMs);
      if (spawnSync("mount", [], { encoding: "utf8" }).stdout.includes(` on ${image.root} (`)) console.error(`[law-storage] fixed image still mounted after force detach: ${image.root}`);
      else rmSync(resolve(directory, ".."), { recursive: true, force: true });
    }
  }
}

/** 📐️ The fixed image sizes, the slack each law's image envelope admits and the storage reserve a law's small image keeps free, kept with the Source
 * boundary in the kit's fixture. */
export const lawStoragePolicyV1 = Object.freeze({ stubImageBytes: policy.stubImageBytes, realImageBytes: policy.realImageBytes, reserveBytes: policy.lawReserveBytes });

/** 🛠️ The scheduler binaries every Cargo-backed law shares, compiled once per test process through the repository's canonical issued Cargo entry
 * (`bun repo cargo --store=shared`, the launcher `runRepositoryCargoCommandV1` delegates to), never by a direct `cargo`, and copied out of the shared store's
 * release target to the artifact directory.
 * @see ../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts */
export const schedulerBinaries = (): Promise<ReadonlyMap<string, string>> => (globalThis as any).__semioScheduler ??= (async () => {
  const control = phaseCompilerAdmissionControlV1(AbortSignal.timeout(policy.schedulerBuildMs), policy.schedulerBuildMs, () => undefined, () => new Promise(done => setImmediate(done))), output = mkdtempSync(join(artifacts, "sx-"));
  const manifest = fileURLToPath(new URL("../../🎟️admission/📦️packages/🦀️rust/Cargo.toml", import.meta.url)), declared = Bun.TOML.parse(readFileSync(manifest, "utf8")) as { bin: { name: string }[]; example: { name: string }[] };
  let repository = dirname(manifest);
  while (!existsSync(join(repository, "package.json")) || !(JSON.parse(readFileSync(join(repository, "package.json"), "utf8")).scripts?.repo)) { if (dirname(repository) === repository) throw Error("Cargo laws require the repository that issues their compiler store"); repository = dirname(repository); }
  const issued = (args: readonly string[]) => captureOwnedCommandV1(process.execPath, ["repo", "cargo", "--store=shared", "--configuration=release", "--", ...args, "--manifest-path", manifest], repository, { control, env: { ...process.env, SEMIO_BUILD_MODE: "release" }, output: { maximumBytes: 16777216, maximumLines: 65536 } });
  await issued(["build", "--bins", "--examples"]);
  const metadata = (await issued(["metadata", "--format-version", "1", "--no-deps"])).split("\n").find((line) => line.startsWith("{\"")), release = join(JSON.parse(metadata ?? "null").target_directory, "release");
  return new Map([...declared.bin.map((row) => [row.name, join(release, row.name)] as const), ...declared.example.map((row) => [row.name, join(release, "examples", row.name)] as const)].map(([name, path]) => { const copy = join(output, name); copyFileSync(path, copy); return [name, copy] as const; }));
})();

/** 🦀️ A dependency-free Cargo workspace: `base` ← `left`, `right` ← `top` (rlibs), so pipelining, fan-in and a critical chain are observable in seconds; `right` is ten times `left`, so the chain through it is unambiguous. */
export function tinyCargoWorkspaceV1(root: string, crates: readonly string[]): string {
  mkdirSync(root, { recursive: true });
  const dependencies: Record<string, readonly string[]> = { base: [], left: ["base"], right: ["base"], top: ["left", "right"] };
  writeFileSync(join(root, "Cargo.toml"), `[workspace]\nmembers = ${JSON.stringify(crates)}\nresolver = "2"\n\n[profile.wasm-release]\ninherits = "release"\n\n[profile.wasm-prototype]\ninherits = "dev"\n`);
  for (const name of crates) {
    mkdirSync(join(root, name, "src"), { recursive: true });
    writeFileSync(join(root, name, "Cargo.toml"), `[package]\nname = "${name}"\nversion = "0.1.0"\nedition = "2021"\n\n[dependencies]\n${(dependencies[name] ?? []).map((dependency) => `${dependency} = { path = "../${dependency}" }`).join("\n")}\n`);
    const body = Array.from({ length: ({ base: 150, left: 40, right: 400, top: 80 } as Record<string, number>)[name] ?? 120 }, (_, index) => `pub fn ${name}_${index}<T: Clone + core::fmt::Debug>(value: T) -> (T, String) { let text = format!("{:?}{}", value.clone(), ${index}); (value, text.repeat(${(index % 7) + 1})) }`).join("\n");
    writeFileSync(join(root, name, "src", "lib.rs"), `${(dependencies[name] ?? []).map((dependency) => `pub use ${dependency};`).join("\n")}\n${body}\n`);
  }
  expect(spawnSync("cargo", ["generate-lockfile", "--offline", "--manifest-path", join(root, "Cargo.toml")], { encoding: "utf8" }).status).toBe(0);
  return join(root, "Cargo.toml");
}

/** 🏃️ Builds the tiny workspace through the real wrapper, broker and jobserver; returns the broker's unit records and Cargo's exit status. */
export async function buildThroughBrokerV1(options: Readonly<{ manifest: string; tokens: number; physicalBytes: number; extraArgs?: readonly string[] }>): Promise<{ readonly units: readonly UnitRecordV1[]; readonly runs: readonly BuildScriptRunV1[]; readonly exit: number; readonly peakRunning: number; readonly targetDirectory: string }> {
  const scheduler = (await schedulerBinaries()).get("semio-build-scheduler")!, directory = mkdtempSync(join(artifacts, "c-"));
  const pool = await createJobserverV1({ version: 1, tokens: options.tokens, style: defaultJobserverStyleV1() }, join(directory, "j"));
  const broker = await startAdmissionBrokerV1({ physicalBytes: options.physicalBytes, history: new Map() });
  broker.registerPass({ id: "law", profile: "dev", target: "host" });
  let peakRunning = 0;
  const sampler = setInterval(() => { peakRunning = Math.max(peakRunning, broker.stats().running); }, 5);
  try {
    const slot = await pool.acquire();
    const child = spawn("cargo", ["build", "--offline", "--manifest-path", options.manifest, "--workspace", ...(options.extraArgs ?? [])], { env: { ...process.env, ...pool.environment(), RUSTC_WRAPPER: scheduler, SEMIO_BUILD_BROKER: broker.endpoint, SEMIO_BUILD_PASS: "law", CARGO_TARGET_DIR: join(directory, "t"), CARGO_BUILD_BUILD_DIR: join(directory, "b"), CARGO_TERM_COLOR: "never" }, stdio: ["ignore", "ignore", "pipe"] });
    let stderr = "";
    child.stderr!.on("data", (chunk) => { stderr += chunk; });
    const exit = await new Promise<number>((accept) => child.once("close", (code) => accept(code ?? 1)));
    slot.release();
    if (exit !== 0) console.error(stderr);
    return { units: broker.units(), runs: [], exit, peakRunning, targetDirectory: join(directory, "t") };
  } finally { clearInterval(sampler); await broker.close(); pool.close(); }
}

