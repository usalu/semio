/** 🏭️ Laws of the closure walk farm: the farm answers exactly what the in-process deriver answers (differential over 240 closures, several children used), every message on the wire conforms to the schema, a child that dies, fails or is cancelled never fails a read (the settler walks the question itself and the result equals the run without a farm), no child outlives its farm, and the size follows granted processors and memory. */
import{expect,test}from"bun:test";
import{spawn}from"node:child_process";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{compileJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{CLOSURE_SETTLE_V1,createClosureSettlerV1}from"../../🟨️.mjs";
import{PLAN_STAGE_POLICY_V1}from"../../../../../🧠️memory/🪜️plan-stage/🟨️.mjs";
import{WALK_FARM_V1,createWalkFarmV1,offloadClosureRecipeV1,partIdOfV1,walkFarmCapV1,walkFarmSizeV1}from"../🟨️.mjs";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{createNodeStoreV1}from"../../../💾️store/🟨️.mjs";
import{openDeriverV1}from"./🧫️child/🟨️.mjs";

const module=join(dirname(fileURLToPath(import.meta.url)),"🧫️child/🟨️.mjs");
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}};
const until=async(check:()=>boolean,milliseconds=15000)=>{const end=Date.now()+milliseconds;while(Date.now()<end&&!check())await new Promise(resolve=>setTimeout(resolve,25));return check();};

async function settleWith(names:string[],farm:object|false){
 const store=mkdtempSync(join(tmpdir(),"farm-store-")),derive=await openDeriverV1(),derived:string[]=[];
 try{
  const settler=createClosureSettlerV1({store:createNodeStoreV1({root:store}),limits:{...CLOSURE_SETTLE_V1,deriveConcurrency:2,settleChunkClosures:20},farm:farm as never});
  const settled=await settler.settle({pseudoRows:new Map(names.map(name=>[name,{kind:"derived",path:name,fact:"old"}])),derive:async(row:{path:string;kind:string;fact:unknown})=>{derived.push(row.path);return derive(row);},hold:async()=>true} as never);
  const answers=new Map<string,string>();
  for(const[name,rows]of settled.held)answers.set(name,"held:"+JSON.stringify([...rows].slice(1)));
  return{settled,derived,changed:[...settled.changed.keys()].sort(),failed:settled.failed,answers};
 }finally{rmSync(store,{recursive:true,force:true});}
}

test("the farm size follows the granted processors (one stays with the parent) and the granted memory after the reserve, and below two there is none",()=>{
 const gb=1073741824;
 expect(walkFarmSizeV1({cpus:4,availableBytes:20*gb})).toBe(3);
 expect(walkFarmSizeV1({cpus:64,availableBytes:512*gb})).toBe(WALK_FARM_V1.maximumChildren);
 expect(walkFarmSizeV1({cpus:5,availableBytes:10*gb,reserveBytes:2*gb})).toBe(4);expect(walkFarmSizeV1({cpus:16,availableBytes:6*gb,reserveBytes:2*gb}),"five children of 768 MiB fit in the four gigabytes left").toBe(5);expect(walkFarmSizeV1({cpus:10,availableBytes:11*gb,reserveBytes:2.8*gb}),"the grant of the live read: the processors bound it").toBe(9>8?8:9);
 expect(walkFarmSizeV1({cpus:2,availableBytes:20*gb})).toBe(0);
 expect(walkFarmSizeV1({cpus:16,availableBytes:2*gb,reserveBytes:gb})).toBe(0);
});

