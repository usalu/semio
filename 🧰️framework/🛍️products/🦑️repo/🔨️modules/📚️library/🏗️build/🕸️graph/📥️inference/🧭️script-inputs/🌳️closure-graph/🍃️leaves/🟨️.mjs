import{createHash}from"node:crypto";
import{lstatSync,readFileSync,readdirSync}from"node:fs";
import{join,relative,resolve}from"node:path";
import{GlobV1}from"../../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🌐️glob/🟦️.ts";
import{sourceTreeFilterV1}from"../../../../../👁️observation/📂️source-names/🟨️.mjs";
import{moduleNodeDigestV1}from"../🟨️.mjs";

const WORKSPACE_V1="{workspaceRoot}/",ABSENT_V1="absent";
const sha=bytes=>createHash("sha256").update(bytes).digest("hex");

/** 🏷️ The workspace name of an absolute path (`{workspaceRoot}/…`), as every node of the closure graph names its file. */
export const workspaceNameV1=(workspaceRoot,path)=>WORKSPACE_V1+relative(workspaceRoot,path).split("\\").join("/");
/** 📍️ The absolute path of a workspace name. */
export const workspacePathV1=(workspaceRoot,name)=>resolve(workspaceRoot,name.slice(WORKSPACE_V1.length));
/** 🔑️ The key of the leaf of one file (distinct from every module key, which starts with a workspace name). */
export const fileLeafKeyV1=name=>JSON.stringify(["file",name]);
/** 🔑️ The key of the node of one glob below a base directory. */
export const globNodeKeyV1=(base,pattern)=>JSON.stringify(["glob",base,pattern]);

/** 🪪️ The stamp of a path as a leaf trusts it on the machine that made it: size, modification and change time, inode, or `!` and the error code; a file written within `racyMilliseconds` of now gets no stamp (it may still change within the clock's resolution), so its leaf is read again. */
export function pathStampV1(path,now=Date.now(),racyMilliseconds=2000){
 let status;try{status=lstatSync(path);}catch(error){return"!"+(error?.code??"E");}
 if(status.mtimeMs>=now-racyMilliseconds||status.ctimeMs>=now-racyMilliseconds)return undefined;
 return status.size+":"+status.mtimeMs+":"+status.ctimeMs+":"+status.ino+":"+(status.isDirectory()?"d":status.isFile()?"f":"o");
}

/** 🍃️ The leaf node of one file: its digest is the SHA-256 of its bytes (`absent` for a file that is not there), so an edit of the bytes moves exactly the closures that reach the file and an edit that leaves the bytes moves none; `extra.stamp` lets the same machine trust it again by one stat, `extra.sha256` lets any machine trust it by its own bytes. */
export function fileLeafNodeV1(workspaceRoot,path){
 const name=workspaceNameV1(workspaceRoot,path),stamp=pathStampV1(path);
 let bytes;try{bytes=readFileSync(path);}catch{bytes=undefined;}
 const content=bytes===undefined?ABSENT_V1:sha(bytes);
 return{key:fileLeafKeyV1(name),name,kind:"file",digest:moduleNodeDigestV1({kind:"file",name,content}),edges:[],extra:{stamp:stamp??null,sha256:content}};
}

/** 🗂️ The files a glob matches below `base` (a workspace-relative directory, `/`-separated), in code-unit order: a sequential walk (one directory handle at a time) that skips the generated and tool directories the source tree skips everywhere; the directories it listed come with their stamps, so a name added or removed below them is seen by one stat each. */
export function globMatchesV1(workspaceRoot,base,pattern){
 const glob=new GlobV1(pattern),files=[],directories=[];
 const walk=directory=>{
  const absolute=join(workspaceRoot,directory);let entries;
  try{entries=readdirSync(absolute,{withFileTypes:true});}catch{directories.push([directory,pathStampV1(absolute)??null]);return;}
  directories.push([directory,pathStampV1(absolute)??null]);
  for(const entry of sourceTreeFilterV1(entries,row=>row.name,row=>row.isDirectory()).sort((left,right)=>left.name<right.name?-1:left.name>right.name?1:0)){
   const path=directory?directory+"/"+entry.name:entry.name;
   if(entry.isDirectory())walk(path);else if(entry.isFile()&&glob.match(base?relative(base,path).split("\\").join("/"):path))files.push(path);
  }
 };
 walk(base);
 return{files:files.sort(),directories};
}

/** 🌐️ The node of one declared glob of non-module inputs below `base`: its digest names the base, the pattern and the matched names, its edges are the leaves of the matched files, so adding or removing a matching file moves the glob and an edit of a matched file moves its leaf, both exactly the closures that reach the glob. Answers the glob node and its file leaves. */
export function globNodesV1(workspaceRoot,base,pattern){
 const{files,directories}=globMatchesV1(workspaceRoot,base,pattern),leaves=files.map(file=>fileLeafNodeV1(workspaceRoot,join(workspaceRoot,file))),names=leaves.map(leaf=>leaf.name);
 return[{key:globNodeKeyV1(base,pattern),name:WORKSPACE_V1+base+"#"+pattern,kind:"glob",digest:moduleNodeDigestV1({kind:"glob",base,pattern,names}),edges:leaves.map(leaf=>leaf.key),extra:{directories}},...leaves];
}

/** 🪪️ Whether a stored leaf or glob node still stands for the disk of this machine without reading a file: every stamp it recorded is the stamp of the path now (a node made inside the racy window has none and is never trusted this way). `memo` shares the stats of one round. */
export function leafStampHoldsV1(workspaceRoot,record,memo=new Map()){
 const now=path=>{let stamp=memo.get(path);if(stamp===undefined){stamp=pathStampV1(path,Number.POSITIVE_INFINITY,0)??"?";memo.set(path,stamp);}return stamp;};
 if(record.kind==="file"){const stamp=record.extra?.stamp;return typeof stamp==="string"&&now(workspacePathV1(workspaceRoot,record.name))===stamp;}
 if(record.kind==="glob"){const listed=record.extra?.directories;return Array.isArray(listed)&&listed.every(([directory,stamp])=>typeof stamp==="string"&&now(join(workspaceRoot,directory))===stamp);}
 return false;
}

/** 🛰️ Whether a leaf another machine made stands for this machine's disk: the SHA-256 of the local bytes equals the one it recorded (`absent` when both have no file). This is the check a worker runs instead of walking: one read per file, no parse, and only the leaves whose bytes match are trusted. */
export function leafContentHoldsV1(workspaceRoot,record){
 if(record.kind!=="file"||typeof record.extra?.sha256!=="string")return false;
 let bytes;try{bytes=readFileSync(workspacePathV1(workspaceRoot,record.name));}catch{bytes=undefined;}
 return(bytes===undefined?ABSENT_V1:sha(bytes))===record.extra.sha256;
}
