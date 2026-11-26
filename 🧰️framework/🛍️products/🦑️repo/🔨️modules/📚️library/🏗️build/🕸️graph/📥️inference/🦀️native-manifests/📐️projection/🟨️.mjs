const isTable=value=>value!==null&&typeof value==="object"&&!Array.isArray(value);
const DEPENDENCY_TABLES_V1=Object.freeze(["dependencies","build-dependencies"]);

/** 📐️ The roles a Cargo manifest is read in, and what each role depends on: `graph` the path dependencies a crate compiles with (dev-dependencies only for `graph-tests`, the root of a test closure), `sources` the entry points and build script a crate compiles from (test, bench and example targets only for `sources-tests`). */
export const CARGO_MANIFEST_ROLES_V1=Object.freeze(["graph","graph-tests","sources","sources-tests"]);

function reducedDependenciesV1(table){
 const out={};
 for(const[alias,declaration]of Object.entries(isTable(table)?table:{})){
  if(!isTable(declaration))continue;
  if(declaration.workspace===true)out[alias]={workspace:true};
  else if(declaration.path)out[alias]={path:declaration.path,...declaration.package===undefined?{}:{package:declaration.package}};
 }
 return out;
}

function reducedWorkspaceDependenciesV1(table){
 const out={};
 for(const[alias,declaration]of Object.entries(isTable(table)?table:{}))
  if(isTable(declaration)&&declaration.path)out[alias]={path:declaration.path,...declaration.package===undefined?{}:{package:declaration.package}};
 return out;
}

function graphV1(manifest,tests){
 const kinds=tests?[...DEPENDENCY_TABLES_V1,"dev-dependencies"]:DEPENDENCY_TABLES_V1,tables=owner=>{const out={};for(const kind of kinds){const reduced=reducedDependenciesV1(owner[kind]);if(Object.keys(reduced).length)out[kind]=reduced;}return out;};
 const out={...tables(manifest)};
 if(isTable(manifest.package)&&manifest.package.workspace!==undefined)out.package={workspace:manifest.package.workspace};
 if(isTable(manifest.target)){const target={};for(const[name,owner]of Object.entries(manifest.target)){const reduced=isTable(owner)?tables(owner):{};if(Object.keys(reduced).length)target[name]=reduced;}if(Object.keys(target).length)out.target=target;}
 if(manifest.workspace)out.workspace=isTable(manifest.workspace)?{dependencies:reducedWorkspaceDependenciesV1(manifest.workspace.dependencies)}:{};
 return out;
}

function pathsV1(list){return Array.isArray(list)?list.map(item=>isTable(item)&&item.path!==undefined?{path:item.path}:{}):undefined;}

function sourcesV1(manifest,tests){
 const out={},build=manifest.package?.build,assertion=manifest.package?.metadata?.semio?.["build-script"];
 if(isTable(manifest.package)){const pack={};if(build!==undefined)pack.build=build;if(assertion!==undefined)pack.metadata={semio:{"build-script":assertion}};if(Object.keys(pack).length)out.package=pack;}
 if(isTable(manifest.lib)&&manifest.lib.path!==undefined)out.lib={path:manifest.lib.path};
 for(const kind of tests?["bin","test","bench","example"]:["bin"]){const listed=pathsV1(manifest[kind]);if(listed!==undefined)out[kind]=listed;}
 if(isTable(manifest["build-dependencies"])&&Object.keys(manifest["build-dependencies"]).length)out["build-dependencies"]=manifest["build-dependencies"];
 if(isTable(manifest.target)){const target={};for(const[name,owner]of Object.entries(manifest.target))if(isTable(owner)&&isTable(owner["build-dependencies"])&&Object.keys(owner["build-dependencies"]).length)target[name]={"build-dependencies":owner["build-dependencies"]};if(Object.keys(target).length)out.target=target;}
 return out;
}

/** 🧮️ The part of a parsed Cargo manifest that one role depends on, in the shape the readers consume (so a reader of the projection gets the same answer as of the whole manifest); everything else (versions, features, registry dependencies, lints, metadata, order, comments) is not part of it. The pure function behind the `cargo-manifest` field row, shared by the inferring process and every judge.
 * @param {unknown} manifest
 * @param {string} role
 * @returns {import("../🟨️.d.mts").CargoManifestV1 & {workspace?: Readonly<{dependencies?: Readonly<Record<string, Readonly<{path?: string; package?: string}>>>}>}}
 * @see ./🧪️tests/🟦️.ts */
export function cargoManifestProjectionV1(manifest,role){
 if(!CARGO_MANIFEST_ROLES_V1.includes(role))throw Error("Cargo manifest role required: "+role);
 if(!isTable(manifest))throw Error("Cargo manifest table required");
 return role==="graph"?graphV1(manifest,false):role==="graph-tests"?graphV1(manifest,true):sourcesV1(manifest,role==="sources-tests");
}
