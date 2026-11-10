import{canonicalJsonV1,compareCodePointsV1}from"../../🕰️generations/🧮️fact-codec/🟨️.mjs";
import{createHash}from"node:crypto";
import{PROJECT_BASENAME_V1,ROSTER_RULES_V1}from"../../🧭️roster/🟨️.mjs";

const sorted=list=>[...list].sort(compareCodePointsV1);
const unique=list=>sorted(new Set(list));
const digestOf=value=>createHash("sha256").update(canonicalJsonV1(value)).digest("hex");
const escape=text=>text.replace(/[.+^$()|[\]\\{}]/gu,"\\$&");

/** ✳️ The regular expression of one glob: `**` any depth, `*` one segment part, `?` one character, `{a,b}` alternatives. */
export function globRegexV1(pattern){return new RegExp("^"+globBody(pattern)+"$","u");}

function globBody(pattern){
 let out="";
 for(let at=0;at<pattern.length;at++){
  const c=pattern[at];
  if(c==="*"&&pattern[at+1]==="*"){if(pattern[at+2]==="/"){out+="(?:.*/)?";at+=2;}else{out+=".*";at++;}}
  else if(c==="*")out+="[^/]*";
  else if(c==="?")out+="[^/]";
  else if(c==="{"){const end=pattern.indexOf("}",at);if(end<0)out+="\\{";else{out+="(?:"+pattern.slice(at+1,end).split(",").map(globBody).join("|")+")";at=end;}}
  else out+=escape(c);
 }
 return out;
}

/** ✳️ A matcher of relative paths for a pattern list; a leading `!` excludes, `{projectRoot}/` is the root itself. */
export function globMatcherV1(patterns){
 const clean=pattern=>pattern.replace(/^(!?)\{projectRoot\}\//u,"$1"),include=[],exclude=[];
 for(const pattern of patterns)(pattern.startsWith("!")?exclude:include).push(globRegexV1(clean(pattern).replace(/^!/u,"")));
 return path=>include.some(regex=>regex.test(path))&&!exclude.some(regex=>regex.test(path));
}

function tomlValue(text,at){
 const skip=()=>{for(;;){while(at<text.length&&/\s/u.test(text[at]))at++;if(text[at]!=="#")return;while(at<text.length&&text[at]!=="\n")at++;}};
 skip();const c=text[at];
 if(c==='"'||c==="'"){
  let out="";at++;
  while(at<text.length&&text[at]!==c){if(c==='"'&&text[at]==="\\"){const n=text[at+1];out+=n==="n"?"\n":n==="t"?"\t":n;at+=2;}else out+=text[at++];}
  return[out,at+1];
 }
 if(c==="["){const list=[];at++;for(;;){skip();if(text[at]==="]")return[list,at+1];if(text[at]===","){at++;continue;}if(at>=text.length)throw Error("toml");const[value,next]=tomlValue(text,at);list.push(value);at=next;}}
 if(c==="{"){const table={};at++;for(;;){skip();if(text[at]==="}")return[table,at+1];if(text[at]===","){at++;continue;}if(at>=text.length)throw Error("toml");const[key,next]=tomlKey(text,at);skip();at=next;skip();if(text[at]!=="=")throw Error("toml");const[value,after]=tomlValue(text,at+1);put(table,key,value);at=after;}}
 const word=/^[^\s,\]}]+/u.exec(text.slice(at))?.[0];if(!word)throw Error("toml");
 return[word==="true"?true:word==="false"?false:/^[+-]?\d+(\.\d+)?$/u.test(word)?Number(word):word,at+word.length];
}

function tomlKey(text,at){
 const parts=[];
 for(;;){
  while(text[at]===" "||text[at]==="\t")at++;
  if(text[at]==='"'||text[at]==="'"){const[value,next]=tomlValue(text,at);parts.push(value);at=next;}
  else{const word=/^[A-Za-z0-9_-]+/u.exec(text.slice(at))?.[0];if(!word)throw Error("toml");parts.push(word);at+=word.length;}
  while(text[at]===" "||text[at]==="\t")at++;
  if(text[at]===".")at++;else return[parts,at];
 }
}

function put(table,path,value){
 let at=table;
 for(const part of path.slice(0,-1)){if(typeof at[part]!=="object"||at[part]===null||Array.isArray(at[part]))at[part]={};at=at[part];}
 at[path[path.length-1]]=value;
}

