import {closeSync,fsyncSync,lstatSync,mkdirSync,openSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {homedir} from "node:os";
import {join,relative} from "node:path";
import {createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const limits=schema.definitions.Limits.const;

/** 📁️ The per-user directory of the memos: `SEMIO_NX_ENGINE_REFERENCES_DIRECTORY` when it names an absolute path, else the platform cache directory of the user, so every process of the machine shares it.
 * @see ./🧬️schema/🔣️.json the limits */
export function nxEngineReferencesDirectoryV1(environment=process.env,platform=process.platform,home=homedir()){
 const named=environment[limits.environment];if(typeof named==="string"&&named.startsWith("/"))return named;
 const base=platform==="darwin"?join(home,"Library","Caches"):platform==="win32"?environment.LOCALAPPDATA??join(home,"AppData","Local"):environment.XDG_CACHE_HOME?.startsWith("/")?environment.XDG_CACHE_HOME:join(home,".cache");
 return join(base,"semio","nx-engine-references");
}

const sha256=text=>createHash("sha256").update(text).digest("hex");
const emptyV1=()=>({version:1,files:{},references:{}});
const stampOfV1=path=>{const row=lstatSync(path,{bigint:true,throwIfNoEntry:false});return row===undefined?"-":row.dev+":"+row.ino+":"+row.size+":"+row.mtimeNs;};
function loadV1(file){
 try{const row=lstatSync(file,{throwIfNoEntry:false});if(!row?.isFile()||row.size>limits.maximumBytes)return emptyV1();const value=JSON.parse(readFileSync(file,"utf8"));return validateJsonSchemaSubset(schema,value).length?emptyV1():value;}catch{return emptyV1();}
}

/** 🧊️ Opens the memo of one checkout: `references(path,scan)` answers a source's literal references from the stamp it was read at, else from the digest of its text, else by one scan; `save()` writes only what the closure used, atomically, and never fails the caller (an absent, oversized, malformed or unwritable memo is an empty one). Any stamp mismatch rereads the text, so a moved or replaced file is never answered from a stale entry.
 * @see ./🧬️schema/🔣️.json the memo format */
export function openNxEngineReferencesMemoV1(root,directory=nxEngineReferencesDirectoryV1()){
 const file=join(directory,sha256(root).slice(0,32)+".json"),state=loadV1(file),used=new Set();let dirty=false;
 return{
  references(path,scan){
   const name=relative(root,path).replaceAll("\\","/"),stamp=stampOfV1(path),known=state.files[name];used.add(name);
   if(known!==undefined&&known.stamp===stamp&&state.references[known.sha256]!==undefined)return state.references[known.sha256];
   const text=readFileSync(path,"utf8"),digest=sha256(text);let found=state.references[digest];
   if(found===undefined){found=scan(text);state.references[digest]=found;dirty=true;}
   if(known===undefined||known.stamp!==stamp||known.sha256!==digest){state.files[name]={stamp,sha256:digest};dirty=true;}
   return found;
  },
  save(){
   const stale=Object.keys(state.files).some(name=>!used.has(name));
   if(!dirty&&!stale)return false;
   try{
    const files={},references={};for(const name of [...used].sort()){const row=state.files[name];if(row===undefined)continue;files[name]=row;references[row.sha256]=state.references[row.sha256];}
    const temporary=file+"."+process.pid+".tmp";mkdirSync(directory,{recursive:true});writeFileSync(temporary,JSON.stringify({version:1,files,references}));const descriptor=openSync(temporary,"r");try{fsyncSync(descriptor);}finally{closeSync(descriptor);}
    try{renameSync(temporary,file);}catch(error){rmSync(temporary,{force:true});throw error;}return true;
   }catch{return false;}
  }
 };
}
