import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {mkdtempSync,rmSync} from "node:fs";
import {cp} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {GENERATION_POLICY_V1,createGenerationStoreV1,createNodeFilePortV1} from "../../../🟨️.mjs";
import {implementationDigestV1,loadGenerationGraphV1,restoreExpectedWholeGenerationV1} from "../../🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";
import fixture from "./🔣️.json";
import retryFixture from "./🕓️retry.json";
import localWaitFixture from "./🖥️local-wait.json";

const projectNames=(graph:unknown):string[]=>{if(typeof graph==="object"&&graph!==null&&"projects"in graph&&typeof graph.projects==="object"&&graph.projects!==null)return Object.keys(graph.projects);throw Error("graph with projects required");};

test("a restore-only request preserves a matching seal and attempts one exact head only under structural custody",async()=>{
 using oracle=new Database(":memory:");
 const rule=oracle.query(`SELECT CASE $case WHEN 'matching' THEN 'local' WHEN 'wrong-code' THEN 'foreign' WHEN 'busy' THEN 'busy'
  WHEN 'raced-local' THEN 'local' WHEN 'missing-custody' THEN 'busy' WHEN 'stale-head' THEN 'incomplete' ELSE 'restored' END AS status,
  $case NOT IN ('matching','wrong-code','busy') AS builders,
  $case IN ('stale-head','restored') AS restores,
  CASE WHEN $case='restored' THEN 2 ELSE 0 END AS fences`);
 const area=mkdtempSync(join(tmpdir(),"whole-adoption-"));
 try{
  const producer=join(area,"producer");await seed(producer,ringFiles(6));
  const source=createMockHost(producer),files=createNodeFilePortV1(),base=join(producer,".gen");
  await loadGenerationGraphV1(source.host);
  const pointerBytes=(await files.read(join(base,"current.json")))!;
  const pointer=JSON.parse(Buffer.from(pointerBytes).toString());
  const manifestBytes=(await files.read(join(base,"m",pointer.manifest+".json")))!;
  const identity=await source.host.identity(),implementation=implementationDigestV1(await source.host.implementationRows());
  for(const row of fixture){
   const root=join(area,row.name);await seed(root,ringFiles(6));
   const mock=createMockHost(root),host=mock.host,gen=join(root,".gen"),localPointer=join(gen,"current.json"),counts={builders:0,restores:0,fences:0};
   const expected={origin:"g1-producer",scope:"all",generation:7,manifest:"1".repeat(64),identity,platform:"darwin-arm64",implementation:row.name==="wrong-code"?"2".repeat(64):implementation};
   const putLocal=async()=>{await files.write(join(gen,"m",pointer.manifest+".json"),manifestBytes);await files.write(localPointer,pointerBytes);};
   if(row.name==="matching")await putLocal();
   Object.assign(host,{
    builderIdle:async()=>row.name!=="busy",
    withBuilder:async(body:(store:unknown,control:unknown,claim:unknown)=>Promise<unknown>,request:{structural:boolean},_wait:undefined,purpose?:string)=>{
     expect(request.structural).toBe(true);expect(purpose).toBe("restore");counts.builders++;
     if(row.name==="raced-local")await putLocal();
     const claim=row.name==="missing-custody"?undefined:{structural:true,fence:async(body:()=>Promise<unknown>)=>{counts.fences++;return body();}};
     return body({root:gen},{},claim);
    },
    exchange:{async restore(options:any){
     counts.restores++;expect(options.expectedHead).toEqual(expected);expect(options.requireComplete).toBe(true);
     if(row.name==="stale-head")return{status:"incomplete"};
     await options.fs.write(join(gen,"m",pointer.manifest+".json"),manifestBytes);
     await options.fs.write(localPointer,pointerBytes);
     return{status:"restored",head:expected,generation:pointer.generation,owners:6,documents:1,bytes:manifestBytes.length};
    }},
    discoverPart:()=>{throw Error("Unexpected inference");},expand:()=>{throw Error("Unexpected assembly");},
   });
   const intents:Array<{generation:number;localManifest:string}>=[];
   const result=await restoreExpectedWholeGenerationV1(host,expected,{beforePublication:async local=>{
    expect(await files.read(localPointer),row.name).toBeUndefined();
    expect(counts.fences,row.name).toBe(1);
    intents.push(local);
   }});
   expect(intents,row.name).toEqual(row.status==="restored"?[{generation:pointer.generation,localManifest:pointer.manifest}]:[]);
   const observed={status:result.status,...counts};
   expect<unknown>(observed,row.name).toEqual(rule.get({$case:row.name}));
   expect<unknown>(observed,row.name).toEqual({status:row.status,builders:row.builders,restores:row.restores,fences:row.fences});
   if(result.status==="restored")expect(result.localManifest).toBe(pointer.manifest);
   expect(mock.log,row.name).toEqual([]);
  }
 }finally{rmSync(area,{recursive:true,force:true});}
});


