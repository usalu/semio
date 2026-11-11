import {createHash} from "node:crypto";
import {createReadStream} from "node:fs";
import {lstat,opendir,readlink,stat} from "node:fs/promises";
import {join,sep} from "node:path";

/** 🙈️ A compiled root `.gitignore`: the digest of its text and the verdict for one worktree-relative path; `child` decides a path whose parent directories are already known to be shared, without walking or caching the ancestors. */
export type FleetIgnoreV1=Readonly<{digest:string;ignores(path:string,isDirectory:boolean):boolean;child?(path:string,isDirectory:boolean):boolean}>;

/** 🚫️ Why a path is never shared. */
export type FleetPathRefusalV1="not-nfc"|"too-long"|"component-too-long"|"illegal-character"|"trailing-dot-or-space"|"reserved-name"|"case-collision"|"unsafe-link";

/** 📄️ One shared file or link as the scanner saw it. */
export type FleetScanEntryV1=Readonly<{path:string;kind:"file"|"link";bytes:number;mtimeMs:number;executable:boolean;target?:string}>;

/** 🗂️ The shared entries and the refused paths of one scan, both sorted by path. */
export type FleetScanV1=Readonly<{entries:readonly FleetScanEntryV1[];refused:readonly Readonly<{path:string;reason:FleetPathRefusalV1}>[]}>;

type Rule=Readonly<{index:number;negate:boolean;dirOnly:boolean;anchored:boolean;expression:RegExp;literal:string|undefined}>;

