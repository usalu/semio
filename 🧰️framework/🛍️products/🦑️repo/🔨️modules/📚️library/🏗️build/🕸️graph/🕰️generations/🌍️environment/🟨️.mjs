import{createHash}from"node:crypto";
import{canonicalJsonV1}from"../🟨️.mjs";

const sha256=value=>createHash("sha256").update(value).digest("hex");

/** 🌍️ Reads the one field of a manifest that other projects' edges depend on: the project name and native root, the package name, the Go module or the crate name. The same function derives a fact and checks it, so a change the extractor cannot see is never reported as stable by a different reader. */
export function extractEnvironmentFieldV1(extractor,text){
 if(text===undefined)return null;
 try{
  if(extractor==="project-json"){const json=JSON.parse(text);return{name:json.name??null,nativeRoot:json.metadata?.nativeRoot??null};}
  if(extractor==="package-json"){const json=JSON.parse(text);return{name:json.name??null};}
  if(extractor==="go-mod")return{module:/^module\s+(\S+)/mu.exec(text)?.[1]??null};
  if(extractor==="cargo-package"){const section=/^\[package\][^\n]*\n((?:(?!^\[)[^\n]*\n?)*)/mu.exec(text)?.[1]??"";const match=/^\s*name\s*=\s*(?:"([^"]+)"|'([^']+)')/mu.exec(section);return{name:match?.[1]??match?.[2]??null};}
 }catch{return{error:"parse"};}
 throw Error("Closed environment extractor required: "+extractor);
}

/** 🌍️ True when an environment row (a `field` extracted from a manifest or a `digest` of a whole file) still describes the tree; `read(path)` returns the text or undefined and `stat(path)` the size and modification time or undefined. */
export async function environmentRowHoldsV1(row,{read,stat}){
 if(row.kind==="field")return canonicalJsonV1(extractEnvironmentFieldV1(row.fact.extract,await read(row.path)))===canonicalJsonV1(row.fact.value);
 if(row.kind==="digest"){
  const status=await stat(row.path);
  if(!status)return row.fact.sha256===null;
  if(row.fact.sha256===null)return false;
  if(status.size===row.fact.size&&status.mtimeMs===row.fact.mtimeMs)return true;
  const text=await read(row.path);return text!==undefined&&sha256(text)===row.fact.sha256;
 }
 throw Error("Closed environment row kind required: "+row.kind);
}

/**
 * 🌍️ Derives the environment under which edges are valid: the owner table (name, root, native root), the package and Go module names, the workspace manifests, the lock file and the taxonomy. The digest changes exactly when an edge of an unrelated owner may change; the rows let a reader prove it unchanged without recomputing it.
 * `projects` are `{name, root, nativeRoot?, config?}`; `workspaceManifests` lists every `Cargo.toml` of the workspace, `lock` and `taxonomy` name single files.
 */
export async function deriveEnvironmentV1({projects,workspaceManifests,lock,taxonomy,read,stat}){
 const rows=[],owners=[],packages=[],modules=[];
 const field=async(path,extract)=>{const value=extractEnvironmentFieldV1(extract,await read(path));rows.push({kind:"field",path,fact:{extract,value}});return value;};
 for(const project of projects){
  owners.push([project.name,project.root,project.nativeRoot??null]);
  if(project.config)await field(project.config,project.config.endsWith("Cargo.toml")?"cargo-package":"project-json");
  const packageName=await field(project.root+"/package.json","package-json");if(packageName?.name)packages.push([packageName.name,project.name]);
  const go=await field(project.root+"/go.mod","go-mod");if(go?.module)modules.push([go.module,project.name]);
 }
 const digests=[],digest=async path=>{const status=await stat(path),text=status?await read(path):undefined;const fact=text===undefined?{sha256:null}:{sha256:sha256(text),size:status.size,mtimeMs:status.mtimeMs};rows.push({kind:"digest",path,fact});return fact.sha256;};
 for(const path of[...workspaceManifests].sort()){const text=await read(path);if(text!==undefined&&text.includes("[workspace"))digests.push([path,await digest(path)]);}
 const lockDigest=lock?await digest(lock):null,taxonomyDigest=taxonomy?await digest(taxonomy):null;
 const sorted=list=>list.sort((a,b)=>canonicalJsonV1(a)<canonicalJsonV1(b)?-1:1);
 return{digest:sha256(canonicalJsonV1({owners:sorted(owners),packages:sorted(packages),modules:sorted(modules),workspaces:sorted(digests),lock:lockDigest,taxonomy:taxonomyDigest})),rows};
}
