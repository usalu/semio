import {afterAll,beforeAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {execFile} from "node:child_process";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";
import {CHANGE_JOURNAL_POLICY_V1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {createChangeJournalClientV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {applyChangeRowsV1,diffTreeStampsV1,walkTreeStampsV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🌳️tree/🟦️.ts";
import {runChangeJournalServiceV1} from "../../🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url));
const repository=process.env.SEMIO_FUZZ_REPOSITORY??(()=>{let at=here;while(at!==dirname(at)&&!existsSync(join(at,"AGENTS.md")))at=dirname(at);return at;})();
const peer=join(repository,"🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🎲️fuzz/🐍️.py"),script=join(here,"🐍️.py");
const binary=process.env.SEMIO_REPO_WATCH_BINARY??"";
const supervisorModule=join(repository,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔭️watch/🟨️.mjs"),policyModule=join(repository,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📨️protocol/🎚️policy/🟨️.mjs");
const seeds=Number(process.env.SEMIO_FUZZ_SEEDS??"12");
let sandbox="",service:Awaited<ReturnType<typeof runChangeJournalServiceV1>>|undefined;
const python=(...args:string[])=>new Promise<string>((accept,reject)=>execFile("python3",["-I",script,...args],{encoding:"utf8"},(error,stdout,stderr)=>error?reject(Error(stderr||String(error))):accept(stdout)));

beforeAll(async()=>{
 if(!binary)return;
 sandbox=realpathSync(mkdtempSync(join(tmpdir(),"journal-fuzz-")));
 const{createWatchSupervisorV1}=await import(pathToFileURL(supervisorModule).href),{POLICY_V1}=await import(pathToFileURL(policyModule).href);
 service=await runChangeJournalServiceV1({root:sandbox,directory:join(sandbox,".journal"),fence:".fence",exclude:{rooted:[".journal"],names:[]},binary,watcherPolicy:POLICY_V1,createSupervisor:createWatchSupervisorV1,residentBytes:()=>0});
});
afterAll(async()=>{await service?.stop("test");if(sandbox)rmSync(sandbox,{recursive:true,force:true});});

test.skipIf(!binary)("the change set derived from the journal equals a complete walk on a fuzz corpus of renames, deletes, tears, swaps, replacements and moves (Python peer, Python delta oracle)",async()=>{
 const client=createChangeJournalClientV1({directory:join(sandbox,".journal"),root:sandbox,insensitive:process.platform==="darwin"||process.platform==="win32"});
 const tally={checked:0,files:0,kinds:new Set<string>()};
 for(let seed=0;seed<seeds;seed++){
  const name="s"+seed,work=join(sandbox,name);
  await python("make","--peer",peer,"--work",work);
  const skip=(path:string)=>path.startsWith(name+"/tmp/")&&false;
  const cursor=await client.cursor();
  expect("epoch" in cursor).toBe(true);
  const before=await walkTreeStampsV1(sandbox,[name],skip);
  const applied=JSON.parse(await python("ops","--peer",peer,"--work",work,"--seed",String(seed),"--count","8")) as {applied:string[]};
  for(const kind of applied.applied)tally.kinds.add(kind);
  const answer=await client.since(cursor as {epoch:string;seq:number},[name]);
  expect(answer).toMatchObject({exact:true});
  if(!answer.exact)continue;
  const derived=await applyChangeRowsV1(sandbox,before,answer.rows,[name],skip);
  const walked=await walkTreeStampsV1(sandbox,[name],skip),reference=diffTreeStampsV1(before,walked);
  expect({seed,stamps:[...derived.stamps].sort()}).toEqual({seed,stamps:[...walked].sort()});
  expect({seed,delta:derived.delta}).toEqual({seed,delta:reference});
  const file=join(sandbox,"d"+seed+".json");writeFileSync(file,JSON.stringify({before:Object.fromEntries(before),after:Object.fromEntries(walked)}));
  const oracle=JSON.parse(await python("diff",file));
  expect({seed,delta:derived.delta}).toEqual({seed,delta:oracle});
  tally.checked++;tally.files+=derived.delta.added.length+derived.delta.removed.length+derived.delta.changed.length;
 }
 expect(tally.checked).toBe(seeds);
 expect(tally.files).toBeGreaterThan(0);
},180000);

test.skipIf(!binary)("a quiet tree costs no rows and a root outside the changed area gets none",async()=>{
 const client=createChangeJournalClientV1({directory:join(sandbox,".journal"),root:sandbox,insensitive:process.platform==="darwin"||process.platform==="win32"});
 mkdirSync(join(sandbox,"quiet/a"),{recursive:true});mkdirSync(join(sandbox,"busy"),{recursive:true});
 const cursor=await client.cursor() as {epoch:string;seq:number};
 writeFileSync(join(sandbox,"busy/x"),"x");
 const answer=await client.since(cursor,["quiet"]);
 expect(answer.exact&&answer.rows).toEqual([]);
 const busy=await client.since(cursor,["busy"]);
 expect(busy.exact&&busy.rows.map(row=>row[2])).toEqual(["busy/x"]);
 expect(CHANGE_JOURNAL_POLICY_V1.fenceTimeoutMilliseconds).toBeGreaterThan(0);
});

test.skipIf(!binary)("negative control: a journal that loses every directory row is caught by the same corpus",async()=>{
 const client=createChangeJournalClientV1({directory:join(sandbox,".journal"),root:sandbox,insensitive:process.platform==="darwin"||process.platform==="win32"});
 let wrong=0;
 for(let seed=100;seed<100+seeds;seed++){
  const name="n"+seed,work=join(sandbox,name);
  await python("make","--peer",peer,"--work",work);
  const cursor=await client.cursor() as {epoch:string;seq:number},before=await walkTreeStampsV1(sandbox,[name]);
  await python("ops","--peer",peer,"--work",work,"--seed",String(seed),"--count","8");
  const answer=await client.since(cursor,[name]);
  if(!answer.exact)continue;
  const lossy=await applyChangeRowsV1(sandbox,before,answer.rows.filter(row=>row[1]===0),[name]);
  const walked=await walkTreeStampsV1(sandbox,[name]),delta=diffTreeStampsV1(lossy.stamps,walked);
  if(delta.added.length+delta.removed.length+delta.changed.length>0)wrong++;
 }
 expect(wrong).toBeGreaterThan(0);
},180000);
