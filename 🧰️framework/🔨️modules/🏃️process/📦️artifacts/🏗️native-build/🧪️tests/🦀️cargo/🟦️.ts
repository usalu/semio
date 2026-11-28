import {registeredToolExecutableV1} from "../../../../🧰️tools/🏃️run/🟦️.ts";
import { phaseCompilerAdmissionControlV1, type CompilerAdmissionControl } from "../../../../🏗️build/🎟️capacity/🟦️.ts";
import { captureOwnedCommandV1 } from "../../../../🎛️owned-execution/🟦️.ts";
import { expect } from "bun:test";
import { withBuildTestInvocationV1 } from "../../../../🧪️testing/📥️invocation/🟦️.ts";
import { spawn, spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { checkResourceLeaseControl } from "../../../../🔒️leases/🟦️.ts";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { startAdmissionBrokerV1, type UnitRecordV1 } from "../../🎟️admission/🟦️.ts";
import { verifyCapturedCargoArtifactsV1, type CargoArtifactBuildOptionsV1 } from "../../🟦️.ts";
import { CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1 } from "../../🧾️receipt/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../🏗️build/🎟️capacity/🟦️.ts";
import { createIssuedResourceControlV1, ResourceLoanDeadlineExhaustedV1, withScriptInvocationSignalV1, type IssuedScriptInvocationV1 } from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { createJobserverV1, defaultJobserverStyleV1 } from "../../🎰️jobserver/🟦️.ts";
import type { BuildScriptRunV1 } from "../../📈️telemetry/🟦️.ts";
import { acquireNativeStorageV1, nativeStorageRetirementPortV1, type NativeStorageReservationV1 } from "../../💾️storage/🟦️.ts";
import { withNativePlatformImageStorageV1 } from "../../💾️storage/🌐️platform/🟦️.ts";
import { withRepositoryCompilerRuntimeEntriesV1 } from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🟦️.ts";
import type { ScriptInvocation } from "../../../../🧭️routing/📥️invocation/🟦️.ts";
import type { NativeFixedPlatformOptionsV1 } from "../../💾️storage/🔌️port/🟦️.ts";
import type { NativeImageStorageV1 } from "../../💾️storage/🌐️platform/📋️contract/🟦️.ts";
import type { CargoSourceBoundaryPolicyV1 } from "../../📥️source/🎟️admission/🟦️.ts";
import policy from "./🧫️fixtures/🔣️.json" with { type: "json" };

const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!), lawWorkspaceV1 = resolve(import.meta.dirname, "../../../../../../..");

/** 🧾️ Writes the issued-free compiler configuration (central profile tables only, no compiler ingress) a Cargo-backed law compiles under. */
export function lawCompilerConfigurationV1(directory: string): string { const path = join(directory, "compiler.toml"); writeFileSync(path, policy.compilerConfiguration); return path; }

/** 📏️ The Source boundary every Cargo-backed law admits under: the repository's configured ceiling, kept as a language-neutral fixture. */
export const lawSourceBoundaryV1: CargoSourceBoundaryPolicyV1 = policy.sourceBoundary as CargoSourceBoundaryPolicyV1;

/** 💾️ The bounded compiler backing one Cargo-backed law holds: an original compiler-workspace reservation on a fresh fixed image, its `owned` directory and the
 * Cargo home and temporary directories every bounded child must use. */
export type LawStorageV1 = Readonly<{ storage: NativeStorageReservationV1; root: string; environment: Readonly<Record<string, string>> }>;

/** 🌐️ Runs the body inside the backing this host enforces through the product's scoped platform storage; off macOS the directory partition's helper needs the repository's captured runtime entry provider. */
async function withPlatformLawImageV1<T>(options:NativeFixedPlatformOptionsV1,invocation:ScriptInvocation|undefined,body:(image:NativeImageStorageV1,control:CompilerAdmissionControl)=>Promise<T>):Promise<T>{
 const run=():Promise<T>=>withNativePlatformImageStorageV1(options,(image,control)=>body(image,control as CompilerAdmissionControl));
 if(!invocation||process.platform==="darwin")return run();
 const settled=await withRepositoryCompilerRuntimeEntriesV1(lawWorkspaceV1,invocation,options.control,()=>run().then(value=>({value}),(error:unknown)=>({error})));
 if("error"in settled)throw settled.error;return settled.value;
}

