import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import {createHash} from "node:crypto";
import { dirname, isAbsolute, join } from "node:path";
import { CARGO_CONTRACT_NAME, JsonMap, BUILD_CONTRACT_NAME, StdioArtifactPackageRecord, canonicalStdioArtifactNames, stdioArtifactPackageContract, slashStdioPath } from "../../../../🔌️plugins/🗄️stdio/🗿️artifacts/📇️inventory/🟦️.ts";
import { cargoWorkspaceForManifest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
import { readBuildGraphV1,resolveBuildInputsV1, type BuildGraphV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import pinSchema from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️pin.json";
import {checkScriptInvocation,withScriptInvocationSignalV1,createIssuedResourceControlV1,type ScriptInvocation} from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {nativeCallerPhaseV1} from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {captureOwnedCommandResultV1} from "../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {withRepositoryToolingStoreV1} from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import {withRepositoryRustToolchainV1} from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🦀️rustup/📥️installation/🟦️.ts";

import { CARGO_COMPOSITION_NAME, COMPOSITION_RUST_NAME, COMPOSITION_TYPESCRIPT_NAME } from "../🪪️identity/🟦️.ts";

function assertActualCargoDag(metadata: JsonMap, artifactCargoNames: ReadonlySet<string>): void {
  const packages = new Map((metadata.packages as JsonMap[]).map((entry) => [String(entry.name), entry]));
  const artifactGraph = new Map<string, string[]>();
  for (const name of artifactCargoNames) {
    const cargo = packages.get(name);
    assert(cargo, `Cargo metadata misses ${name}`);
    artifactGraph.set(name, (cargo.dependencies as JsonMap[]).map((dependency) => String(dependency.name)).filter((dependency) => artifactCargoNames.has(dependency)));
  }
  const complete = new Set<string>();
  const visitArtifacts = (name: string, route: string[]): void => {
    const repeated = route.indexOf(name);
    if (repeated >= 0) throw new Error(`Cargo artifact dependency cycle: ${[...route.slice(repeated), name].join(" -> ")}`);
    if (complete.has(name)) return;
    for (const dependency of artifactGraph.get(name) ?? []) visitArtifacts(dependency, [...route, name]);
    complete.add(name);
  };
  const visitComposition = (name: string, route: string[], visited: Set<string>): void => {
    if (name === CARGO_COMPOSITION_NAME) throw new Error(`Cargo artifact reaches stdio composition: ${route.join(" -> ")}`);
    if (visited.has(name)) return;
    visited.add(name);
    const cargo = packages.get(name);
    if (!cargo) return;
    for (const dependency of cargo.dependencies as JsonMap[]) visitComposition(String(dependency.name), [...route, String(dependency.name)], visited);
  };
  for (const name of artifactCargoNames) {
    visitArtifacts(name, []);
    visitComposition(name, [name], new Set());
  }
}

/** 🦀️ Reads Cargo's own package graph without compiling Stdio. */
export async function stdioCargoMetadata(repoRoot:string,invocation:ScriptInvocation):Promise<JsonMap>{
 checkScriptInvocation(invocation);const phase=nativeCallerPhaseV1(invocation,repoRoot);await phase.advance("stdio:cargo-metadata");
 const timeout=new AbortController(),timer=setTimeout(()=>timeout.abort(Error("Original native child deadline exhausted")),phase.budget());
 try{return await withScriptInvocationSignalV1(invocation,timeout.signal,bounded=>{
 const control=createIssuedResourceControlV1(bounded,"stdio:cargo-metadata",Math.floor(phase.budget()));
 return withRepositoryToolingStoreV1(repoRoot,bounded,store=>withRepositoryRustToolchainV1(repoRoot,store,async generation=>{
  const manifests=[...new Set(stdioArtifactPackageContract(repoRoot).packages.map(entry=>cargoWorkspaceForManifest(repoRoot,entry.rust.manifest).manifest))],packages:JsonMap[]=[];
  for(const manifest of manifests){
   await phase.advance("stdio:cargo-metadata");await generation.recheck();
   const result=await captureOwnedCommandResultV1(generation.cargo,["metadata","--locked","--no-deps","--format-version","1","--manifest-path",join(repoRoot,manifest)],repoRoot,{control,env:phase.options({...process.env,...generation.environment}).env,output:{maximumBytes:Math.min(16777216,phase.authority.transport.maximumBytes),maximumLines:Math.min(65536,phase.authority.transport.maximumLines)}});
   assert.equal(result.status,0,result.stderr||"Original Cargo metadata failed");assert.equal(result.signal,null,"Original Cargo metadata interrupted");
   const metadata=JSON.parse(result.stdout) as JsonMap;assert(Array.isArray(metadata.packages),"Original Cargo metadata packages required");packages.push(...metadata.packages);await generation.recheck();phase.budget();checkScriptInvocation(invocation);
   console.log("[DEBUG] Stdio sealed Cargo metadata "+JSON.stringify({manifest,stdoutBytes:Buffer.byteLength(result.stdout),stderrBytes:Buffer.byteLength(result.stderr),packages:metadata.packages.length,generation:generation.digest}));
  }
  return{packages:[...new Map(packages.map(entry=>[entry.id,entry])).values()]};
 }));
 });}finally{clearTimeout(timer);}
}

/** 🕸️ Compares the admitted artifact contract with Cargo's actual native metadata graph. */
export function assertStdioArtifactCargoMetadata(repoRoot: string, contract: { packages: StdioArtifactPackageRecord[] }, metadata: JsonMap): void {
  const byName = new Map((metadata.packages as JsonMap[]).map((entry) => [String(entry.name), entry]));
  for (const entry of contract.packages) {
    const cargo = byName.get(entry.rust.cargoName);
    assert(cargo, `Cargo metadata misses ${entry.rust.cargoName}`);
    assert.equal(slashStdioPath(cargo.manifest_path), slashStdioPath(join(repoRoot, entry.rust.manifest)));
    const dependencies = new Set((cargo.dependencies as JsonMap[]).map((dependency) => dependency.name));
    assert(!dependencies.has(CARGO_COMPOSITION_NAME));
    assert(dependencies.has(CARGO_CONTRACT_NAME), `${entry.rust.cargoName} misses ${CARGO_CONTRACT_NAME}`);
  }
  assertActualCargoDag(metadata, new Set(contract.packages.map((entry) => entry.rust.cargoName)));
  console.log(`[stdio-package-contract] Cargo metadata packages=${contract.packages.length}`);
}

/** 🔏️ Reads only the exact immutable neutral graph issued by the outer owned build invocation. */
export function readStdioArtifactBuildGraphPin(environment:Readonly<Record<string,string|undefined>>=process.env):BuildGraphV1{
 const pin={path:environment.SEMIO_BUILD_GRAPH_PATH,digest:environment.SEMIO_BUILD_GRAPH_DIGEST};assert.equal(validateJsonSchemaSubset(pinSchema,pin).length,0,"Original immutable build graph path and digest required");assert(pin.path&&isAbsolute(pin.path),"Original absolute immutable build graph pin required");const source=readFileSync(pin.path,"utf8");assert.equal(createHash("sha256").update(source).digest("hex"),pin.digest,"Original Stdio build graph source changed");return readBuildGraphV1(JSON.parse(source));
}

/** 🔬️ Compares the admitted artifact dependency projection with the current portable build graph. */
export async function assertStdioArtifactBuildGraph(repoRoot: string, contract: { packages: StdioArtifactPackageRecord[] }, metadata: JsonMap): Promise<void> {
  assertActualCargoDag(metadata, new Set(contract.packages.map((entry) => entry.rust.cargoName)));
  const graph=readStdioArtifactBuildGraphPin();
  assert.equal(graph.version, 1, "Portable build graph version required");
  assert(graph.projects && typeof graph.projects === "object", "Portable build projects required");
  assert(Array.isArray(graph.dependencies), "Portable build dependency array required");
  const dependencies = graph.dependencies;
  const projects = graph.projects;
  const cargoPackages = new Map((metadata.packages as JsonMap[]).map((entry) => [String(entry.name), entry]));
  for (const entry of contract.packages) {
    assert(projects[entry.rust.projectName], `Build graph misses ${entry.rust.projectName}`);
    assert(projects[entry.typescript.projectName], `Build graph misses ${entry.typescript.projectName}`);
    const rustEdges = new Set(dependencies.filter((edge) => edge.source === entry.rust.projectName).map((edge) => edge.target));
    const typescriptEdges = new Set(dependencies.filter((edge) => edge.source === entry.typescript.projectName).map((edge) => edge.target));
    assert(!rustEdges.has(COMPOSITION_RUST_NAME), `${entry.rust.projectName} has a composition back-edge`);
    assert(rustEdges.has(BUILD_CONTRACT_NAME), `${entry.rust.projectName} misses build edge ${BUILD_CONTRACT_NAME}`);
    const cargo = cargoPackages.get(entry.rust.cargoName);
    assert(cargo, `Cargo metadata misses ${entry.rust.cargoName}`);
    const expectedRustEdges = (cargo.dependencies as JsonMap[]).map((dependency) => String(dependency.name)).filter((name) => name.startsWith("semio-s-artifact-stdio-") && name !== CARGO_CONTRACT_NAME).map((name) => canonicalStdioArtifactNames(name.slice("semio-s-artifact-stdio-".length)).rustProject);
    for (const target of expectedRustEdges) assert(rustEdges.has(target), `${entry.rust.projectName} misses build edge ${target}`);
    const typescript = JSON.parse(await Bun.file(join(repoRoot, entry.typescript.manifest)).text()) as JsonMap;
    for (const target of Object.keys(typescript.dependencies ?? {})) assert(typescriptEdges.has(target), `${entry.typescript.projectName} misses build edge ${target}`);
    const ownerRoot = slashStdioPath(dirname(entry.source));
    for (const node of [projects[entry.rust.projectName], projects[entry.typescript.projectName]]) {
      const defaultInputs = resolveBuildInputsV1(graph, node, node.namedInputs?.default ?? []);
      assert(defaultInputs.some((input: unknown) => typeof input === "string" && input.startsWith(`{workspaceRoot}/${ownerRoot}/`)), `${node.name} does not hash ${ownerRoot}`);
    }
  }
  assert(projects[BUILD_CONTRACT_NAME], `Build graph misses ${BUILD_CONTRACT_NAME}`);
  assert(projects[COMPOSITION_TYPESCRIPT_NAME], `Build graph misses ${COMPOSITION_TYPESCRIPT_NAME}`);
  console.log(`[stdio-package-contract] Build graph rust=${contract.packages.length} typescript=${contract.packages.length}`);
}