test("a waiting whole read retries its same exact candidate after a transient restore failure",async()=>{
 using oracle=new Database(":memory:");
 const area=mkdtempSync(join(tmpdir(),"whole-retry-"));
 try{
  const producer=join(area,"producer"),reader=join(area,"reader"),source=createMockHost(producer),target=createMockHost(reader),files=createNodeFilePortV1();
  await seed(producer,ringFiles(6));await seed(reader,ringFiles(6));await loadGenerationGraphV1(source.host);
  const sourceRoot=join(producer,".gen"),targetRoot=join(reader,".gen"),pointer=(await files.read(join(sourceRoot,"current.json")))!;
  const identity=await source.host.identity(),implementation=implementationDigestV1(await source.host.implementationRows());
  const expected={origin:"g1-producer",scope:"all",generation:7,manifest:"1".repeat(64),identity,platform:"darwin-arm64",implementation};
  let clock=0,restores=0;
  const availability:number[]=[];
  Object.assign(target.host,{
   now:()=>clock,
   waitNoticeMilliseconds:1,
   async prepareWholeRebuild(wait:{available():Promise<boolean>;restore():Promise<unknown>}){
    for(const offset of retryFixture.observations){
     clock=offset;
     availability.push(Number(await wait.available()));
    }
    const expectedAvailability=oracle.query("select value >= ?1 as eligible from json_each(?2) order by key").values(retryFixture.retryMilliseconds,JSON.stringify(retryFixture.observations)).flat();
    expect<unknown>(availability).toEqual(expectedAvailability);
    return{restored:await wait.restore()};
   },
   async wholeGate({look,fund}:{look():Promise<{ahead:boolean;candidate?:typeof expected}>;fund():Promise<unknown>}){
    const seen=await look();
    if(restores===0)return{candidate:seen.candidate,release:async()=>{}};
    expect(seen.ahead).toBe(false);
    expect(await fund()).toBeUndefined();
    return{release:async()=>{}};
   },
   exchange:{
    async look(){return{state:"complete",ahead:true,held:0,candidate:expected};},
    async restore(options:{expectedHead:typeof expected;requireComplete:boolean;fs:typeof files}){
     expect(options.expectedHead).toEqual(expected);expect(options.requireComplete).toBe(true);
     if(++restores===1)return{status:"incomplete",reason:"closure temporarily unavailable"};
     for(const name of ["m","o","f","x"])await cp(join(sourceRoot,name),join(targetRoot,name),{recursive:true});
     await options.fs.write(join(targetRoot,"current.json"),pointer);
     return{status:"restored",head:expected,generation:7};
    }
   },
  });
  const result=await loadGenerationGraphV1(target.host,{lean:true});
  expect(restores).toBe(2);
  expect(target.log).toEqual([]);
  expect(projectNames(result.graph)).toHaveLength(6);
 }finally{rmSync(area,{recursive:true,force:true});}
});


