/** 🧷️ Contract: the responsible process of a service is watched; the verdicts of the watch equal an independent Python model over shared cases, and on macOS the system call answers what Python's ctypes answers for the same process, and an orphan is its own responsible process once the app that was responsible for it ended.
 * @see ../../🔐️consent/🧷️responsibility/🟨️.mjs */
import {expect,test} from "bun:test";
import {copyFileSync,existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {HERE} from "../🟦️.ts";
import {createResponsibilityWatchV1,responsiblePidV1} from "../../🔐️consent/🧷️responsibility/🟨️.mjs";

const CASES=join(HERE,"🧷️responsibility/🧫️cases.json"),ORACLE=join(HERE,"🧷️responsibility/🐍️oracle.py"),ORPHAN=join(HERE,"🧷️responsibility/🐍️orphan.py"),python=(...args:string[])=>spawnSync("python3",["-I",ORACLE,...args],{encoding:"utf8"});

test("the verdicts of the watch equal the independent model over every shared case",async()=>{
  const spec=JSON.parse(readFileSync(CASES,"utf8")),model=python(CASES);
  expect(model.status,model.stderr).toBe(0);
  const expected=JSON.parse(model.stdout);
  for(const item of spec.cases){
   const answers=[...item.answers],watch=createResponsibilityWatchV1({pid:spec.pid,ask:async()=>{const answer=answers.shift();return answer===null?undefined:answer;}}),seen:string[]=[];
   for(let index=0;index<item.answers.length;index++){const verdict=await watch.tick();seen.push(verdict.lost?"lost":verdict.own?"own":"watch");}
   expect(seen,item.name).toEqual(item.expect);
   expect(seen,item.name).toEqual(expected[item.name]);
  }
});

test("a watch adopts the present responsible app as its baseline after the service proved its access",async()=>{
  const answers=[7,8,8],watch=createResponsibilityWatchV1({pid:100,ask:async()=>answers.shift()});
  await watch.tick();expect((await watch.tick()).lost).toBe(true);
  await watch.adopt();
  answers.push(8);expect(watch.was()).toBe(8);
  expect((await watch.tick()).lost).toBeUndefined();
});

test("no responsible app is named off macOS and when the system call fails",async()=>{
  expect(await responsiblePidV1(process.pid,{platform:"linux"})).toBeUndefined();
  expect(await responsiblePidV1(process.pid,{platform:"darwin",call:()=>{throw Error("no symbol");}})).toBeUndefined();
  expect(await responsiblePidV1(process.pid,{platform:"darwin",call:()=>-1})).toBeUndefined();
  expect(await responsiblePidV1(process.pid,{platform:"darwin",call:()=>321})).toBe(321);
});

test.skipIf(process.platform!=="darwin")("on macOS the system answers what Python's ctypes answers for this very process, and an orphan answers for itself once the app responsible for it ended",async()=>{
  const system=python("--system",String(process.pid));
  expect(system.status,system.stderr).toBe(0);
  expect(await responsiblePidV1(process.pid)).toBe(Number(system.stdout.trim()));
  const directory=mkdtempSync(join("/tmp","rp-")),report=join(directory,"report.json"),script=join(directory,"orphan.py");
  copyFileSync(ORPHAN,script);
  try{
   const started=spawnSync("python3",["-I",script,"launch",report],{encoding:"utf8",cwd:directory});
   expect(started.status,started.stderr).toBe(0);
   for(let waited=0;!existsSync(report)&&waited<8000;waited+=100)await new Promise(done=>setTimeout(done,100));
   const seen=JSON.parse(readFileSync(report,"utf8"));
   expect(seen.before).toBe(seen.a);
   expect(seen.after).toBe(seen.b);
   expect(await responsiblePidV1(seen.b)).toBeUndefined();
  }finally{rmSync(directory,{recursive:true,force:true});}
});
