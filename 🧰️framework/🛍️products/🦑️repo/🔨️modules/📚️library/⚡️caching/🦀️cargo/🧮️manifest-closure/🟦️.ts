import {createHash} from "node:crypto";
import {existsSync,readdirSync,readFileSync,lstatSync,mkdirSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {runtimeParseTomlV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const;
type Table=Record<string,any>;
/** 🧱️ One local crate of the closure: its repository-relative directory, package name, manifest digest and whether it is a member of the selected workspace. */
export type ManifestCrateV1=Readonly<{directory:string;name:string;digest:string;member:boolean}>;
/** 🧮️ The local crates a Cargo build of the selected roots reads, derived from `Cargo.toml` files alone, with every manifest, listing and absence the answer rests on. */
export type ManifestClosureV1=Readonly<{version:1;scope:"workspace"|"package";tests:boolean;crates:readonly ManifestCrateV1[];inputs:readonly Readonly<{path:string;digest:string|null}>[];unsupported:readonly string[]}>;
/** 🧠️ Recalls and remembers one closure by key; a recall answers only while every recorded input still answers the same. */
export interface ManifestClosureMemoV1{recall(key:string):ManifestClosureV1|undefined;remember(key:string,closure:ManifestClosureV1):void}
/** 🎛️ Selects test dependencies of the roots, the root scope and an optional memo. */
export type ManifestClosureOptionsV1=Readonly<{tests?:boolean;scope?:"workspace"|"package";memo?:ManifestClosureMemoV1}>;

const sha256=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex");
const posix=(path:string)=>path.split(sep).join("/");
const fill=(text:{en:string;de:string},values:Record<string,string|number>)=>[text.en,text.de].map(row=>row.replace(/\{(\w+)\}/gu,(_all,name:string)=>String(values[name]))).join(" | ");
const parsed=new Map<string,Table>();

/** 🔎️ Reads one manifest with a parse memoized by content digest; `undefined` when the file is absent. */
function readManifestV1(path:string,inputs:Map<string,string|null>):Readonly<{table:Table;digest:string}>|undefined{
 let bytes:Buffer;try{if(lstatSync(path).size>policy.maximumManifestBytes)throw Error("Bounded Cargo manifest required: "+path);bytes=readFileSync(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"||(error as NodeJS.ErrnoException).code==="ENOTDIR"){inputs.set(path,null);return undefined;}throw error;}
 const digest=sha256(bytes);inputs.set(path,digest);let table=parsed.get(digest);if(!table){table=runtimeParseTomlV1(bytes.toString("utf8")) as Table;if(parsed.size>8192)parsed.clear();parsed.set(digest,table);}
 return Object.freeze({table,digest});
}

/** 📁️ Lists one directory's names for a member glob and records the listing as an input. */
function listV1(path:string,inputs:Map<string,string|null>):string[]{
 let names:string[];try{names=readdirSync(path).sort();}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"||(error as NodeJS.ErrnoException).code==="ENOTDIR")names=[];else throw error;}
 inputs.set(policy.listing+path,sha256(JSON.stringify(names)));return names;
}

function globRegexpV1(segment:string):RegExp{return new RegExp("^"+segment.replace(/[.+^${}()|\\]/gu,"\\$&").replace(/\*/gu,"[^/]*").replace(/\?/gu,"[^/]")+"$","u");}

/** 🧭️ Expands the `members` or `exclude` patterns of a workspace the way Cargo does: `*`, `?` and `[..]`-free segments, relative to the workspace directory. */
function expandV1(base:string,patterns:readonly string[],inputs:Map<string,string|null>):string[]{
 const result:string[]=[];
 for(const pattern of patterns){
  let current=[base];
  for(const segment of pattern.split("/").filter(part=>part&&part!==".")){
   const next:string[]=[];
   for(const directory of current){
    if(/[*?]/u.test(segment)){const expression=globRegexpV1(segment);for(const name of listV1(directory,inputs))if(expression.test(name))next.push(join(directory,name));}
    else next.push(join(directory,segment));
   }
   current=next;
  }
  result.push(...current);
 }
 return result;
}

type WorkspaceV1=Readonly<{root:string;declaration:Table;members:ReadonlySet<string>}>;

/** 🏢️ Finds the workspace a manifest belongs to: `package.workspace`, else the nearest ancestor with a `[workspace]` table below the repository root. */
function workspaceOfV1(directory:string,table:Table,root:string,inputs:Map<string,string|null>,workspaces:Map<string,WorkspaceV1|null>):WorkspaceV1|null{
 const escapes=(path:string)=>{const local=relative(root,path);return isAbsolute(local)||local===".."||local.startsWith(".."+sep);};
 let at=typeof table.package?.workspace==="string"?resolve(directory,table.package.workspace):directory;
 for(;;){
  if(escapes(at))throw Error(fill(policy.texts.escapes,{path:posix(directory)}));
  const cached=workspaces.get(at);
  if(cached!==undefined){if(cached)return cached;}
  else{
   const row=readManifestV1(join(at,policy.manifest),inputs),declaration=row?.table.workspace as Table|undefined;
   if(declaration){
    const excluded=new Set(expandV1(at,declaration.exclude??[],inputs)),members=new Set<string>(expandV1(at,declaration.members??[],inputs).filter(path=>!excluded.has(path)&&existsSync(join(path,policy.manifest))));
    if(row.table.package)members.add(at);
    const found:WorkspaceV1={root:at,declaration,members};workspaces.set(at,found);return found;
   }
   workspaces.set(at,null);
  }
  if(at===root)return null;
  at=dirname(at);
 }
}

/** 🧶️ Lists the local path dependency directories one manifest declares, with workspace inheritance, in every target table; test dependencies only when asked. */
function dependenciesV1(directory:string,table:Table,workspace:WorkspaceV1|null,tests:boolean):string[]{
 const result:string[]=[],tables=[table,...Object.values(table.target??{}) as Table[]],inherited:Table=workspace?.declaration.dependencies??{};
 for(const row of tables)for(const kind of policy.kinds){
  if(kind==="dev-dependencies"&&!tests)continue;
  for(const[alias,declaration]of Object.entries<any>(row[kind]??{})){
   if(declaration===null||typeof declaration!=="object")continue;
   if(declaration.workspace===true){const shared=inherited[alias];if(shared&&typeof shared==="object"&&typeof shared.path==="string"&&workspace)result.push(resolve(workspace.root,shared.path));}
   else if(typeof declaration.path==="string")result.push(resolve(directory,declaration.path));
  }
 }
 return result;
}

/** 🔑️ Keys one request by roots, scope and test choice; the answer is proven by the recorded inputs. */
export function manifestClosureKeyV1(workspaceRoot:string,roots:readonly string[],options:ManifestClosureOptionsV1={}):string{
 return sha256(JSON.stringify([1,resolve(workspaceRoot),[...roots].sort(),options.scope??"package",options.tests===true]));
}

/** 🧮️ A path dependency below its workspace root is a member of that workspace, as Cargo makes it. Derives the local Cargo crates of `roots` (repository-relative directories) from manifests alone: path dependencies, build dependencies, target tables and `workspace = true` inheritance, test dependencies only of selected roots, and with scope `workspace` also every member of the roots' workspaces and the test dependencies of every local crate, which is what `cargo metadata` lists. A `[patch]` or `[replace]` table is reported in `unsupported` because only a resolver can say which patch applies.
 * @see https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-path-dependencies
 * @see ./🧬️schema/🔣️.json */
export function manifestClosureV1(workspaceRoot:string,roots:readonly string[],options:ManifestClosureOptionsV1={}):ManifestClosureV1{
 const base=resolve(workspaceRoot),scope=options.scope??"package",tests=options.tests===true,key=options.memo?manifestClosureKeyV1(base,roots,options):undefined,recalled=key===undefined?undefined:options.memo!.recall(key);
 if(recalled)return recalled;
 const inputs=new Map<string,string|null>(),workspaces=new Map<string,WorkspaceV1|null>(),crates=new Map<string,ManifestCrateV1>(),unsupported=new Set<string>(),selected=new Set(roots.map(root=>resolve(base,root)));
 const done=new Map<string,boolean>(),edges=new Map<string,string[]>(),owners=new Map<string,string|undefined>(),listed=new Set<string>(),all=scope==="workspace",visit=(directory:string,testsOf:boolean):void=>{
  const local=relative(base,directory);if(isAbsolute(local)||local===".."||local.startsWith(".."+sep))throw Error(fill(policy.texts.escapes,{path:posix(directory)}));
  const before=done.get(directory);if(before!==undefined&&(before||!testsOf))return;done.set(directory,testsOf);
  const row=readManifestV1(join(directory,policy.manifest),inputs);if(!row)throw Error(fill(policy.texts.missing,{path:posix(join(directory,policy.manifest))}));
  if(crates.size>=policy.maximumCrates)throw Error(fill(policy.texts.limit,{limit:policy.maximumCrates}));
  const workspace=workspaceOfV1(directory,row.table,base,inputs,workspaces);
  for(const name of policy.unsupportedTables)if(row.table[name]!==undefined||workspace&&readManifestV1(join(workspace.root,policy.manifest),inputs)?.table[name]!==undefined)unsupported.add(name);
  owners.set(directory,workspace?.root);if(workspace?.members.has(directory))listed.add(directory);
  if(row.table.package)crates.set(directory,Object.freeze({directory:posix(local)||".",name:String(row.table.package.name),digest:row.digest,member:false}));
  else if(!row.table.workspace)throw Error(fill(policy.texts.missing,{path:posix(join(directory,policy.manifest))}));
  const children=dependenciesV1(directory,row.table,workspace,testsOf);edges.set(directory,children);
  for(const child of children)visit(child,scope==="workspace");
  if(scope==="workspace"&&workspace&&selected.has(directory))for(const entry of workspace.members)visit(entry,true);
 };
 for(const directory of selected)visit(directory,tests||all);
 const members=new Set(listed),inside=(path:string,root:string)=>path===root||path.startsWith(root+sep);
 for(let grown=true;grown;){grown=false;for(const member of [...members])for(const child of edges.get(member)??[]){const root=owners.get(member);if(!members.has(child)&&root!==undefined&&owners.get(child)===root&&inside(child,root)){members.add(child);grown=true;}}}
 const ordered=[...crates.values()].map(row=>Object.freeze({...row,member:members.has(resolve(base,row.directory))})).sort((a,b)=>Buffer.compare(Buffer.from(a.directory),Buffer.from(b.directory)));
 const closure:ManifestClosureV1=Object.freeze({version:1,scope,tests,crates:Object.freeze(ordered),inputs:Object.freeze([...inputs].sort(([a],[b])=>Buffer.compare(Buffer.from(a),Buffer.from(b))).map(([path,digest])=>{const listed=path.startsWith(policy.listing),name=listed?path.slice(policy.listing.length):path;return Object.freeze({path:(listed?policy.listing:"")+(posix(relative(base,name))||"."),digest});})),unsupported:Object.freeze([...unsupported].sort())});
 if(validateJsonSchemaSubset(schema.definitions.Closure,closure,schema).length)throw Error("Closed Cargo manifest closure required");
 if(key!==undefined)options.memo!.remember(key,closure);
 return closure;
}

/** 🗄️ Answers one input question now: a manifest digest, a listing digest (path starts with the listing marker) or absence. */
function answerV1(base:string,row:Readonly<{path:string;digest:string|null}>):string|null{
 const path=resolve(base,row.path);
 try{if(row.path.startsWith(policy.listing))return sha256(JSON.stringify(readdirSync(resolve(base,row.path.slice(policy.listing.length))).sort()));return sha256(readFileSync(path));}catch{return null;}
}

/** 🧠️ Keeps one closure record per key in `directory`; a recall serves only while every recorded manifest digest, listing and absence is unchanged, so an edited or added manifest recomputes and nothing else does. */
export function manifestClosureMemoV1(workspaceRoot:string,directory:string):ManifestClosureMemoV1{
 const base=resolve(workspaceRoot),file=(key:string)=>join(directory,key+policy.memoSuffix);
 return Object.freeze({
  recall(key:string){
   let record:{version:number;key:string;closure:ManifestClosureV1};
   try{record=JSON.parse(readFileSync(file(key),"utf8"));}catch{return undefined;}
   if(validateJsonSchemaSubset(schema.definitions.Memo,record,schema).length||record.key!==key)return undefined;
   for(const row of record.closure.inputs)if(answerV1(base,row)!==row.digest)return undefined;
   return Object.freeze(record.closure);
  },
  remember(key:string,closure:ManifestClosureV1){
   const temporary=file(key)+"."+process.pid+".tmp";
   try{mkdirSync(directory,{recursive:true});writeFileSync(temporary,JSON.stringify({version:1,key,closure}));renameSync(temporary,file(key));}catch{try{rmSync(temporary,{force:true});}catch{}}
  },
 });
}