/** 🗄️ Retains the real scoped Native body and its allocation until accepted work and terminal cleanup settle. */
export function withLawStorageV1<T>(control:CompilerAdmissionControl,filesystemBytes:number,body:(held:LawStorageV1,control:CompilerAdmissionControl)=>Promise<T>,invocation?:ScriptInvocation):Promise<T>{
 const directory=join(mkdtempSync(join(artifacts,"i-")),"i");return withPlatformLawImageV1({directory,leaseDirectory:artifacts,filesystemBytes,maximumPhysicalBytes:filesystemBytes+policy.imageSlackBytes,control},invocation,async(image,control)=>{const root=join(image.root,"o");let storage:NativeStorageReservationV1|undefined,primary:{error:unknown}|undefined;try{storage=await acquireNativeStorageV1({policy:{version:1,root:image.root,leaseDirectory:artifacts,capacityBytes:image.maximumPhysicalBytes,maximumWorkspaceBytes:filesystemBytes,maximumArtifactBytes:filesystemBytes},control,port:nativeStorageRetirementPortV1(image.port,async()=>rmSync(root,{recursive:true,force:true})),owner:"law:"+basename(resolve(directory,"..")),kind:"compiler-workspace",bytes:filesystemBytes});const home=join(root,"h"),temporary=join(root,"tmp");for(const path of[root,home,temporary])mkdirSync(path,{recursive:true});return await body({storage,root,environment:{CARGO_HOME:home,TMPDIR:temporary,TMP:temporary,TEMP:temporary}},control);}catch(error){primary={error};throw error;}finally{if(storage)try{await storage.retire();}catch(error){if(!primary)throw error;if(!control.signal.aborted&&!(error instanceof ResourceLoanDeadlineExhaustedV1))throw new AggregateError([primary.error,error],"Law body and storage retirement failed");}}});
}

/** ⚖️ Retains the real scoped Native body inside a compiler reservation derived from an issued invocation and the case's own cancellation, so the build, relay and storage laws share one original authority. */
export function withIssuedLawStorageV1<T>(original:IssuedScriptInvocationV1,signal:AbortSignal,body:(held:LawStorageV1,control:CompilerAdmissionControl,invocation:IssuedScriptInvocationV1)=>Promise<T>):Promise<T>{
 return withScriptInvocationSignalV1(original,signal,invocation=>withLawStorageV1(scriptCompilerAdmissionControlV1(invocation,"law:storage",120000),lawStoragePolicyV1.realImageBytes,(held,control)=>body(held,control,invocation),invocation));
}

/** 🏗️ Completes one build's options with the reservation, Source boundary and finite completed consumer every build requires; the ticket mirror stays unset unless the case names it. */
export function lawBuildOptionsV1(held:LawStorageV1,control:CompilerAdmissionControl,extra:Partial<CargoArtifactBuildOptionsV1>={}):CargoArtifactBuildOptionsV1{
 return{control,storage:held.storage,sourceBoundary:lawSourceBoundaryV1,maximumObservationBytes:CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1,consume:verifyCapturedCargoArtifactsV1,...extra,environment:{...process.env,...held.environment,SEMIO_TEST_ARTIFACT_DIR:undefined,...extra.environment}};
}

/** 📐️ The fixed image sizes, the slack each law's image envelope admits and the storage reserve a law's small image keeps free, kept with the Source
 * boundary in the kit's fixture. */
export const lawStoragePolicyV1 = Object.freeze({ stubImageBytes: policy.stubImageBytes, realImageBytes: policy.realImageBytes, reserveBytes: policy.lawReserveBytes });

