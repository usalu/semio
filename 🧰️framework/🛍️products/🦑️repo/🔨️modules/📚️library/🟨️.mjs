import{refuseBuildV1}from"./🏗️build/📖️texts/🟦️.ts";
import {nxTaskArgumentsSourceIdentityV1} from "./⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟨️.mjs";
import {readFileSync,existsSync,lstatSync,realpathSync} from "node:fs";
import {join,resolve,relative,dirname,isAbsolute,sep} from "node:path";
import {createHash} from "node:crypto";
import {fileURLToPath} from "node:url";
import {createRequire} from "node:module";
import {nxBuildDependencySourceIdentityV1,nxBuildDependencySourceChangesV1,nxEngineNamedInputV1 as engineNamedInput,nxEngineNativeNamedInputV1 as engineNativeNamedInput} from "./⚡️caching/🚀️bootstrap/🔌️nx-plugin/🔏️implementation/🟨️.mjs";
import {nxEngineUnkeyedPrefixesV1,splitNxEngineFilesV1,nxEngineFamilyV1,nxEnginePolicyV1,nxEngineSpawnOnlyV1} from "./⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🟦️.ts";
import {withNxBuildGraphPublicationV1} from "./⚡️caching/🚀️bootstrap/🔌️nx-plugin/📌️graph-pin/🟨️.mjs";
import { isRuntimeBuiltinSpecifierV1, runtimeIdsV1 } from "../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

/** 🔏️ Attests the exact evaluated private projection and its still-current imported contract owner. */
export function nxBuildDependencyImplementationV1(){nxTaskArgumentsSourceIdentityV1();return nxBuildDependencySourceIdentityV1();}
/** 👁️ Retains the exact admitted Source projection while identifying physical dependency changes before a mutable child starts. */
export function nxBuildDependencyImplementationChangesV1(){return nxBuildDependencySourceChangesV1();}


/** 🔎️ Classifies exact literal module and URL references as portable source facts. */
export function nxBuildEngineReferencesV1(source){
 const ts=createRequire(import.meta.url)("typescript"),references=[],identities=new Set(),add=(kind,specifier)=>{const identity=kind+"\0"+specifier;if(!identities.has(identity)){identities.add(identity);references.push({kind,specifier});}};
 for(const imported of ts.preProcessFile(source,true,true).importedFiles)add("module",imported.fileName);
 if(!source.includes("URL"))return references;
 const syntax=ts.createSourceFile("engine.ts",source,ts.ScriptTarget.Latest,false),visit=node=>{
  if(ts.isNewExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="URL"&&node.arguments?.length===2&&ts.isStringLiteralLike(node.arguments[0])&&node.arguments[0].text.startsWith(".")&&node.arguments[1].getText(syntax)==="import.meta.url"){const specifier=node.arguments[0].text;add(new URL(specifier,"file:///engine.ts").pathname.endsWith("/")?"directory-url":"file-url",specifier);}
  ts.forEachChild(node,visit);
 };visit(syntax);return references;
}

/** 📄️ Resolves actual engine modules, retaining directory-index imports and missing-file refusals. */
export function nxBuildEngineSourceFileV1(owner,specifier){
 const path=resolve(dirname(owner),specifier),candidate=[path,...[".ts",".mjs",".js",".json","/index.ts","/index.mjs","/index.js"].map(suffix=>path+suffix)].find(file=>existsSync(file)&&lstatSync(file).isFile());
 if(!candidate)throw Error(`Private Nx engine import missing: ${specifier} from ${owner}`);return candidate;
}

