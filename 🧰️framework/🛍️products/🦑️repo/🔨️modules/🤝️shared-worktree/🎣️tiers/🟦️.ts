import {randomBytes} from "node:crypto";
import {chmod,mkdir,readdir,readFile,realpath,rename,rm,stat,unlink,writeFile} from "node:fs/promises";
import {basename,dirname,isAbsolute,join,relative,resolve,sep} from "node:path";

/** 🎣 The path book a `DirectoryTableV1` shares with its replica: paths in, small integer ids out. */
export type PathBookV1=Readonly<{intern(path:string):number;find(path:string):number|undefined;name(id:number):string}>;

/** 📈 The passive latency of one replica over its newest samples: the 50th and 95th percentile (nearest rank, whole milliseconds, `null` without samples) of the time from an edit to its published event and of the time from a remote event to its application, with the sample counts. */
export type LatencyReportV1=Readonly<{publishMs:Readonly<{p50:number|null;p95:number|null}>;applyMs:Readonly<{p50:number|null;p95:number|null}>;samples:Readonly<{publish:number;apply:number}>;tiers:Readonly<Record<TierNameV1,TierReportV1>>}>;

/** ⏱️ The background tiers that are timed: `hot` pass, directory `sweep`, known-path `file` sweep, full tree `walk` and the `view` (`git status`). */
export type TierNameV1="hot"|"sweep"|"file"|"walk"|"view";

/** 🪫 What the last pass of one tier cost: how many passes ran, the cost of the last one in milliseconds (CPU time of the pass, wall time for the view) and the duty cycle it was scheduled for, in percent of one core. */
export type TierReportV1=Readonly<{passes:number;lastMs:number|null;dutyPercent:number|null}>;

/** 🔋 The wait before a tier starts again: the larger of its `floorMs` and the time that keeps its duty cycle at `dutyPercent` of one core after a pass that cost `costMs`. */
export function nextDelayV1(floorMs:number,costMs:number,dutyPercent:number):number{return Math.max(floorMs,Math.ceil(costMs*(100/dutyPercent-1)));}

/** 🕰️ The timings of the tiers: `record` notes the cost of a pass and the wait that was chosen after it, `report` states passes, last cost and the resulting duty cycle. */
export class TierClocksV1{
 private readonly clocks=new Map<TierNameV1,{passes:number;cost:number;delay:number}>();
 record(name:TierNameV1,costMs:number,delayMs:number):void{
  const known=this.clocks.get(name);
  this.clocks.set(name,{passes:(known?.passes??0)+1,cost:costMs,delay:delayMs});
 }
 report():Readonly<Record<TierNameV1,TierReportV1>>{
  const row=(name:TierNameV1):TierReportV1=>{const known=this.clocks.get(name);return known===undefined?{passes:0,lastMs:null,dutyPercent:null}:{passes:known.passes,lastMs:Math.round(known.cost),dutyPercent:Math.round(known.cost/Math.max(1,known.cost+known.delay)*1000)/10};};
  return {hot:row("hot"),sweep:row("sweep"),file:row("file"),walk:row("walk"),view:row("view")};
 }
}

/** 🤫 The latency report of a replica that has measured nothing yet. */
export function silentLatencyV1():LatencyReportV1{return new LatencyV1().report();}

/** 🔢 The most hint paths one take returns. */
export const hintPathLimitV1=5000;

/** 📏 The largest hint file that is read; larger ones are deleted unread. */
export const hintBytesLimitV1=1<<20;

/** 🪝 The largest hook payload file (`*.hook`, a whole `PostToolUse` payload that a shell wrote unparsed) that is read: a `Write` carries the file content. */
export const hookBytesLimitV1=32<<20;

/** 🛟 The age after which an unfinished hint file (name starting with a dot) is removed. */
export const hintStaleMillisecondsV1=60_000;

const shallowDepthV1=3,staleTemporaryMilliseconds=hintStaleMillisecondsV1,sealedPrefix=".cache/f";
const token=():string=>randomBytes(6).toString("hex");
const sealedPath=(path:string):boolean=>path===sealedPrefix||path.startsWith(`${sealedPrefix}/`);

/** 🐚 The command of the Claude Code `PostToolUse` hook for POSIX shells (and Git Bash on Windows): no launcher, no JSON parser, four tiny processes. It copies the hook payload into the drop directory under a dot name and renames it to `*.hook`, which the daemon parses; without a drop directory it does nothing, and it always exits 0. */
export const hookCommandV1='d="$CLAUDE_PROJECT_DIR/.cache/f/hints";[ -d "$d" ]&&cat >"$d/.h$$"&&mv "$d/.h$$" "$d/h$$-$(date +%s).hook";exit 0';

