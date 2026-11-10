import{stat}from"node:fs/promises";
import{basename,dirname,isAbsolute,resolve}from"node:path";
import{createYielderV1}from"../🧱️guards/🟨️.mjs";

const ABSENT=new Set(["exists","fileExists","directoryExists"]),LISTING=new Set(["readdir","readdirNames"]);
const parentOf=path=>{const cut=Math.max(path.lastIndexOf("/"),path.lastIndexOf("\\"));return cut<0?"":path.slice(0,cut);};

/** 🗂️ The directory whose stamp vouches for a structural row: the listed directory of a listing, the parent of the entry of an `lstat` or of an absence; undefined for every other row (an existing entry may be a symlink whose target moves without touching its directory). */
export function structuralDirectoryV1(row){
 if(LISTING.has(row.kind))return row.path;
 if(row.kind==="lstat")return parentOf(row.path);
 if(ABSENT.has(row.kind)&&row.fact===false)return parentOf(row.path);
 return undefined;
}

const EXISTING=new Set(["exists","fileExists","directoryExists","stat"]);

/** 🧱️ The directory and child name of a row that says an entry exists (`exists`, `fileExists`, `directoryExists` true, `stat` of a type): its answer cannot change while the directory keeps its stamp and the entry is no symbolic link (a plain entry only changes by unlink and create, which move the directory's times); undefined for every other row. */
export function existingEntryV1(row){
 if(!EXISTING.has(row.kind)||typeof row.path!=="string"||row.path===""||row.path.endsWith("/")||row.path.endsWith("\\"))return undefined;
 if(row.kind==="stat"?(typeof row.fact!=="string"||row.fact.startsWith("!")||row.fact==="symlink"):row.fact!==true)return undefined;
 return{directory:parentOf(row.path),name:basename(row.path)};
}

/** 🧭️ The directory and base name a relative or absolute module resolution found its file in: the probes of Node's file resolution (the name, then each extension) are all entries of that one directory, so the answer cannot change while the directory keeps its stamp and none of the probed names (every name that starts with the base name) is a symbolic link; undefined for bare specifiers and directory resolutions (their probe chains are not recorded). */
export function siblingResolutionV1(row){
 if(row.kind!=="resolve"||typeof row.fact!=="string"||row.fact.startsWith("!"))return undefined;
 let from,request;try{[from,request]=JSON.parse(row.path);}catch{return undefined;}
 if(typeof from!=="string"||typeof request!=="string"||!(request.startsWith("./")||request.startsWith("../")||isAbsolute(request)))return undefined;
 const base=resolve(dirname(from),request),directory=dirname(base);
 return dirname(row.fact)===directory?{directory,name:basename(base)}:undefined;
}

/** 🕳️ Whether a row says an entry is absent (`exists`, `fileExists`, `directoryExists` false): while its parent directory is missing (or no directory) the entry cannot exist, whatever any stamp says. */
const absenceV1=row=>ABSENT.has(row.kind)&&row.fact===false;

const nestedOf=row=>row.kind==="analysis"&&Array.isArray(row.fact?.nestedObservations)?row.fact.nestedObservations:[];
const contentOf=row=>row.kind==="file"&&row.fact&&row.fact.error===undefined?row.fact:row.kind==="analysis"?row.fact?.source:undefined;
const digestOf=row=>row.kind==="digest"&&row.fact&&row.fact.sha256!==null?row.fact:undefined;

/** 🧭️ Every directory and content path a row and its nested observations depend on. */
export function rowDependenciesV1(row,directories,paths){
 const directory=structuralDirectoryV1(row);if(directory!==undefined)directories.add(directory);
 const entry=existingEntryV1(row)??siblingResolutionV1(row);if(entry!==undefined)directories.add(entry.directory);
 if(contentOf(row)||digestOf(row))paths.add(row.path);
 for(const nested of nestedOf(row))rowDependenciesV1(nested,directories,paths);
}

/**
 * 🔎️ Decides which fact rows are doubtful when no journal vouches: a structural row is vouched for by the unchanged, non-racy stamp of its directory, a `file`, `digest` or `analysis` row by one stat of its file that equals the recorded stamp (an analysis also needs its nested observations vouched for), every other row is doubtful. Only doubtful rows reach the precise check, so a fact check costs one stat per distinct directory and file instead of one call per row.
 */
