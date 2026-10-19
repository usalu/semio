import {readNativeCallerAuthorityV1} from "../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {withInstalledNxFixtureV1} from "../🔒️nx/🟦️.ts";
import {scriptInvocationEnvironment,createRequestedIssuedResourceControlV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {type ResourceLeaseControl} from "@semio-tech/process/leases";
import {captureOwnedCommandResultV1} from "@semio-tech/process/execution";
import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";

/** 🧊️ Restores the production native renderer publisher's outputs with Nx and executes them against Cargo. */
export async function testNativeRendererOutputs(workspace: string, output: string, invocation: ScriptInvocation, phaseControl: ResourceLeaseControl): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🧊️native-renderer-outputs/🔣️.json"), "utf8"));
  const authoredRust = fixture.owner + "/📦️packages/🦀️rust", authoredTypescript = fixture.owner + "/📦️packages/🟦️typescript";
  const project = JSON.parse(readFileSync(join(workspace, authoredTypescript, "📋️project.json"), "utf8"));
  assert.equal(project.name, fixture.project);
  for (const row of fixture.profiles) {
    assert.equal(project.targets[row.target].cache, true);
    assert.deepEqual(project.targets[row.target].outputs.map((path:string)=>path.replaceAll("{options.env.SEMIO_BUILD_MODE}",row.profile)), [`{workspaceRoot}/${authoredRust}/${row.output}`], "Nx must restore the Rust package output consumed by the native runner");
  }
  const mode=readNativeCallerAuthorityV1(invocation.capabilities).original.buildMode,profiles=fixture.profiles.filter((row:{profile:string})=>row.profile===mode);assert.equal(profiles.length,1,"Exactly the genuinely admitted native build mode is exercised");
  await withInstalledNxFixtureV1(workspace,{kind:"products"},invocation,phaseControl,async(root,namespaceEnvironment,_productOutput,actualPhase,toolkitRoot)=>{
  const rust="r",typescript="t";
  const put = (path: string, text: string) => { if(join(root,path).length>256)throw Error("Bounded native fixture Source path required");mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); };
  const targets: Record<string,{executor:string;cache:boolean;parallelism:boolean;inputs:string[];outputs:string[];options:{command:string;cwd:string;env:{SEMIO_BUILD_MODE:string}}}> = {};
  for (const row of profiles) targets[row.profile] = {
    executor: "nx:run-commands", cache: true, parallelism: false,
    inputs: [`{workspaceRoot}/${rust}/Cargo.toml`, `{workspaceRoot}/${rust}/Cargo.lock`, `{workspaceRoot}/${rust}/🦀️.rs`, "{workspaceRoot}/📜️script.ts"],
    outputs: project.targets[row.target].outputs.map((path:string)=>path.replaceAll(authoredRust,rust).replaceAll("{options.env.SEMIO_BUILD_MODE}",row.profile)),
    options: { command: `bun ./📜️script.ts ${row.profile}`, cwd: ".",env:{SEMIO_BUILD_MODE:row.profile} }
  };
  put("package.json", JSON.stringify({ name: "native-renderer-output-fixture", private: true }));
  put(".cargo/config.toml", `[profile.prototype]\ninherits="dev"\n[profile.release]\nopt-level=3\n`);
  put("nx.json", JSON.stringify({ useDaemonProcess: false }));
  const selectedTarget=targets[mode];assert(selectedTarget!==undefined&&typeof selectedTarget==="object");
  put(`${typescript}/project.json`, JSON.stringify({ name: "native-renderer", targets: { "native-build": { ...selectedTarget,outputs:project.targets["native-build"].outputs.map((path:string)=>path.replaceAll(authoredRust,rust)),configurations:Object.fromEntries(profiles.map((row:any)=>[row.profile,{command:`bun ./📜️script.ts ${row.profile}`,env:{SEMIO_BUILD_MODE:row.profile}}])), defaultConfiguration:mode } } }));
  put(".gitignore", "node_modules\n.nx\ndist\noracle\n.runs\n");
  put(`${rust}/Cargo.toml`, `[workspace]\n[package]\nname=${JSON.stringify(fixture.crate)}\nversion="0.0.0"\nedition="2021"\n[features]\nnative-bin=[]\n[[bin]]\nname=${JSON.stringify(fixture.binary)}\npath="🦀️.rs"\nrequired-features=["native-bin"]\n`);
  put(`${rust}/Cargo.lock`, `version=4\n[[package]]\nname=${JSON.stringify(fixture.crate)}\nversion="0.0.0"\n`);
  const source = (stdout: string) => `fn main() { let output=${JSON.stringify(stdout)}; print!("{}",output); let args:Vec<String>=std::env::args().collect(); if args.get(1).map(String::as_str)==Some("--hold") { std::fs::write(&args[2],output).unwrap(); while !std::path::Path::new(&args[3]).exists() { std::thread::sleep(std::time::Duration::from_millis(10)); } } }\n`;
  put(`${rust}/🦀️.rs`, source(fixture.stdout));
  put("📜️script.ts", `import { appendFileSync } from "node:fs";
import { join } from "node:path";
import { buildNativeRenderer } from ${JSON.stringify(join(workspace, fixture.owner, "🏗️compiler/🦀️native/📜️script.ts"))};
import {scriptCompilerAdmissionControlV1} from ${JSON.stringify(join(workspace,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts"))};
import {receiveScriptProcessInvocation} from ${JSON.stringify(join(workspace,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"))};
import {repositoryCargoNetworkV1} from ${JSON.stringify(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts"))};
const root = process.cwd(), profile = process.argv[2];
await receiveScriptProcessInvocation(process.env,original=>buildNativeRenderer(join(root, ${JSON.stringify(rust)}), profile, ${JSON.stringify(workspace)},scriptCompilerAdmissionControlV1(original,"test:native-renderer",120000),repositoryCargoNetworkV1(original,${JSON.stringify(workspace)}),original));
appendFileSync(join(root, ".runs"), profile + "\\n");
`);
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  const env = scriptInvocationEnvironment(invocation,{ ...process.env,...namespaceEnvironment, NX_DAEMON: "false", SEMIO_WORKSPACE_ROOT: workspace, SEMIO_WORKSPACE_ROOT_PATH: workspace, SEMIO_REPO_ROOT: workspace, CARGO_NET_OFFLINE: "true" });
  const execute = async (args: string[]) => {
    const [command,...rest]=args;assert(command);const result=await captureOwnedCommandResultV1(command,rest,root,{control:createRequestedIssuedResourceControlV1(actualPhase,120000),env,output:{maximumBytes:16777216,maximumLines:65536}});
    assert.equal(result.status, 0, result.stdout + result.stderr);assert.equal(result.signal,null);return result.stdout;
  };
  const cli = join(toolkitRoot,"node_modules/nx/dist/bin/nx.js");
  const run = async () => {for(const profile of profiles)await execute(["node",cli,"run","native-renderer:native-build","--configuration="+profile.profile,"--outputStyle=static"]);};
  const runs = () => readFileSync(join(root, ".runs"), "utf8").trim().split("\n").sort();
  const binary = fixture.binary + (process.platform === "win32" ? ".exe" : "");
  const check = async (expected: string) => {
    const oracle = await execute(["cargo", "run", "--locked", "--offline", "--manifest-path", join(root, rust, "Cargo.toml"), "--features", "native-bin", "--bin", fixture.binary, "--target-dir", join(root, "oracle")]);
    assert.equal(oracle, expected);
    for (const row of profiles) {
      assert.equal(await execute([join(root, rust, row.output, binary)]), oracle);
      assert.ok(readdirSync(join(root, rust, row.output)).includes(".semio-artifact.json"));
    }
  };
  await run(); assert.deepEqual(runs(), [mode]); await check(fixture.stdout);
  const bytes = profiles.map((row: { output: string }) => readFileSync(join(root, rust, row.output, binary)));
  await run(); assert.deepEqual(runs(), [mode]);
  for (const row of profiles) rmSync(join(root, rust, row.output), { recursive: true });
  await run(); assert.deepEqual(runs(), [mode]);
  for (const [index, row] of profiles.entries()) assert.deepEqual(readFileSync(join(root, rust, row.output, binary)), bytes[index]);
  await check(fixture.stdout);
  const heldProfile=profiles[0];
  const ready=join(root,"held-ready"),release=join(root,"held-release"),controller=new AbortController();
  const {runNativeBinary}=await import(join(workspace,fixture.owner,"⌨️native-entrypoint/📜️script.ts"));
  const held=runNativeBinary(join(root,rust,heldProfile.output,binary),["--hold",ready,release],env,workspace,invocation,createRequestedIssuedResourceControlV1(actualPhase,0,controller.signal));
  const settled=held.then(()=>({error:undefined}),(error:unknown)=>({error}));
  try {
    const started=Date.now();
    while(!existsSync(ready)){assert.ok(Date.now()-started<fixture.heldRun.readyTimeoutMs,"actual native child did not report readiness");await new Promise(accept=>setTimeout(accept,10));}
    assert.equal(readFileSync(ready,"utf8"),fixture.stdout);
    put(`${rust}/🦀️.rs`, source(fixture.changedStdout));
    await run(); assert.deepEqual(runs(), [mode,mode]); await check(fixture.changedStdout);
    assert.equal(readFileSync(ready,"utf8"),fixture.stdout,"the already running child retains its original image");
  } finally {
    writeFileSync(release,"release\n");
    const timeout=setTimeout(()=>controller.abort(),5000);
    const result=await settled;clearTimeout(timeout);
    assert.equal(result.error,undefined,String(result.error));
  }

  });
}