test("the farm obeys the memory plan of the process like the project farm: tree share of the host, minus what the parent holds, the cut of the load, and none on a serving machine",()=>{
 const base={environment:{SEMIO_SLICE_LEAN:"off"} as Record<string,string|undefined>,hostMegabytes:16384,residentMegabytes:3000};
 expect(walkFarmCapV1(base),"16 GB worker: the plan limit share of the host for the whole tree, 3000 MB held, 768 MiB a child").toBe(Math.floor((16384*0.4-Math.max(3000,PLAN_STAGE_POLICY_V1.parentMegabytes))/(WALK_FARM_V1.childBytes/1048576)));
 expect(walkFarmCapV1({...base,hostMegabytes:8192,residentMegabytes:3000}),"8 GB worker: the parent alone holds the share, no child").toBe(0);
 expect(walkFarmCapV1({...base,environment:{SEMIO_SLICE_LEAN:"off",SEMIO_GRAPH_CUT_CONCURRENCY:"2"}}),"the cut of a load that grew too big caps the farm").toBe(2);
 expect(walkFarmCapV1({...base,environment:{},residentMegabytes:1000}),"lean: the parent is planned at 1500 MB, so 16 GB leaves room for more children").toBe(Math.floor((16384*0.4-1500)/(WALK_FARM_V1.childBytes/1048576)));
 expect(walkFarmCapV1({...base,hostMegabytes:32768})).toBe(Infinity);
 expect(walkFarmCapV1({...base,environment:{SEMIO_GRAPH_PLAN_LIMIT_MB:"off"}})).toBe(Infinity);
});

test("a settle whose load was cut to one project at a time starts no walk child, however many processors and bytes the host grants",async()=>{
 const names=Array.from({length:120},(_,at)=>"q"+String(at).padStart(3,"0")),before=process.env.SEMIO_GRAPH_CUT_CONCURRENCY;
 process.env.SEMIO_GRAPH_CUT_CONCURRENCY="1";
 try{const cut=await settleWith(names,{root:"/farm",module,afterDerivations:4});expect(cut.settled.farmed).toBe(0);expect(cut.failed).toBe(0);expect(cut.derived.length).toBe(120);}
 finally{if(before===undefined)delete process.env.SEMIO_GRAPH_CUT_CONCURRENCY;else process.env.SEMIO_GRAPH_CUT_CONCURRENCY=before;}
});

test("a settle with the farm answers exactly what a settle without it answers, and the questions were walked by several children",async()=>{
 const names=Array.from({length:240},(_,at)=>"q"+String(at).padStart(3,"0")),plain=await settleWith(names,false),farmed=await settleWith(names,{root:"/farm",module,size:3,afterDerivations:4});
 expect(plain.derived.length).toBe(240);
 expect(farmed.derived.length,"after the first few the parent walks nothing itself, the children do").toBeLessThan(240);
 expect(farmed.derived.length).toBeGreaterThanOrEqual(4);
 expect(farmed.settled.farmed,"questions answered by children").toBeGreaterThan(100);
 expect(farmed.changed).toEqual(plain.changed);expect(farmed.failed).toBe(plain.failed);expect(farmed.changed.length).toBe(240);
});

