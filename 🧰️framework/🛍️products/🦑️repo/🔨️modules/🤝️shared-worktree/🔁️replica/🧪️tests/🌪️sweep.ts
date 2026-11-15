import {existsSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {replayV1,runChaosV1,shrinkV1,type ChaosOpV1} from "./🌪️harness.ts";

/** 🧹️ The first ancestor directory that holds `.🧬semio`: the checkout root. */
function checkoutRoot(from:string):string{
 let current=from;
 while(!existsSync(join(current,".🧬semio"))&&dirname(current)!==current)current=dirname(current);
 return current;
}

const root=checkoutRoot(import.meta.dirname),arg=(name:string,fallback:string):string=>{const index=process.argv.indexOf(`--${name}`);return index<0?fallback:process.argv[index+1]!;};
const flag=(name:string):boolean=>process.argv.includes(`--${name}`);
const machines=Number(arg("machines","3")),steps=Number(arg("steps","30")),restartCheck=!flag("no-restart-check"),settleMilliseconds=Number(arg("settle","45000")),profile=arg("profile","mixed") as "mixed"|"hostile",loose=flag("loose");
const out=arg("out",join(root,".🧬semio","🦑️repo","🎫️tickets","🎆️26","🌙️11","☀️09","SHARED-WORKTREE-FLEET-OVER-NFS","🗑️generated","chaos"));

type Result={seed:number;violations:string[];ops:ChaosOpV1[];notes:string[];rounds:number;milliseconds:number};

if(flag("one")){
 const report=await runChaosV1({seed:Number(arg("one","1")),machines,steps,restartCheck,settleMilliseconds,profile,loose});
 console.log(`CHAOS_RESULT ${JSON.stringify(report)}`);
 process.exit(0);
}

if(flag("shrink-file")){
 const file=arg("shrink-file",""),saved=JSON.parse(readFileSync(file,"utf8")) as Result&{machines:number},category=saved.violations[0]!.split(":")[0]!;
 const fails=async(candidate:readonly ChaosOpV1[]):Promise<boolean>=>(await replayV1(saved.machines,candidate,{restartCheck,settleMilliseconds})).violations.some(item=>item.startsWith(category));
 const shrunk=await shrinkV1(saved.ops,fails,Number(arg("budget","80")));
 const final=await replayV1(saved.machines,shrunk,{restartCheck,settleMilliseconds});
 writeFileSync(file,JSON.stringify({...saved,shrunk,shrunkViolations:final.violations,shrunkNotes:final.notes},null,1));
 console.log(`shrunk ${saved.ops.length} -> ${shrunk.length}`);
 process.exit(0);
}

async function child(args:readonly string[],timeoutMs:number):Promise<string>{
 const env={...process.env};
 for(const key of Object.keys(env))if(key.startsWith("SEMIO_FLEET_")||key.startsWith("SEMIO_REMOTE_CACHE_"))delete env[key];
 const proc=Bun.spawn([process.execPath,"run",import.meta.path,...args],{stdout:"pipe",stderr:"pipe",env});
 const timer=setTimeout(()=>proc.kill(),timeoutMs);
 const text=await new Response(proc.stdout).text();
 clearTimeout(timer);
 return text;
}

const from=Number(arg("from","1")),count=Number(arg("count","20")),parallel=Number(arg("parallel","3")),timeout=Number(arg("timeout","240"))*1000,common=["--machines",String(machines),"--steps",String(steps),...restartCheck?[]:["--no-restart-check"],"--settle",String(settleMilliseconds),"--profile",profile,...loose?["--loose"]:[]];
mkdirSync(out,{recursive:true});
const started=Date.now(),messages=new Map<string,number>(),categories=new Map<string,number>(),failing:number[]=[];
let next=from,passed=0,timedOut=0;
await Promise.all(Array.from({length:parallel},async()=>{
 while(next<from+count){
  const seed=next++,text=await child(["--one",String(seed),...common],timeout),line=text.split("\n").find(item=>item.startsWith("CHAOS_RESULT "));
  if(line===undefined){timedOut++;failing.push(seed);writeFileSync(join(out,`seed-${seed}.json`),JSON.stringify({seed,machines,violations:["timeout: the run printed no result"],ops:[],notes:[]}));console.log(`seed ${seed}: no result`);continue;}
  const report=JSON.parse(line.slice("CHAOS_RESULT ".length)) as Result;
  if(report.violations.length===0){passed++;continue;}
  failing.push(seed);
  for(const category of new Set(report.violations.map(item=>item.split(":")[0]!)))categories.set(category,(categories.get(category)??0)+1);
  for(const item of report.violations.filter(entry=>entry.startsWith("error:")))messages.set(item.replace(/^error: [^:]+: /u,"").replaceAll(/\/var\/folders\/\S*?\/chaos-\w+\/wt\/[^/']+/gu,"<wt>").slice(0,160),(messages.get(item)??0)+1);
  writeFileSync(join(out,`seed-${seed}.json`),JSON.stringify({...report,machines},null,1));
  console.log(`seed ${seed}: FAIL ${report.violations[0]!.slice(0,200)}`);
 }
}));
if(flag("shrink"))for(const seed of failing.sort((a,b)=>a-b).slice(0,Number(arg("shrink-max","6")))){console.log((await child(["--shrink-file",join(out,`seed-${seed}.json`),...common],1_800_000)).trim());}
const summary={from,count,machines,steps,restartCheck,passed,failed:failing.length,noResult:timedOut,seconds:Math.round((Date.now()-started)/1000),failingSeeds:failing.sort((a,b)=>a-b),categories:Object.fromEntries(categories),errorMessages:Object.fromEntries(messages)};
writeFileSync(join(out,`summary-${from}-${count}.json`),JSON.stringify(summary,null,1));
console.log(JSON.stringify(summary));
