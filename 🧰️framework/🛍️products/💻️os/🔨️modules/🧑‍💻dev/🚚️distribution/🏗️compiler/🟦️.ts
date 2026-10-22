import {runtimeCatalogBuildInventoryV1} from "@semio-tech/plugin-registry/catalog-view";
import {requirePlaygroundVariant} from "@semio-tech/plugin-registry/playground/default";
import { runtimeFixturePathV1 } from "@semio-tech/repo-lib/inventory";
import {loadDevContribution,resolveDevContributionFile} from "../../🧩️contribution/📥️loading/🟦️.ts";
import type {OwnedBuildConfig} from "@semio-tech/ui-react/build-tooling";
import type {InlineConfig} from "vite";
import {readFileSync,existsSync,statSync,realpathSync} from "node:fs";
import {fileURLToPath,pathToFileURL} from "node:url";
import { withRepositoryCacheStoreV1 } from "@semio-tech/repo-lib/storage";
import { withViteStorageV1, withViteArtifactInputsV1 } from "@semio-tech/repo-lib/dev";
import { withArtifactScratchV1, withArtifactDestinationTaskV1, withRepositoryArtifactDestinationV1, stageRepositoryArtifacts, readNativeCallerAuthorityV1 } from "@semio-tech/repo-lib/artifacts";
import {withCompilerState} from "@semio-tech/process/compiler-state";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import type {ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {requireNativeStoragePathV1} from "@semio-tech/process/artifacts/native-build/storage/platform/contract";
import {frameworkOsViteArtifactInputsV1} from "../../🏗️builder/🌐️vite/📥️artifacts/🟦️.ts";
import {createHash} from "node:crypto";
import {createRequire} from "node:module";
import {mkdir,writeFile} from "node:fs/promises";
import {dirname,join,resolve,relative} from "node:path";
import {distributionProgress,type DistributionBundlePlan} from "../📋️plan/🟦️.ts";
import {distributionFileWitness,distributionStaticSourcePaths,distributionPathOrder} from "../📥️source/🟦️.ts";
import {DISTRIBUTION_LAYOUT,distributionLayoutForModeV1,distributionOutputOwner,parseDistributionManifest,parseDistributionStaticInputs,type DistributionInput} from "../🟦️.ts";
import {readDistributionCompileReceiptV1,type DistributionCompileReceiptV1} from "./📋️contract/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import { runtimeKeyV1 } from "../../../../../../🔨️modules/⚙️runtime/🏷️identity/🟦️.ts";

/** 🏗️ Owns actual inputs, paid compiler state and bounded publication within one isolated compile process. */
export async function compileDistributionProductV1(workspace:string,invocation:ScriptInvocation):Promise<DistributionCompileReceiptV1>{
 const authority=readNativeCallerAuthorityV1(invocation.capabilities),mode=authority.original.buildMode,owner=schema.definitions.Protocol.const.owner,phase=scriptCompilerAdmissionControlV1(invocation,"vite:distribution-compile",0);
 if(resolve(workspace)!==authority.repositoryRoot||process.env.SEMIO_BUILD_MODE!==mode||process.env.SEMIO_COMPILER_CAPACITY_HELD!==undefined)throw Error("Exact unmarked distribution compiler Original required");
 return withRepositoryCacheStoreV1(workspace,"dependencies",invocation,phase,(dependencies,dependenciesPhase)=>withViteStorageV1(workspace,mode,owner,invocation,dependenciesPhase,(products,layout,_temporary,productsPhase)=>withViteArtifactInputsV1({workspace,invocation,store:products,control:productsPhase,requests:frameworkOsViteArtifactInputsV1(products.root,{version:1,mode,renderer:"react",command:"build",engines:"distribution"})},control=>withArtifactScratchV1({workspace,invocation,storage:products.storage,control:products.control,phaseControl:control,request:{version:1,mode,owner:schema.definitions.Protocol.const.scratchOwner}},token=>withArtifactDestinationTaskV1(token,invocation,async sink=>{
  const compiler=createRequire(join(workspace,"package.json"))("vite/package.json").version;
  return withCompilerState({capacity:{kind:"whole-host"},root:layout.state,owner,compiler:"vite:"+compiler+":"+runtimeKeyV1(),configuration:JSON.stringify(DISTRIBUTION_LAYOUT),mode,control:sink.control,storage:products.storage,storageControl:products.control,files:["deps/_metadata.json"],environment:{...process.env,...dependencies.environment,...products.environment,...sink.environment}},async(directory,environment,control)=>{
   requireNativeStoragePathV1(products.root,directory);const selected={...environment,SEMIO_VITE_CACHE_DIRECTORY:directory,SEMIO_BUILD_MODE:mode},previous=new Map(Object.keys(selected).map(name=>[name,process.env[name]]));
   try{
    for(const[name,value]of Object.entries(selected))if(value===undefined)delete process.env[name];else process.env[name]=value;
    const plan=await renderDistributionBundle(workspace,products.root,sink.directory,mode),files=new Map<string,string>();
    for(const[path,bytes]of plan.files){const target=requireNativeStoragePathV1(products.root,join(sink.directory,path));await mkdir(dirname(target),{recursive:true});await writeFile(target,bytes,{flag:"wx",mode:0o644,signal:control.signal});files.set(path,target);}
    const manifest=JSON.stringify(plan.manifest,null,2)+"\n",manifestPath=join(sink.directory,DISTRIBUTION_LAYOUT.manifest);await writeFile(manifestPath,manifest,{flag:"wx",mode:0o644,signal:control.signal});files.set(DISTRIBUTION_LAYOUT.manifest,manifestPath);
    await withRepositoryArtifactDestinationV1({workspace,invocation,storage:products.storage,control:products.control,phaseControl:control,directory:layout.output},destination=>stageRepositoryArtifacts(destination,owner,files));
    return readDistributionCompileReceiptV1(JSON.stringify({version:1,mode,owner,manifestSha256:createHash("sha256").update(manifest).digest("hex")}),mode);
   }finally{for(const[name,value]of previous)if(value===undefined)delete process.env[name];else process.env[name]=value;}
  });
 })))));
}

/** 🏗️ Runs the real compiler without copy hooks, capturing main and worker input byte witnesses. */
/** 🔁 The one crossing between the owned build contract and the build tool that consumes it.
 * `🏗️builder/🌐️vite` states the production configuration in the tool-independent owned shapes of
 * `🛠️build-tooling` — readonly arrays, structural server and plugin hooks — which is exactly why the
 * tool's own mutable declarations cannot be reached by assignment. The two descriptions cover the
 * same runtime objects, and this is the single place a distribution build hands one to the other. */
function buildToolConfig(config: OwnedBuildConfig): InlineConfig {
  return config as unknown as InlineConfig;
}

async function renderDistributionBundle(workspace: string, productsRoot:string,artifactRoot: string, mode: "prototype" | "release"): Promise<DistributionBundlePlan> {
  const distributionLayout=distributionLayoutForModeV1(workspace,productsRoot,mode);
  const scriptDirectory = dirname(fileURLToPath(import.meta.url)), playDirectory = resolve(scriptDirectory, "../.."), authorityDirectory = join(playDirectory, "🚚️distribution");
  const authority = parseDistributionStaticInputs(JSON.parse(readFileSync(join(authorityDirectory, "🔗️inputs.json"), "utf8")));
  const witnesses = new Map<string, DistributionInput>(), pending = new Map<string, Promise<void>>();
  const observe = (id: string): Promise<void> => {
    if (id.includes("\0")) return Promise.resolve();
    const absolute = resolve(playDirectory, id.split(/[?#]/u)[0]!);
    if (!existsSync(absolute) || !statSync(absolute).isFile()) return Promise.resolve();
    const canonical = realpathSync(absolute), path = relative(realpathSync(workspace), canonical).replaceAll("\\", "/").normalize("NFC");
    if (path === ".." || path.startsWith("..")) throw new Error(`Compiler input escapes workspace: ${id}`);
    if (runtimeFixturePathV1(path)) throw new Error("Production compiler input belongs to a testing fixture collection: " + path);
    if (path.startsWith(relative(workspace, join(playDirectory, DISTRIBUTION_LAYOUT.directory)).replaceAll("\\", "/") + "/")) throw new Error(`Compiler input consumes its own output: ${path}`);
    const current = pending.get(path);
    if (current) return current;
    const operation = distributionFileWitness(canonical, path).then(row => { witnesses.set(path, row); if (witnesses.size % 500 === 0) distributionProgress(`recorded ${witnesses.size} compiler input witnesses`); });
    pending.set(path, operation);
    return operation;
  };
  for (const path of [...new Set([...authority.paths, ...await distributionStaticSourcePaths(workspace, authority.moduleEntries)])]) {
    if (path.startsWith("/") || path.includes("\\") || path.split("/").some(part => !part || part === "." || part === "..") || !existsSync(join(workspace, path))) throw new Error(`Missing or unsafe distribution static input: ${path}`);
    await observe(join(workspace, path));
  }
  for (const row of [...distributionLayout.chunks, ...distributionLayout.assets]) if (row.kind !== "name") await observe(join(workspace, row.source));
  await observe(join(workspace, DISTRIBUTION_LAYOUT.entry.source));
  const collector = () => ({
    name: "semio-distribution-input-witness", enforce: "pre" as const,
    async load(id: string) { await observe(id); return null; },
    async buildEnd(this: { getWatchFiles(): string[] }) {
      const files = this.getWatchFiles();
      distributionProgress(`collecting ${files.length} compiler watch inputs`);
      for (const path of files) await observe(path);
      distributionProgress("compiler watch inputs collected");
    },
  });
  const defaultVariant = requirePlaygroundVariant(runtimeCatalogBuildInventoryV1().defaultVariant);
  const original = { SEMIO_PLUGIN: process.env.SEMIO_PLUGIN, SEMIO_RENDERER: process.env.SEMIO_RENDERER, SEMIO_BRAND: process.env.SEMIO_BRAND };
  process.env.SEMIO_PLUGIN = defaultVariant;
  process.env.SEMIO_RENDERER = "react";
  process.env.SEMIO_BRAND = "";
  let result: Awaited<ReturnType<typeof import("vite")["build"]>>;
  try {
    const target = runtimeCatalogBuildInventoryV1().playgrounds.find((row) => row.variant === defaultVariant);
    if (!target) throw new Error("Missing default host build contribution");
    const contribution = loadDevContribution(workspace, target);
    const configPath = contribution ? resolveDevContributionFile(workspace, contribution.viteConfig, contribution.ownerRoot) : fileURLToPath(new URL("../../🏗️builder/🌐️vite/🟦️.ts", import.meta.url));
    const { build } = await import("vite"), { default: createConfig } = await import(pathToFileURL(configPath).href);
    const config = buildToolConfig(await createConfig({ command: "build", mode, isSsrBuild: false, isPreview: false }));
    distributionProgress("actual production configuration loaded");
    const workerPlugins = config.worker?.plugins;
    result = await build({ ...config, configFile: false, publicDir: false, logLevel: "silent", cacheDir: join(artifactRoot, "vite-distribution-cache"), plugins: [collector(), ...(config.plugins ?? [])], worker: { ...config.worker, plugins: () => [collector(), ...(workerPlugins?.() ?? [])] }, build: { ...config.build, write: false, emptyOutDir: false, outDir: join(artifactRoot, "distribution-unwritten") } });
  } finally {
    for (const [name, value] of Object.entries(original)) if (value === undefined) delete process.env[name]; else process.env[name] = value;
  }
  const files = new Map<string, Uint8Array>();
  distributionProgress("compiler returned output bytes");
  for (const bundle of Array.isArray(result) ? result : [result]) {
    if (!("output" in bundle)) throw new Error("Distribution generation cannot use a watch-mode compiler");
    for (const item of bundle.output) {
      if (files.has(item.fileName)) throw new Error(`Duplicate compiler output: ${item.fileName}`);
      distributionOutputOwner(DISTRIBUTION_LAYOUT, item.fileName);
      files.set(item.fileName, Buffer.from(item.type === "chunk" ? item.code : item.source));
    }
  }
  distributionProgress(`rechecking ${witnesses.size} compiler input witnesses`);
  for (const row of witnesses.values()) {
    const current = await distributionFileWitness(join(workspace, row.path), row.path);
    if (current.bytes !== row.bytes || current.sha256 !== row.sha256) throw new Error(`Compiler input changed during production build: ${row.path}`);
  }
  const manifest = parseDistributionManifest({
    version: 1, contractId: "dev-distribution-bundle", layoutSha256: createHash("sha256").update(readFileSync(join(authorityDirectory, "📇️layout.json"))).digest("hex"),
    inputs: [...witnesses.values()].sort((left, right) => distributionPathOrder(left.path, right.path)),
    outputs: [...files].map(([path, bytes]) => ({ ownerId: distributionOutputOwner(DISTRIBUTION_LAYOUT, path), path, bytes: bytes.length, sha256: createHash("sha256").update(bytes).digest("hex") })).sort((left, right) => distributionPathOrder(left.path, right.path)),
  }, DISTRIBUTION_LAYOUT);
  return { manifest, files };
}

