/** 🎲️ Fixture build for observation churn fuzz. */
// Fixture build of the churn fuzz harness: one process that behaves like a build step. It loads an "implementation" (modules a peer may half-edit), observes a source tree
// (every file hashed once), and writes ONE result. It runs under the real restart supervisor of the library under test (--lib), so a restart is a real new process.
// usage: bun 🟨️.mjs --lib <root holding the library tree> --work <workspace> [--sync] [--naive] [--workMs 5] [--scale 0.05]
// `--naive` observes the tree the way a consumer without the stable snapshot does (list, hash each file, no check across files): the negative control of the oracle.
import {createHash} from "node:crypto";
import {existsSync,lstatSync,readFileSync,readdirSync,renameSync,writeFileSync} from "node:fs";
import {join,relative} from "node:path";
import {pathToFileURL} from "node:url";

const argv=process.argv.slice(2),arg=(name,fallback)=>{const at=argv.indexOf("--"+name);return at<0?fallback:argv[at+1];};
const lib=arg("lib"),work=arg("work"),sync=argv.includes("--sync"),naive=argv.includes("--naive"),workMs=Number(arg("workMs","5")),scale=Number(arg("scale","0.05"));
const F="🧰️framework",B=F+"/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const url=path=>pathToFileURL(join(lib,path)).href;
const churn=await import(url(F+"/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts"));
const revision=await import(url(B+"/📥️inputs/🔏️revision/🟨️.mjs"));
const reexec=await import(url(B+"/🧷️format-stamps/🔄️reexec/🟨️.mjs"));
const snapshot=await import(url(F+"/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/📸️snapshot/🟦️.ts")).catch(()=>undefined);
const pause=milliseconds=>Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,Math.max(0,milliseconds));
const ctl=join(work,"ctl");
let counter=0;
globalThis.__tick=label=>{
 if(!sync)return;
 const name=process.pid+"."+(++counter)+"."+label;writeFileSync(join(ctl,"req-"+name),"");
 for(let waited=0;waited<10000;waited+=2){if(existsSync(join(ctl,"ack-"+name)))return;pause(2);}
};
const tick=label=>globalThis.__tick(label);
const hashOf=bytes=>createHash("sha256").update(bytes).digest("hex");

function listTree(root){
 const files=[];
 const visit=directory=>{for(const entry of readdirSync(directory,{withFileTypes:true}).sort((left,right)=>left.name<right.name?-1:1)){const path=join(directory,entry.name);if(entry.isDirectory())visit(path);else files.push(path);}};
 visit(root);return files;
}
const consume=async path=>{
 tick("hash");
 const capture=revision.captureBoundedSourceRevisionV1(path,65536,false);
 if(workMs>0)await new Promise(accept=>setTimeout(accept,workMs));
 return capture.sha256;
};

async function observeNaive(root){
 let attempts=0;
 const files=await churn.retryOnConcurrentEditV1("snapshot",async attempt=>{
  attempts=attempt;
  const values=new Map();
  const paths=listTree(root);
  for(let at=0;at<paths.length;at+=32)await Promise.all(paths.slice(at,at+32).map(async path=>values.set(path,await consume(path))));
  return new Map([...values].sort((left,right)=>left[0]<right[0]?-1:1));
 },{});
 return{files,attempts};
}
async function observeStable(root){
 const result=await snapshot.observeStableTreeV1([root],consume,{sleepSync:pause});
 return{files:result.files,attempts:result.attempts};
}

async function run(){
 tick("start");
 const impl=await import(join(work,"impl","main.mjs"));
 tick("impl");
 const loadedProtocol=JSON.parse(readFileSync(join(work,"impl","protocol.json"),"utf8"));
 tick("protocol");
 const src=join(work,"src"),seen=snapshot&&!naive?await observeStable(src):await observeNaive(src);
 tick("hashed");
 tick("verify-protocol");
 const currentProtocol=JSON.parse(readFileSync(join(work,"impl","protocol.json"),"utf8"));
 if(JSON.stringify(currentProtocol)!==JSON.stringify(loadedProtocol))throw naive?Error("Exact captured parent closure protocol required"):churn.staleImplementationV1("Stale loaded protocol: the captured closure differs from the one this process loaded; restart required");
 const manifest={};for(const[path,sha]of seen.files)manifest[relative(src,path).split("\\").join("/")]=sha;
 const sorted=Object.fromEntries(Object.keys(manifest).sort().map(key=>[key,manifest[key]]));
 const result={manifest:sorted,digest:hashOf(JSON.stringify(sorted)),impl:impl.version(),attempts:seen.attempts,restarts:Number(process.env.SEMIO_STALE_REEXEC??"0"),variant:snapshot&&!naive?"stable":"naive"};
 const temporary=join(work,"out","result.json.tmp");writeFileSync(temporary,JSON.stringify(result));renameSync(temporary,join(work,"out","result.json"));
}

const outcome=await reexec.runWithStaleReexecV1(run,{environment:process.env,outermost:true,sleep:milliseconds=>new Promise(accept=>setTimeout(accept,milliseconds*scale)),argv:process.argv,execArgv:process.execArgv});
process.exit(outcome.status==="completed"?0:outcome.code);
