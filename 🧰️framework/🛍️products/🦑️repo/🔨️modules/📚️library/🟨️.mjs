import {readFileSync,existsSync,lstatSync} from "node:fs";
import {join,resolve,relative,dirname,isAbsolute} from "node:path";
import {createHash} from "node:crypto";
import {fileURLToPath} from "node:url";
import {createRequire} from "node:module";
import {conformsOwnerContractV1 as conforms,ownerArgumentsSourceIdentityV1} from "./🏗️build/📤️arguments/🟨️.mjs";
const publicationSchemaSource=readFileSync(new URL("./🏗️build/🧬️schema/📥️pin.json",import.meta.url)),publicationSchema=JSON.parse(publicationSchemaSource.toString("utf8"));
const engineSchemaSource=readFileSync(new URL("./🏗️build/🧬️schema/📥️engine.json",import.meta.url)),engineSchema=JSON.parse(engineSchemaSource.toString("utf8")),engineNamedInput=engineSchema.definitions.NamedInput.const;
const privateTaskSources=["./🔌️nx-plugin/📤️arguments/🟨️.mjs","./🔌️nx-plugin/📤️arguments/🧬️schema/🔣️executor.json","./🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json","./🔌️nx-plugin/📤️arguments/🧬️schema/📥️source.json","./🏗️build/📥️caller/📤️orchestration/🟨️.mjs","./🏗️build/📥️caller/📤️orchestration/🧬️schema/🔣️.json","./🏗️build/🧬️schema/📥️caller.json"],privateTaskBytes=privateTaskSources.map(path=>readFileSync(new URL(path,import.meta.url)));
const dependencySourceIdentity=(captured=false)=>createHash("sha256").update(JSON.stringify([createHash("sha256").update(readFileSync(new URL(import.meta.url))).digest("hex"),createHash("sha256").update(captured?publicationSchemaSource:readFileSync(new URL("./🏗️build/🧬️schema/📥️pin.json",import.meta.url))).digest("hex"),createHash("sha256").update(captured?engineSchemaSource:readFileSync(new URL("./🏗️build/🧬️schema/📥️engine.json",import.meta.url))).digest("hex"),ownerArgumentsSourceIdentityV1(),...privateTaskSources.map((path,index)=>createHash("sha256").update(captured?privateTaskBytes[index]:readFileSync(new URL(path,import.meta.url))).digest("hex"))])).digest("hex"),loadedDependencySourceIdentity=dependencySourceIdentity(true);

/** 🔏️ Attests the exact evaluated private projection and its still-current imported contract owner. */
export function nxBuildDependencyImplementationV1(){if(dependencySourceIdentity()!==loadedDependencySourceIdentity)throw Error("Loaded private Nx dependency Source changed");return loadedDependencySourceIdentity;}


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

let engineAuthority;
/** 🧮️ Closes only the private executor's actual source imports behind a string-only parser port. */
export function nxBuildEngineInputsV1(){
 if(!engineAuthority){
  const started=performance.now();let loadMilliseconds=0,parseMilliseconds=0,resolutionMilliseconds=0;
  const root=fileURLToPath(new URL("../../../../../",import.meta.url)),pending=[import.meta.url,...["./📦️packages/🟦️typescript/📋️executors.json","./🔌️nx-plugin/🏃️execution/🟨️.mjs","./🔌️nx-plugin/📤️arguments/📜️script.ts","./🔌️nx-plugin/📤️arguments/🧬️schema/🔣️command.json"].map(path=>new URL(path,import.meta.url).href)],files=new Set(),packages=new Set(["nx","typescript"]);
  const admit=(owner,specifier)=>{
   if(specifier.startsWith(".")){
    const candidate=nxBuildEngineSourceFileV1(owner,specifier);if(relative(root,candidate).startsWith(".."))throw Error("Private Nx engine source outside workspace");pending.push(candidate);
   }else if(!specifier.includes(":")&&specifier!=="bun")packages.add(specifier.startsWith("@")?specifier.split("/").slice(0,2).join("/"):specifier.split("/")[0]);
  };
  while(pending.length){
   const reference=pending.pop(),path=reference.startsWith("file:")?fileURLToPath(reference):reference;if(files.has(path))continue;files.add(path);let phase=performance.now();const source=readFileSync(path,"utf8");loadMilliseconds+=performance.now()-phase;if(path.endsWith(".json"))continue;
   phase=performance.now();const references=nxBuildEngineReferencesV1(source);parseMilliseconds+=performance.now()-phase;phase=performance.now();
   for(const reference of references)if(reference.kind==="module")admit(path,reference.specifier);else if(reference.kind==="file-url"){const target=resolve(dirname(path),reference.specifier);if(!existsSync(target)||!lstatSync(target).isDirectory())admit(path,reference.specifier);}resolutionMilliseconds+=performance.now()-phase;
  }
  if(process.env.SEMIO_NX_PROFILE_INPUTS==="1")console.log("[DEBUG] Nx engine Source closure "+JSON.stringify({files:files.size,loadMilliseconds,parseMilliseconds,resolutionMilliseconds,totalMilliseconds:performance.now()-started}));
  engineAuthority={files:[...files].map(path=>"{workspaceRoot}/"+relative(root,path).replaceAll("\\","/")).sort(),packages:[...packages].sort()};
 }
 return [...engineAuthority.files,{externalDependencies:[...engineAuthority.packages]},{runtime:"node --version"},{runtime:"bun --version"}];
}

