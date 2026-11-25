import{createHash}from"node:crypto";
import{readFileSync,readdirSync}from"node:fs";
import{join,relative,resolve}from"node:path";
import"../../../🟨️.mjs";
import{cargoSourceSetV1,nativeDependencies,nativeWorkspaceV1}from"../../../🦀️native-manifests/🟨️.mjs";
import{cargoManifestProjectionV1}from"../../../🦀️native-manifests/📐️projection/🟨️.mjs";
import{observeDetachedSyncV1}from"../../../../../👁️observation/🟨️.mjs";
import{runtimeParseTomlV1}from"../../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";
import{moduleNodeDigestV1}from"../🟨️.mjs";
import{fileLeafNodeV1,leafStampHoldsV1,pathStampV1,workspaceNameV1,workspacePathV1}from"../🍃️leaves/🟨️.mjs";

/** 🔑️ The key of the node of one crate (the directory that holds its `Cargo.toml`, as a workspace name), read as the root of a test closure (`tests`: its dev-dependencies count) or as a dependency. */
export const cargoCrateKeyV1=(name,tests)=>JSON.stringify(["crate",name,tests?1:0]);
/** 🔑️ The key of the node of the Rust sources one crate compiles from. */
export const cargoSourcesKeyV1=(name,tests)=>JSON.stringify(["sources",name,tests?1:0]);
const SOURCE_DIRECTORIES_V1=Object.freeze(["src","src/bin","tests","examples","benches"]);
const fileSha256V1=path=>{try{return createHash("sha256").update(readFileSync(path)).digest("hex");}catch{return null;}};
const stampsOf=(workspaceRoot,paths)=>paths.map(path=>[workspaceNameV1(workspaceRoot,path),pathStampV1(path)??null]);

/** 🦀️ The nodes of one crate and of every crate it depends on by path, as nodes of the module closure graph: a crate node (digest: the `graph` and `sources` projections of its manifest, the parts a compilation and its path dependencies are read from) with edges to the crate nodes of its direct path dependencies (GR18's reader `nativeDependencies` with the crate's own Cargo workspace for inherited paths, development dependencies only at a test root; the closure of the graph is the closure Cargo compiles it with), to its sources node and to the leaves of its workspace manifest and lock file; a sources node (digest: the file list of the crate's static Rust reading, or the reason there is none) with edges to the leaves of those files (the SHA-256 of their bytes); for a crate without a static reading (a build script without an assertion, a macro mount) the leaves of every file below its directory; a manifest without entry points (a virtual workspace) has no sources. So an edit of one crate's sources moves its leaves, its sources node and the closures of exactly the crates whose dependency closure reaches it; an edit of the lock file moves every crate of that Cargo workspace; an edit of a file the crate does not read moves nothing. `held(key)` answers a node this machine already trusts (it is reused, not read again); `nodes` collects what was built. Reads go through detached observations, so nothing is recorded into a running inference. */
export function cargoCrateNodesV1(workspaceRoot,directory,{tests=true,held=()=>undefined,nodes=new Map(),facts=new Map()}={}){
 const visit=(crate,asRoot)=>{
  const name=workspaceNameV1(workspaceRoot,crate),key=cargoCrateKeyV1(name,asRoot);
  if(nodes.has(key))return key;
  const kept=held(key);
  if(kept!==undefined){nodes.set(key,kept);for(const edge of kept.edges)follow(edge);return key;}
  const manifestPath=join(crate,"Cargo.toml"),parsed=observeDetachedSyncV1(()=>runtimeParseTomlV1(readFileSync(manifestPath,"utf8"))).value,{root:workspace,declaration}=observeDetachedSyncV1(()=>nativeWorkspaceV1(crate,workspaceRoot,parsed)).value;
  const children=[...new Set(nativeDependencies(parsed,declaration).filter(dependency=>asRoot||dependency.kind!=="dev-dependencies").map(dependency=>resolve(dependency.workspace?workspace:crate,dependency.path)))].filter(child=>child!==crate).sort();
  const sources=sourcesNode(crate,name,asRoot),workspaceManifest=fileLeafNodeV1(workspaceRoot,join(workspace,"Cargo.toml")),lock=fileLeafNodeV1(workspaceRoot,join(workspace,"Cargo.lock"));
  for(const leaf of[workspaceManifest,lock])if(!nodes.has(leaf.key))nodes.set(leaf.key,leaf);
  const record={key,name,kind:"crate",digest:moduleNodeDigestV1({kind:"crate",name,tests:asRoot,graph:cargoManifestProjectionV1(parsed,asRoot?"graph-tests":"graph"),sources:cargoManifestProjectionV1(parsed,asRoot?"sources-tests":"sources")}),edges:[...children.map(child=>cargoCrateKeyV1(workspaceNameV1(workspaceRoot,child),false)),sources,workspaceManifest.key,lock.key],extra:{stamps:stampsOf(workspaceRoot,[manifestPath]),sha256:fileSha256V1(manifestPath)}};
  nodes.set(key,record);
  for(const child of children)visit(child,false);
  return key;
 };
 const follow=edge=>{
  const[kind,name,flag]=JSON.parse(edge);
  if(kind==="crate"){visit(workspacePathV1(workspaceRoot,name),flag===1);return;}
  if(nodes.has(edge))return;
  const kept=held(edge);
  if(kept!==undefined){nodes.set(edge,kept);for(const next of kept.edges)follow(next);return;}
  if(kind==="file"){nodes.set(edge,fileLeafNodeV1(workspaceRoot,workspacePathV1(workspaceRoot,name)));return;}
  if(kind==="sources")sourcesNode(workspacePathV1(workspaceRoot,name),name,flag===1);
 };
 const sourcesNode=(crate,name,asRoot)=>{
  const key=cargoSourcesKeyV1(name,asRoot);
  if(nodes.has(key))return key;
  const kept=held(key);
  if(kept!==undefined){nodes.set(key,kept);for(const edge of kept.edges)follow(edge);return key;}
  const set=observeDetachedSyncV1(()=>cargoSourceSetV1(relative(workspaceRoot,crate)||".",workspaceRoot,facts,asRoot,false)).value,files=set.resolved?set.files.map(file=>workspacePathV1(workspaceRoot,file)):set.reason==="no-entries"||set.reason==="no-manifest"?[]:treeFilesV1(crate);
  const leaves=files.map(file=>fileLeafNodeV1(workspaceRoot,file));
  for(const leaf of leaves)if(!nodes.has(leaf.key))nodes.set(leaf.key,leaf);
  nodes.set(key,{key,name,kind:"sources",digest:moduleNodeDigestV1({kind:"sources",name,tests:asRoot,resolved:set.resolved,reason:set.resolved?null:set.reason,files:leaves.map(leaf=>leaf.name)}),edges:leaves.map(leaf=>leaf.key),extra:{stamps:stampsOf(workspaceRoot,[crate,...SOURCE_DIRECTORIES_V1.map(part=>join(crate,part)),...(set.resolved?files:[])])}});
  return key;
 };
 const root=visit(resolve(workspaceRoot,directory),tests);
 return{root,nodes};
}