test("every message on the wire conforms to the schema and the farm uses its children one question at a time",async()=>{
 const validate=(name:string)=>compileJsonSchemaSubset({$ref:"#/definitions/"+name,definitions:schema.definitions} as never);
 const ready=validate("Ready"),request=validate("Request"),answer=validate("Answer"),failure=validate("Failure"),seen={ready:0,request:0,answer:0,failure:0};
 const tap=((command:string,args:string[],options:object)=>{
  const child=spawn(command,args,options as never),write=child.stdin!.write.bind(child.stdin);
  (child.stdin as any).write=(text:string)=>{for(const line of String(text).split("\n").filter(Boolean)){expect(line.startsWith(WALK_FARM_V1.marker)).toBe(true);expect(request.validate(JSON.parse(line.slice(WALK_FARM_V1.marker.length))),line).toEqual([]);seen.request++;}return write(text);};
  child.stdout!.on("data",chunk=>{for(const line of String(chunk).split("\n").filter(Boolean)){const at=line.lastIndexOf(WALK_FARM_V1.marker);if(at<0)continue;const message=JSON.parse(line.slice(at+WALK_FARM_V1.marker.length)),kind=message.ready?ready:message.answer?answer:failure;expect(kind.validate(message),line).toEqual([]);seen[message.ready?"ready":message.answer?"answer":"failure"]++;}});
  return child;
 }) as never;
 const farm=createWalkFarmV1({root:"/farm",module,size:2,spawn:tap});
 try{
  farm.warm();expect(await until(()=>farm.ready())).toBe(true);
  const rows=Array.from({length:30},(_,at)=>({kind:"derived",path:"w"+at,fact:"old"})),answers=await Promise.all(rows.map(row=>farm.derive(row)));
  const direct=await openDeriverV1();
  const seenRows=new Map<string,unknown>();
  for(const[at,row]of rows.entries()){
   const want=await direct(row),got=answers[at] as {fact:unknown;parts:{id:string;rows?:unknown[]}[]};
   expect(got.fact).toEqual(want.fact);expect(got.parts.map(part=>part.id).sort()).toEqual(want.parts.map((rows:unknown[])=>partIdOfV1(rows as never)).sort());
   for(const part of got.parts)if(part.rows!==undefined)seenRows.set(part.id,part.rows);
   for(const rows of want.parts)expect(seenRows.get(partIdOfV1(rows)),"a part comes with its rows the first time and by id after").toEqual(JSON.parse(JSON.stringify(rows)));
  }
  expect(answers.filter(answer=>(answer as {parts:{rows?:unknown}[]}).parts.some(part=>part.rows===undefined)).length,"the shared part travels once per child").toBeGreaterThanOrEqual(rows.length-2);
  await expect(farm.derive({kind:"derived",path:"fail",fact:"old"})).rejects.toMatchObject({code:"WALK_FARM_QUESTION_FAILED",childCode:"E_FAIL"});
  expect(seen.ready).toBeGreaterThanOrEqual(1);expect(seen.request).toBe(31);expect(seen.answer).toBe(30);expect(seen.failure).toBe(1);
 }finally{farm.close();}
});

test("a child that dies on a question, a question that fails and a missing farm leave the settle correct: the parent walks them itself",async()=>{
 const names=[...Array.from({length:100},(_,at)=>"q"+String(at).padStart(3,"0")),"crash","fail","q100","q101"],plain=await settleWith(names,false),farmed=await settleWith(names,{root:"/farm",module,size:3,afterDerivations:4});
 expect(farmed.changed).toEqual(plain.changed);
 expect(farmed.failed,"the failing question fails in the parent as it does without a farm").toBe(plain.failed);
 const broken=await settleWith(names.filter(name=>name!=="crash"),{root:"/farm",module:"/nonexistent/child.mjs",size:3,afterDerivations:4});
 expect(broken.failed).toBe(1);expect(broken.changed.length,"every closure is answered, the failing one as changed").toBe(names.length-1);
});

test("a cancelled question ends its child, and closing the farm ends every child",async()=>{
 const farm=createWalkFarmV1({root:"/farm",module,size:2});
 try{
  farm.warm();expect(await until(()=>farm.ready())).toBe(true);
  const stop=new AbortController(),pending=farm.derive({kind:"derived",path:"slow",fact:"old"},stop.signal);
  await new Promise(resolve=>setTimeout(resolve,200));stop.abort(new Error("stop"));
  await expect(pending).rejects.toThrow("stop");
  const warm=await farm.derive({kind:"derived",path:"after",fact:"old"});expect(warm.fact.length).toBe(12);
  const pids=Object.keys(farm.stats().byPid).map(Number);expect(pids.length).toBeGreaterThan(0);
  farm.close();
  expect(await until(()=>pids.every(pid=>!alive(pid)))).toBe(true);
  await expect(farm.derive({kind:"derived",path:"late",fact:"old"})).rejects.toMatchObject({code:"WALK_FARM_CLOSED"});
 }finally{farm.close();}
},60000);

