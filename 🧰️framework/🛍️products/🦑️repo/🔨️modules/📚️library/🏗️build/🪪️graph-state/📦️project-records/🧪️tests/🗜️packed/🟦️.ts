import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createProjectFleetPortV1,projectRecordStateV1,type ProjectRecordInfoV1}from"../../🟦️.ts";
import{bundleKeysV1,hostOfRootV1,localBundleV1,neutralBundleV1,neutralRecordV1,packBundleV1,packRecordV1,unpackBundleV1,unpackRecordV1,type ClosureBundleV1}from"../../🧳️portable/🟦️.ts";
import{graphFleetClientV1,type GraphFleetClientV1}from"../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../🧶️nx/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{bindClosureStoreV1,closureRecipeKeyV1,closureTravelV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import{createClosureStoreV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs";
import{PROVIDER_CLOSURE_POLICY_V1,providerReferenceV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🟨️.mjs";

/** 🗜️ Laws of the packed record form: an owner's record grows with the package and not with the closure of its tools, a closure entry is stored once for every owner that names it, a taker downloads only the documents it lacks, and the facts derived from the packed form equal those of the expanded form. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`packed-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const view=Object.keys(PROVIDER_CLOSURE_POLICY_V1.views)[0]!;
const recipeOf=(at:number)=>({fn:"script-provider",port:`link-${at}`,view});

const populate=async(root:string,depth:number,layers:number)=>{
 mkdirSync(join(root,"ce"),{recursive:true});
 const store=createClosureStoreV1({directory:join(root,"ce")}),stamp=(name:string)=>({sha256:sha(name),size:20,mtimeMs:1,ctimeMs:1,ino:1});
 const base=Array.from({length:150},(_,at)=>({kind:"lstat",path:`${root}/d${at}`,fact:"directory"}));
 for(let at=0;at<depth;at++){
  writeFileSync(join(root,`m${at}.ts`),`export const m${at}=1;\n`);
  const own={kind:"file",path:`${root}/m${at}.ts`,fact:stamp(`m${at}`)},value=[`{workspaceRoot}/m${at}.ts`,...at+1<depth?[providerReferenceV1("providers",`link-${at+1}`,view)]:[]];
  const lists=Array.from({length:layers},(_,layer)=>[...base,own,{kind:"exists",path:`${root}/probe${at}-${layer}/package.json`,fact:false}]);
  await store.save(`${root}\0${closureRecipeKeyV1(recipeOf(at))}`,{recipe:recipeOf(at),value,rows:[own,...base],layers:lists,sidecar:{facts:lists.map(()=>null),members:lists.map(()=>null)}});
 }
 bindClosureStoreV1(root,join(root,"ce"),true);
};
const imageOf=(at:number)=>JSON.stringify({name:"owner",closureRecipes:{[closureRecipeKeyV1(recipeOf(at))]:recipeOf(at)}});
const infoOf=(root:string,name:string,files:number,taken:unknown[]):ProjectRecordInfoV1=>({cachePath:join(root,"np",name+".json"),fleet:"libs/"+name,keyText:JSON.stringify(["rev",null,[["authority"],root+"/libs/"+name,{name}]]),
 exportRecord:()=>({digest:sha(name),authority:[],observations:Array.from({length:files},(_,at)=>({kind:"file",path:`${root}/libs/${name}/src/f${at}.ts`,fact:{sha256:sha(name+at),size:at,mtimeMs:1,ctimeMs:1,ino:1}})),image:imageOf(0)}),
 async import(portable){taken.push(portable);return true;},closures:closureTravelV1});

async function world(){
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 return{server,environment};
}
const clientOf=(root:string,environment:Record<string,string>,fetched?:string[]):GraphFleetClientV1=>{
 const client=graphFleetClientV1(graphFleetSettingsV1(root,environment),{fetch:remoteCacheFetchV1(environment,{root})});
 return fetched===undefined?client:{...client,settings:client.settings,usable:()=>client.usable(),claimsUsable:()=>client.claimsUsable(),getResult:key=>client.getResult(key),getBlob:async(hash,size)=>{fetched.push(hash);return client.getBlob(hash,size);},hasBlob:hash=>client.hasBlob(hash),putBlob:(hash,bytes)=>client.putBlob(hash,bytes),putResult:(key,bytes,slot,expect)=>client.putResult(key,bytes,slot,expect),claim:(...args)=>client.claim(...args),renew:(...args)=>client.renew(...args),release:(...args)=>client.release(...args),await:(...args)=>client.await(...args)};
};
const documentsOf=async(root:string,environment:Record<string,string>,name:string,depth:number,files:number)=>{
 const port=createProjectFleetPortV1({root,environment,owner:name+":1"})!,gate=await port.gate(infoOf(root,name,files,[]));
 expect(gate.seeded).toBe(false);gate.publish();await port.flush();expect(port.stats().published).toBe(1);
 return port;
};
const noteOf=async(root:string,environment:Record<string,string>,name:string)=>{
 const {decideGraphEntryV1}=await import("../../../🛰️exchange/🟦️.ts"),client=clientOf(root,environment),state=projectRecordStateV1({root,fleet:"libs/"+name,keyText:JSON.stringify(["rev",null,[["authority"],root+"/libs/"+name,{name}]]),platform:(await import("../../../🟦️.ts")).graphPlatformV1()});
 const decision=await decideGraphEntryV1({client,state,owner:"reader:1"});
 if(decision.outcome!=="hit")throw new Error("no entry for "+name);
 return decision;
};

test("law: the record document grows with the package, the closure entries with their own distinct rows, and both stay far below the expanded size",async()=>{
 const {server,environment}=await world();
 try{
  const sizes:{depth:number;record:number;closures:number;expanded:number}[]=[];
  for(const depth of[2,10]){
   const root=temp("size"+depth);await populate(root,depth,60);
   const port=createProjectFleetPortV1({root,environment,owner:"size:"+depth})!,name="size"+depth,gate=await port.gate(infoOf(root,name,25,[]));gate.publish();await port.flush();
   const decision=await noteOf(root,environment,name),record=decision.documents.find(row=>row.name==="record")!,closures=decision.documents.filter(row=>row.name.startsWith("closure-"));
   const expanded=(await closureTravelV1.export(root,imageOf(0))).reduce((sum,entry)=>{const neutral=neutralBundleV1({recipeKey:entry.recipeKey,bundle:entry.bundle as ClosureBundleV1},hostOfRootV1(root),new Map())!;return sum+JSON.stringify(neutral.bundle).length;},0);
   expect(closures.length).toBe(depth);
   sizes.push({depth,record:record.bytes.length,closures:closures.reduce((sum,row)=>sum+row.bytes.length,0),expanded});
  }
  const[small,large]=sizes as [typeof sizes[0],typeof sizes[0]];
  expect(Math.abs(large.record-small.record)/small.record,"the record does not grow with the closure depth").toBeLessThan(0.05);
  expect(large.record,"the record of 25 files stays in the tens of kilobytes").toBeLessThan(60*1024);
  expect(large.closures/small.closures,"closure documents grow with the number of entries (5x), not faster").toBeLessThan(5.5);
  expect(large.closures,`packed ${large.closures} of expanded ${large.expanded}`).toBeLessThan(large.expanded/15);
 }finally{await server.stop();}
},180000);

test("law: two owners that name the same closure chain store its documents once, and the second owner adds only its own record",async()=>{
 const {server,environment}=await world();
 try{
  const rootA=temp("sa"),rootB=temp("sb");await populate(rootA,6,40);await populate(rootB,6,40);
  const a=await documentsOf(rootA,environment,"alpha",6,10),b=createProjectFleetPortV1({root:rootB,environment,owner:"beta:1"})!;
  const gateB=await b.gate(infoOf(rootB,"beta",12,[]));gateB.publish();await b.flush();
  const first=await noteOf(rootA,environment,"alpha"),second=await noteOf(rootA,environment,"beta");
  const hashes=(decision:typeof first,filter:(name:string)=>boolean)=>decision.note.documents.filter(row=>filter(row.name)).map(row=>row.hash).sort();
  expect(a.stats().published).toBe(1);
  const closureFirst=hashes(first,name=>name.startsWith("closure-")),closureSecond=hashes(second,name=>name.startsWith("closure-"));
  expect(closureFirst.length).toBe(6);
  expect(closureSecond,"the closure documents are the same blobs").toEqual(closureFirst);
  const recordFirst=hashes(first,name=>name==="record.br"),recordSecond=hashes(second,name=>name==="record.br");
  expect(recordFirst).not.toEqual(recordSecond);
  expect(new Set([...first.note.documents,...second.note.documents].map(row=>row.hash)).size,"distinct blobs: the chain once and two records").toBe(6+2);
 }finally{await server.stop();}
},180000);

test("a taker downloads only the documents it lacks: a second record that names the same closure entries costs its own record alone, and a lost entry is downloaded again",async()=>{
 const {server,environment}=await world();
 try{
  const rootA=temp("ta"),rootB=temp("tb");await populate(rootA,5,30);mkdirSync(join(rootB,"ce"),{recursive:true});bindClosureStoreV1(rootB,join(rootB,"ce"),true);
  const a=createProjectFleetPortV1({root:rootA,environment,owner:"a:9"})!;
  for(const name of["one","two"]){const gate=await a.gate(infoOf(rootA,name,8,[]));gate.publish();}
  await a.flush();expect(a.stats().published).toBe(2);
  const fetched:string[]=[],b=createProjectFleetPortV1({root:rootB,environment,owner:"b:9",client:clientOf(rootB,environment,fetched)})!;
  const takenOne:unknown[]=[],gateOne=await b.gate(infoOf(rootB,"one",8,takenOne));
  expect([gateOne.seeded,takenOne.length,b.stats().closures]).toEqual([true,1,5]);
  expect(fetched.length,"record and five closure documents").toBe(6);
  fetched.length=0;
  const takenTwo:unknown[]=[],gateTwo=await b.gate(infoOf(rootB,"two",8,takenTwo));
  expect([gateTwo.seeded,takenTwo.length]).toEqual([true,1]);
  expect(fetched.length,"only the record of the second owner").toBe(1);
  const store=createClosureStoreV1({directory:join(rootB,"ce")}),key=`${rootB}\0${closureRecipeKeyV1(recipeOf(2))}`,entry=await store.bundle(key);
  expect(entry?.layers.length).toBe(30);expect(JSON.stringify(entry)).not.toContain(rootA);
  rmSync(join(rootB,"ce","e"),{recursive:true,force:true});
  fetched.length=0;
  const takenAgain:unknown[]=[],again=await b.gate(infoOf(rootB,"two",8,takenAgain));
  expect(again.seeded).toBe(true);expect(fetched.length,"the entries are gone, so they are downloaded again").toBe(6);
  expect(await store.bundle(key)).toEqual(entry);
 }finally{await server.stop();}
},180000);

test("differential: the packed forms give back exactly the rows, layers and values of the expanded forms, in the neutral and in the local spelling",async()=>{
 const rootA=temp("da"),rootB=temp("db");await populate(rootA,4,25);
 const hostA=hostOfRootV1(rootA),hostB=hostOfRootV1(rootB),exported=await closureTravelV1.export(rootA,imageOf(0)),entries=exported.map(entry=>({recipeKey:entry.recipeKey,bundle:entry.bundle as ClosureBundleV1})),keys=bundleKeysV1(entries,hostA,"neutral");
 for(const entry of entries){
  const expanded=neutralBundleV1(entry,hostA,keys)!,packed=packBundleV1(entry,hostA,keys)!;
  expect(packed.recipeKey).toBe(expanded.recipeKey);
  const through=unpackBundleV1(JSON.parse(JSON.stringify(packed.document)));
  expect(JSON.parse(JSON.stringify({...through,layers:through.layers})),"neutral").toEqual(JSON.parse(JSON.stringify(expanded.bundle)));
  const back=localBundleV1({recipeKey:expanded.recipeKey,bundle:through},hostB,new Map([...keys,...bundleKeysV1([expanded],hostB,"local")]),{}),direct=localBundleV1(expanded,hostB,new Map([...keys,...bundleKeysV1([expanded],hostB,"local")]),{});
  expect(JSON.parse(JSON.stringify(back)),"local").toEqual(JSON.parse(JSON.stringify(direct)));
  expect(JSON.stringify(back)).not.toContain(rootA);
 }
 const portable={digest:sha("r"),authority:[{kind:"lstat",path:rootA,fact:"directory"}],observations:Array.from({length:30},(_,at)=>({kind:"file",path:`${rootA}/f${at%7}.ts`,fact:{sha256:sha(String(at%7)),size:at%7,mtimeMs:1,ctimeMs:1,ino:1}})),image:imageOf(0)};
 const neutral=neutralRecordV1(portable,hostA).record;
 expect(JSON.parse(JSON.stringify(unpackRecordV1(JSON.parse(JSON.stringify(packRecordV1(neutral))))))).toEqual(JSON.parse(JSON.stringify(neutral)));
 expect(JSON.stringify(packRecordV1(neutral)).length,"repeated rows are written once").toBeLessThan(JSON.stringify(neutral).length);
});

test("a record in the old form is never found: the state a record is stored under names the form",()=>{
 const input={root:"/r",fleet:"libs/a",keyText:"k",platform:"neutral"};
 expect(projectRecordStateV1(input).digest).toMatch(/^[0-9a-f]{64}$/u);
 expect(projectRecordStateV1(input).digest).not.toBe(createHash("sha256").update(JSON.stringify(["project-inference","k","neutral","libs/a"])).digest("hex"));
});

test("law: a document above the declared limit is refused before any compression, naming the document in both languages",async()=>{
 const {graphDocumentProblemV1}=await import("../../../🛰️exchange/🟦️.ts"),{GRAPH_STATE_POLICY_V1}=await import("../../../🟦️.ts");
 expect(graphDocumentProblemV1([{name:"record",bytes:new Uint8Array(1024)}])).toBeUndefined();
 const limit=GRAPH_STATE_POLICY_V1.documentMaximumBytes,problem=graphDocumentProblemV1([{name:"closure-x",bytes:new Uint8Array(limit+1)}]);
 expect(problem).toContain("closure-x");expect(problem).toContain(" / das Dokument ");
 const many=Array.from({length:3},(_,at)=>({name:"d"+at,bytes:new Uint8Array(Math.floor(GRAPH_STATE_POLICY_V1.entryMaximumBytes/2))}));
 expect(graphDocumentProblemV1(many)).toContain("der Eintrag");
 expect(limit).toBeLessThanOrEqual(2**28);
});
