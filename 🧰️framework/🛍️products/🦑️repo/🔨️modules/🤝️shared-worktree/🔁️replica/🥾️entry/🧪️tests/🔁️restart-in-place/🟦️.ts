/** 🔁️ The boot stage restarts the daemon in place: a daemon that is killed or crashes after it was ready (the watchdog, an out-of-memory kill, a code change) is started again by the boot program without the boot program, the shim and so the service manager ever seeing an exit; only the deliberate ends 0 (stop) and 75 (another daemon holds the tree) and a stop signal end the boot program. */
import {expect,test} from "bun:test";
import {appendFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {limitsV1,runBootV1} from "../../🟨️.mjs";

const relative="m",limits={...limitsV1({}),earlyMs:100,readyMs:5000,graceMs:2000,pollMs:20,backoffMs:40,backoffMaxMs:200,refuseMs:60_000};
const stub=(codes:readonly (number|"stay")[])=>`import {appendFileSync,mkdirSync,writeFileSync} from "node:fs";\nimport {join} from "node:path";\nconst root=process.env.SEMIO_FLEET_BOOT_ROOT;\nmkdirSync(join(root,".cache","f","boot"),{recursive:true});\nappendFileSync(join(root,"runs.txt"),"run\\n");\nconst run=require("node:fs").readFileSync(join(root,"runs.txt"),"utf8").split("\\n").filter(Boolean).length,codes=${JSON.stringify(codes)};\nwriteFileSync(join(root,".cache","f","boot","ready.json"),JSON.stringify({pid:process.pid,waiting:false}));\nprocess.on("SIGTERM",()=>{process.exit(0);});\nconst next=codes[Math.min(run-1,codes.length-1)];\nif(next==="stay")setInterval(()=>{},1000);else setTimeout(()=>process.exit(next),300);\n`;
const boot=(codes:readonly (number|"stay")[])=>{
 const base=mkdtempSync(join(tmpdir(),"restart-in-place-")),root=join(base,"wt"),entry=join(base,"entry"),file=join(root,relative,"🔁️replica","🛟️supervisor","🟦️.ts");
 mkdirSync(dirname(file),{recursive:true});mkdirSync(entry,{recursive:true});writeFileSync(file,stub(codes));
 const controller=new AbortController(),lines:string[]=[],finished=runBootV1({entry,worktree:root,moduleRel:relative,storeRoot:()=>undefined,environment:{...process.env},limits,args:["run"],say:line=>lines.push(line)},controller.signal);
 const runs=():number=>existsSync(join(root,"runs.txt"))?readFileSync(join(root,"runs.txt"),"utf8").split("\n").filter(Boolean).length:0;
 return{base,controller,lines,finished,runs};
};
const until=async(check:()=>boolean,milliseconds=15_000)=>{const end=Date.now()+milliseconds;while(!check()&&Date.now()<end)await new Promise(done=>setTimeout(done,20));};

test("a daemon that is killed or crashes after it was ready is started again by the boot program, which does not end, so launchd never sees an exit",async()=>{
 const run=boot([137,1,"stay"]);
 try{
  await until(()=>run.runs()>=3);
  expect(run.runs()).toBe(3);
  expect(await Promise.race([run.finished,new Promise(done=>setTimeout(()=>done("still running"),500))])).toBe("still running");
  expect(run.lines.some(line=>line.includes("starts it again")&&line.includes("startet ihn"))).toBe(true);
  run.controller.abort();
  expect(await Promise.race([run.finished,new Promise(done=>setTimeout(()=>done("timeout"),10_000))])).not.toBe("timeout");
 }finally{run.controller.abort();rmSync(run.base,{recursive:true,force:true});}
});

test("the deliberate ends stay ends: a daemon that stops with 0 or reports another daemon with 75 ends the boot program with that code",async()=>{
 for(const code of [0,75]){
  const run=boot([code]);
  try{
   expect(await Promise.race([run.finished,new Promise(done=>setTimeout(()=>done("timeout"),15_000))]),String(code)).toBe(code);
   expect(run.runs(),String(code)).toBe(1);
  }finally{run.controller.abort();rmSync(run.base,{recursive:true,force:true});}
 }
});

test("a crash loop is paced by a doubling pause, not restarted at full speed",async()=>{
 const run=boot([1]);
 try{
  const started=Date.now();
  await until(()=>run.runs()>=4,20_000);
  expect(run.runs()).toBeGreaterThanOrEqual(4);
  expect(Date.now()-started).toBeGreaterThan(40+80+160);
 }finally{run.controller.abort();rmSync(run.base,{recursive:true,force:true});}
});
