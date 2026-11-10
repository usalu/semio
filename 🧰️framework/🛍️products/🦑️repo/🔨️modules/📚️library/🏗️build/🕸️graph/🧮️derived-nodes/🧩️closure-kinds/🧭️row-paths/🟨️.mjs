import{dirname,isAbsolute,resolve}from"node:path";

const PATH_ROWS=new Set(["file","exists","stat","lstat","fileExists","directoryExists","readdir","readdirNames","typescriptText","analysis","realpath","realpathNative","typescriptRealpath"]);
const FILE_STEP=/[\\/]/u;

/** 🧭️ Everything that can change what one observed row says, as workspace-relative names, so a dirty set that names none of them leaves the row alone: the files it read (`paths`), the directories whose entries it probed (`dirs`), the ancestors of a bare resolution inside the workspace, whose `node_modules` it walked (`bare`; the ones above the checkout are not inputs of the build), and whether a project, manifest or compiler configuration change can move it (`config`); `outside` for a row about a path above the checkout (the directory chain of the root, which no journal watches and which the settler verifies once per pass), `volatile` only for a row nothing is known about.
 * @param {{kind:string;path:string;fact?:unknown}} row
 * @param {(path:string)=>string} relative maps an absolute path to the workspace-relative name (a name outside the workspace is the volatile marker)
 * @param {string} volatileMarker */
export function rowPathsV1(row,relative,volatileMarker){
 const out={paths:[],dirs:[],bare:[],config:false,volatile:false,outside:false},add=(list,path)=>{const name=relative(path);if(name===volatileMarker)out.volatile=true;else list.push(name);};
 if(row.kind==="field"){for(const path of[row.path,...row.fact?.paths??[]])add(out.paths,path);return out;}
 if(PATH_ROWS.has(row.kind)){const name=relative(row.path);if(name===volatileMarker)out.outside=true;else out.paths.push(name);return out;}
 if(row.kind==="registry"){out.config=true;return out;}
 if(row.kind==="resolve"){
  let from,request;try{[from,request]=JSON.parse(row.path);}catch{out.volatile=true;return out;}
  if(typeof from!=="string"||typeof request!=="string"){out.volatile=true;return out;}
  const here=dirname(from);
  if(request.startsWith(".")||isAbsolute(request)){const base=resolve(here,request);add(out.dirs,dirname(base));add(out.dirs,base);}
  else{out.config=true;for(let at=here;;at=dirname(at)){const name=relative(at);if(name===volatileMarker)break;out.bare.push(name);if(at===dirname(at)||!FILE_STEP.test(at))break;}}
  if(typeof row.fact==="string"&&!row.fact.startsWith("!")&&isAbsolute(row.fact)){add(out.paths,row.fact);add(out.dirs,dirname(row.fact));}
  return out;
 }
 out.volatile=true;return out;
}

/** 🧪️ Whether a dirty set can move a row whose reach is `meta`: the row is touched when the set names a file it read, an entry of a directory it probed (or of an ancestor of a bare resolution), a configuration file when it depends on one, or anything in a region the journal does not watch.
 * @param {{paths:readonly string[];dirs:readonly string[];bare:readonly string[];config:boolean;volatile:boolean}} meta
 * @param {{paths:ReadonlySet<string>;directories:ReadonlySet<string>;configs:boolean;modules:boolean;excluded?:(path:string)=>boolean}} dirty */
export function rowTouchedV1(meta,dirty){
 if(meta.volatile)return true;
 if(meta.config&&dirty.configs)return true;
 for(const path of meta.paths)if(dirty.paths.has(path)||dirty.excluded?.(path)===true)return true;
 for(const dir of meta.dirs)if(dirty.directories.has(dir)||dirty.excluded?.(dir)===true)return true;
 if(meta.bare.length&&dirty.modules)return true;
 for(const dir of meta.bare)if(dirty.directories.has(dir))return true;
 return false;
}

const CONFIG_NAMES=new Set(["📋️project.json","Cargo.toml","package.json","tsconfig.json","bun.lock","go.mod"]);
/** 🗂️ The summary of a dirty set the row test needs: its paths, every directory it names or lies in, whether it names a configuration file and whether it reaches into an install tree.
 * @param {{paths:readonly string[];directories?:readonly string[];excluded?:(path:string)=>boolean}} dirty */
export function dirtySummaryV1(dirty){
 const paths=new Set(dirty.paths),directories=new Set(dirty.directories??[]);let configs=false,modules=false;
 for(const path of dirty.paths){
  const cut=path.lastIndexOf("/"),name=path.slice(cut+1);directories.add(cut<0?"":path.slice(0,cut));directories.add(path);
  if(CONFIG_NAMES.has(name))configs=true;
  if(path.includes("node_modules/"))modules=true;
 }
 return{paths,directories,configs,modules,...dirty.excluded?{excluded:dirty.excluded}:{}};
}