/** 🪟 The same hook for PowerShell on Windows without Git Bash (a per-machine `shell: powershell` setting). */
export const hookPowerShellCommandV1='$d=Join-Path $env:CLAUDE_PROJECT_DIR ".cache/f/hints";if(Test-Path $d){$t=Join-Path $d ".h$PID";[Console]::In.ReadToEnd()|Set-Content -NoNewline -Encoding utf8 $t;Move-Item $t (Join-Path $d "h$PID-$([DateTimeOffset]::UtcNow.ToUnixTimeSeconds()).hook")};exit 0';

/** 📮 The drop directory of a worktree: any process may create a file there (temp name starting with a dot, then rename inside the directory) and the daemon consumes it. */
export function hintsPathV1(worktree:string):string{return join(worktree,".cache","f","hints");}

/** 📭 Creates the drop directory open for every local account (mode 1777: anybody creates, only the owner of the directory or of the file removes). */
export async function ensureHintsV1(worktree:string):Promise<string>{
 const directory=hintsPathV1(worktree);
 await mkdir(directory,{recursive:true});
 await chmod(directory,0o1777).catch(()=>{});
 return directory;
}

/** ✂️ Splits hint text at NUL and newline into raw paths; a trailing carriage return is dropped, empty items vanish, nothing else is trimmed. */
export function parseHintTextV1(text:string):readonly string[]{
 return text.split(/[\0\n]/u).map(item=>item.endsWith("\r")?item.slice(0,-1):item).filter(item=>item!=="");
}

function insideV1(root:string,target:string):string|undefined{
 const rel=relative(root,target);
 if(rel===""||rel===".."||rel.startsWith(`..${sep}`)||isAbsolute(rel))return undefined;
 const path=rel.split(sep).join("/").normalize("NFC");
 return path.split("/").some(part=>part===""||part==="."||part==="..")||sealedPath(path)?undefined:path;
}

/** 🧶 Turns one raw hint path (absolute, or relative to the worktree) into a worktree-relative POSIX NFC path; `undefined` for the worktree itself, anything outside it, a path below the sealed `.cache/f` and an empty or NUL-carrying text. An absolute path outside the lexical root is retried through the real paths of the worktree and of the parent directory of the target (symbolic links, case on macOS). */
export async function normaliseHintPathV1(worktree:string,raw:string):Promise<string|undefined>{
 if(raw===""||raw.includes("\0"))return undefined;
 const root=resolve(worktree),target=isAbsolute(raw)?resolve(raw):resolve(root,raw),lexical=insideV1(root,target);
 if(lexical!==undefined||!isAbsolute(raw))return lexical;
 try{return insideV1(await realpath(root),join(await realpath(dirname(target)),basename(target)));}catch{return undefined;}
}

/** 🪣 Consumes the hint files of a worktree: reads every finished file (name not starting with a dot; a name ending `.hook` holds a hook payload that is parsed like `touch --hook`) in name order up to `limit` paths, removes it, and returns the distinct normalised paths in first-seen order; unfinished files older than a minute are removed; a missing directory or a failing file yields nothing and never throws. */
export async function takeHintsV1(worktree:string,limit:number=hintPathLimitV1,now:number=Date.now()):Promise<readonly string[]>{
 const directory=hintsPathV1(worktree),found=new Set<string>();
 let names:string[];
 try{names=await readdir(directory);}catch{return [];}
 for(const name of names.sort()){
  const file=join(directory,name);
  try{
   const info=await stat(file);
   if(name.startsWith(".")){if(now-info.mtimeMs>staleTemporaryMilliseconds)await rm(file,{force:true});continue;}
   if(found.size>=limit)break;
   if(!info.isFile())continue;
   const hook=name.endsWith(".hook"),text=info.size<=(hook?hookBytesLimitV1:hintBytesLimitV1)?await readFile(file,"utf8"):"";
   await unlink(file);
   for(const raw of hook?parseHookInputV1(text):parseHintTextV1(text)){const path=await normaliseHintPathV1(worktree,raw);if(path!==undefined)found.add(path);}
  }catch{}
 }
 return [...found];
}