/** 📜 The restricted TOML subset Cargo manifests use: tables, dotted keys, strings, arrays, inline tables, booleans, numbers. */
export function parseTomlSubsetV1(text){
 const root={};let table=root,at=0;
 const lines=text.split(/\r?\n/u);
 for(let index=0;index<lines.length;index++){
  let line=lines[index];
  const stripped=line.replace(/^\s+/u,"");
  if(stripped===""||stripped[0]==="#")continue;
  if(stripped[0]==="["){
   const array=stripped.startsWith("[[");
   const[path]=tomlKey(stripped,array?2:1);
   if(array){table={};continue;}
   table=root;for(const part of path){if(typeof table[part]!=="object"||table[part]===null)table[part]={};table=table[part];}
   continue;
  }
  while(!balanced(line)&&index+1<lines.length)line+="\n"+lines[++index];
  const[key,next]=tomlKey(line,0);
  at=next;while(line[at]===" "||line[at]==="\t")at++;
  if(line[at]!=="=")throw Error("toml");
  put(table,key,tomlValue(line,at+1)[0]);
 }
 return root;
}

function balanced(line){
 let depth=0,quote="";
 for(let at=0;at<line.length;at++){
  const c=line[at];
  if(quote){if(c==="\\"&&quote==='"')at++;else if(c===quote)quote="";}
  else if(c==='"'||c==="'")quote=c;
  else if(c==="#"){while(at<line.length&&line[at]!=="\n")at++;}
  else if(c==="["||c==="{")depth++;
  else if(c==="]"||c==="}")depth--;
 }
 return depth<=0;
}

const SPEC_KEYS=["version","path","git","branch","tag","rev","package","optional","default-features","workspace"];
const specOf=spec=>{
 if(typeof spec==="string")return{version:spec};
 const out={};
 for(const key of SPEC_KEYS)if(spec[key]!==undefined)out[key]=spec[key];
 if(Array.isArray(spec.features))out.features=sorted(spec.features);
 return out;
};
const tablesOf=toml=>{
 const out={};
 for(const section of["dependencies","dev-dependencies","build-dependencies"])if(toml[section])out[section]=toml[section];
 if(toml.workspace?.dependencies)out["workspace.dependencies"]=toml.workspace.dependencies;
 for(const[target,body]of Object.entries(toml.target??{}))for(const section of["dependencies","dev-dependencies","build-dependencies"])if(body[section])out["target."+target+"."+section]=body[section];
 return out;
};

/** 🦀 The parsed dependency set of a Cargo.toml: package name and every dependency table with normalized specs. */
export function cargoDependenciesV1(text){
 const toml=parseTomlSubsetV1(text),tables={};
 for(const[section,body]of Object.entries(tablesOf(toml)))tables[section]=Object.fromEntries(sorted(Object.keys(body)).map(name=>[name,specOf(body[name])]));
 return{format:"Cargo.toml",name:toml.package?.name??null,tables};
}

const sortedObject=object=>Object.fromEntries(sorted(Object.keys(object??{})).map(key=>[key,object[key]]));

/** 📦 The parsed dependency set of a package.json. */
export function packageDependenciesV1(text){
 const json=JSON.parse(text),out={format:"package.json",name:json.name??null};
 for(const section of["dependencies","devDependencies","peerDependencies","optionalDependencies"])out[section]=sortedObject(json[section]);
 return out;
}

/** 📋 The parsed declaration of a project.json: name, implicit dependencies, named inputs and targets. */
export function projectDeclarationV1(text){
 const json=JSON.parse(text),targets={};
 for(const name of sorted(Object.keys(json.targets??{}))){const target=json.targets[name];targets[name]={command:target.command??target.options?.command??null,inputs:Array.isArray(target.inputs)?target.inputs:[]};}
 const named={};
 for(const name of sorted(Object.keys(json.namedInputs??{})))named[name]=json.namedInputs[name];
 return{format:"project.json",name:json.name??null,implicitDependencies:unique(json.implicitDependencies??[]),namedInputs:named,targets};
}