const POSIX:Readonly<Record<string,string>>={alpha:"A-Za-z",digit:"0-9",alnum:"A-Za-z0-9",upper:"A-Z",lower:"a-z",space:"\\t\\n\\v\\f\\r ",blank:"\\t ",punct:"!-/:-@\\[-`{-~",xdigit:"0-9A-Fa-f",cntrl:"\\u0000-\\u001f\\u007f",print:" -~",graph:"!-~",word:"A-Za-z0-9_"};
const ILLEGAL=/[<>:"|?*\\\u0000-\u001f\u007f-\u009f]/u;
const RESERVED=/^(?:CON|PRN|AUX|NUL|COM[1-9\u00b9\u00b2\u00b3]|LPT[1-9\u00b9\u00b2\u00b3])$/u;
const LIMIT=20000;

const code=(char:string):number=>char.codePointAt(0) as number;
const escapeChar=(char:string):string=>code(char)>127||/^[A-Za-z0-9]$/u.test(char)?char:`\\u{${code(char).toString(16)}}`;
/** 🔤️ Folds a file name so that names which collide on a case-insensitive file system (NTFS, APFS, HFS+) fold to the same text: upper case first, so that the final and the medial sigma, or `ß` and `SS`, meet, then lower case. */
export const foldNameV1=(name:string):string=>name.toUpperCase().toLowerCase();
const compare=(left:string,right:string):number=>left<right?-1:left>right?1:0;
const windowsSeparators=(target:string):string=>process.platform==="win32"?target.replaceAll("\\","/"):target;
const vanished=(error:unknown):boolean=>{const name=(error as {code?:string}).code;return name==="ENOENT"||name==="ENOTDIR";};
const overlong=(error:unknown):boolean=>(error as {code?:string}).code==="ENAMETOOLONG";

function parseClass(chars:readonly string[],open:number):Readonly<{source:string;next:number}>|undefined{
 let j=open+1,negate=false,items="",first=true;
 if(chars[j]==="!"||chars[j]==="^"){negate=true;j++;}
 for(;j<chars.length;first=false){
  let char=chars[j];
  if(char==="]"&&!first)return {source:items===""&&!negate?"(?!)":`[${negate?"^/":""}${items}]`,next:j+1};
  if(char==="["&&chars[j+1]===":"){
   const close=chars.findIndex((c,k)=>k>j+1&&c===":"&&chars[k+1]==="]"),name=close<0?"":chars.slice(j+2,close).join("");
   if(Object.hasOwn(POSIX,name)){items+=POSIX[name];j=close+2;continue;}
  }
  if(char==="\\"&&j+1<chars.length)char=chars[++j];
  if(chars[j+1]==="-"&&j+2<chars.length&&chars[j+2]!=="]"){
   let high=chars[j+2],skip=3;
   if(high==="\\"&&j+3<chars.length){high=chars[j+3];skip=4;}
   if(code(char)<=code(high))items+=`${escapeChar(char)}-${escapeChar(high)}`;
   j+=skip;continue;
  }
  items+=escapeChar(char);j++;
 }
 return undefined;
}

function translate(glob:string):Readonly<{source:string;literal:string|undefined}>{
 const chars=Array.from(glob);
 let source="",literal="",plain=true,i=0;
 while(i<chars.length){
  const char=chars[i];
  if(char==="*"){
   let end=i;while(chars[end+1]==="*")end++;
   const next=chars[end+1];plain=false;
   if(end>i&&(i===0||chars[i-1]==="/")&&(next===undefined||next==="/")){
    if(next==="/"){source+="(?:.*/)?";i=end+2;}else{source+=i===0?".*":".+";i=end+1;}
    continue;
   }
   source+="[^/]*";i=end+1;continue;
  }
  if(char==="?"){source+="[^/]";plain=false;i++;continue;}
  if(char==="["){
   const parsed=parseClass(chars,i);
   if(parsed){source+=parsed.source;plain=false;i=parsed.next;continue;}
  }
  const literalChar=char==="\\"&&i+1<chars.length?chars[++i]:char;
  source+=escapeChar(literalChar);literal+=literalChar;i++;
 }
 return {source,literal:plain?literal:undefined};
}

function parseLine(raw:string,index:number):Rule|undefined{
 let line=raw.endsWith("\r")?raw.slice(0,-1):raw,end=line.length;
 while(end>0&&line[end-1]===" ")end--;
 if(end<line.length){let slashes=0;while(slashes<end&&line[end-1-slashes]==="\\")slashes++;if(slashes%2===1)end++;}
 line=line.slice(0,end);
 if(line===""||line.startsWith("#"))return undefined;
 const negate=line.startsWith("!");
 if(negate)line=line.slice(1);
 const dirOnly=line.endsWith("/");
 if(dirOnly)line=line.slice(0,-1);
 const anchored=line.includes("/");
 if(line.startsWith("/"))line=line.slice(1);
 if(line==="")return undefined;
 const {source,literal}=translate(line);
 return {index,negate,dirOnly,anchored,expression:new RegExp(`^(?:${source})$`,"su"),literal};
}

/** 🧱️ Compiles gitignore text with full semantics (anchoring, `**`, negation, directory-only rules, classes, escapes) into a dependency-free matcher; a path below an excluded directory is always excluded. */
export function compileIgnoreV1(gitignoreText:string):FleetIgnoreV1{
 const rules:Rule[]=[],literals=new Map<string,Rule[]>(),wild:Rule[]=[],anchored:Rule[]=[];
 for(const line of gitignoreText.split("\n")){const rule=parseLine(line,rules.length);if(rule)rules.push(rule);}
 for(const rule of rules){
  if(rule.anchored)anchored.push(rule);
  else if(rule.literal!==undefined)literals.set(rule.literal,[...(literals.get(rule.literal)??[]),rule]);
  else wild.push(rule);
 }
 const names=new Map<string,number>(),directories=new Map<string,boolean>();
 const byName=(name:string,isDirectory:boolean):number=>{
  const key=`${isDirectory?1:0}${name}`,known=names.get(key);
  if(known!==undefined)return known;
  let best=-1;
  for(const rule of literals.get(name)??[])if(isDirectory||!rule.dirOnly)best=Math.max(best,rule.index);
  for(let k=wild.length-1;k>=0&&wild[k].index>best;k--){const rule=wild[k];if((isDirectory||!rule.dirOnly)&&rule.expression.test(name)){best=rule.index;break;}}
  if(names.size>LIMIT)names.clear();
  names.set(key,best);return best;
 };
 const decide=(path:string,isDirectory:boolean):boolean=>{
  let best=byName(path.slice(path.lastIndexOf("/")+1),isDirectory);
  for(let k=anchored.length-1;k>=0&&anchored[k].index>best;k--){const rule=anchored[k];if((isDirectory||!rule.dirOnly)&&rule.expression.test(path)){best=rule.index;break;}}
  return best<0?false:!rules[best].negate;
 };
 const ignores=(path:string,isDirectory:boolean):boolean=>{
  const clean=path.endsWith("/")?path.slice(0,-1):path;
  if(clean==="")return false;
  for(let cut=clean.indexOf("/");cut>0;cut=clean.indexOf("/",cut+1)){
   const prefix=clean.slice(0,cut);
   let state=directories.get(prefix);
   if(state===undefined){state=decide(prefix,true);if(directories.size>LIMIT)directories.clear();directories.set(prefix,state);}
   if(state)return true;
  }
  const state=decide(clean,isDirectory);
  if(isDirectory)directories.set(clean,state);
  return state;
 };
 const child=(path:string,isDirectory:boolean):boolean=>decide(path.endsWith("/")?path.slice(0,-1):path,isDirectory);
 return Object.freeze({digest:createHash("sha256").update(gitignoreText).digest("hex"),ignores,child});
}

/** 🛂️ Names why a worktree-relative path is never shared (NFC, length, illegal characters, trailing dot or space, device names); `undefined` means the path is acceptable. Case collisions are decided by the scanner. */
export function refusePathV1(path:string):FleetPathRefusalV1|undefined{
 if(path!==path.normalize("NFC"))return "not-nfc";
 if(path.length>256&&Array.from(path).length>256)return "too-long";
 const parts=path.split("/");
 if(parts.some(part=>Buffer.byteLength(part)>255))return "component-too-long";
 if(ILLEGAL.test(path))return "illegal-character";
 if(parts.some(part=>part.endsWith(".")||part.endsWith(" ")))return "trailing-dot-or-space";
 if(parts.some(part=>RESERVED.test(part.split(".")[0].replace(/ +$/u,"").toUpperCase())))return "reserved-name";
 return undefined;
}

function refuseLeaf(path:string,name:string):FleetPathRefusalV1|undefined{
 if(name!==name.normalize("NFC"))return "not-nfc";
 if(path.length>256&&Array.from(path).length>256)return "too-long";
 if(Buffer.byteLength(name)>255)return "component-too-long";
 if(ILLEGAL.test(name))return "illegal-character";
 if(name.endsWith(".")||name.endsWith(" "))return "trailing-dot-or-space";
 if(RESERVED.test(name.split(".")[0].replace(/ +$/u,"").toUpperCase()))return "reserved-name";
 return undefined;
}

/** 🔗️ Refuses a symbolic link whose target is absolute, drive-qualified, empty or climbs out of the worktree; backslashes count as separators. */
export function linkRefusalV1(path:string,target:string):FleetPathRefusalV1|undefined{
 const normal=target.replaceAll("\\","/");
 if(normal===""||normal.includes("\0")||normal.startsWith("/")||/^[A-Za-z]:/u.test(normal))return "unsafe-link";
 let depth=path.split("/").length-1;
 for(const part of normal.split("/")){
  if(part===""||part===".")continue;
  if(part===".."){if(--depth<0)return "unsafe-link";}else depth++;
 }
 return undefined;
}

/** 🤝️ Tells whether a worktree-relative path is shared: not ignored, not below any `.git`, and acceptable to the path policy. */
export function sharedV1(path:string,isDirectory:boolean,ignore:FleetIgnoreV1):boolean{
 return !path.split("/").includes(".git")&&!ignore.ignores(path,isDirectory)&&refusePathV1(path)===undefined;
}

type Candidate={name:string;relative:string;kind:"directory"|"file"|"link"};

async function scanDirectory(worktree:string,directory:string,ignore:FleetIgnoreV1,signal:AbortSignal|undefined,entries:Pick<FleetScanEntryV1[],"push">,refused:{path:string;reason:FleetPathRefusalV1}[]):Promise<string[]>{
 signal?.throwIfAborted();
 const absolute=directory===""?worktree:join(worktree,directory),candidates:Candidate[]=[],subdirectories:string[]=[],within=(name:string):string=>`${absolute}${sep}${name}`;
 try{
  for await(const item of await opendir(absolute)){
   signal?.throwIfAborted();
   if(item.name===".git")continue;
   const relative=directory===""?item.name:`${directory}/${item.name}`;
   let kind:Candidate["kind"]|undefined=item.isDirectory()?"directory":item.isSymbolicLink()?"link":item.isFile()?"file":undefined;
   if(kind===undefined&&!item.isSocket()&&!item.isFIFO()&&!item.isCharacterDevice()&&!item.isBlockDevice()){
    const info=await lstat(within(item.name));
    kind=info.isDirectory()?"directory":info.isSymbolicLink()?"link":info.isFile()?"file":undefined;
   }
   if(kind!==undefined&&!(ignore.child??ignore.ignores)(relative,kind==="directory"))candidates.push({name:item.name,relative,kind});
  }
 }catch(error){
  if(directory!==""&&overlong(error)){refused.push({path:directory,reason:"too-long"});return [];}
  if(directory!==""&&vanished(error))return [];
  throw error;
 }
 candidates.sort((left,right)=>compare(left.name,right.name));
 const seen=new Set<string>();
 for(const {name,relative,kind} of candidates){
  signal?.throwIfAborted();
  const reason=refuseLeaf(relative,name);
  if(reason){refused.push({path:relative,reason});continue;}
  const fold=foldNameV1(name);
  if(seen.has(fold)){refused.push({path:relative,reason:"case-collision"});continue;}
  seen.add(fold);
  if(kind==="directory"){subdirectories.push(relative);continue;}
  try{
   const info=await lstat(within(name));
   if(kind==="link"){
    const target=windowsSeparators(await readlink(within(name))),unsafe=linkRefusalV1(relative,target);
    if(unsafe)refused.push({path:relative,reason:unsafe});
    else entries.push({path:relative,kind:"link",bytes:info.size,mtimeMs:info.mtimeMs,executable:false,target});
   }else if(info.isFile())entries.push({path:relative,kind:"file",bytes:info.size,mtimeMs:info.mtimeMs,executable:process.platform!=="win32"&&(info.mode&0o111)!==0});
  }catch(error){
   if(overlong(error))refused.push({path:relative,reason:"too-long"});
   else if(!vanished(error))throw error;
  }
 }
 return subdirectories;
}

/** 🔍️ Walks the worktree once, depth first with bounded concurrency (the pending directories stay few), and returns every shared file and link plus every refused path, sorted; it never descends into ignored directories or `.git` and never follows symbolic links. With `visit` the entries are handed over one by one in discovery order and not kept (`entries` stays empty), so a caller can fold a huge tree in constant memory. A path that the operating system itself refuses as too long (`ENAMETOOLONG`, e.g. a short policy limit but a deep worktree) is listed as refused `too-long` instead of failing the walk. Rejects with the signal reason when aborted. */
export async function scanSharedV1(worktree:string,ignore:FleetIgnoreV1,signal?:AbortSignal,visit?:(entry:FleetScanEntryV1)=>void):Promise<FleetScanV1>{
 const entries:FleetScanEntryV1[]=[],refused:{path:string;reason:FleetPathRefusalV1}[]=[],sink=visit===undefined?entries:{push:(entry:FleetScanEntryV1)=>{visit(entry);return 0;}};
 const pending=[""],waiting:(()=>void)[]=[];
 let active=0;
 const wake=():void=>{for(const resume of waiting.splice(0))resume();};
 const worker=async():Promise<void>=>{
  for(;;){
   const directory=pending.pop();
   if(directory===undefined){
    if(active===0)return;
    await new Promise<void>(resume=>{waiting.push(resume);});
    continue;
   }
   active++;
   try{for(const child of await scanDirectory(worktree,directory,ignore,signal,sink,refused))pending.push(child);}
   catch(error){pending.length=0;throw error;}
   finally{active--;wake();}
  }
 };
 await Promise.all(Array.from({length:16},worker));
 entries.sort((left,right)=>compare(left.path,right.path));
 refused.sort((left,right)=>compare(left.path,right.path)||compare(left.reason,right.reason));
 return {entries,refused};
}

/** 🔐️ Streams a file through sha256; `stable` is false when its size or modification time changed, or the bytes read differ from the size, between the stat before and the stat after hashing. */
export async function hashFileV1(file:string):Promise<Readonly<{sha256:string;bytes:number;stable:boolean}>>{
 const before=await stat(file),hash=createHash("sha256");
 let bytes=0;
 for await(const chunk of createReadStream(file)){hash.update(chunk);bytes+=chunk.length;}
 const after=await stat(file);
 return {sha256:hash.digest("hex"),bytes,stable:before.size===after.size&&before.mtimeMs===after.mtimeMs&&bytes===after.size};
}