/** 📌️ Reads the exact portable graph pinned by this owned build invocation. */
function graphPublication(workspaceRoot){
 nxBuildDependencyImplementationV1();
 const publication={path:process.env.SEMIO_BUILD_GRAPH_PATH,digest:process.env.SEMIO_BUILD_GRAPH_DIGEST};if(!conforms(publicationSchema,publication)||!isAbsolute(publication.path))throw Error("Original immutable build graph path and digest required");
 const source=readFileSync(publication.path,"utf8"),digest=createHash("sha256").update(source).digest("hex");
 if(digest!==publication.digest)throw Error("Pinned build graph source changed");
 const graph=JSON.parse(source);if(graph.version!==1||!graph.projects||!Array.isArray(graph.dependencies))throw Error("Prepared portable build graph required");
 return {graph,digest};
}

/** 🔌️ Projects only portable process execution onto Nx's internal executor contract. */
export function projectBuildTargetForNxV1(target){
 const {execution,configurations,...properties}=target,{kind,ownerArguments,...options}=execution;
 const result={...properties,executor:kind==="noop"?"nx:noop":ownerArguments?"@semio-tech/repo-lib:owner-command":"@semio-tech/repo-lib:command",...(kind==="noop"?{}:{options,inputs:[...properties.inputs??["default","^default"],engineNamedInput]})};
 if(configurations)result.configurations=Object.fromEntries(Object.entries(configurations).map(([name,configuration])=>{const {execution,...properties}=configuration;return[name,{...properties,...execution,...(options.env||execution?.env?{env:{...options.env,...execution?.env}}:{})}];}));
 return result;
}

/** 📽️ Projects one portable publication without discovering implementation-specific projects. */
export function projectBuildGraphForNxV1(graph){
 if(Object.hasOwn(graph.namedInputs??{},engineNamedInput)||Object.values(graph.projects).some(project=>Object.hasOwn(project.namedInputs??{},engineNamedInput)))throw Error("Private engine named input is reserved");
 const engine=nxBuildEngineInputsV1(),projects=Object.fromEntries(Object.entries(graph.projects).map(([name,project])=>[name,{...project,namedInputs:{...graph.namedInputs,...project.namedInputs,[engineNamedInput]:engine},targets:Object.fromEntries(Object.entries(project.targets).map(([name,target])=>[name,projectBuildTargetForNxV1(target)]))}]));
 return {projects,externalNodes:graph.externalNodes??{}};
}

/** 🧩️ Makes every backend consume the same complete project publication. */
async function nodes(files,_options,context){
 const {graph}=graphPublication(context.workspaceRoot);
 return [[files.includes("📋️project.json")?"📋️project.json":files[0]??"📋️project.json",projectBuildGraphForNxV1(graph)]];
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

/** 🪢️ Projects portable dependencies without reconstructing a second graph. */
export function createDependencies(_options,context){const identity=nxBuildDependencyImplementationV1();if(process.env.SEMIO_NX_IMPLEMENTATION_IDENTITY!==identity)throw Error("Original private Nx dependency implementation differs");const result=projectBuildDependenciesForNxV1(graphPublication(context.workspaceRoot).graph,context);nxBuildDependencyImplementationV1();return result;}

/** 🧬️ Binds dependency reuse to the exact published portable project and edge authority. */
export function dependencyResolutionAuthority(workspaceRoot,projects){
 const {graph,digest}=graphPublication(workspaceRoot),owners=Object.entries(projects).map(([name,project])=>[name,project.root]).sort(([a],[b])=>a.localeCompare(b));
 for(const[name,root]of owners)if(graph.projects[name]&&graph.projects[name].root!==root)throw Error(`Build project authority differs: ${name}`);
 return createHash("sha256").update(JSON.stringify([resolve(workspaceRoot),digest,owners,nxBuildDependencyImplementationV1()])).digest("hex");
}

export default {name:"@repo/emoji-project-json",createNodesV2:["📋️project.json",nodes],createDependencies};