/** 🐹 The parsed dependency set of a go.mod. */
export function goDependenciesV1(text){
 let module=null,block=false;const requires=[];
 for(const raw of text.split(/\r?\n/u)){
  const line=raw.replace(/\/\/.*$/u,"").trim();
  if(line==="")continue;
  if(block){if(line===")")block=false;else{const[path,version]=line.split(/\s+/u);requires.push([path,version]);}continue;}
  if(line.startsWith("module "))module=line.slice(7).trim();
  else if(line==="require ("||line==="require(")block=true;
  else if(line.startsWith("require ")){const[path,version]=line.slice(8).trim().split(/\s+/u);requires.push([path,version]);}
 }
 return{format:"go.mod",module,requires:requires.sort((a,b)=>compareCodePointsV1(a[0],b[0]))};
}

/** 🧾 The parsed dependency set of one manifest by its file name; a manifest that does not parse is the value `{error}`. */
export function parseManifestV1(path,text){
 const name=path.slice(path.lastIndexOf("/")+1);
 try{
  if(name==="package.json")return packageDependenciesV1(text);
  if(name.endsWith("project.json"))return projectDeclarationV1(text);
  if(name==="Cargo.toml")return cargoDependenciesV1(text);
  if(name==="go.mod")return goDependenciesV1(text);
 }catch{return{error:"unparseable"};}
 return{error:"unsupported"};
}

const dependencyNames=manifest=>{
 if(!manifest||manifest.error)return[];
 if(manifest.format==="package.json")return["dependencies","devDependencies","peerDependencies","optionalDependencies"].flatMap(section=>Object.keys(manifest[section]));
 if(manifest.format==="Cargo.toml")return Object.values(manifest.tables).flatMap(table=>Object.keys(table));
 if(manifest.format==="go.mod")return manifest.requires.map(item=>item[0]);
 return manifest.implicitDependencies;
};

/** 🏗️ The configuration files of one project root, in the order the assembly reads them. */
export const PROJECT_CONFIG_FILES_V1=Object.freeze(["📋️project.json","project.json","package.json","Cargo.toml","go.mod"]);
/** 🏗️ The glob patterns that list the project roots of a workspace. */
export const PROJECT_ROSTER_PATTERNS_V1=Object.freeze(["**/"+PROJECT_BASENAME_V1,"**/Cargo.toml"]);

const configEntry=(name,bytes)=>{
 const text=Buffer.from(bytes).toString("utf8");
 try{
  if(name==="Cargo.toml")return{ok:true,toml:Bun.TOML.parse(text)};
  if(name==="go.mod")return{ok:true,go:goDependenciesV1(text)};
  return{ok:true,json:JSON.parse(text)};
 }catch(error){return{ok:false,sha256:createHash("sha256").update(bytes).digest("hex"),error:String(error?.message??error)};}
};

/** 🏗️ The name and availability of the project of one root, as inference decides them: a root with an authored project file is named by that file (null when it declares no name, unavailable when it does not parse); every other root with a Cargo manifest is named by its package (null without one, unavailable when the manifest does not parse). */
export function projectOfConfigV1(config,hasProjectFile){
 const entry=hasProjectFile?config.files[PROJECT_BASENAME_V1]:config.files["Cargo.toml"];
 if(entry===null||entry===undefined)return{name:null,unavailable:false};
 if(entry.ok===false)return{name:null,unavailable:true};
 const name=hasProjectFile?entry.json?.name:entry.toml?.package?.name;
 return{name:typeof name==="string"&&name!==""?name:null,unavailable:false};
}

const dirOf=path=>{const cut=path.lastIndexOf("/");return cut<0?"":path.slice(0,cut);};

const manifestKey=(ctx,path)=>ctx.key("manifest",path,{});
const projectKey=(ctx,project)=>ctx.key("project",project.root,project);

