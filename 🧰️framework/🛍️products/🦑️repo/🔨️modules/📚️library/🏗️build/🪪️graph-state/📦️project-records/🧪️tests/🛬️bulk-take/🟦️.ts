import{afterAll,expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{lstatSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createProjectFleetPortV1,runTakeLookV1,type ProjectClosureTravelV1,type ProjectRecordInfoV1}from"../../🟦️.ts";
import{graphFleetClientV1}from"../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../🧶️nx/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{observationLayerHoldsV1,observationRowsHoldV1}from"../../../../👁️observation/🟨️.mjs";
import{bindClosureStoreV1,closureRecipeKeyV1,closureTravelV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import{createClosureStoreV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs";
import{stretchedBudgetMillisecondsV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";

/** 🛬️ Law: a machine that takes the records of a selection from the fleet asks the cache a constant number of times per owner (one lookup and one record), fetches the closure document that a group of owners shares once and verifies it once, and takes the owners in a time that does not grow with the number of closures. 200 owners in 3 closure groups, one cache server, one reader taking 24 owners at once.
 * @see ../🚚️closures/🟦️.ts */
const OWNERS=200,GROUPS=3,WIDTH=24,scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`bulk-take-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(bytes:Buffer|string)=>createHash("sha256").update(bytes).digest("hex");
const recipeOf=(group:number)=>({fn:"native-command",group});
const rowOf=(root:string,name:string)=>{const path=`${root}/${name}`,status=lstatSync(path);return{kind:"file",path,fact:{sha256:sha(readFileSync(path)),size:status.size,mtimeMs:status.mtimeMs,ctimeMs:status.ctimeMs,ino:status.ino}};};
const tree=(root:string,names:readonly string[])=>{mkdirSync(join(root,"ce"),{recursive:true});for(const name of names){mkdirSync(join(root,"libs",name),{recursive:true});writeFileSync(join(root,"libs",name,"a.ts"),`export const ${name.replace(/\W/gu,"_")}=1;\n`);}for(let group=0;group<GROUPS;group++)writeFileSync(join(root,`e${group}.ts`),`export const e=${group};\n`);bindClosureStoreV1(root,join(root,"ce"),true);};
const walk=async(root:string)=>{const store=createClosureStoreV1({directory:join(root,"ce")});for(let group=0;group<GROUPS;group++){const row=rowOf(root,`e${group}.ts`),recipe=recipeOf(group);await store.save(`${root}\0${closureRecipeKeyV1(recipe)}`,{recipe,value:[`{workspaceRoot}/e${group}.ts`],rows:[row],layers:[[row]],sidecar:{facts:[null],members:[null]}});}};
const groupOf=(name:string)=>Number(name.slice(6))%GROUPS;
const infoOf=(root:string,name:string,travel:ProjectClosureTravelV1):ProjectRecordInfoV1=>({cachePath:join(root,"np",`${name}.json`),fleet:`libs/${name}`,keyText:JSON.stringify(["rev",null,[["authority"],`${root}/libs/${name}`,{name}]]),exportRecord:()=>({digest:sha(name),authority:[],observations:[rowOf(root,`libs/${name}/a.ts`)],image:JSON.stringify({name,closureRecipes:{[closureRecipeKeyV1(recipeOf(groupOf(name)))]:recipeOf(groupOf(name))}})}),async import(){return true;},closures:travel});

test("200 owners in 3 closure groups are taken from the fleet with two requests per owner and one fetch and one verification per closure, in a bounded time",async()=>{
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 try{
  const names=Array.from({length:OWNERS},(_all,at)=>`owner-${at}`),[producer,reader]=["producer","reader"].map(label=>{const root=temp(label);tree(root,names);return root;}) as [string,string];
  await walk(producer);
  const sender=createProjectFleetPortV1({root:producer,environment,owner:"producer:1"})!;
  for(const name of names){const claim=await sender.claim(infoOf(producer,name,closureTravelV1));expect(claim.state,name).toBe("granted");if(claim.state==="granted")expect(await claim.publish(),name).toBe(true);}
  await sender.flush();
  const requests={results:0,blobs:0},seen=new Map<string,number>();let imports=0;
  const counting=((input:RequestInfo|URL,init?:RequestInit)=>{const url=String(input);if((init?.method??"GET")==="GET"){if(url.includes("/ac/"))requests.results++;else if(url.includes("/cas/")){requests.blobs++;const hash=url.slice(url.lastIndexOf("/")+1);seen.set(hash,(seen.get(hash)??0)+1);}}return fetch(input,init);}) as typeof fetch;
  const settings=graphFleetSettingsV1(reader,environment)!,client=graphFleetClientV1({...settings,stateDirectory:temp("state")},{fetch:remoteCacheFetchV1(environment,{root:reader,base:counting})});
  const travel:ProjectClosureTravelV1={...closureTravelV1,import:async(...args)=>{imports++;return closureTravelV1.import(...args);}};
  const port=createProjectFleetPortV1({root:reader,environment,client,owner:"reader:1"})!,begun=performance.now();let next=0;const seeded:boolean[]=[];
  await Promise.all(Array.from({length:WIDTH},async()=>{while(next<names.length){const name=names[next++]!;const gate=await runTakeLookV1(()=>port.gate(infoOf(reader,name,travel)));seeded.push(gate.seeded);}}));
  const elapsed=performance.now()-begun;
  expect(seeded.length).toBe(OWNERS);expect(seeded.every(Boolean),"every record came from the fleet").toBe(true);
  expect(requests.results,"one lookup per owner").toBe(OWNERS);
  expect(seen.size,"every blob is fetched once").toBe(OWNERS+GROUPS);expect(requests.blobs,"one record per owner and each closure document once").toBe(OWNERS+GROUPS);
  expect(imports,"each closure is verified and stored once").toBe(GROUPS);
  const store=createClosureStoreV1({directory:join(reader,"ce")});
  for(let group=0;group<GROUPS;group++){const entry=await store.bundle(`${reader}\0${closureRecipeKeyV1(recipeOf(group))}`);expect(entry!==undefined&&observationRowsHoldV1(entry.rows)&&[...entry.layers].every(layer=>observationLayerHoldsV1(layer)),"closure "+group).toBe(true);}
  expect(elapsed,"200 owners in a bounded time").toBeLessThan(stretchedBudgetMillisecondsV1(20000));
 }finally{await server.stop();}
},stretchedBudgetMillisecondsV1(120000));