test("a closure of two thousand shared rows costs the wire its rows once per child: the bytes per answer stay small and the parent keeps up",async()=>{
 const farm=createWalkFarmV1({root:"/farm",module,size:2});
 try{
  farm.warm();expect(await until(()=>farm.ready())).toBe(true);
  const began=Date.now(),answers=await Promise.all(Array.from({length:120},(_,at)=>farm.derive({kind:"derived",path:"b"+at,fact:"old"})));
  const stats=farm.stats();
  expect(answers.length).toBe(120);expect(stats.bytes/stats.answered,"bytes per answer: unshared it would be about 100 000").toBeLessThan(8000);
  expect(Object.keys(stats.byPid).length).toBe(2);
  expect(Date.now()-began,"120 closures of 2 000 rows").toBeLessThan(20000);
 }finally{farm.close();}
},60000);

test("a wave of shared closure entries is walked by the children: after the first few the parent walks none and its event loop is never blocked by one",async()=>{
 const recipes=Array.from({length:40},(_,at)=>({fn:"fixture",name:"r"+at,burn:60})),gaps:number[]=[];
 let last=Date.now();const timer=setInterval(()=>{const now=Date.now();gaps.push(now-last);last=now;},5);
 let parentWalks=0;
 try{
  const walk=async(recipe:{name:string;burn:number})=>{const farmed=await offloadClosureRecipeV1("/farm",recipe,{module,size:3,after:3});if(farmed!==undefined)return farmed;parentWalks++;return(await (await openDeriverV1("x") as any).recipe(recipe));};
  for(const recipe of recipes.slice(0,5))await walk(recipe);
  last=Date.now();gaps.length=0;parentWalks=0;
  const results=await Promise.all(recipes.map(recipe=>walk(recipe)));
  expect(results.map(result=>result.value.recipe.name)).toEqual(recipes.map(recipe=>recipe.name));
  expect(results.every(result=>result.sidecar?.members[0][0]==="question"&&result.value.walked===true&&result.rows.length===1&&result.layers.length===1&&result.layers[0].length===1)).toBe(true);
  expect(parentWalks,"derivations on the thread of the parent once the farm is warm").toBe(0);
  const blocked=gaps.filter(gap=>gap>40).reduce((sum,gap)=>sum+gap,0);
  expect(Math.max(...gaps),"longest block of the parent").toBeLessThan(120);
  expect(blocked/recipes.length,"blocked milliseconds of the parent per entry").toBeLessThan(10);
 }finally{clearInterval(timer);}
},60000);

test("the request timer of a walk child is the authored time times the speed factor of the machine: a child that never answers is killed at it",async()=>{
 const silent=()=>{
  const listeners=new Map<string,((...rest:unknown[])=>void)[]>(),on=(name:string,run:(...rest:unknown[])=>void)=>{listeners.set(name,[...(listeners.get(name)??[]),run]);};
  const stdout={setEncoding:()=>undefined,on},child={pid:99999,stdin:{on:()=>undefined,write:()=>true},stdout,unref:()=>undefined,kill:()=>undefined,on,once:on};
  queueMicrotask(()=>{for(const run of stdout===undefined?[]:(listeners.get("data")??[]))run(WALK_FARM_V1.marker+JSON.stringify({ready:true})+"\n");});
  return child;
 };
 const timed=async(speed:number)=>{
  const farm=createWalkFarmV1({root:"/farm",module,size:1,spawn:silent as never,policy:{...WALK_FARM_V1,requestTimeoutMilliseconds:300},speed});
  try{farm.warm();expect(await until(()=>farm.ready(),2000)).toBe(true);const started=Date.now(),error=await farm.derive({kind:"derived",path:"w",fact:"old"}).then(()=>undefined,failure=>failure as {code:string});return{code:error?.code,milliseconds:Date.now()-started};}finally{farm.close();}
 };
 const plain=await timed(1),slow=await timed(2.5);
 expect(plain.code).toBe("WALK_FARM_REQUEST_TIMEOUT");expect(plain.milliseconds).toBeGreaterThanOrEqual(250);expect(plain.milliseconds).toBeLessThan(700);
 expect(slow.code).toBe("WALK_FARM_REQUEST_TIMEOUT");expect(slow.milliseconds).toBeGreaterThanOrEqual(700);
});
