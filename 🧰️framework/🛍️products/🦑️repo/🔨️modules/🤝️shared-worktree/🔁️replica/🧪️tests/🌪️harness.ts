import {chmodSync,closeSync,existsSync,ftruncateSync,lstatSync,mkdirSync,mkdtempSync,openSync,readdirSync,readFileSync,renameSync,rmSync,symlinkSync,utimesSync,writeFileSync,writeSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {basename,dirname,join,relative} from "node:path";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {formatHlcV1,listOriginsV1,readHeadV1} from "../../📒️journal/🟦️.ts";
import {compileIgnoreV1,foldNameV1,scanSharedV1,type FleetIgnoreV1} from "../../🚧️boundary/🟦️.ts";
import {forgetGitV1,gitStateV1,gitViewV1,requestRebaseV1} from "../../🌿️git/🟦️.ts";
import {createAttesterV1,fleetReportV1,readAttestationV1} from "../../🩺️attest/🟦️.ts";
import {closeReplicaV1,followOnceV1,joinV1,openReplicaV1,publishOnceV1,repairV1,seedV1,storeReachableV1,verifyV1,type ReplicaEventV1,type ReplicaOptionsV1,type ReplicaV1} from "../🟦️.ts";

/** 🎲️ A seeded generator (mulberry32): the same seed yields the same sequence on every platform. */
export type RngV1=Readonly<{next():number;int(count:number):number;pick<T>(items:readonly T[]):T;chance(probability:number):boolean;hex(length:number):string}>;

/** 🧬️ One concrete, serialisable chaos operation: a trace is a list of these and replays without the generator. */
export type ChaosOpV1=Readonly<{op:"create"|"modify"|"replace"|"delete"|"rename"|"case"|"tree"|"chmod"|"symlink"|"add"|"rmcached"|"unstage"|"commit"|"status"|"gitignore"|"restart"|"crash"|"repair"|"rebase"|"rounds"|"outage"|"rmdir"|"swap"|"dirswap"|"torn"|"stale";m:number;path?:string;to?:string;text?:string;files?:readonly string[];exec?:boolean;target?:string;message?:string;empty?:boolean;pattern?:string;remove?:boolean;loose?:boolean;set?:readonly number[];count?:number;down?:boolean}>;

/** 🖥️ One simulated machine: its own worktree and store link, its replica and the events its sink saw. */
export type ChaosMachineV1={name:string;options:ReplicaOptionsV1;link:string;events:ReplicaEventV1[];replica?:ReplicaV1;down:boolean};

/** 🌐️ The simulated fleet: one temp store behind one symlink per machine, so an outage removes a single link. */
export type ChaosFleetV1={root:string;real:string;machines:ChaosMachineV1[];messages:string[];notes:string[];cased:Set<string>;retyped:Set<string>};

/** 📋️ The outcome of one chaos run. */
export type ChaosReportV1=Readonly<{seed:number;machines:number;ops:readonly ChaosOpV1[];notes:readonly string[];violations:readonly string[];rounds:number;milliseconds:number;kept?:string}>;

/** ⚙️ Run parameters; `keep` leaves the temp directories for inspection. */
export type ChaosRunV1=Readonly<{seed:number;machines:number;steps:number;profile?:"mixed"|"hostile";loose?:boolean;keep?:boolean;settleMilliseconds?:number;restartCheck?:boolean}>;

export function rngV1(seed:number):RngV1{
 let state=seed>>>0;
 const next=():number=>{state=(state+0x6d2b79f5)>>>0;let t=state;t=Math.imul(t^(t>>>15),t|1);t^=t+Math.imul(t^(t>>>7),t|61);return((t^(t>>>14))>>>0)/4294967296;};
 const int=(count:number):number=>Math.floor(next()*count);
 return {next,int,pick:<T>(items:readonly T[]):T=>items[int(items.length)]!,chance:probability=>next()<probability,hex:length=>Array.from({length},()=>"0123456789abcdef"[int(16)]).join("")};
}

const environment={...process.env,GIT_AUTHOR_NAME:"t",GIT_AUTHOR_EMAIL:"t@example.test",GIT_COMMITTER_NAME:"t",GIT_COMMITTER_EMAIL:"t@example.test",GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1",GIT_TERMINAL_PROMPT:"0",LC_ALL:"C"};
const fast={pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0};
const settled=(ms:number)=>new Promise<void>(done=>setTimeout(done,ms));

/** 🔧️ Runs the real git CLI in a machine's worktree and never throws. */
export function gitV1(who:ChaosMachineV1,args:readonly string[],input?:string,extra:Record<string,string>={}):Readonly<{code:number;out:string;err:string}>{
 const result=Bun.spawnSync(["git",...args],{cwd:who.options.worktree,env:{...environment,...extra},stdin:input===undefined?undefined:Buffer.from(input),stdout:"pipe",stderr:"pipe"});
 return {code:result.exitCode??-1,out:result.stdout.toString(),err:result.stderr.toString()};
}

const at=(who:ChaosMachineV1,path:string):string=>join(who.options.worktree,...path.split("/"));
const textOf=(who:ChaosMachineV1,path:string):string=>{try{return readFileSync(at(who,path),"utf8");}catch{return "";}};
const ignores=new Map<string,FleetIgnoreV1>();
const sealed=(path:string):boolean=>path===".cache/f"||path.startsWith(".cache/f/");
const ignoreOf=(text:string):FleetIgnoreV1=>{
 let known=ignores.get(text);
 if(known===undefined){
  if(ignores.size>200)ignores.clear();
  const inner=compileIgnoreV1(text);
  known=Object.freeze({digest:inner.digest,ignores:(path:string,isDirectory:boolean)=>sealed(path)||inner.ignores(path,isDirectory),child:(path:string,isDirectory:boolean)=>sealed(path)||(inner.child??inner.ignores)(path,isDirectory)});
  ignores.set(text,known);
 }
 return known;
};
const frozen=(fleet:ChaosFleetV1,path:string):boolean=>[...fleet.retyped].some(root=>path===root||path.startsWith(`${root}/`))||fleet.machines.some(who=>ignoreOf(textOf(who,".gitignore")).ignores(path,false));

function listFiles(who:ChaosMachineV1,base:string=""):string[]{
 const found:string[]=[];
 let names:string[];
 try{names=readdirSync(join(who.options.worktree,base)).sort();}catch{return found;}
 for(const name of names){
  const path=base===""?name:`${base}/${name}`;
  if(path===".git"||path===".cache")continue;
  let info;
  try{info=lstatSync(at(who,path));}catch{continue;}
  if(info.isDirectory())found.push(...listFiles(who,path));else found.push(path);
 }
 return found;
}

/** 🏗️ Builds a fleet of git-backed replicas: machine 0 seeds a committed tree, the others join onto the store. */
export async function buildFleetV1(count:number):Promise<ChaosFleetV1>{
 const root=mkdtempSync(join(tmpdir(),"chaos-")),real=join(root,"store");
 mkdirSync(real);mkdirSync(join(root,"links"));mkdirSync(join(root,"wt"));
 const machines:ChaosMachineV1[]=Array.from({length:count},(_,index)=>{
  const name=index===0?"g1-server":`g1-m0${index}`,link=join(root,"links",name),worktree=join(root,"wt",name);
  mkdirSync(worktree);symlinkSync(real,link,"dir");
  return {name,link,down:false,events:[],options:{worktree,storeRoot:link,originId:name,groupId:"g1",...fast}};
 });
 const fleet:ChaosFleetV1={root,real,machines,messages:["initial"],notes:[],cased:new Set(),retyped:new Set()},[first]=machines as [ChaosMachineV1,...ChaosMachineV1[]];
 for(const [path,text] of [[".gitignore",fixture.gitignore],["a.txt","alpha\n"],["b.txt","bravo\n"],["dir/c.txt","charlie\n"]] as const){mkdirSync(dirname(at(first,path)),{recursive:true});writeFileSync(at(first,path),text);}
 gitV1(first,["init","-q","-b","main"]);gitV1(first,["add","-A"]);gitV1(first,["commit","-q","-m","initial"]);
 await seedV1(first.options);
 for(const other of machines.slice(1))await joinV1(other.options);
 for(const who of machines)await openMachine(who);
 return fleet;
}

async function openMachine(who:ChaosMachineV1):Promise<void>{who.replica=await openReplicaV1(who.options,event=>who.events.push(event));}
function fail(who:ChaosMachineV1,error:unknown):void{who.events.push({version:1,event:"error",message:`thrown: ${error instanceof Error?error.message:String(error)}`});}

/** 🧹️ Closes every replica and removes the temp directories unless `keep`. */
export async function disposeFleetV1(fleet:ChaosFleetV1,keep:boolean=false):Promise<void>{
 for(const who of fleet.machines){if(who.replica){try{await closeReplicaV1(who.replica);}catch{}who.replica=undefined;}}
 if(!keep)rmSync(fleet.root,{recursive:true,force:true});
}

function reconnect(who:ChaosMachineV1,fleet:ChaosFleetV1):void{if(who.down){symlinkSync(fleet.real,who.link,"dir");who.down=false;}}

/** 🔁️ One driven round on a machine, as `runV1` does it: skipped while the store is unreachable, thrown errors become `error` events. */
export async function roundV1(who:ChaosMachineV1):Promise<void>{
 if(who.replica===undefined||!await storeReachableV1(who.options.storeRoot))return;
 try{await publishOnceV1(who.replica);await followOnceV1(who.replica);}catch(error){fail(who,error);}
}

async function signature(fleet:ChaosFleetV1):Promise<string>{
 const heads:unknown[]=[];
 for(const origin of await listOriginsV1(fleet.real)){const head=await readHeadV1(fleet.real,origin);heads.push([origin,head?.segment,head?.bytes]);}
 return JSON.stringify([heads,fleet.machines.map(who=>[who.replica?.queue.size,who.replica?.files.manifestHex(),JSON.stringify(who.replica?.cursors)])]);
}

/** 🕰️ Reconnects every machine and runs rounds on all of them until the store heads, cursors and manifests stay unchanged for three rounds with empty queues; returns the rounds used or -1 on timeout. */
export async function settleV1(fleet:ChaosFleetV1,milliseconds:number=45_000):Promise<number>{
 for(const who of fleet.machines){reconnect(who,fleet);if(who.replica===undefined)await openMachine(who);}
 const end=Date.now()+milliseconds;
 let previous="",quiet=0;
 for(let round=1;Date.now()<end;round++){
  for(const who of fleet.machines)await roundV1(who);
  for(const who of fleet.machines)await roundV1(who);
  const now=await signature(fleet),empty=fleet.machines.every(who=>who.replica!.queue.size===0);
  quiet=now===previous&&empty?quiet+1:0;previous=now;
  if(quiet>=3)return round;
  await settled(15);
 }
 return -1;
}

const dirs=["","","","d0/","d1/","d1/d2/","d3/e/f/","ignored/"],stems=["f0","f1","f2","f3","f4","f5","run","x","sp ace","é","日本"],exts=[".txt",".txt",".txt",".sh",".log"];
const nameOf=(rng:RngV1):string=>`${rng.pick(dirs)}${rng.pick(stems)}${rng.pick(exts)}`;
const textFor=(rng:RngV1):string=>rng.chance(0.04)?"":rng.chance(0.04)?`${rng.hex(8)}\n`.repeat(8000):`x${rng.hex(8)}\n`;
const toggle=(path:string):string=>{const name=basename(path),flipped=name[0]===name[0]!.toUpperCase()?name[0]!.toLowerCase()+name.slice(1):name[0]!.toUpperCase()+name.slice(1);return join(dirname(path),flipped).replace(/^\.\//u,"");};
type Weights=readonly (readonly [ChaosOpV1["op"],number])[];
const mixed:Weights=[["rounds",14],["create",12],["modify",10],["replace",7],["delete",7],["rename",6],["case",3],["tree",3],["chmod",4],["symlink",3],["add",8],["rmcached",3],["unstage",3],["commit",6],["status",3],["gitignore",2],["restart",2],["crash",2],["repair",2],["rebase",2],["outage",3],["rmdir",2],["swap",2],["dirswap",2],["torn",3]];
const hostile:Weights=[["rounds",6],["create",10],["modify",8],["replace",6],["delete",8],["rename",8],["case",2],["tree",4],["chmod",3],["symlink",3],["add",8],["rmcached",4],["unstage",4],["commit",9],["status",3],["gitignore",2],["restart",4],["crash",7],["repair",3],["rebase",3],["outage",7],["rmdir",4],["swap",4],["dirswap",4],["torn",5]];

/** 🎰️ Draws the next concrete operation from the live fleet state; mutations avoid paths any machine ignores, because a private path may diverge by design. */
export function generateV1(rng:RngV1,fleet:ChaosFleetV1,profile:"mixed"|"hostile"="mixed",loose:boolean=false):ChaosOpV1{
 const m=rng.int(fleet.machines.length),who=fleet.machines[m]!,files=listFiles(who).filter(path=>path!==".gitignore"&&!frozen(fleet,path)),weights=profile==="hostile"?hostile:mixed,total=weights.reduce((sum,[,weight])=>sum+weight,0);
 let draw=rng.int(total),op=weights[0]![0];
 for(const [name,weight] of weights){if(draw<weight){op=name;break;}draw-=weight;}
 const existing=():string|undefined=>files.length===0?undefined:rng.pick(files);
 const fresh=():string=>{for(let attempt=0;attempt<6;attempt++){const path=nameOf(rng);if(!frozen(fleet,path)&&!fleet.cased.has(foldNameV1(path)))return path;}return `n${rng.hex(4)}.txt`;};
 const path=existing();
 switch(op){
  case "rounds":{const set=fleet.machines.map((_,index)=>index).filter(()=>rng.chance(0.6));return {op,m,set:set.length===0?[m]:set,count:1+rng.int(3)};}
  case "create":return {op,m,path:fresh(),text:textFor(rng)};
  case "modify":case "replace":return path===undefined?{op:"create",m,path:fresh(),text:textFor(rng)}:{op,m,path,text:textFor(rng)};
  case "delete":case "unstage":case "rmcached":case "chmod":return path===undefined?{op:"create",m,path:fresh(),text:textFor(rng)}:{op,m,path,exec:rng.chance(0.5)};
  case "rename":return path===undefined?{op:"create",m,path:fresh(),text:textFor(rng)}:{op,m,path,to:fresh()};
  case "tree":return {op,m,files:Array.from({length:1+rng.int(3)},()=>`t${rng.int(3)}/${rng.pick(["a","b"])}/${rng.pick(["c","d/e"])}/${rng.pick(stems)}.txt`),text:textFor(rng)};
  case "symlink":return path===undefined?{op:"create",m,path:fresh(),text:textFor(rng)}:{op,m,path:`l${rng.int(4)}`,target:path};
  case "add":return {op,m,files:files.filter(()=>rng.chance(0.4)).slice(0,6)};
  case "commit":return {op,m,message:`c${fleet.messages.length}-${rng.hex(4)}`,empty:rng.chance(0.3)};
  case "gitignore":{
   const lines=textOf(who,".gitignore").split("\n").filter(line=>line.startsWith("/"));
   return lines.length>0&&rng.chance(0.4)?{op,m,pattern:rng.pick(lines),remove:true}:path===undefined?{op:"status",m}:{op,m,pattern:rng.chance(0.2)&&path.includes("/")?`/${path.split("/")[0]}/`:`/${path}`};
  }
  case "outage":return {op,m,down:!who.down};
  case "case":return path===undefined?{op:"create",m,path:fresh(),text:textFor(rng)}:{op,m,path,loose};
  case "torn":return {op,m,path:path??fresh(),text:`x${rng.hex(8)}\n`.repeat(1+rng.int(20))};
  case "swap":return path===undefined?{op:"create",m,path:fresh(),text:textFor(rng)}:{op,m,path,target:rng.pick(files),text:textFor(rng)};
  case "dirswap":case "rmdir":{
   const nested=files.filter(item=>item.includes("/"));
   if(nested.length===0)return {op:"create",m,path:fresh(),text:textFor(rng)};
   const directory=dirname(rng.pick(nested)),inside=listFiles(who,directory);
   return inside.some(item=>frozen(fleet,item))?{op:"status",m}:{op,m,path:op==="dirswap"&&rng.chance(0.4)&&path!==undefined?path:directory,text:textFor(rng),loose};
  }
  default:return {op,m};
 }
}

function writeIn(file:string,text:string):void{
 if(lstatSync(file,{throwIfNoEntry:false})?.isSymbolicLink()===true)throw Object.assign(Error("symlink"),{code:"ESYMLINK"});
 mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);
}

/** ⚡️ Applies one operation; filesystem races that make an operation impossible are noops and never fail the run. Returns a short outcome note. */
export async function performV1(fleet:ChaosFleetV1,op:ChaosOpV1):Promise<string>{
 const who=fleet.machines[op.m];
 if(who===undefined)return "noop:machine";
 const file=op.path===undefined?"":at(who,op.path);
 try{
  switch(op.op){
   case "create":writeIn(file,op.text!);return "ok";
   case "modify":{
    const info=lstatSync(file);
    if(!info.isFile())return "noop:not-a-file";
    const fd=openSync(file,"r+");
    try{ftruncateSync(fd,0);writeSync(fd,op.text!);}finally{closeSync(fd);}
    return "ok";
   }
   case "replace":{if(!lstatSync(file).isFile())return "noop:not-a-file";const temporary=`${file}.swp`;writeFileSync(temporary,op.text!);renameSync(temporary,file);return "ok";}
   case "delete":if(lstatSync(file).isDirectory())return "noop:directory";rmSync(file);return "ok";
   case "rename":if(lstatSync(file).isSymbolicLink())return "noop:symlink";mkdirSync(dirname(at(who,op.to!)),{recursive:true});if(existsSync(at(who,op.to!)))return "noop:exists";renameSync(file,at(who,op.to!));return "ok";
   case "case":{
    const to=toggle(op.path!);
    if(to===op.path||fleet.cased.has(foldNameV1(to))||lstatSync(file).isSymbolicLink())return "noop:no-letter";
    if(op.loose!==true)await settleV1(fleet);
    fleet.cased.add(foldNameV1(to));renameSync(file,at(who,to));
    if(op.loose!==true)await settleV1(fleet);return "ok";
   }
   case "tree":for(const path of op.files!)writeIn(at(who,path),op.text!);return "ok";
   case "chmod":if(!lstatSync(file).isFile())return "noop:not-a-file";chmodSync(file,op.exec===true?0o755:0o644);return "ok";
   case "symlink":{if(existsSync(file)||lstatSync(file,{throwIfNoEntry:false})!==undefined)return "noop:exists";mkdirSync(dirname(file),{recursive:true});symlinkSync(relative(dirname(file),at(who,op.target!)),file);return "ok";}
   case "add":return gitV1(who,["add","--",...(op.files??[])]).code===0?"ok":"git-failed";
   case "rmcached":return gitV1(who,["rm","--cached","-q","--",op.path!]).code===0?"ok":"git-failed";
   case "unstage":return gitV1(who,["reset","-q","--",op.path!]).code===0?"ok":"git-failed";
   case "status":gitV1(who,["status","--porcelain"]);return "ok";
   case "commit":{
    const result=gitV1(who,["commit","-q",...(op.empty===true?["--allow-empty"]:[]),"-m",op.message!]);
    if(result.code!==0)return "git-failed";
    fleet.messages.push(op.message!);return "ok";
   }
   case "gitignore":{
    await settleV1(fleet);
    const lines=textOf(who,".gitignore").split("\n").filter(line=>line!==""&&line!==op.pattern);
    if(op.remove!==true)lines.push(op.pattern!);
    writeFileSync(at(who,".gitignore"),`${lines.join("\n")}\n`);return "ok";
   }
   case "restart":case "crash":{
    if(who.replica===undefined||who.down)return "noop:down";
    const old=who.replica;who.replica=undefined;
    if(op.op==="restart")await closeReplicaV1(old);else{await old.release();forgetGitV1(who.options.worktree);}
    await openMachine(who);return "ok";
   }
   case "repair":if(who.replica===undefined||who.down)return "noop:down";return `repaired:${await repairV1(who.replica)}`;
   case "rebase":await requestRebaseV1(who.options.worktree);return "ok";
   case "rounds":{
    for(let round=0;round<(op.count??1);round++){for(const index of op.set??[]){const peer=fleet.machines[index];if(peer!==undefined)await roundV1(peer);}await settled(10);}
    return "ok";
   }
   case "rmdir":if(!lstatSync(file).isDirectory())return "noop:not-a-directory";rmSync(file,{recursive:true});return "ok";
   case "swap":{
    const info=lstatSync(file);
    if(info.isSymbolicLink()){rmSync(file);writeFileSync(file,op.text!);return "ok";}
    if(!info.isFile()||op.target===op.path)return "noop:not-a-file";
    rmSync(file);symlinkSync(relative(dirname(file),at(who,op.target!)),file);return "ok";
   }
   case "dirswap":{
    const info=lstatSync(file);
    if(op.loose!==true)await settleV1(fleet);
    fleet.retyped.add(op.path!);
    if(info.isDirectory()){rmSync(file,{recursive:true});writeFileSync(file,op.text!);}
    else{rmSync(file);mkdirSync(file);writeFileSync(join(file,"inner.txt"),op.text!);}
    if(op.loose!==true)await settleV1(fleet);return "ok";
   }
   case "torn":{
    writeIn(file,op.text!.slice(0,Math.max(1,op.text!.length>>1)));
    await roundV1(who);
    writeFileSync(file,op.text!);return "ok";
   }
   case "stale":{
    const before=lstatSync(file);
    const fd=openSync(file,"r+");
    try{writeSync(fd,op.text!,0);}finally{closeSync(fd);}
    utimesSync(file,before.atime,before.mtime);return "ok";
   }
   case "outage":{
    if(op.down===true&&!who.down){rmSync(who.link);who.down=true;return "down";}
    if(op.down!==true)reconnect(who,fleet);
    return "up";
   }
  }
 }catch(error){return `noop:${(error as {code?:string}).code??(error instanceof Error?error.message.slice(0,60):"error")}`;}
}

const sha=(bytes:Uint8Array):string=>createHash("sha256").update(bytes).digest("hex");

async function snapshotTree(who:ChaosMachineV1):Promise<Map<string,string>>{
 const scan=await scanSharedV1(who.options.worktree,ignoreOf(textOf(who,".gitignore"))),tree=new Map<string,string>();
 for(const entry of scan.entries){
  const content=entry.kind==="link"?`link ${entry.target}`:`file ${entry.bytes} ${entry.executable?"x":"-"} ${sha(readFileSync(at(who,entry.path)))}`;
  tree.set(entry.path,content);
 }
 return tree;
}

const viewOf=(who:ChaosMachineV1):string=>gitV1(who,["--no-optional-locks","status","--porcelain=v2","-z","--untracked-files=all","--no-renames"]).out;
const same=<T>(values:readonly T[]):boolean=>values.every(value=>JSON.stringify(value)===JSON.stringify(values[0]));

/** 🔎️ Asserts the fleet invariants after quiescence and returns every violation as `category: detail` (empty = converged): shared trees, HEAD, write-tree, stage listing, git status (v2 and `gitViewV1`), linear history with every message once, fsck, `verifyV1`, empty queues, no error events, and the fleet verdict. */
export async function checkFleetV1(fleet:ChaosFleetV1):Promise<string[]>{
 const bad:string[]=[],{machines}=fleet;
 const compare=<T>(category:string,read:(who:ChaosMachineV1)=>T,show:(left:T,right:T)=>string=(left,right)=>`${JSON.stringify(left)} != ${JSON.stringify(right)}`):void=>{
  const first=read(machines[0]!);
  for(const who of machines.slice(1)){const value=read(who);if(JSON.stringify(value)!==JSON.stringify(first))bad.push(`${category}: ${machines[0]!.name} vs ${who.name}: ${show(first,value).slice(0,600)}`);}
 };
 const trees=await Promise.all(machines.map(snapshotTree));
 for(let index=1;index<machines.length;index++){
  const left=trees[0]!,right=trees[index]!,diffs:string[]=[];
  for(const path of new Set([...left.keys(),...right.keys()]))if(left.get(path)!==right.get(path))diffs.push(`${path} [${left.get(path)?.slice(0,40)??"absent"}] vs [${right.get(path)?.slice(0,40)??"absent"}]`);
  if(diffs.length>0)bad.push(`tree: ${machines[0]!.name} vs ${machines[index]!.name}: ${diffs.length} paths: ${diffs.slice(0,6).join("; ")}`);
 }
 compare("head",who=>gitV1(who,["rev-parse","HEAD"]).out.trim());
 compare("write-tree",who=>gitV1(who,["write-tree"]).out.trim());
 compare("stage",who=>gitV1(who,["ls-files","-s","-z"]).out);
 compare("status",viewOf);
 const views=await Promise.all(machines.map(who=>gitViewV1(who.options.worktree)));
 if(!same(views.map(view=>view?.digest)))bad.push(`view: digests differ: ${views.map(view=>view?.lines.join("|").slice(0,200)).join(" // ")}`);
 for(const who of machines){
  const rows=gitV1(who,["rev-list","--parents","HEAD"]).out.split("\n").filter(Boolean).map(row=>row.split(" ")),subjects=gitV1(who,["log","--format=%s","HEAD"]).out.split("\n").filter(Boolean);
  if(rows.some(row=>row.length>2))bad.push(`history: ${who.name}: merge commit in history`);
  if(rows.length!==fleet.messages.length)bad.push(`history: ${who.name}: ${rows.length} commits, expected ${fleet.messages.length}`);
  for(const message of fleet.messages){const count=subjects.filter(subject=>subject===message).length;if(count!==1)bad.push(`history: ${who.name}: message ${message} appears ${count} times`);}
  const fsck=gitV1(who,["fsck","--no-dangling","--no-reflogs"]);
  if(fsck.code!==0||/missing|broken|corrupt|error/iu.test(fsck.out+fsck.err))bad.push(`fsck: ${who.name}: ${(fsck.out+fsck.err).trim().slice(0,300)}`);
  if(existsSync(at(who,".git/index.lock")))bad.push(`lock: ${who.name}: .git/index.lock left behind`);
  if(who.replica!.queue.size!==0)bad.push(`queue: ${who.name}: ${who.replica!.queue.size} entries left`);
  const errors=who.events.filter(event=>event.event==="error");
  if(errors.length>0)bad.push(`error: ${who.name}: ${errors.length} error events, first: ${errors[0]!.message?.slice(0,300)}`);
  const report=await verifyV1(who.options);
  if(!report.ok)bad.push(`verify: ${who.name}: ${report.drift.slice(0,5).map(item=>`${item.kind} ${item.path}`).join("; ")}`);
 }
 const python=testPythonV1();
 if(python!==null){
  const ran=Bun.spawnSync([python,"-I",join(import.meta.dirname,"🌪️chaos.py"),"--count",String(fleet.messages.length),...fleet.messages.flatMap(message=>["--message",message]),...machines.map(who=>who.options.worktree)],{stdout:"pipe",stderr:"pipe"});
  if(ran.exitCode!==0)bad.push(`oracle: ${ran.stderr.toString().trim().replaceAll(fleet.root,"").slice(0,600)}`);
 }
 const attesters=machines.map(who=>createAttesterV1({storeRoot:fleet.real,originId:who.name,groupId:"g1",now:Date.now,heartbeatMilliseconds:1,viewMilliseconds:0,viewDebounceMilliseconds:0,git:signal=>gitStateV1(who.options.worktree,signal),view:signal=>gitViewV1(who.options.worktree,signal),reachable:()=>storeReachableV1(fleet.real)}));
 for(const [index,attester] of attesters.entries()){
  const who=machines[index]!,replica=who.replica!;
  await attester.start();
  let written=false,complete=false;
  const tick=():Promise<"idle"|"written"|"restart">=>attester.tick({sync:true,files:replica.files.shared,manifest:replica.files.manifestHex(),buckets:replica.files.bucketsHex(),cursors:replica.cursors,incoming:replica.queue.size,busy:replica.applying>0,hlc:formatHlcV1(replica.clock,who.name)});
  for(let attempt=0;attempt<150&&!complete;attempt++){
   if(await tick()==="written")written=true;
   const found=written?await readAttestationV1(fleet.real,who.name):undefined,checkpoints=found?.checkpoints;
   complete=checkpoints?.manifest!==undefined&&checkpoints.git!==undefined&&(found!.view===null||checkpoints.view!==undefined);
   if(!complete)await settled(20);
  }
  if(!written)bad.push(`attest: ${who.name}: attestation never written`);
  else if(!complete)bad.push(`attest: ${who.name}: checkpoints never complete`);
 }
 const report=await fleetReportV1(fleet.real);
 if(!report.ok)bad.push(`verdict: ${JSON.stringify(report.rows.filter(row=>row.verdict!=="ok").map(row=>[row.originId,row.verdict,row.differs]))} ${JSON.stringify(report.differing).slice(0,400)}`);
 return bad;
}

async function restartAll(fleet:ChaosFleetV1):Promise<void>{
 for(const who of fleet.machines){const old=who.replica!;who.replica=undefined;await closeReplicaV1(old);await openMachine(who);}
}

/** 🚦️ Runs a serialisable trace on a fresh fleet, settles, checks the invariants and (optionally) checks that a clean restart of every machine stays quiet. */
export async function replayV1(machines:number,ops:readonly ChaosOpV1[],options:Readonly<{keep?:boolean;settleMilliseconds?:number;restartCheck?:boolean}>={}):Promise<Readonly<{violations:string[];notes:string[];rounds:number;root:string}>> {
 const fleet=await buildFleetV1(machines),notes:string[]=[];
 try{
  for(const op of ops)notes.push(`${op.op}@${fleet.machines[op.m]?.name} ${await performV1(fleet,op)}`);
  const rounds=await settleV1(fleet,options.settleMilliseconds);
  const violations=rounds<0?["settle: the fleet did not quiesce"]:[];
  violations.push(...await checkFleetV1(fleet));
  if(options.restartCheck===true&&violations.length===0){
   const before=await signature(fleet);
   await restartAll(fleet);await settleV1(fleet,options.settleMilliseconds);
   if(await signature(fleet)!==before)violations.push("restart: a clean restart of every machine changed the store or a manifest");
   violations.push(...(await checkFleetV1(fleet)).map(item=>`after-restart ${item}`));
  }
  return {violations,notes,rounds,root:fleet.root};
 }finally{await disposeFleetV1(fleet,options.keep);}
}

/** 🌪️ Generates and applies `steps` random operations from `seed`, then settles and checks; the returned report carries the full trace for replay and shrinking. */
export async function runChaosV1(run:ChaosRunV1):Promise<ChaosReportV1>{
 const started=Date.now(),rng=rngV1(run.seed),fleet=await buildFleetV1(run.machines),ops:ChaosOpV1[]=[];
 try{
  for(let step=0;step<run.steps;step++){const op=generateV1(rng,fleet,run.profile,run.loose);ops.push(op);fleet.notes.push(`${op.op}@${fleet.machines[op.m]!.name} ${await performV1(fleet,op)}`);}
  const rounds=await settleV1(fleet,run.settleMilliseconds),violations=rounds<0?["settle: the fleet did not quiesce"]:[];
  violations.push(...await checkFleetV1(fleet));
  if(run.restartCheck===true&&violations.length===0){
   const before=await signature(fleet);
   await restartAll(fleet);await settleV1(fleet,run.settleMilliseconds);
   if(await signature(fleet)!==before)violations.push("restart: a clean restart of every machine changed the store or a manifest");
   violations.push(...(await checkFleetV1(fleet)).map(item=>`after-restart ${item}`));
  }
  return {seed:run.seed,machines:run.machines,ops,notes:fleet.notes,violations,rounds,milliseconds:Date.now()-started,kept:run.keep===true?fleet.root:undefined};
 }finally{await disposeFleetV1(fleet,run.keep);}
}

/** ✂️ Delta-debugging shrinker: removes chunks of the trace while `fails` still holds, halving the chunk size down to single operations; `budget` bounds the number of replays. */
export async function shrinkV1<T>(items:readonly T[],fails:(candidate:readonly T[])=>Promise<boolean>,budget:number=60):Promise<readonly T[]>{
 let current=[...items],size=Math.max(1,Math.floor(current.length/2)),runs=0;
 while(size>=1&&runs<budget){
  let removed=false;
  for(let start=0;start<current.length&&runs<budget;){
   const candidate=[...current.slice(0,start),...current.slice(start+size)];
   runs++;
   if(candidate.length<current.length&&await fails(candidate)){current=candidate;removed=true;}else start+=size;
  }
  if(!removed||size===1){if(size===1)break;size=Math.max(1,Math.floor(size/2));}
 }
 return current;
}
