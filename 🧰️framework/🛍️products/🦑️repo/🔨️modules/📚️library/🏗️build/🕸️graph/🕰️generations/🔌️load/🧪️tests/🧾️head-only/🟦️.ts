import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {mkdtempSync,rmSync} from "node:fs";
import {cp} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createNodeFilePortV1} from "../../../🟨️.mjs";
import {implementationDigestV1,loadGenerationGraphV1,restoreExpectedWholeGenerationV1} from "../../🟨️.mjs";
import {createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";
import fixture from "./🔣️.json";
import retryFixture from "./🕓️retry.json";

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
   expect(observed,row.name).toEqual(rule.get({$case:row.name}));
   expect(observed,row.name).toEqual({status:row.status,builders:row.builders,restores:row.restores,fences:row.fences});
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
    expect(availability).toEqual(expectedAvailability);
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
  expect(Object.keys(result.graph.projects)).toHaveLength(6);
 }finally{rmSync(area,{recursive:true,force:true});}
});