const GLOB_PART=/[*?{]/u;
const resolveGlob=(root,pattern)=>{
 const clean=pattern.replace(/^\{projectRoot\}\//u,"");
 return clean.startsWith("{workspaceRoot}/")?clean.slice(16):root===""?clean:root+"/"+clean;
};
const splitGlob=full=>{
 const parts=full.split("/"),at=parts.findIndex(part=>GLOB_PART.test(part)),cut=at<0?parts.length-1:at;
 return{base:parts.slice(0,cut).join("/"),rest:parts.slice(cut).join("/")};
};

const literalSuffix=rest=>{
 const segment=rest.slice(rest.lastIndexOf("/")+1);
 if(segment.includes("{")||segment.includes("}"))return"";
 return segment.slice(Math.max(segment.lastIndexOf("*"),segment.lastIndexOf("?"))+1);
};

/** 🧭 The compiled form of a pattern list below a root: the directory bases to list and a membership test of a workspace relative path (`!` excludes, `{workspaceRoot}/` and `{projectRoot}/` resolve to the longest literal directory prefix). */
export function globSpecV1(root,patterns){
 const groups=new Map(),exclude=[],excluded=[];
 for(const pattern of patterns){
  if(typeof pattern!=="string")continue;
  if(pattern.startsWith("!")){const full=resolveGlob(root,pattern.slice(1));excluded.push(full);exclude.push(globRegexV1(full));}
  else{const group=splitGlob(resolveGlob(root,pattern));groups.set(group.base+"\0"+group.rest,{...group,suffix:literalSuffix(group.rest),regex:new RegExp("^"+(group.base===""?"":escape(group.base)+"/")+globBody(group.rest)+"$","u")});}
 }
 const list=[...groups.values()];
 return{
  groups:list,bases:unique(list.map(group=>group.base)),key:JSON.stringify([unique(list.map(group=>group.base+"\0"+group.rest)),unique(excluded)]),
  has:path=>{
   let hit=false;
   for(let at=0;at<list.length&&!hit;at++){const group=list[at];hit=path.endsWith(group.suffix)&&group.regex.test(path);}
   if(!hit)return false;
   for(let at=0;at<exclude.length;at++)if(exclude[at].test(path))return false;
   return true;
  },
 };
}

const MODULUS=1n<<256n,MEMO_LIMIT=524288,ENTRY_MEMO=new Map(),ROSTER_MEMO=new Map();

/** #️⃣ The additive multiset hash of one `(path, file digest)` entry: sums of these (mod 2^256) are patched by adding and subtracting single entries. */
export const entryHashV1=(path,digest)=>{
 const known=ENTRY_MEMO.get(path);
 if(known!==undefined&&known[0]===digest)return known[1];
 const value=BigInt("0x"+createHash("sha256").update(canonicalJsonV1([path,digest])).digest("hex"));
 if(ENTRY_MEMO.size>=MEMO_LIMIT)ENTRY_MEMO.clear();
 ENTRY_MEMO.set(path,[digest,value]);return value;
};

/** #️⃣ The additive multiset hash of one roster member. */
export const rosterHashV1=path=>{
 let value=ROSTER_MEMO.get(path);
 if(value===undefined){
  value=BigInt("0x"+createHash("sha256").update(path).digest("hex"));
  if(ROSTER_MEMO.size>=MEMO_LIMIT)ROSTER_MEMO.clear();
  ROSTER_MEMO.set(path,value);
 }
 return value;
};

/** #️⃣ A sum modulo 2^256 as 64 hex digits. */
export const hexOfV1=value=>(((value%MODULUS)+MODULUS)%MODULUS).toString(16).padStart(64,"0");

const globDigest=async(ctx,root,patterns)=>await ctx.globDelta(root,patterns)??ctx.glob(root,patterns);

/** 🔎 Re-checks an observation the `directory-names` rule recorded: the listing of the directory is still the recorded one. */
export const verifyObservedV1=async(ref,{environment})=>{
 const names=sorted(await environment.readdir(ref.rows.dir));
 return names.length===ref.rows.names.length&&names.every((name,index)=>name===ref.rows.names[index]);
};

/** 🧩 The rule of every node kind: a pure function of its raw reads and the nodes it reads. */
export const KIND_RULES_V1=Object.freeze({
 async manifest(ctx){
  const bytes=await ctx.file(ctx.self.subject);
  return bytes===undefined?null:parseManifestV1(ctx.self.subject,Buffer.from(bytes).toString("utf8"));
 },
 async project(ctx){
  const project=ctx.params,manifests=await ctx.nodes(project.manifests.map(path=>manifestKey(ctx,path)));
  const values=manifests.map(node=>node.value),declared=values.find(value=>value?.format==="project.json"),named=values.find(value=>value?.name);
  return{root:project.root,name:declared?.name??named?.name??null,dependencies:unique(values.flatMap(dependencyNames)),namedInputs:declared?.namedInputs??{},targets:declared?.targets??{}};
 },
 async "named-input"(ctx){
  const{root,name,project}=ctx.params,declared=await ctx.node(projectKey(ctx,project));
  const entries=(declared.namedInputs[name]??[]).filter(entry=>typeof entry==="string"),refs=unique(entries.filter(entry=>entry!==name&&declared.namedInputs[entry]!==undefined));
  const tails=await ctx.digests(refs.map(ref=>ctx.key("named-input",root+"#"+ref,{root,name:ref,project}))),result=await globDigest(ctx,root,entries.filter(entry=>!refs.includes(entry)));
  return refs.length?{files:result.files,digest:digestOf([result.digest,...tails])}:result;
 },
 async "target-inputs"(ctx){
  const{root,target,project}=ctx.params,declared=await ctx.node(projectKey(ctx,project)),spec=declared.targets[target];
  if(!spec)return{command:null,inputs:[]};
  const inputs=[],named=[],globs=[];
  for(const entry of spec.inputs.filter(item=>typeof item==="string")){const name=entry.replace(/^\^/u,"");if(declared.namedInputs[name]!==undefined)named.push(name);else globs.push(entry);}
  const nodes=await ctx.digests(unique(named).map(name=>ctx.key("named-input",root+"#"+name,{root,name,project})));
  unique(named).forEach((name,index)=>inputs.push(["named:"+name,nodes[index]]));
  if(globs.length)inputs.push(["globs",(await globDigest(ctx,root,globs)).digest]);
  return{command:spec.command,inputs};
 },
 async "action-key"(ctx){
  const{root,target,project,env}=ctx.params,inputs=await ctx.node(ctx.key("target-inputs",root+":"+target,{root,target,project}));
  return{key:digestOf({command:inputs.command,inputs:inputs.inputs,env:env??{}})};
 },
 async "project-config"(ctx){
  const root=ctx.self.subject,base=root===""?"":root+"/",files={};
  for(const name of PROJECT_CONFIG_FILES_V1){const bytes=await ctx.file(base+name);files[name]=bytes===undefined?null:configEntry(name,bytes);}
  return{root,files};
 },
 async "project-set"(ctx){
  await ctx.glob("",[...PROJECT_ROSTER_PATTERNS_V1]);
  const{projectFiles,cargoFiles}=ROSTER_RULES_V1.candidates(await ctx.environment.listProjectFiles(ctx.signal)),withProject=new Set(projectFiles.map(dirOf)),roots=unique([...withProject,...cargoFiles.map(dirOf)]);
  const configs=await ctx.nodes(roots.map(root=>ctx.key("project-config",root,{})));
  const projects=roots.map((root,index)=>({root,...projectOfConfigV1(configs[index].value,withProject.has(root)),files:PROJECT_CONFIG_FILES_V1.filter(name=>configs[index].value.files[name]!==null)})),names={},duplicates=new Set();
  for(const project of projects)if(project.name!==null){if(Object.hasOwn(names,project.name))duplicates.add(project.name);else names[project.name]=project.root;}
  return{projects,names,duplicates:sorted(duplicates)};
 },
 async "task-key"(ctx){
  const{root,target,env,declaration}=ctx.params,config=await ctx.node(ctx.key("project-config",root,{})),entry=config.files["📋️project.json"]??config.files["project.json"],spec=declaration??entry?.json?.targets?.[target];
  if(spec===undefined)return{key:null};
  const base=root===""?"":root+"/",project={root,manifests:PROJECT_CONFIG_FILES_V1.filter(name=>config.files[name]!==null).map(name=>base+name)};
  const inputs=await ctx.node(ctx.key("target-inputs",root+":"+target,{root,target,project}));
  return{key:digestOf({spec,inputs:inputs.inputs,env:env??{}})};
 },
 async "directory-names"(ctx){
  const dir=ctx.self.subject,names=sorted(await ctx.environment.readdir(dir));
  ctx.observed({paths:[dir],digest:digestOf(["readdir",dir,names]),rows:{dir,names}});
  return{names};
 },
 async closure(ctx){
  const members=ctx.params.members,digests=await ctx.digests(members.map(member=>ctx.key(member.kind,member.subject,member.params)));
  return{digest:digestOf(digests)};
 },
});
