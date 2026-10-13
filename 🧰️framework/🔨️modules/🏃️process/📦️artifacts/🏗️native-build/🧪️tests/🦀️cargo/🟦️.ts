import { expect } from "bun:test";
import { spawn, spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { buildSchedulerV1, startAdmissionBrokerV1, type UnitRecordV1 } from "../../🎟️admission/🟦️.ts";
import { createJobserverV1, defaultJobserverStyleV1 } from "../../🎰️jobserver/🟦️.ts";
import type { BuildScriptRunV1 } from "../../📈️telemetry/🟦️.ts";

const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);

/** 🛠️ The scheduler binary every Cargo-backed law shares, compiled once per test process into the artifact directory. */
export const schedulerBinaries = (): Promise<ReadonlyMap<string, string>> => (globalThis as any).__semioScheduler ??= buildSchedulerV1({ cwd: process.cwd(), targetDirectory: join(artifacts, "st"), buildDirectory: join(artifacts, "sb"), environment: process.env, examples: true, signal: AbortSignal.timeout(600_000) });

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

