import{createHash}from"node:crypto";
import{lstat,readdir,readFile,readlink}from"node:fs/promises";
import{dirname,isAbsolute,join,resolve}from"node:path";

const PATH_ROWS=new Set(["file","exists","stat","lstat","fileExists","directoryExists","readdir","readdirNames","typescriptText","analysis","realpath","realpathNative","typescriptRealpath"]);
const FILE_STEP=/[\\/]/u;

const OWNER_REGISTRY_FILE="📋️build-owners.json";

/** 🧭️ Everything that can change what one observed row says, as workspace-relative names, so a dirty set that names none of them leaves the row alone: the files it read or probed (`paths`, a probe of an absent name included), the directories whose entries a relative resolution probed (`dirs`), the registry file an owner row was read from, and `outside` for a row the journal cannot vouch for and that is therefore checked by asking it again (a bare module resolution, which depends on install trees, manifests and compiler configuration that no path list can bound soundly, and every row about a path above the checkout); `volatile` only for a row nothing is known about.
 * @param {{kind:string;path:string;fact?:unknown}} row
 * @param {(path:string)=>string} relative maps an absolute path to the workspace-relative name (a name outside the workspace is the volatile marker)
 * @param {string} volatileMarker */
export function rowPathsV1(row,relative,volatileMarker){
 /** @type {{paths:string[];dirs:string[];outside:boolean;volatile:boolean}} */
 const out={paths:[],dirs:[],outside:false,volatile:false},add=(list,path)=>{const name=relative(path);if(name===volatileMarker)out.volatile=true;else list.push(name);};
 if(row.kind==="field"){for(const path of[row.path,...row.fact?.paths??[]])add(out.paths,path);return out;}
 if(PATH_ROWS.has(row.kind)){const name=relative(row.path);if(name===volatileMarker)out.outside=true;else out.paths.push(name);return out;}
 if(row.kind==="registry"){
  let root;try{[root]=JSON.parse(row.path);}catch{out.volatile=true;return out;}
  if(typeof root!=="string"){out.volatile=true;return out;}
  const name=relative(join(root,OWNER_REGISTRY_FILE));if(name===volatileMarker)out.outside=true;else out.paths.push(name);
  return out;
 }
 if(row.kind==="resolve"){
  let from,request;try{[from,request]=JSON.parse(row.path);}catch{out.volatile=true;return out;}
  if(typeof from!=="string"||typeof request!=="string"){out.volatile=true;return out;}
  if(request.startsWith(".")||isAbsolute(request)){const base=resolve(dirname(from),request);add(out.dirs,dirname(base));add(out.dirs,base);}
  else out.outside=true;
  if(typeof row.fact==="string"&&!row.fact.startsWith("!")&&isAbsolute(row.fact)){add(out.paths,row.fact);add(out.dirs,dirname(row.fact));}
  return out;
 }
 out.volatile=true;return out;
}

/** 🌳️ Whether a name lies below one of the roots (directories that were added, removed or renamed, so everything that was or is below them moved). */
export function belowRootsV1(name,roots){
 if(roots===undefined||roots.size===0)return false;
 for(let at=name;;){const cut=at.lastIndexOf("/");if(cut<0)return false;at=at.slice(0,cut);if(roots.has(at))return true;}
}

/** 🧪️ Whether a dirty set can move a row whose reach is `meta`: the row is touched when the set names a file it read or probed, an entry of a directory it probed, anything below a changed directory subtree, or anything in a region the journal does not watch.
 * @param {{paths:readonly string[];dirs:readonly string[];volatile:boolean}} meta
 * @param {{paths:ReadonlySet<string>;directories:ReadonlySet<string>;subtrees?:ReadonlySet<string>;excluded?:(path:string)=>boolean}} dirty */
export function rowTouchedV1(meta,dirty){
 if(meta.volatile)return true;
 for(const path of meta.paths)if(dirty.paths.has(path)||dirty.excluded?.(path)===true||belowRootsV1(path,dirty.subtrees))return true;
 for(const dir of meta.dirs)if(dirty.directories.has(dir)||dirty.excluded?.(dir)===true||belowRootsV1(dir,dirty.subtrees))return true;
 return false;
}