/** 🗂️ Every file below a crate directory that has no static Rust reading, in code-unit order, without the directories of nested crates, generated outputs and tools: what Cargo may read for it. */
function treeFilesV1(crate){
 const files=[],walk=(directory,top)=>{
  let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{return;}
  if(!top&&entries.some(entry=>entry.isFile()&&entry.name==="Cargo.toml"))return;
  for(const entry of entries.sort((left,right)=>left.name<right.name?-1:left.name>right.name?1:0)){if(entry.name==="target"||entry.name.startsWith("."))continue;const path=join(directory,entry.name);if(entry.isDirectory())walk(path,false);else if(entry.isFile())files.push(path);}
 };
 walk(crate,true);
 return files;
}

/** 🪪️ Whether a stored crate, sources or leaf node still stands for the disk of this machine by its stamps alone (one stat per path, shared by the round through `memo`). */
export function cargoNodeStampHoldsV1(workspaceRoot,record,memo=new Map()){
 if(record.kind==="file"||record.kind==="glob")return leafStampHoldsV1(workspaceRoot,record,memo);
 const stamps=record.extra?.stamps;
 if(!Array.isArray(stamps)||record.kind!=="crate"&&record.kind!=="sources")return false;
 return stamps.every(([name,stamp])=>{if(typeof stamp!=="string"||typeof name!=="string")return false;const path=workspacePathV1(workspaceRoot,name);let now=memo.get(path);if(now===undefined){now=pathStampV1(path,Number.POSITIVE_INFINITY,0)??"?";memo.set(path,now);}return now===stamp;});
}

/** 🦀️ The closure question of an owner that has only a `Cargo.toml`: its crate node read as a test root; its closure digest is `graph.closureDigest([root])`. */
export const cargoOwnerRootV1=(workspaceRoot,directory)=>cargoCrateKeyV1(workspaceNameV1(workspaceRoot,resolve(workspaceRoot,directory)),true);