export function createEpochDoubtV1({stamps,statFile=path=>stat(path).catch(()=>undefined),concurrency=128}){
 const directories=new Map(),files=new Map(),links=new Map(),missing=new Set();let measured=new Map();
 const settle=async(wanted,known,signal,measure,progress,phase)=>{
  const list=[...wanted].filter(path=>!known.has(path));let next=0,done=0;
  await progress?.({phase,done:0,total:list.length});
  await Promise.all(Array.from({length:Math.min(concurrency,list.length)},async()=>{while(next<list.length){signal?.throwIfAborted();const path=list[next++];await measure(path);done++;if(done%2048===0||done===list.length)await progress?.({phase,done,total:list.length});}}));
 };
 const doubtful=row=>{
  const directory=structuralDirectoryV1(row);
  if(directory!==undefined){
   if(directories.get(directory)===true)return ABSENT.has(row.kind)&&links.get(directory)?.has(basename(row.path))===true;
   return !(absenceV1(row)&&missing.has(directory));
  }
  const existing=existingEntryV1(row);
  if(existing!==undefined)return !(directories.get(existing.directory)===true&&!links.get(existing.directory)?.has(existing.name));
  const sibling=siblingResolutionV1(row);
  if(sibling!==undefined)return !(directories.get(sibling.directory)===true&&![...links.get(sibling.directory)??[]].some(name=>name.startsWith(sibling.name)));
  const nested=nestedOf(row).some(doubtful);
  if(nested)return true;
  const content=contentOf(row);
  if(content){const found=files.get(row.path);return !(found&&found[0]===content.size&&found[1]===content.mtimeMs&&found[2]===content.ctimeMs&&found[3]===content.ino);}
  const digest=digestOf(row);
  if(digest){const found=files.get(row.path);return !(found&&found[0]===digest.size&&found[1]===digest.mtimeMs);}
  return true;
 };
 return Object.freeze({
  /** 🔢️ Classifies `rows`: 1 where the row is doubtful. */
  async classify(rows,{signal,progress}={}){
   const wantedDirectories=new Set(),wantedPaths=new Set();
   for(const row of rows)rowDependenciesV1(row,wantedDirectories,wantedPaths);
   const fresh=[...wantedDirectories].filter(path=>!directories.has(path));
   if(fresh.length){const{unchanged,measured:taken,links:names,missing:gone}=await stamps.classify(fresh,{signal,progress});for(const path of gone)missing.add(path);for(const path of fresh)directories.set(path,unchanged.has(path));for(const[path,list]of names)links.set(path,new Set(list));for(const[path,row]of taken)measured.set(path,row);}
   await settle(wantedPaths,files,signal,async path=>{const found=await statFile(path);files.set(path,found?.isFile?.()?[found.size,found.mtimeMs,found.ctimeMs,Number(found.ino)]:null);},progress,"files");
   return Uint8Array.from(rows,row=>doubtful(row)?1:0);
  },
  /** 💾️ After the check: keeps the stamps of the directories whose rows were all verified to hold (`verified(id)`) and none stale, so the next pass vouches for them. */
  async settled({rows,ids,stale}){
   const selected=new Uint8Array(rows.length),verified=id=>selected[id]===1,exclude=new Set();for(const id of ids)selected[id]=1;
   for(let id=0;id<rows.length;id++){
    const row=rows[id],own=new Set(),paths=new Set();rowDependenciesV1(row,own,paths);
    if(!own.size)continue;
    if(!verified(id)||stale.has(id))for(const path of own)exclude.add(path);
   }
   return stamps.save(measured,exclude);
  },
  /** 📦️ After a generation is published: stamps every directory its structural rows look into, as far as it was untouched since `observedAt` (before the facts were observed), so a fresh process vouches for those directories at once. */
  async published({rows,observedAt}){
   const wanted=new Set(),ignored=new Set();
   for(const row of rows)rowDependenciesV1(row,wanted,ignored);
   const{measured:taken}=await stamps.classify([...wanted],{observedAt});
   return stamps.save(taken);
  },
  stats:()=>({directories:directories.size,files:files.size}),
 });
}

/** 🔢️ Keeps the ids whose row is doubtful: `doubt.classify(rows)` answers 1 for a row that must be checked, 0 for one a journal or a stamp vouches for. Cancellable, reports `{phase:"doubt",done,total}`. */
export async function selectDoubtfulRowsV1(index,ids,doubt,{signal,progress,chunk}){
 const kept=[],yielder=createYielderV1();let done=0;
 await progress?.({phase:"doubt",done:0,total:ids.length});
 for(let at=0;at<ids.length;at+=chunk){
  signal?.throwIfAborted();await yielder();
  const part=ids.slice(at,at+chunk),flags=await doubt.classify(part.map(id=>index.rows[id]),{signal,progress});
  for(let offset=0;offset<part.length;offset++)if(flags[offset])kept.push(part[offset]);
  done+=part.length;await progress?.({phase:"doubt",done,total:ids.length});
 }
 return kept;
}