/** 📦️ Resolves a bare import of a workspace package (a link in the root `node_modules` that leads back into the workspace) to the Source file its `exports` map names, so the engine closure follows the real files instead of naming the package as an external npm dependency the graph does not carry; `undefined` for every other package. */
export function nxBuildEngineWorkspacePackageFileV1(root,specifier){
 const parts=specifier.split("/"),name=specifier.startsWith("@")?parts.slice(0,2).join("/"):parts[0],subpath="."+specifier.slice(name.length);let directory;
 try{directory=realpathSync(join(root,"node_modules",name));}catch{return undefined;}
 const distance=relative(root,directory);if(!distance||distance.startsWith("..")||isAbsolute(distance)||distance.split(sep).includes("node_modules"))return undefined;
 const manifest=JSON.parse(readFileSync(join(directory,"package.json"),"utf8")),pick=value=>{if(typeof value==="string")return value;if(Array.isArray(value))return value.map(pick).find(Boolean);if(value&&typeof value==="object")for(const condition of["development","import","default","node",...runtimeIdsV1()])if(condition in value){const found=pick(value[condition]);if(found)return found;}return undefined;};
 const exported=manifest.exports===undefined?(subpath==="."?manifest.main??"index.ts":undefined):pick(typeof manifest.exports==="object"&&!Array.isArray(manifest.exports)&&Object.keys(manifest.exports).some(key=>key.startsWith("."))?manifest.exports[subpath]:subpath==="."?manifest.exports:undefined);
 if(typeof exported!=="string")throw refuseBuildV1("missing","nx-engine.package-export-missing",{specifier:String(specifier)});
 const file=resolve(directory,exported);if(!existsSync(file)||!lstatSync(file).isFile()||relative(directory,file).startsWith(".."))throw refuseBuildV1("missing","nx-engine.package-file-missing",{specifier:String(specifier),exported:String(exported)});return file;
}

const defaultEngineRoot=fileURLToPath(new URL("../../../../../",import.meta.url)),engineDigestEntry="./⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🟨️.mjs";
/** 🧮️ Closes only the private executor's actual source imports behind a string-only parser port: the workspace-relative files and the npm packages they name. */
export function nxBuildEngineClosureV1(policy=nxEnginePolicyV1()){
  const root=fileURLToPath(new URL("../../../../../",import.meta.url)),pending=[import.meta.url,...["./📦️packages/🟦️typescript/📋️executors.json","./⚡️caching/🚀️bootstrap/🔌️nx-plugin/🏃️execution/🟨️.mjs","./⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/📜️script.ts","./⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧬️schema/🔣️command.json","./⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🟨️.mjs"].map(path=>new URL(path,import.meta.url).href)],files=new Set(),packages=new Set(["nx","typescript"]);
  const admit=(owner,specifier)=>{
   if(specifier.startsWith(".")){
    const candidate=nxBuildEngineSourceFileV1(owner,specifier);if(relative(root,candidate).startsWith(".."))throw Error("Private Nx engine source outside workspace");pending.push(candidate);
   }else if(!specifier.includes(":")&&!isRuntimeBuiltinSpecifierV1(specifier)){const workspaceFile=nxBuildEngineWorkspacePackageFileV1(root,specifier);if(workspaceFile)pending.push(workspaceFile);else packages.add(specifier.startsWith("@")?specifier.split("/").slice(0,2).join("/"):specifier.split("/")[0]);}
  };
  while(pending.length){
   const reference=pending.pop(),path=reference.startsWith("file:")?fileURLToPath(reference):reference;if(files.has(path))continue;files.add(path);const source=readFileSync(path,"utf8");if(path.endsWith(".json"))continue;
   const references=nxBuildEngineReferencesV1(source);
   for(const reference of references)if(reference.kind==="module")admit(path,reference.specifier);else if(reference.kind==="file-url"){const target=resolve(dirname(path),reference.specifier);if((!existsSync(target)||!lstatSync(target).isDirectory())&&!nxEngineSpawnOnlyV1(relative(root,path).replaceAll("\\","/"),reference.specifier,policy))admit(path,reference.specifier);}
  }
 return {files:[...files].map(path=>relative(root,path).replaceAll("\\","/")).sort(),packages:[...packages].sort()};
}

/** 🔑️ Digests the bytes of the engine files under their workspace-relative names, one `name NUL sha256 LF` row each, then one `NUL unkeyed NUL sha256-of-the-prefixes LF` row when prefixes stay out, so a prefix that moves re-keys. */
export function nxBuildEngineFilesDigestV1(root,files,unkeyedPrefixes=[]){
 const digest=createHash("sha256");for(const file of files)digest.update(file).update("\0").update(createHash("sha256").update(readFileSync(join(root,file))).digest("hex")).update("\n");
 if(unkeyedPrefixes.length)digest.update("\0unkeyed\0").update(createHash("sha256").update(unkeyedPrefixes.join("\n")).digest("hex")).update("\n");return digest.digest("hex");
}