/** 🖋️ Drops one hint file into the drop directory: NUL-separated paths written to a temporary name and renamed inside the directory, so the daemon never reads half a file; returns the final file name. Nothing is normalised here. */
export async function writeHintV1(worktree:string,paths:readonly string[]):Promise<string>{
 const directory=await ensureHintsV1(worktree),name=`${Date.now()}-${token()}`,temporary=join(directory,`.${name}`),final=`${name}.hint`;
 try{await writeFile(temporary,paths.join("\0"));await rename(temporary,join(directory,final));}
 catch(error){await rm(temporary,{force:true});throw error;}
 return final;
}

/** 🧷 Reads the paths a Claude Code `PostToolUse` hook payload names: `tool_input.file_path` and `tool_input.notebook_path`, when they are non-empty strings; everything else in the payload is ignored, and text that is not a JSON object yields nothing. Never throws. */
export function parseHookInputV1(text:string):readonly string[]{
 try{
  const value=JSON.parse(text) as unknown;
  if(typeof value!=="object"||value===null)return [];
  const input=(value as {tool_input?:unknown}).tool_input;
  if(typeof input!=="object"||input===null)return [];
  const found:string[]=[];
  for(const key of ["file_path","notebook_path"] as const){const item=(input as Record<string,unknown>)[key];if(typeof item==="string"&&item!==""&&!found.includes(item))found.push(item);}
  return found;
 }catch{return [];}
}

/** 🔥 The hot set: the paths that were published, applied, hinted or seen by the watcher recently, newest last. A path is kept until `ttlMilliseconds` after its latest note, the oldest are dropped beyond `limit`. */
export class HotSetV1{
 private readonly stamps=new Map<string,number>();
 readonly limit:number; readonly ttlMilliseconds:number; constructor(limit:number=20_000,ttlMilliseconds:number=600_000){this.limit=limit;this.ttlMilliseconds=ttlMilliseconds;}
 get size():number{return this.stamps.size;}
 note(path:string,now:number):void{
  this.stamps.delete(path);this.stamps.set(path,now);
  if(this.stamps.size>this.limit){const oldest=this.stamps.keys().next().value;if(oldest!==undefined)this.stamps.delete(oldest);}
 }
 noteMany(paths:Iterable<string>,count:number,now:number,bulk:number=500):void{if(count<=bulk)for(const path of paths)this.note(path,now);}
 has(path:string):boolean{return this.stamps.has(path);}
 expire(now:number):void{
  for(const [path,stamp] of this.stamps){if(now-stamp<=this.ttlMilliseconds)break;this.stamps.delete(path);}
 }
 snapshot(now:number):string[]{this.expire(now);return [...this.stamps.keys()];}
}

