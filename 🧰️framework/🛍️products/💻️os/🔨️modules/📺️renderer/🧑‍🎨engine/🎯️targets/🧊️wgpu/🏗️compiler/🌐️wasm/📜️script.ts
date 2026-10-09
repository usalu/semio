import { buildBudgetMs } from "../../../../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { createHash, randomUUID } from "node:crypto";
import {homedir} from "node:os";
import {CurrentPhysicalOwnerV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts";
import { basename, delimiter, dirname, join, relative, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { stageRepositoryArtifacts } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";
import {repositoryCargoPreparationStorageV1, runTool } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts";
import { preparedBinaryen } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts";

import { CARGO_RELAY_SCRIPT } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { cargoDirectories } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { writeCompletedCargoInvocationProvenanceV1 } from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
import { repoCacheDirectory } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

export interface TrunkRendererObservationV1 {
  readonly version:1; readonly kind:"wgpu-trunk"; readonly profile:string; readonly manifest:string; readonly outputDirectory:string; readonly builtAtMs:number; readonly observedAtMs:number;
  readonly owner:{readonly path:string;readonly sha256:string}; readonly configuration:{readonly html:string;readonly toml:string}; readonly toolVersions:{readonly bindgen:string;readonly binaryen:string;readonly trunk:string};
  readonly compiler:{readonly path:string;readonly sha256:string}; readonly raw:{readonly path:string;readonly sha256:string}; readonly outputs:readonly {readonly path:string;readonly sha256:string}[];
}

export type TrunkRendererBuild = { readonly rustPackageRoot: string; readonly workspace: string; readonly profile: string; readonly toolWorkspace?: string; readonly stateRoot?: string; readonly signal?: AbortSignal; readonly environment?:Readonly<Record<string,string|undefined>> };

/** 🎯️ Gives finite compiler results one owner per optimization profile. */
export function wasmRendererDirectory(root: string, profile: string): string {
  if (profile !== "dev" && profile !== "release") throw new Error("Select the wasm or wasm-release Nx target");
  return join(root, "dist", `wasm-${profile}`);
}

/** 🏗️ Compiles only Rust renderer bytes; browser composition consumes its completed artifacts separately. */
export async function buildTrunkRenderer(options: TrunkRendererBuild): Promise<void> {
  const builtAtMs=Date.now(),hash=(path:string):string=>createHash("sha256").update(readFileSync(path)).digest("hex");
  const { rustPackageRoot: root, workspace, profile } = options, output = wasmRendererDirectory(root, profile), toolWorkspace = options.toolWorkspace ?? workspace;
  const budget = buildBudgetMs(), signal = AbortSignal.any([options.signal ?? new AbortController().signal, ...(budget > 0 ? [AbortSignal.timeout(budget)] : [])]);
  const manifest = Bun.TOML.parse(readFileSync(join(root, "Cargo.toml"), "utf8")) as { package: { name: string } };
  const lock = Bun.TOML.parse(readFileSync(join(toolWorkspace, "Cargo.lock"), "utf8")) as { package: { name: string; version: string }[] };
  const bindgen = [...new Set(lock.package.filter(row => row.name === "wasm-bindgen").map(row => row.version))];
  if (bindgen.length !== 1) throw new Error("Renderer tooling requires one locked wasm-bindgen version");
  const optimizer = preparedBinaryen(toolWorkspace), environment = { ...(options.environment??process.env) };
  const compilerDirectories = cargoDirectories(workspace, environment);
  environment.CARGO_BUILD_BUILD_DIR = compilerDirectories.build;
  environment.SEMIO_COMPILER_RESOURCE_ROOT = join(compilerDirectories.build, "semio-compiler-resources");
  mkdirSync(environment.SEMIO_COMPILER_RESOURCE_ROOT, { recursive: true });
  for (const key of Object.keys(environment)) if (key.toUpperCase().startsWith("TRUNK_") || ["NO_COLOR", "FORCE_COLOR"].includes(key.toUpperCase())) delete environment[key];
  const pathKey = Object.keys(environment).find(key => key.toUpperCase() === "PATH") ?? "PATH";
  environment[pathKey] = `${dirname(optimizer)}${delimiter}${environment[pathKey] ?? ""}`;
  const version = await runTool("wasm-bindgen", ["--version"], root, signal, true, environment,repositoryCargoPreparationStorageV1(process.cwd()));
  if (version.trim() !== `wasm-bindgen ${bindgen[0]}`) throw new Error(`Prepared wasm-bindgen must match Cargo.lock: ${bindgen[0]}`);
  const binaryenVersion=(await runTool(optimizer,["--version"],root,signal,true,environment,repositoryCargoPreparationStorageV1(process.cwd()))).trim(),trunkVersion=(await runTool("trunk",["--version"],root,signal,true,environment,repositoryCargoPreparationStorageV1(process.cwd()))).trim(),ownerSha256=hash(import.meta.path);
  const staging = join(resolve(workspace, options.stateRoot ?? repoCacheDirectory(workspace, "trunk")), "staging");
  mkdirSync(staging, { recursive: true });
  const temporary = mkdtempSync(join(staging, `${profile}-`)), dist = join(temporary, "dist");
  try {
    const href = relative(temporary, join(root, "Cargo.toml")).replaceAll("\\", "/").replaceAll("&", "&amp;").replaceAll('"', "&quot;").replaceAll("<", "&lt;");
    writeFileSync(join(temporary, "index.html"), `<!doctype html><html><head><link data-trunk rel="rust" href="${href}" data-wasm-opt="z" data-type="worker" data-bindgen-target="web" /></head><body></body></html>\n`);
    writeFileSync(join(temporary, "Trunk.toml"), `required_version = "=0.21.14"\noffline = true\n[build]\nlocked = true\nfrozen = true\ntarget = "index.html"\ndist = "dist"\nfilehash = false\nrelease = ${profile === "release"}\n[tools]\nwasm_bindgen = ${JSON.stringify(bindgen[0])}\nwasm_opt = "version_130"\n`);
    environment.CARGO_TARGET_DIR = join(temporary, "target");
    await runTool("cargo", ["metadata", "--locked", "--offline", "--format-version=1", "--manifest-path", join(root, "Cargo.toml")], root, signal, "ignore", environment,repositoryCargoPreparationStorageV1(process.cwd()));
    await runTool("trunk", ["build", "--config", join(temporary, "Trunk.toml"), "--skip-version-check", "--offline", "true", "--color", "never"], root, signal, false, environment,repositoryCargoPreparationStorageV1(process.cwd()));
    const rawSource=join(temporary,"target","wasm32-unknown-unknown",profile==="release"?"release":"debug",manifest.package.name.replaceAll("-","_")+".wasm"),rawSha256=hash(rawSource);
    const ledger=join(cargoDirectories(workspace,environment).build,"semio-cargo-provenance"),before=new Set(existsSync(ledger)?readdirSync(ledger):[]);
    const queryArgs=["build","--target=wasm32-unknown-unknown","--manifest-path",join(root,"Cargo.toml"),...(profile==="release"?["--release"]:[]),"--offline","--frozen","--locked","--message-format=json"];
    await runTool(process.execPath,[CARGO_RELAY_SCRIPT,"relay",...queryArgs],temporary,signal,false,{...environment,SEMIO_TEST_ARTIFACT_DIR:undefined},repositoryCargoPreparationStorageV1(process.cwd()));
    const captured=readdirSync(ledger).filter(name=>!before.has(name)).map(name=>({path:join(ledger,name),receipt:JSON.parse(readFileSync(join(ledger,name),"utf8"))})).filter(({receipt:row})=>row.status===0&&!row.cancelled&&resolve(row.manifest)===join(root,"Cargo.toml")&&resolve(row.cwd)===temporary&&JSON.stringify(row.args)===JSON.stringify(queryArgs));
    if(captured.length!==1||hash(rawSource)!==rawSha256||!captured[0].receipt.units.some((unit:any)=>unit.message.filenames.includes(rawSource)&&unit.artifacts.some((artifact:any)=>artifact.path===rawSource&&artifact.sha256===rawSha256)))throw Error("Trunk output lacks the identical unchanged actual Cargo compiler artifact query");
    const {receipt:capture,path:producer}=captured[0],producerSha256=hash(producer),observationRoot=join(cargoDirectories(workspace,environment).build,"semio-trunk-provenance",randomUUID());mkdirSync(observationRoot,{recursive:true});
    const raw=join(observationRoot,"raw.wasm");copyFileSync(rawSource,raw);
    const staged=new Map<string,string>([[rawSource,raw]]);
    for(const unit of capture.units)for(const artifact of unit.artifacts)if(artifact.path.replaceAll("\\","/").startsWith(temporary.replaceAll("\\","/")+"/")&&!staged.has(artifact.path)){const path=join(observationRoot,"compiler",relative(temporary,artifact.path));mkdirSync(dirname(path),{recursive:true});copyFileSync(artifact.path,path);staged.set(artifact.path,path);}
    const compiler=join(observationRoot,"cargo-unit-provenance-trunk-"+basename(observationRoot)+".json");const observedAt=performance.now(),physical=new CurrentPhysicalOwnerV1(workspace,{maxBytes:128*1024*1024,maxWork:65536,chunkBytes:1024*1024,cancelled:()=>signal.aborted,remainingMs:()=>60000-(performance.now()-observedAt),onProgress:row=>console.log("[cargo-provenance] "+JSON.stringify(row))});await writeCompletedCargoInvocationProvenanceV1(compiler,{...capture,units:capture.units.map((unit:any,index:number)=>({...unit,evidence:{version:1,kind:"received",paths:unit.depInfo.map((row:any)=>row.path),producer:{path:producer,sha256:producerSha256,unit:index}}}))},staged,resolve(environment.CARGO_HOME??join(homedir(),".cargo")),physical);
    const files = new Map<string, string>();
    for (const entry of readdirSync(dist, { recursive: true, withFileTypes: true })) if (entry.isFile() && entry.name !== "index.html") {
      const path = join(entry.parentPath, entry.name);
      files.set(relative(dist, path).replaceAll("\\", "/"), path);
    }
    for (const name of [`${manifest.package.name}.js`, `${manifest.package.name}_bg.wasm`]) if (!files.has(name)) throw new Error(`Trunk omitted its renderer artifact: ${name}`);
    await stageRepositoryArtifacts(output, `wgpu-renderer:${profile}`, files, { signal, leaseDirectory: repoCacheDirectory(workspace, "agents", "resource-leases") });
    if(hash(import.meta.path)!==ownerSha256)throw Error("Trunk producer source changed during actual build");
    const receipt:TrunkRendererObservationV1={version:1,kind:"wgpu-trunk",profile,manifest:join(root,"Cargo.toml"),outputDirectory:output,builtAtMs,observedAtMs:Date.now(),owner:{path:import.meta.path,sha256:ownerSha256},configuration:{html:readFileSync(join(temporary,"index.html"),"utf8"),toml:readFileSync(join(temporary,"Trunk.toml"),"utf8")},toolVersions:{bindgen:version.trim(),binaryen:binaryenVersion,trunk:trunkVersion},compiler:{path:compiler,sha256:hash(compiler)},raw:{path:raw,sha256:rawSha256},outputs:[...files].map(([name,path])=>{const stagedPath=join(output,name);if(hash(path)!==hash(stagedPath))throw Error("Staged Trunk artifact differs from completed transformation");return{path:stagedPath,sha256:hash(stagedPath)}})};
    writeFileSync(join(observationRoot,"trunk.json"),JSON.stringify(receipt,null,2)+"\n");
    console.log(`[nx-trunk] Published ${files.size} renderer files: ${output}; provenance ${join(observationRoot,"trunk.json")}`);
  } finally { rmSync(temporary, { recursive: true, force: true }); }
}

class BuildScript extends BundleScript {
  async run([profile, ...args]: string[]): Promise<void> {
    if (args.length) throw new Error("Renderer compilation accepts only its Nx profile");
    const controller = new AbortController(), stop = (): void => controller.abort();
    process.once("SIGINT", stop); process.once("SIGTERM", stop);
    try { await buildTrunkRenderer({ rustPackageRoot: this.root, workspace: this.repoRoot, profile, signal: controller.signal }); }
    finally { process.removeListener("SIGINT", stop); process.removeListener("SIGTERM", stop); }
  }
}

if (import.meta.main) await new ScriptRouter(resolve(import.meta.dir, "../../📦️packages/🦀️rust")).register("build", BuildScript).run(process.argv.slice(2));
