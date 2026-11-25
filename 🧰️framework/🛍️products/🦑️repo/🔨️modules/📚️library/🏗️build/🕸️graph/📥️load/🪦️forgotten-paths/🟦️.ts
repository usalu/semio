import{statSync,readFileSync}from"node:fs";
import{mkdir,rename,rm,writeFile}from"node:fs/promises";
import{dirname}from"node:path";
import{randomUUID}from"node:crypto";
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🪦️ The paths a seal proved stale, as the persisted stores know them: path to the time it was proven.
 * @see ./🧬️schema/🔣️.json */
export type ForgottenPathsV1=Readonly<{paths:ReadonlyMap<string,number>;listings:ReadonlyMap<string,number>}>;

const limits=schema.$defs.Limits.const,none:ForgottenPathsV1={paths:new Map(),listings:new Map()};
const parentOf=(path:string)=>path.slice(0,Math.max(path.lastIndexOf("/"),path.lastIndexOf("\\")));
const held=new Map<string,{key:string;entries:readonly {path:string;at:number}[]}>();
const valid=(value:unknown)=>validateJsonSchemaSubset(schema,value).length===0;

function entriesOf(file:string,now:number):readonly {path:string;at:number}[]{
 try{
  const status=statSync(file),key=status.size+":"+status.mtimeMs,known=held.get(file);
  if(known?.key===key)return known.entries.filter(entry=>now-entry.at<limits.maximumAgeMilliseconds);
  const parsed=JSON.parse(readFileSync(file,"utf8"));
  if(!valid(parsed))return[];
  const entries=(parsed.paths as {path:string;at:number}[]).filter(entry=>now-entry.at<limits.maximumAgeMilliseconds);
  held.set(file,{key,entries});
  return entries;
 }catch{return[];}
}

/** 🪦️ The forgotten paths a file holds (a missing, corrupt or foreign file holds none; entries past the maximum age are dropped), with the directories whose listings they changed. */
export function readForgottenPathsV1(file:string,now:number=Date.now()):ForgottenPathsV1{
 const entries=entriesOf(file,now);
 if(entries.length===0)return none;
 const paths=new Map<string,number>(),listings=new Map<string,number>();
 for(const{path,at}of entries){paths.set(path,Math.max(at,paths.get(path)??0));const parent=parentOf(path);listings.set(parent,Math.max(at,listings.get(parent)??0));}
 return{paths,listings};
}

/** 🪦️ Remembers on disk that a seal proved `paths` stale at `at`, so the rows of persisted stores that name them are refused from then on (merged with what the file holds, the newest entries kept up to the maximum, replaced atomically; a failed write is no refusal of the load and answers false). */
export async function forgetPersistedPathsV1(file:string,paths:readonly string[],at:number=Date.now()):Promise<boolean>{
 const fresh=paths.filter(path=>path.length>0&&path.length<=limits.maximumPathCharacters);
 if(fresh.length===0)return true;
 const merged=new Map<string,number>();
 for(const entry of entriesOf(file,at))merged.set(entry.path,entry.at);
 for(const path of fresh)merged.set(path,at);
 const kept=[...merged].sort((left,right)=>right[1]-left[1]).slice(0,limits.maximumPaths).map(([path,when])=>({path,at:when}));
 const temporary=file+"."+randomUUID();
 try{
  await mkdir(dirname(file),{recursive:true});
  await writeFile(temporary,JSON.stringify({version:1,paths:kept}),{flag:"wx"});
  await rename(temporary,file);
  return true;
 }catch{return false;}
 finally{await rm(temporary,{force:true}).catch(()=>undefined);}
}

/** 🪦️ Whether a stored record whose rows are `rows`, stored at `storedAt`, is refused: one of its rows names a forgotten path (or lists its directory) that was proven stale at or after the record was stored. */
export function rowsForgottenV1(forgotten:ForgottenPathsV1,rows:readonly unknown[],storedAt:number):boolean{
 if(forgotten.paths.size===0)return false;
 const stored=Math.floor(storedAt);
 for(const row of rows){
  const{kind,path}=row as {kind?:unknown;path?:unknown};
  if(typeof path!=="string")continue;
  const named=forgotten.paths.get(path);
  if(named!==undefined&&stored<=named)return true;
  if(kind==="readdir"||kind==="readdirNames"){const listed=forgotten.listings.get(path);if(listed!==undefined&&stored<=listed)return true;}
 }
 return false;
}