/** 🪪️ Digests the current bytes of the keyed part of the private engine closure of one family: the one value a hash plan carries for the engine instead of every file path; scheduling code and the other output-neutral domains of `./⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🔣️.json` stay out. */
export function nxBuildEngineDigestV1(family="generic"){const name=nxEngineFamilyV1(family);return nxBuildEngineFilesDigestV1(defaultEngineRoot,splitNxEngineFilesV1(nxBuildEngineClosureV1().files,name).keyed,nxEngineUnkeyedPrefixesV1(name));}

const engineAuthorities=new Map();
/** 🏷️ Names the engine of one family as one runtime digest of its keyed closure plus the real npm packages it imports; the hasher runs the digest once per client, so no task plan lists engine file paths. */
export function nxBuildEngineInputsV1(family="generic"){
 const name=nxEngineFamilyV1(family);let authority=engineAuthorities.get(name);
 if(!authority)engineAuthorities.set(name,authority={command:`bun "${relative(defaultEngineRoot,fileURLToPath(new URL(engineDigestEntry,import.meta.url))).replaceAll("\\","/").replace(/^(?!\.\.?\/)/u,"./")}"${name==="generic"?"":" "+name}`,packages:nxBuildEngineClosureV1().packages});
 return [{runtime:authority.command},{externalDependencies:[...authority.packages]},{runtime:"node --version"},{runtime:"bun --version"}];
}

/** 🔌️ Declares the external package roots required by both private executor families before graph selection. */
export function nxBuildEngineExternalInputsV1(){return[...new Set([...nxBuildEngineInputsV1(),...nxBuildEngineInputsV1("native")].flatMap(row=>row.externalDependencies??[]))].sort();}

/** 📌️ Reads the exact portable graph pinned by this owned build invocation under the plugin implementation identity. */
const graphPublication=(workspaceRoot,environment,body)=>withNxBuildGraphPublicationV1(workspaceRoot,environment,body,nxBuildDependencyImplementationV1);


const cachePolicyPath="./⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json",cacheablePrefixes=["rust","products"].map(id=>"{workspaceRoot}/"+JSON.parse(readFileSync(new URL(cachePolicyPath,import.meta.url),"utf8")).partitions[id].directory+"/m/");
/** 🎟️ Admits an output list to the Nx task cache only when every positive output lies in the rust or the products cache partition, the two stores whose custody the cache hook can hold; a declared output elsewhere (generated sources inside the workspace) is never restored by Nx. */
export function nxOutputsCacheableV1(outputs){return outputs.every(output=>typeof output!=="string"||output.startsWith("!")||cacheablePrefixes.some(prefix=>output.startsWith(prefix)));}

/** 🔌️ Projects only portable process execution onto Nx's internal executor contract. */
export function projectBuildTargetForNxV1(target){
 const {execution,configurations,...properties}=target,{kind,ownerArguments,program,...declared}=execution,native=declared.nativeOwnerCommand!==undefined,options=program===undefined?declared:{...declared,command:"'"+program.replaceAll("'","'\\''")+"'"};
 const result={...properties,executor:kind==="noop"?"nx:noop":ownerArguments?"@semio-tech/repo-lib:owner-command":"@semio-tech/repo-lib:command",...(kind==="noop"?{}:{options,inputs:[...properties.inputs??["default","^default"],native?engineNativeNamedInput:engineNamedInput]}),...(properties.cache!==false&&properties.outputs&&!nxOutputsCacheableV1(properties.outputs)?{cache:false}:{})};
 if(configurations)result.configurations=Object.fromEntries(Object.entries(configurations).map(([name,configuration])=>{const {execution,...properties}=configuration;return[name,{...properties,...execution,...(options.env||execution?.env?{env:{...options.env,...execution?.env}}:{})}];}));
 return result;
}

/** 📽️ Projects one portable publication without discovering implementation-specific projects. */
export function projectBuildGraphForNxV1(graph){
 if([engineNamedInput,engineNativeNamedInput].some(name=>Object.hasOwn(graph.namedInputs??{},name)||Object.values(graph.projects).some(project=>Object.hasOwn(project.namedInputs??{},name))))throw Error("Private engine named input is reserved");
 const engine=nxBuildEngineInputsV1(),engineNative=nxBuildEngineInputsV1("native"),known=new Set(Object.keys(graph.projects)),present=names=>Array.isArray(names)?names.filter(name=>typeof name!=="string"||/[*!:,]/u.test(name)||known.has(name)):names,projects=Object.fromEntries(Object.entries(graph.projects).map(([name,project])=>[name,{...project,...(project.implicitDependencies===undefined?{}:{implicitDependencies:present(project.implicitDependencies)}),namedInputs:{...graph.namedInputs,...project.namedInputs,[engineNamedInput]:engine,[engineNativeNamedInput]:engineNative},targets:Object.fromEntries(Object.entries(project.targets).map(([name,target])=>[name,projectBuildTargetForNxV1(target)]))}]));
 return {projects:shareNxProjectInputsV1(liftNxNamedInputReferencesV1(projects)),externalNodes:graph.externalNodes??{}};
}