test("a queued whole read reuses one compatible local seal through the shared reader",async()=>{
 using oracle=new Database(":memory:");
 const oracleResult=oracle.query(`SELECT $sealed AND $compatible AS nominated,
  $sealed AND $compatible AND NOT $withdrawn AND NOT $stale AND NOT $missingClosure AS served`);
 const area=mkdtempSync(join(tmpdir(),"whole-local-wait-"));
 try{
  const files=createNodeFilePortV1();
  for(const row of localWaitFixture.cases){
   const root=join(area,row.name),target=createMockHost(root),producer=createMockHost(root,{stamp:!row.stale});
   await seed(root,ringFiles(3));
   const generationRoot=join(root,".gen"),pointerPath=join(generationRoot,GENERATION_POLICY_V1.pointer);
   const admitted=target.host.withBuilder,storeOptions=target.host.storeOptions,reads={pointers:0,manifests:0},controller=new AbortController();
   if(!row.compatible)Object.assign(producer.host,{implementationRows:async()=>[{kind:"stamp",path:"producer",fact:"other implementation"}]});
   let closureAvailable=false;
   const expand=target.host.expand;
   if("missingClosure" in row)Object.assign(target.host,{expand:async(...args:Parameters<typeof expand>)=>{
    if(closureAvailable)return expand(...args);
    throw Object.assign(Error("Fixture closure is unavailable"),{code:"closure-missing",missingClosure:"f".repeat(64),missingProjects:["p2"]});
   }});
   let clock=0,entered=0;
   Object.assign(target.host,{
    now:()=>clock,waitNoticeMilliseconds:localWaitFixture.interval,signal:controller.signal,
    storeOptions:(store:{root:string})=>({...storeOptions(store),fs:{...files,read:async(path:string)=>{
     if(path===pointerPath)reads.pointers++;
     if(dirname(path)===join(generationRoot,GENERATION_POLICY_V1.manifests))reads.manifests++;
     return files.read(path);
    }}}),
    async withBuilder(body:Parameters<typeof admitted>[0],request:unknown,_peer:unknown,_purpose:unknown,wait?:{available():Promise<boolean>;restore():Promise<any>}){
     entered++;
     await loadGenerationGraphV1(producer.host);
     const engine=createGenerationStoreV1({...storeOptions({root:generationRoot}),fs:files});
     if(!row.sealed)await engine.invalidate({reason:"closure-missing",expected:(await engine.read())!.pointer});
     if(row.stale)await applyOps(root,[{op:"write",path:"pkg/p2/src/index.ts",content:"export const v = 99;\n"}]);
     const expected=oracleResult.get({$sealed:+row.sealed,$compatible:+row.compatible,$withdrawn:+row.withdrawn,$stale:+row.stale,$missingClosure:+("missingClosure" in row)}) as {nominated:number;served:number};
     const nominated=await wait?.available()??false;
     expect(Number(nominated),row.name).toBe(expected.nominated);
     const first={...reads};
     for(const at of localWaitFixture.busyPolls){clock=at;await wait?.available();}
     expect(reads,row.name).toEqual(first);
     if(row.withdrawn)await engine.invalidate({reason:"closure-missing",expected:(await engine.read())!.pointer});
     const resumed=nominated?await wait!.restore():undefined;
     expect(Number(resumed?.graph!==undefined),row.name).toBe(expected.served);
     clock=localWaitFixture.interval;
     const before={...reads};
     expect(await wait?.available(),row.name).toBe(false);
     expect(reads,row.name).toEqual({...before,pointers:before.pointers+1});
     if(resumed)return resumed;
     if("recover" in row){
      const unchanged=await files.read(pointerPath),eligibleAt=Number(oracle.query("SELECT max(value)+2*$interval FROM json_each($polls)").values({$interval:localWaitFixture.interval,$polls:JSON.stringify(localWaitFixture.busyPolls)})[0]![0]);
      closureAvailable=true;
      clock=eligibleAt+1;
      expect(await wait?.available(),row.name).toBe(true);
      expect(await files.read(pointerPath),row.name).toEqual(unchanged);
      const recovered=await wait!.restore();
      expect(recovered?.graph,row.name).toBeDefined();
      return recovered;
     }
     controller.abort(Error("private capacity waiter cancelled"));
     throw controller.signal.reason;
    },
   });
   const result=await loadGenerationGraphV1(target.host).catch(error=>{if(error!==controller.signal.reason)throw error;return undefined;});
   expect(entered,row.name).toBe(1);
   expect(target.log,row.name).toEqual([]);
   if(result)expect(projectNames(result.graph),row.name).toEqual(["p0","p1","p2"]);
  }
 }finally{rmSync(area,{recursive:true,force:true});}
});