/** 🛠️ The scheduler binaries every Cargo-backed law shares, compiled once per test process through the repository's canonical issued Cargo entry
 * (`repo cargo --store=shared` in the build mode of the Original, a launch the suite row declares; the launcher `runRepositoryCargoCommandV1` delegates to), never by a direct `cargo`, and copied out of the shared store's
 * target to the artifact directory.
 * @see ../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts */
export const schedulerBinaries = (): Promise<ReadonlyMap<string, string>> => (globalThis as any).__semioScheduler ??= withBuildTestInvocationV1(async original => {
  const control = createIssuedResourceControlV1(original, "law:scheduler-binaries", policy.schedulerBuildMs, AbortSignal.timeout(policy.schedulerBuildMs)), output = mkdtempSync(join(artifacts, "sx-"));
  const manifest = fileURLToPath(new URL("../../🎟️admission/📦️packages/🦀️rust/Cargo.toml", import.meta.url)), declared = Bun.TOML.parse(readFileSync(manifest, "utf8")) as { bin: { name: string }[]; example: { name: string }[] };
  let repository = dirname(manifest);
  while (!existsSync(join(repository, "package.json")) || !(JSON.parse(readFileSync(join(repository, "package.json"), "utf8")).scripts?.repo)) { if (dirname(repository) === repository) throw Error("Cargo laws require the repository that issues their compiler store"); repository = dirname(repository); }
  const mode = process.env.SEMIO_BUILD_MODE === "release" ? "release" : "prototype", issued = (args: readonly string[]) => captureOwnedCommandV1("repo", ["cargo", "--store=shared", "--configuration=" + mode, "--", ...args, "--manifest-path", manifest], repository, { control, env: { ...process.env, SEMIO_BUILD_MODE: mode }, output: { maximumBytes: 16777216, maximumLines: 65536 } });
  const built = await issued(["build", "--bins", "--examples", "--message-format=json"]);
  const executables = new Map<string, string>();
  for (const line of built.split("\n")) {
    if (!line.startsWith('{"')) continue;
    const message = JSON.parse(line);
    if (message.reason === "compiler-artifact" && typeof message.executable === "string") {
      executables.set(message.target.name, message.executable);
    }
  }
  return new Map([...declared.bin, ...declared.example].map(({name}) => {
    const executable = executables.get(name);
    if (!executable) throw Error(`Cargo did not publish the scheduler test executable ${name}`);
    const copy = join(output, basename(executable));
    copyFileSync(executable, copy);
    return [name, copy] as const;
  }));
});

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
  expect(spawnSync(registeredToolExecutableV1("cargo"), ["generate-lockfile", "--offline", "--manifest-path", join(root, "Cargo.toml")], { encoding: "utf8" }).status).toBe(0);
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
    let stderr = "";
    const { RUSTC: _compiler, RUSTC_WRAPPER: _wrapper, RUSTC_WORKSPACE_WRAPPER: _workspaceWrapper, ...inherited } = process.env;
    const exit = await pool.withClient(async environment => {
      const child = spawn(registeredToolExecutableV1("cargo"), ["build", "--offline", "--manifest-path", options.manifest, "--workspace", ...(options.extraArgs ?? [])], { cwd: dirname(options.manifest), env: { ...inherited, ...environment, RUSTC_WRAPPER: scheduler, SEMIO_BUILD_BROKER: broker.endpoint, SEMIO_BUILD_PASS: "law", CARGO_TARGET_DIR: join(directory, "t"), CARGO_BUILD_BUILD_DIR: join(directory, "b"), CARGO_TERM_COLOR: "never" }, stdio: ["ignore", "ignore", "pipe"] });
      child.stderr!.on("data", (chunk) => { stderr += chunk; });
      return new Promise<number>((accept) => child.once("close", (code) => accept(code ?? 1)));
    });
    slot.release();
    if (exit !== 0) console.error(stderr);
    return { units: broker.units(), runs: [], exit, peakRunning, targetDirectory: join(directory, "t") };
  } finally { clearInterval(sampler); await broker.close(); pool.close(); }
}