/** 🪜️ Nx refuses a named input that refers to another project, while the build graph names a provider closure as a `{input,projects}` row inside a command source group (`🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures`): every such row moves out of its group into each target that uses the group, directly or through other groups of its project (a target without inputs uses `default`), so Nx hashes the same inputs. A `^` input of a group that holds such rows is refused: the dependency's rows could not follow it. */
export function liftNxNamedInputReferencesV1(projects){
 const reference=item=>item!==null&&typeof item==="object"&&!Array.isArray(item)&&typeof item.input==="string"&&item.projects!==undefined,holding=new Set();
 for(const project of Object.values(projects))for(const[name,list]of Object.entries(project.namedInputs??{}))if(Array.isArray(list)&&list.some(reference))holding.add(name);
 if(!holding.size)return projects;
 return Object.fromEntries(Object.entries(projects).map(([key,project])=>{
  const named=project.namedInputs??{},reach=(name,seen)=>{if(seen.has(name)||!Array.isArray(named[name]))return[];seen.add(name);return named[name].flatMap(item=>reference(item)?[item]:typeof item==="string"&&!item.startsWith("^")&&Object.hasOwn(named,item)?reach(item,seen):[]);};
  const targets=Object.fromEntries(Object.entries(project.targets??{}).map(([name,target])=>{
   const inputs=target.inputs??["default","^default"];
   if(inputs.some(item=>typeof item==="string"&&item.startsWith("^")&&holding.has(item.slice(1))))throw refuseBuildV1("invalid","nx.dependency-input-not-projectable",{key,name});
   const seen=new Set(),known=new Set(inputs.map(item=>JSON.stringify(item))),lifted=inputs.flatMap(item=>typeof item==="string"&&!item.startsWith("^")&&Object.hasOwn(named,item)?reach(item,seen):[]).filter(item=>{const text=JSON.stringify(item);if(known.has(text))return false;known.add(text);return true;});
   return[name,lifted.length?{...target,inputs:[...inputs,...lifted]}:target];
  }));
  return[key,{...project,namedInputs:Object.fromEntries(Object.entries(named).map(([name,list])=>[name,Array.isArray(list)?list.filter(item=>!reference(item)):list])),targets}];
 }));
}

const sharedInputMinimumBytes=2048;
/** 🗂️ Keeps each project-independent named input group of at least 2 KiB once, in the first project (by name) that holds it: every other project drops the group and its targets name it as `{input,projects}` of that project, which Nx resolves to the same files (a named input definition may not refer to another project, so a group that another named input or a `^` dependency input of the project uses stays whole). The published Nx graph stops repeating the multi-hundred-KiB command source lists of every project (550 projects of the print closure were 497 MB, above the 512 MiB namespace limit). */
export function shareNxProjectInputsV1(projects){
 const external=new Set();
 for(const project of Object.values(projects))for(const target of Object.values(project.targets??{}))for(const item of target.inputs??[])if(item!==null&&typeof item==="object"&&typeof item.input==="string"&&item.projects!==undefined)for(const owner of typeof item.projects==="string"?[item.projects]:item.projects)external.add(owner+"\0"+item.input);
 const owners=new Map(),independent=item=>typeof item==="string"?item.startsWith("{workspaceRoot}/"):item!==null&&typeof item==="object"&&!Array.isArray(item)&&Object.keys(item).length===1&&["externalDependencies","runtime","env"].includes(Object.keys(item)[0]);
 return Object.fromEntries(Object.keys(projects).sort().map(name=>{
  const project=projects[name],redirects=new Map(),referenced=new Set();
  for(const[input,list]of Object.entries(project.namedInputs))if(Array.isArray(list))for(const item of list)if(typeof item==="string")referenced.add(item.replace(/^\^/u,""));
  for(const target of Object.values(project.targets??{}))for(const item of target.inputs??[]){if(typeof item==="string"&&item.startsWith("^"))referenced.add(item.slice(1));else if(item!==null&&typeof item==="object"&&typeof item.input==="string")referenced.add(item.input);}
  const namedInputs=Object.fromEntries(Object.entries(project.namedInputs).flatMap(([input,list])=>{
   if(input===engineNamedInput||input===engineNativeNamedInput||referenced.has(input)||external.has(name+"\0"+input)||!Array.isArray(list)||!list.every(independent))return[[input,list]];
   const text=JSON.stringify(list);if(text.length<sharedInputMinimumBytes)return[[input,list]];
   const owner=owners.get(text);if(!owner){owners.set(text,{project:name,input});return[[input,list]];}
   redirects.set(input,{input:owner.input,projects:[owner.project]});return[];
  }));
  const targets=redirects.size?Object.fromEntries(Object.entries(project.targets??{}).map(([key,target])=>[key,target.inputs?{...target,inputs:target.inputs.map(item=>typeof item==="string"&&redirects.has(item)?redirects.get(item):item)}:target])):project.targets;
  return[name,{...project,namedInputs,targets}];
 }));
}


