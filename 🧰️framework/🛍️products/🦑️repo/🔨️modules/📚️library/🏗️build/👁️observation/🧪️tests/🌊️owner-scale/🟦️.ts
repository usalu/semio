/** 🌊️ Contract: the observation of one owner is bounded by its rows, not by its files: an owner that reads 100000 files in a child process charged 512 MB (`BUN_JSC_forceRAMSize`, the declared heap of an inference child) stays below the charge in physical footprint, with every distinct file one row, in recording order. */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const FILES=100000,CHARGE_MEGABYTES=512;
const child=`
import{mkdtempSync,realpathSync,rmSync}from"node:fs";import{tmpdir}from"node:os";import{join}from"node:path";
const{captureObservedSourceInputsV1,observeInferenceSnapshotV1,observedFs}=await import(${JSON.stringify(new URL("../../🟨️.mjs",import.meta.url).pathname)});
const root=realpathSync(mkdtempSync(join(tmpdir(),"owner-scale-child-"))),paths=Array.from({length:${FILES}},(_,at)=>join(root,"d".repeat(40),String(at)));
const before=process.memoryUsage().rss;let peak=before;const timer=setInterval(()=>{peak=Math.max(peak,process.memoryUsage().rss);},20);
const rows=await observeInferenceSnapshotV1(async()=>(await captureObservedSourceInputsV1(async()=>{for(const path of paths)observedFs.existsSync(path);},undefined,false)).observations);
clearInterval(timer);peak=Math.max(peak,process.memoryUsage().rss);
console.log(JSON.stringify({rows:rows.length,ordered:rows.every((row,at)=>row.path===paths[at]),grewMegabytes:Math.round((peak-before)/1048576)}));
rmSync(root,{recursive:true,force:true});
`;

test("100000 files of one owner grow a child charged 512 MB by less than the charge",()=>{
 const directory=mkdtempSync(join(tmpdir(),"owner-scale-")),file=join(directory,"child.mjs");
 try{
  writeFileSync(file,child);
  const run=spawnSync(process.execPath,[file],{env:{...process.env,BUN_JSC_forceRAMSize:String(CHARGE_MEGABYTES*1048576)},encoding:"utf8",timeout:150000});
  expect(run.status,run.stderr).toBe(0);
  const result=JSON.parse(run.stdout.trim().split("\n").pop()!);
  expect(result.rows).toBe(FILES);expect(result.ordered).toBe(true);
  expect(result.grewMegabytes,`grew ${result.grewMegabytes} MB`).toBeLessThan(CHARGE_MEGABYTES);
 }finally{rmSync(directory,{recursive:true,force:true});}
},170000);