/** 🌳 The known directories of the shared tree with the modification time they had when they were last listed, stored by path id; a pass of `begin` and `next` visits every present directory once, in the order they were learnt. */
export class DirectoryTableV1{
 private mtime:Float64Array=new Float64Array(1024);
 private present:Uint8Array=new Uint8Array(1024);
 private listed:Uint8Array=new Uint8Array(1024);
 private members:Int32Array=new Int32Array(1024);
 private length=0;
 private cursor=0;
 private stale=0;
 private readonly near=new Set<number>();
 size=0;
 readonly book:PathBookV1; constructor(book:PathBookV1){this.book=book;}
 private widen(id:number):void{
  if(id<this.mtime.length)return;
  const size=Math.max(id+1,this.mtime.length*2),mtime=new Float64Array(size),present=new Uint8Array(size),listed=new Uint8Array(size);
  mtime.set(this.mtime);present.set(this.present);listed.set(this.listed);
  this.mtime=mtime;this.present=present;this.listed=listed;
 }
 set(path:string,mtimeMs:number):void{
  let id:number;
  try{id=this.book.intern(path);}catch{return;}
  this.widen(id);
  let depth=path===""?0:1;
  for(let index=0;index<path.length&&depth<=shallowDepthV1;index++)if(path.charCodeAt(index)===47)depth++;
  if(depth<=shallowDepthV1)this.near.add(id);
  if(this.present[id]!==1){
   this.present[id]=1;this.size++;
   if(this.listed[id]===1)this.stale--;
   else{
    if(this.length>=this.members.length){const next=new Int32Array(this.members.length*2);next.set(this.members);this.members=next;}
    this.members[this.length++]=id;this.listed[id]=1;
   }
  }
  this.mtime[id]=mtimeMs;
 }
 get(path:string):number|undefined{
  const id=this.book.find(path);
  return id===undefined||id>=this.present.length||this.present[id]!==1?undefined:this.mtime[id];
 }
 has(path:string):boolean{return this.get(path)!==undefined;}
 delete(path:string):void{
  const id=this.book.find(path);
  if(id===undefined||id>=this.present.length||this.present[id]!==1)return;
  this.present[id]=0;this.size--;this.stale++;
 }
 columns():Readonly<{mtime:Float64Array;present:Uint8Array;members:Int32Array}>{
  let rows=0;
  for(let index=0;index<this.length;index++)rows=Math.max(rows,this.members[index]!+1);
  return {mtime:this.mtime.subarray(0,rows),present:this.present.subarray(0,rows),members:this.members.subarray(0,this.length)};
 }
 adopt(mtime:Float64Array,present:Uint8Array,members:Int32Array,length:number):void{
  this.mtime=mtime;this.present=present;this.members=members;this.listed=new Uint8Array(mtime.length);this.length=length;this.cursor=0;this.size=0;this.stale=0;
  for(let index=0;index<length;index++){const id=members[index]!;this.listed[id]=1;if(present[id]===1)this.size++;else this.stale++;}
 }
 mtimeAt(id:number):number{return this.mtime[id]!;}
 /** 🌿 The present directories at most three levels below the worktree (the root included): few enough to be checked with every hot pass. */
 shallow():string[]{const found:string[]=[];for(const id of this.near)if(this.present[id]===1)found.push(this.book.name(id));return found;}
 nameAt(id:number):string{return this.book.name(id);}
 begin():void{
  if(this.stale*2>this.length){
   let kept=0;
   for(let index=0;index<this.length;index++){const id=this.members[index]!;if(this.present[id]===1)this.members[kept++]=id;else this.listed[id]=0;}
   this.length=kept;this.stale=0;
  }
  this.cursor=0;
 }
 next():number{
  while(this.cursor<this.length){const id=this.members[this.cursor++]!;if(this.present[id]===1)return id;}
  return -1;
 }
 get remaining():number{return this.length-this.cursor;}
}

/** 📐 The nearest-rank percentile of a list: the value at rank `ceil(percent / 100 * n)` of the sorted list; `null` for an empty list. */
export function percentileV1(values:readonly number[],percent:number):number|null{
 if(values.length===0)return null;
 const sorted=[...values].sort((left,right)=>left-right);
 return sorted[Math.min(sorted.length,Math.max(1,Math.ceil(percent/100*sorted.length)))-1]!;
}

/** ⏲️ The newest samples of publish and apply latency (100 each, edits and events older than ten minutes are not samples, a clock skew of up to ten minutes is read as zero). A sample stays in the report for one hour (`retainMilliseconds`) after it was taken: a slow outlier of a blocked period must not describe a machine that has been quiet or fast since. */
export class LatencyV1{
 private readonly publishes:{at:number;ms:number}[]=[];
 private readonly applies:{at:number;ms:number}[]=[];
 readonly tiers=new TierClocksV1();
 readonly capacity:number; readonly windowMilliseconds:number; readonly retainMilliseconds:number; readonly now:()=>number; constructor(capacity:number=100,windowMilliseconds:number=600_000,retainMilliseconds:number=3_600_000,now:()=>number=Date.now){this.capacity=capacity;this.windowMilliseconds=windowMilliseconds;this.retainMilliseconds=retainMilliseconds;this.now=now;}
 private record(list:{at:number;ms:number}[],milliseconds:number):void{
  if(!Number.isFinite(milliseconds)||milliseconds>this.windowMilliseconds||milliseconds<-this.windowMilliseconds)return;
  list.push({at:this.now(),ms:Math.max(0,Math.round(milliseconds))});
  if(list.length>this.capacity)list.shift();
 }
 private current(list:{at:number;ms:number}[]):number[]{
  const oldest=this.now()-this.retainMilliseconds;
  while(list.length>0&&list[0]!.at<oldest)list.shift();
  return list.map(sample=>sample.ms);
 }
 publish(milliseconds:number):void{this.record(this.publishes,milliseconds);}
 apply(milliseconds:number):void{this.record(this.applies,milliseconds);}
 report():LatencyReportV1{
  const publishes=this.current(this.publishes),applies=this.current(this.applies);
  return {publishMs:{p50:percentileV1(publishes,50),p95:percentileV1(publishes,95)},applyMs:{p50:percentileV1(applies,50),p95:percentileV1(applies,95)},samples:{publish:publishes.length,apply:applies.length},tiers:this.tiers.report()};
 }
}