/** 🗂️ The summary of a dirty set the row test needs: its paths, every directory it names or lies in, and the roots of the subtrees that were added, removed or renamed.
 * @param {{paths:readonly string[];directories?:readonly string[];excluded?:(path:string)=>boolean}} dirty */
export function dirtySummaryV1(dirty){
 const paths=new Set(dirty.paths),subtrees=new Set(dirty.directories??[]),directories=new Set(subtrees);
 for(const path of dirty.paths){const cut=path.lastIndexOf("/");directories.add(cut<0?"":path.slice(0,cut));directories.add(path);}
 return{paths,directories,subtrees,...dirty.excluded?{excluded:dirty.excluded}:{}};
}

/** 🧮️ The dirty summary of a read that has no change journal: the fact rows that were just verified against the disk and no longer hold are exactly what moved since the generation, so their reach (files and probed directories) is the set the closure parts are tested against; a row about a path above the checkout is checked once per pass by the settler itself, a row nothing is known about marks its parts volatile, which are always asked again. A part that holds no stale row can reach none of them, so it is never skipped wrongly.
 * @param {Iterable<{kind:string;path:string;fact?:unknown}>} rows @param {(path:string)=>string} relative @param {string} volatileMarker */
export function staleSummaryV1(rows,relative,volatileMarker){
 const paths=new Set(),directories=new Set();
 for(const row of rows){
  const one=rowPathsV1(row,relative,volatileMarker);
  for(const path of one.paths)paths.add(path);
  for(const dir of one.dirs)directories.add(dir);
 }
 const summary=dirtySummaryV1({paths:[...paths]});
 for(const dir of directories)summary.directories.add(dir);
 return summary;
}

const RACY_MILLISECONDS=2000;
const stampMemo=new Map();
/** 🧾️ The stamp of one path as it is on the disk now: the sha256 of the bytes of a file (remembered by its size, times and inode, so an untouched file is read once per process), the sorted names and kinds of a directory, the target of a link, `-` for an absent path; undefined when the path cannot be read, which no vouch may rest on. With `cheap` a file is stamped by its size, both times and inode alone (never read), and a file written within the last two seconds has none: for the install trees and build outputs below the paths the journal does not watch, which are far too many and too large to hash at every start.
 * @param {string} absolute @param {number} [memoEntries] @param {boolean} [cheap] */
export async function stampOfV1(absolute,memoEntries=65536,cheap=false){
 let info;try{info=await lstat(absolute,{bigint:true});}catch{return"-";}
 if(info.isDirectory()){
  const entries=await readdir(absolute,{withFileTypes:true}).catch(()=>undefined);if(entries===undefined)return undefined;
  const hash=createHash("sha256");for(const name of entries.map(entry=>entry.name+(entry.isDirectory()?"/":entry.isSymbolicLink()?"@":"")).sort())hash.update(name).update("\0");
  return"d:"+hash.digest("hex");
 }
 if(info.isSymbolicLink()){const target=await readlink(absolute).catch(()=>undefined);return target===undefined?undefined:"l:"+target;}
 if(!info.isFile())return"o:"+info.mode;
 if(cheap)return Date.now()-Number(info.mtimeNs/1000000n)<RACY_MILLISECONDS?undefined:"s:"+info.size+":"+info.mtimeNs+":"+info.ctimeNs+":"+info.ino;
 const key=absolute+"\0"+info.size+"\0"+info.mtimeNs+"\0"+info.ctimeNs+"\0"+info.ino,known=stampMemo.get(key);
 if(known!==undefined)return known;
 let bytes;try{bytes=await readFile(absolute);}catch{return undefined;}
 const value="f:"+createHash("sha256").update(bytes).digest("hex");
 stampMemo.set(key,value);if(stampMemo.size>memoEntries)stampMemo.delete(stampMemo.keys().next().value);
 return value;
}