/** 🧩️ Makes every backend consume the same complete project publication. */
async function nodes(files,_options,context){
 return graphPublication(context.workspaceRoot,process.env,async({graph})=>[[files.includes("📋️project.json")?"📋️project.json":files[0]??"📋️project.json",projectBuildGraphForNxV1(graph)]]);
}

/** 🪢️ Retains file-bound edges only where Nx owns their actual file-map provenance. */
export function projectBuildDependenciesForNxV1(graph,context){
 const owners=new Map(),nonProject=new Set((context.fileMap?.nonProjectFiles??[]).map(row=>row.file));
 for(const[name,files]of Object.entries(context.fileMap?.projectFileMap??{}))for(const{file}of files){let names=owners.get(file);if(!names)owners.set(file,names=new Set());names.add(name);}
 return graph.dependencies.map(edge=>{
  if(!Object.hasOwn(graph.projects,edge.source)&&!Object.hasOwn(graph.externalNodes??{},edge.source)||!Object.hasOwn(graph.projects,edge.target)&&!Object.hasOwn(graph.externalNodes??{},edge.target))throw Error("Unknown portable build dependency owner");
  if(!Object.hasOwn(graph.projects,edge.source))return edge;
  if(edge.sourceFile){const names=owners.get(edge.sourceFile);if(names&&[...names].some(name=>name!==edge.source))throw Error("Private Nx dependency source file belongs to another project");if(names?.has(edge.source)||nonProject.has(edge.sourceFile))return edge;}
  const {sourceFile,...dependency}=edge;return {...dependency,type:"implicit"};
 });
}

/** 🕸️ Projects portable dependencies without reconstructing a second graph. */
export async function createDependencies(_options,context){const identity=nxBuildDependencyImplementationV1();if(process.env.SEMIO_NX_IMPLEMENTATION_IDENTITY!==identity)throw Error("Original private Nx dependency implementation differs");return graphPublication(context.workspaceRoot,process.env,async({graph})=>{const result=projectBuildDependenciesForNxV1(graph,context);nxBuildDependencyImplementationV1();return result;});}

/** 🌐️ Projects inert portable graph/owner/digest DATA without claiming retained snapshot custody. */
export function projectBuildDependencyAuthorityV1(workspaceRoot,graph,digest,owners,identity){for(const[name,root]of owners)if(graph.projects[name]&&graph.projects[name].root!==root)throw Error(`Build project authority differs: ${name}`);return createHash("sha256").update(JSON.stringify([resolve(workspaceRoot),digest,owners,identity])).digest("hex");}

/** 🧬️ Binds dependency reuse to the exact published portable project and edge authority. */
export function dependencyResolutionAuthority(workspaceRoot,projects){
 const owners=Object.entries(projects).map(([name,project])=>[name,project.root]).sort(([a],[b])=>a.localeCompare(b));return graphPublication(workspaceRoot,process.env,async({graph,digest})=>projectBuildDependencyAuthorityV1(workspaceRoot,graph,digest,owners,nxBuildDependencyImplementationV1()));
}

export default {name:"@repo/emoji-project-json",createNodesV2:["📋️project.json",nodes],createDependencies};
