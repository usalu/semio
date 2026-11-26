/** 📈️ Contract: a counter that survives a restart is the ledger plus the run, never falls, and is written atomically with no orphan temporary file; per-event counter files count one byte per event. Cross-checked with an independent Python sum and Ajv.
 * @see ../🟦️.ts */
import{afterAll,expect,test}from"bun:test";
import Ajv from"ajv";
import{existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{testPythonV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{COUNTER_LEDGER_POLICY_V1,countEventV1,openCounterLedgerV1,parseCounterLedgerV1,readEventCountsV1,retireCounterTemporariesV1,type CounterGroupsV1}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};

const scratch:string[]=[],directory=():string=>{const path=mkdtempSync(join(tmpdir(),"ledger-"));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const ajv=new Ajv({strict:true,allErrors:true}),definition=(name:string)=>ajv.compile({definitions:schema.definitions,$ref:`#/definitions/${name}`});
const oracle=()=>{
 const python=testPythonV1(),run=Bun.spawnSync([python!,"-I",join(import.meta.dir,"🐍️oracle","🐍️.py")],{stdin:Buffer.from(JSON.stringify({...fixture,pattern:COUNTER_LEDGER_POLICY_V1.temporaryPattern}))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 return JSON.parse(run.stdout.toString()) as {cases:{total:CounterGroupsV1;restarts:number}[];temporaries:([string,number]|null)[]};
};

test("every fixture case is valid against its schema definition",()=>{
 const valid=definition("Case");
 for(const row of fixture.cases)expect(valid(row),JSON.stringify(valid.errors)).toBe(true);
});

test("the cumulative count over restarts equals the independent sum, and every written file is valid against the schema",()=>{
 const expected=oracle(),valid=definition("Ledger");
 fixture.cases.forEach((row,index)=>{
  const path=join(directory(),"nested","counters.json");
  for(const run of row.runs){
   const ledger=openCounterLedgerV1({path,name:"test-ledger",flushMilliseconds:3600000});
   ledger.bind(()=>run.groups as CounterGroupsV1);ledger.close();
  }
  const written=JSON.parse(readFileSync(path,"utf8")) as {groups:CounterGroupsV1;restarts:number};
  expect(valid(written),JSON.stringify(valid.errors)).toBe(true);
  expect(written.groups,row.name).toEqual(expected.cases[index]!.total);
  expect(written.restarts).toBe(expected.cases[index]!.restarts);
  expect<unknown>(written.groups,row.name).toEqual(row.total);
  expect<unknown>(parseCounterLedgerV1(readFileSync(path,"utf8"))?.groups).toEqual(row.total);
 });
});

test("a restarting process reads its persisted counters, the cumulative count includes the run in progress and never falls",()=>{
 const path=join(directory(),"counters.json");
 const first=openCounterLedgerV1({path,name:"cache",flushMilliseconds:3600000});let hits=5;first.bind(()=>({ac:{hits}}));first.close();
 const second=openCounterLedgerV1({path,name:"cache",flushMilliseconds:3600000});
 expect(second.total()).toEqual({ac:{hits:5}});expect(second.restarts).toBe(2);
 let run=0;second.bind(()=>({ac:{hits:run}}));
 for(const next of[0,1,4,4,9]){run=next;expect(second.total().ac!.hits).toBe(5+next);}
 second.flush();expect(JSON.parse(readFileSync(path,"utf8")).groups.ac.hits).toBe(14);second.close();
 expect(second.sinceMs).toBe(first.sinceMs);
});

test("a missing, torn or foreign file is an empty ledger, not a failure",()=>{
 for(const text of[undefined,"{\"version\":1,","[]","{\"version\":2}",JSON.stringify({version:1,name:"x",sinceMs:0,updatedMs:0,restarts:1,groups:{a:{b:-1}}}),JSON.stringify({version:1,name:"x",sinceMs:0,updatedMs:0,restarts:1,groups:{},extra:1})]){
  const path=join(directory(),"counters.json");if(text!==undefined)writeFileSync(path,text);
  const ledger=openCounterLedgerV1({path,name:"x",flushMilliseconds:3600000});
  expect(ledger.total()).toEqual({});expect(ledger.restarts).toBe(1);ledger.close();
 }
});

test("a flush writes only when a count changed, and a failed rename leaves no temporary file",()=>{
 const root=directory(),path=join(root,"counters.json");let now=1000;
 const ledger=openCounterLedgerV1({path,name:"cache",now:()=>now,flushMilliseconds:3600000});let hits=1;ledger.bind(()=>({ac:{hits}}));
 ledger.flush();const first=readFileSync(path,"utf8");now=2000;ledger.flush();expect(readFileSync(path,"utf8")).toBe(first);
 hits=2;now=3000;ledger.flush();expect(JSON.parse(readFileSync(path,"utf8")).updatedMs).toBe(3000);ledger.close();
 const blocked=join(root,"blocked");mkdirSync(join(blocked,"counters.json"),{recursive:true});writeFileSync(join(blocked,"counters.json","keep"),"x");
 const broken=openCounterLedgerV1({path:join(blocked,"counters.json"),name:"cache",flushMilliseconds:3600000});broken.bind(()=>({ac:{hits:1}}));broken.close();
 expect(readdirSync(blocked)).toEqual(["counters.json"]);
});

test("temporary files of a process that is gone or too old are retired, the caller's own and other files never, and the pattern equals Python's",()=>{
 const expected=oracle(),root=directory(),path=join(root,"counters.json"),now=Date.now();
 fixture.temporaries.forEach((name,index)=>{
  const found=new RegExp(COUNTER_LEDGER_POLICY_V1.temporaryPattern,"u").exec(name);
  expect(found===null?null:[found[1],Number(found[2])]).toEqual(expected.temporaries[index]);
 });
 for(const name of["counters.json.101.tmp","counters.json.102.tmp",`counters.json.${process.pid}.tmp`,"other.json.101.tmp","counters.json"])writeFileSync(join(root,name),"x");
 const removed=retireCounterTemporariesV1(path,now,pid=>pid===102);
 expect(removed).toEqual(["counters.json.101.tmp"]);
 expect(existsSync(join(root,"counters.json.102.tmp"))).toBe(true);
 expect(existsSync(join(root,`counters.json.${process.pid}.tmp`))).toBe(true);expect(existsSync(join(root,"other.json.101.tmp"))).toBe(true);
 expect(retireCounterTemporariesV1(path,now+COUNTER_LEDGER_POLICY_V1.temporaryMaximumAgeMilliseconds+60000,()=>true)).toEqual(["counters.json.102.tmp"]);
 expect(existsSync(join(root,"counters.json"))).toBe(true);
});

test("opening a ledger retires the temporary file of a dead writer",()=>{
 const root=directory(),path=join(root,"counters.json");let dead=2147483000;while(true){try{process.kill(dead,0);dead--;}catch{break;}}
 writeFileSync(join(root,`counters.json.${dead}.tmp`),"");
 openCounterLedgerV1({path,name:"cache",flushMilliseconds:3600000}).close();
 expect(readdirSync(root)).toEqual(["counters.json"]);
});

test("event counters append one byte per event, count across names and ignore a name that is none",()=>{
 const root=join(directory(),"events");
 for(let index=0;index<3;index++)countEventV1(root,"localHit",()=>1234);
 countEventV1(root,"nxHit");countEventV1(root,"../escape");countEventV1(undefined,"localHit");
 const read=readEventCountsV1(root,["localHit","nxHit","absent"]);
 expect(read.counts).toEqual({localHit:3,nxHit:1,absent:0});expect(read.sinceMs).toBe(1234);
 expect(statSync(join(root,"localHit")).size).toBe(3);expect(existsSync(join(root,"..","escape"))).toBe(false);
 expect(readEventCountsV1(join(root,"none"),["localHit"])).toEqual({counts:{localHit:0}});
});
