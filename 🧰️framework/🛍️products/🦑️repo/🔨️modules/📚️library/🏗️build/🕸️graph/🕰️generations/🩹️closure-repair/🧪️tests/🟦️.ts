import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{join,relative}from"node:path";
import{link,mkdtemp,rm,utimes}from"node:fs/promises";
import{tmpdir}from"node:os";
import{createHash}from"node:crypto";
import Ajv from"ajv";
import{CLOSURE_POLICY_V1}from"../../../🩹️closure-repair/🟨️.mjs";
import{GENERATION_POLICY_V1,canonicalJsonV1,createGenerationStoreV1,createNodeFilePortV1,packGenerationDocumentV1}from"../../🟨️.mjs";
import{CLOSURE_REPAIR_POLICY_V1,prepareClosureRepairV1,readClosureRepairV1,recoverClosureRepairV1,verifyClosureRepairCandidateV1,type ClosureRepairVerifierV1}from"../🟨️.mjs";
import schema from"../🧬️schema/🔣️.json";
import cases from"./🔣️.json";

const hash=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex"),key=hash("source"),env=hash("environment"),identity=hash("identity"),implementation=hash("implementation"),root="repair-fixture",p=GENERATION_POLICY_V1;
const input={namespace:"build-input-closures",key} as const,lock={namespace:"build-lock-closures",key} as const;
const token=(kind:"SOURCE_CLOSURE"|"CARGO_LOCK",digest=key)=>"env:SEMIO_"+kind+"_"+digest;
const basic={identity,implementation,root};
function world({shared=true,global=false,corrupt=false,superseded=false,count=3,nameOf=(index:number)=>["a","b","c"][index]??"p"+index,inputsOf=(index:number):readonly string[]=>[]}={}){
 const data=new Map<string,Uint8Array>(),writes:string[]=[],pointerPath=join(root,p.pointer);let onRead:(path:string)=>void=()=>{};
 const set=(path:string,value:unknown)=>{data.set(path,Buffer.from(canonicalJsonV1(value)));};
 const pack=(body:unknown)=>{
  const bytes=packGenerationDocumentV1(body,p.maximumFragmentBytes),digest=hash(bytes);data.set(join(root,p.fragments,digest+".json.br"),bytes);return{fragment:digest,bytes:bytes.length};
 };
 const names=Array.from({length:count},(_,index)=>nameOf(index));
 const owners=Object.fromEntries(names.map((name,index)=>{
  const root="pkg/"+name,node={name,inputs:[...(shared&&index<2?[token("SOURCE_CLOSURE")]:[]),...index===1?[token("CARGO_LOCK")]:[],...inputsOf(index)]};
  return[name,{root,kind:"project" as const,edgesEnv:env,targets:[],...pack({version:1,name,root,kind:"project",node,edges:[],edgesEnv:env,targets:[]})}];
 }));
 const value={version:1 as const,generation:7,parent:null,identity,implementation,env,membership:hash("members"),frontier:hash("frontier"),factIndex:hash("facts"),global:pack({version:1,name:"@global",value:global?{token:token("SOURCE_CLOSURE")}:null}).fragment,owners,built:{kind:"delta" as const,base:6,dirty:[]}};
 const manifest=hash(canonicalJsonV1(value));set(join(root,p.manifests,manifest+".json"),value);
 const expected={version:1 as const,generation:7,manifest,parent:null},pointer={version:1 as const,generation:8,manifest:null,parent:manifest,reason:"closure-missing"};
 set(pointerPath,expected);
 if(corrupt)data.set(join(root,p.fragments,owners.c!.fragment+".json.br"),Buffer.from("tampered"));
 if(superseded)onRead=path=>{if(path===join(root,p.fragments,owners.c!.fragment+".json.br"))set(pointerPath,{...expected,generation:9});};
 const fs={async read(path:string){onRead(path);return data.get(path);},async write(path:string,bytes:Uint8Array,options?:{ifAbsent?:boolean}){
  if(options?.ifAbsent&&data.has(path))return false;
  writes.push(path);data.set(path,bytes);return true;
 }};
 return{fs,data,writes,value,expected,pointer,pointerPath,move:(value:unknown)=>set(pointerPath,value),whenRead:(fn:(path:string)=>void)=>{onRead=fn;}};
}

test("verified ownership is exact, typed and tied to the observed sealed pointer",async()=>{
 using sql=new Database(":memory:");
 const decision=sql.query(`SELECT ($shared AND NOT ($global OR $corrupt OR $superseded)) AS eligible`);
 for(const [name,shared,global,corrupt,superseded,owners] of cases as [string,boolean,boolean,boolean,boolean,string[]][]){
  const w=world({shared,global,corrupt,superseded}),prepared=await prepareClosureRepairV1({...basic,fs:w.fs,expected:w.expected,missing:[input]});
  expect({eligible:Number(prepared!==undefined)},name).toEqual(decision.get({$shared:Number(shared),$global:Number(global),$corrupt:Number(corrupt),$superseded:Number(superseded)}));
  expect(prepared?.owners??[],name).toEqual(owners);
  expect(w.writes.length,name).toBe(prepared?1:0);
  if(!prepared)continue;
  expect(await readClosureRepairV1({...basic,fs:w.fs,pointer:w.pointer})).toBeUndefined();
  const bytes=w.data.get(w.writes[0]!)!,receipt=JSON.parse(Buffer.from(bytes).toString("utf8"));
  expect(new Ajv({strict:true}).compile({...schema.definitions.Receipt,definitions:schema.definitions})(receipt)).toBe(true);
  expect(hash(bytes)).toBe(prepared.receipt);
  w.move(w.pointer);
  expect(await readClosureRepairV1({...basic,fs:w.fs,pointer:w.pointer})).toEqual(prepared);
  expect(await readClosureRepairV1({...basic,identity:hash("different owner registry"),fs:w.fs,pointer:w.pointer})).toBeUndefined();
  w.data.set(w.writes[0]!,Buffer.from(canonicalJsonV1({...receipt,owners:["a"]})));
  expect(await readClosureRepairV1({...basic,fs:w.fs,pointer:w.pointer})).toBeUndefined();
 }
});

test("an old quarantine uses a full read-only census and checks its pointer again",async()=>{
 const w=world(),calls:string[]=[];w.move(w.pointer);
 const verifyClosure=async(ref:typeof input|typeof lock)=>{calls.push(ref.namespace);return ref.namespace===lock.namespace;};
 expect(await readClosureRepairV1({...basic,fs:w.fs,pointer:w.pointer})).toBeUndefined();
 const recovered=await recoverClosureRepairV1({...basic,fs:w.fs,pointer:w.pointer,verifyClosure});
 expect(recovered?.owners).toEqual(["a","b"]);
 expect(recovered?.missing).toEqual([input]);
 expect(calls.sort()).toEqual([input.namespace,lock.namespace]);
 expect(w.writes).toEqual([]);
 expect(await recoverClosureRepairV1({...basic,fs:w.fs,pointer:w.pointer,verifyClosure:async()=>undefined})).toBeUndefined();
 expect(await recoverClosureRepairV1({...basic,fs:w.fs,pointer:w.pointer,verifyClosure:async()=>{w.move(w.expected);return false;}})).toBeUndefined();
 expect(w.writes).toEqual([]);
});

test("candidate verification includes global and both namespaces and cancellation precedes a receipt write",async()=>{
 const w=world({global:true}),called:string[]=[],controller=new AbortController();
 expect(await verifyClosureRepairCandidateV1({...basic,fs:w.fs,manifest:w.value,verifyClosure:async ref=>{called.push(ref.namespace);return ref.namespace===lock.namespace;}})).toBe(false);
 expect(await verifyClosureRepairCandidateV1({...basic,fs:w.fs,manifest:w.value,verifyClosure:async()=>true})).toBe(true);
 expect(called).toContain(input.namespace);
 const clean=world(),reason=Error("stop receipt");clean.whenRead(path=>{if(path===join(root,p.fragments,clean.value.owners.c!.fragment+".json.br"))controller.abort(reason);});
 await expect(prepareClosureRepairV1({...basic,fs:clean.fs,expected:clean.expected,missing:[input],signal:controller.signal})).rejects.toBe(reason);
 expect(clean.writes).toEqual([]);
 expect(CLOSURE_REPAIR_POLICY_V1.maximumFragmentBytes).toBeLessThanOrEqual(p.maximumFragmentBytes);
});

test("preparation verifies the observed keys and records all and only the missing references",async()=>{
 const w=world(),seen:string[]=[];
 const verifyClosure=async(ref:typeof input|typeof lock)=>{seen.push(ref.namespace);return ref.namespace===lock.namespace;};
 const prepared=await prepareClosureRepairV1({...basic,fs:w.fs,expected:w.expected,missing:[lock],verifyClosure});
 expect(seen.sort()).toEqual([input.namespace,lock.namespace]);
 expect(prepared?.owners).toEqual(["a","b"]);
 expect(prepared?.missing).toEqual([input]);
 const unqualified=world();
 expect(await prepareClosureRepairV1({...basic,fs:unqualified.fs,expected:unqualified.expected,missing:[{...input,key:hash("not named")}],verifyClosure:async()=>{throw Error("Must qualify the observation before probing");}})).toBeUndefined();
 expect(await prepareClosureRepairV1({...basic,fs:unqualified.fs,expected:unqualified.expected,missing:[input],verifyClosure:async()=>undefined})).toBeUndefined();
 expect(await prepareClosureRepairV1({...basic,fs:unqualified.fs,expected:unqualified.expected,missing:[input],verifyClosure:async()=>true})).toBeUndefined();
 expect(unqualified.writes).toEqual([]);
});

test("a bounded census qualifies 5,101 private fragments without dropping owners",async()=>{
 const w=world({count:5100});let scanned=0;
 using sql=new Database(":memory:");
 const expected=sql.query("SELECT ? + 1 AS scanned").get(5100);
 const prepared=await prepareClosureRepairV1({...basic,fs:w.fs,expected:w.expected,missing:[input],verifyClosure:async ref=>ref.namespace===lock.namespace,progress:done=>{scanned=done;}});
 expect({scanned}).toEqual(expected);
 expect(prepared?.owners).toEqual(["a","b"]);
 },20000);

test("generation retirement protects live and staged repair receipts, respects grace, and skips aliases",async()=>{
 const fs=createNodeFilePortV1(),proof=world(),age=GENERATION_POLICY_V1.graceMilliseconds;
 for(const mode of ["sealed","quarantined","pointer moved","cancelled"]){
  const dir=await mkdtemp(join(tmpdir(),"receipt-retirement-")),receipts=join(dir,CLOSURE_REPAIR_POLICY_V1.directory),controller=new AbortController();
  try{
   const pointer=mode==="quarantined"?proof.pointer:proof.expected;
   for(const [path,bytes] of proof.data)await fs.write(join(dir,relative(root,path)),bytes);
   await fs.write(join(dir,p.pointer),Buffer.from(canonicalJsonV1(pointer)));
   const current=8+"-"+proof.expected.manifest+".json",older=7+"-"+hash("older")+".json",wrong=8+"-"+hash("wrong")+".json",future=9+"-"+hash("future")+".json",fresh=6+"-"+hash("fresh")+".json",alias=5+"-"+hash("alias")+".json",temporary=older+"."+"a".repeat(16)+".tmp";
   const names=[[current,1,1],[older,1,1],[wrong,1,1],[future,1,1],[fresh,0,1],["other.json",1,0],[alias,1,0],[temporary,1,1]] as const;
   const now=Date.now(),old=new Date(now-age-5000),bytes=Buffer.from("{}");
   for(const [name,aged] of names){
    if(name!==alias)await fs.write(join(receipts,name),bytes);
    else await link(join(receipts,current),join(receipts,alias));
    if(aged)await utimes(join(receipts,name),old,old);
   }
   const reason=Error("cancel receipt sweep");
   const traced={...fs,async list(path:string){
    const entries=await fs.list(path);
    if(path===receipts){
     if(mode==="pointer moved")await fs.write(join(dir,p.pointer),Buffer.from(canonicalJsonV1({...pointer,generation:8,manifest:hash("future")})));
     if(mode==="cancelled")controller.abort(reason);
    }
    return entries;
   }};
   const store=createGenerationStoreV1({root:dir,fs:traced,holds:async()=>true,now:()=>now,resources:{facts:"receipt-retirement"}});
   if(mode==="cancelled")await expect(store.retire({signal:controller.signal})).rejects.toBe(reason);
   else await store.retire();
   using sql=new Database(":memory:");
   sql.exec("CREATE TABLE receipts(name TEXT, aged INTEGER, owned INTEGER)");
   const insert=sql.query("INSERT INTO receipts VALUES (?,?,?)");for(const row of names)insert.run(...row);
   const expected=sql.query("SELECT name FROM receipts WHERE NOT(aged AND owned AND name != ? AND ?) ORDER BY name").values(current,Number(mode==="sealed"||mode==="quarantined")).flat();
   expect((await fs.names(receipts)).sort(),mode).toEqual(expected);
  }finally{await rm(dir,{recursive:true,force:true});}
 }
});

test("archive-sized typed reference and owner-link inventories fit while larger inventories refuse safely",async()=>{
 using sql=new Database(":memory:");
 const census=sql.query(`WITH RECURSIVE edges(n) AS (SELECT 0 UNION ALL SELECT n+1 FROM edges WHERE n+1 < ?1)
 SELECT count(DISTINCT n % ?2)+2 AS refs, count(*)+3 AS links FROM edges`);
 const limits=CLOSURE_REPAIR_POLICY_V1;
 const fixtures=[
  {refs:20671,links:20672,accepted:true},
  {refs:4096,links:73950,accepted:true},
  {refs:20671,links:73950,accepted:true},
  {refs:24576,links:24577,accepted:true},
  {refs:4096,links:98304,accepted:true},
  {refs:24577,links:24578,accepted:false},
  {refs:4096,links:98305,accepted:false},
 ];
 for(const {refs,links,accepted} of fixtures){
  const unique=refs-2,extra=links-3,ownerCount=128,width=Math.ceil(extra/ownerCount);
  expect(census.get(extra,unique)).toEqual({refs,links});
  const w=world({count:ownerCount,inputsOf:index=>{
   const inputs=Array.from({length:Math.min(width,Math.max(0,extra-index*width))},(_,offset)=>{
    const n=(index*width+offset)%unique;
    return token("SOURCE_CLOSURE",n.toString(16).padStart(64,"0"));
   });
   return inputs.length?[...inputs,inputs[0]!]:inputs;
  }});
  let verified=0;
  const result=await prepareClosureRepairV1({...basic,fs:w.fs,expected:w.expected,missing:[input],verifyClosure:async ref=>{verified++;return ref.namespace!==input.namespace||ref.key!==input.key;}});
  expect(result!==undefined,`refs=${refs} links=${links}`).toBe(accepted);
  expect(verified).toBe(accepted?refs:0);
  expect(result?.owners??[]).toEqual(accepted?["a","b"]:[]);
  expect(w.writes.length).toBe(accepted?1:0);
 }
 expect({missing:limits.maximumMissing,owners:limits.maximumAffectedOwners}).toEqual({missing:768,owners:1024});
},20000);

test("typed verification is bounded in parallel and stops before the next batch on refusal",async()=>{
 const width=CLOSURE_POLICY_V1.concurrency,limit=CLOSURE_REPAIR_POLICY_V1.maximumMissing,reason=Error("cancel leaf verification");
 const inputs=Array.from({length:limit+width*2},(_,i)=>token("SOURCE_CLOSURE",i.toString(16).padStart(64,"0")));
 for(const mode of ["repair","candidate","unknown","not available","overfull","cancelled","candidate cancelled"]){
  const w=world({inputsOf:i=>i===0?inputs:[]}),controller=new AbortController(),cancelled=mode.endsWith("cancelled");
  const seen=new Set<string>();let active=0,peak=0,calls=0;
  const verifyClosure:ClosureRepairVerifierV1=async ref=>{
   seen.add(ref.namespace+"/"+ref.key);active++;peak=Math.max(peak,active);
   const attempt=++calls;
   if(cancelled&&attempt===1)controller.abort(reason);
   await Promise.resolve();active--;
   if(mode==="unknown"&&attempt===1)return undefined;
   if(mode==="not available"&&attempt===1||mode==="overfull")return false;
   return mode!=="repair"||ref.namespace!==input.namespace||ref.key!==input.key;
  };
  if(mode.startsWith("candidate")||mode==="not available"){
   const pending=verifyClosureRepairCandidateV1({...basic,fs:w.fs,manifest:w.value,signal:controller.signal,verifyClosure});
   if(cancelled)await expect(pending).rejects.toBe(reason);
   else expect(await pending).toBe(mode==="candidate");
  }else{
   const pending=prepareClosureRepairV1({...basic,fs:w.fs,expected:w.expected,missing:[input],verifyClosure,signal:controller.signal});
   if(mode==="cancelled")await expect(pending).rejects.toBe(reason);
   else expect((await pending)?.missing).toEqual(mode==="repair"?[input]:undefined);
  }
  const expected=mode==="repair"||mode==="candidate"?inputs.length+2:mode==="overfull"?Math.ceil((limit+1)/width)*width:width;
  expect(calls,mode).toBe(seen.size);
  if(cancelled)expect(seen.size).toBeLessThanOrEqual(width);
  else{expect(seen.size,mode).toBe(expected);expect(peak,mode).toBe(width);}
  expect(active,mode).toBe(0);
  expect(w.writes.length,mode).toBe(mode==="repair"?1:0);
 }
});

test("opposite leaf completion orders produce the same canonical receipt",async()=>{
 const results=[];
 for(const slower of [input.namespace,lock.namespace]){
  const w=world(),finished:string[]=[];
  const prepared=await prepareClosureRepairV1({...basic,fs:w.fs,expected:w.expected,missing:[lock,input],verifyClosure:async ref=>{
   if(ref.namespace===slower)await Bun.sleep(10);
   finished.push(ref.namespace);
   return false;
  }});
  expect(finished.at(-1)).toBe(slower);
  expect(prepared?.missing).toEqual([input,lock]);
  results.push(prepared?.receipt);
 }
 expect(results[0]).toBe(results[1]);
});

test("full-funded repair evidence admits the measured archive shape and keeps its independent bounds",async()=>{
 using sql=new Database(":memory:");
 const eligible=sql.query("SELECT (?1<=768 AND ?2<=1024 AND ?3<=262144) AS accepted");
 const cases=[
  {missing:442,owners:488,longNames:false},
  {missing:768,owners:1024,longNames:false},
  {missing:769,owners:30,longNames:false},
  {missing:2,owners:1025,longNames:false},
  {missing:1,owners:600,longNames:true},
 ];
 for(const {missing,owners,longNames} of cases){
  const references=Array.from({length:missing},(_,index)=>({key:hash("archived-leaf-"+index),namespace:index%2?input.namespace:lock.namespace}));
  const names=Array.from({length:owners},(_,index)=>"owner-"+index+(longNames?"x".repeat(500):"")).sort();
  const w=world({shared:false,count:owners,nameOf:i=>names[i]!,inputsOf:index=>{
   const owned=[references[index%missing]!];
   for(let extra=index+owners;extra<missing;extra+=owners)owned.push(references[extra]!);
   return owned.map(ref=>token(ref.namespace===input.namespace?"SOURCE_CLOSURE":"CARGO_LOCK",ref.key));
  }});
  const named=new Set(references.map(ref=>ref.namespace+"/"+ref.key)),verifyClosure:ClosureRepairVerifierV1=async ref=>!named.has(ref.namespace+"/"+ref.key);
  const document={version:1,generation:w.pointer.generation,parent:w.pointer.parent,identity,implementation,missing:references,owners:names};
  const bytes=Buffer.byteLength(canonicalJsonV1(document)),expected=eligible.get(missing,owners,bytes) as {accepted:number};
  const prepared=await prepareClosureRepairV1({...basic,fs:w.fs,expected:w.expected,missing:[references[0]!],verifyClosure});
  expect(Number(prepared!==undefined),JSON.stringify({missing,owners,bytes})).toBe(expected.accepted);
  expect(w.writes.length).toBe(expected.accepted);
  if(prepared){
   const stored=w.data.get(w.writes[0]!)!;
   expect(stored.length).toBe(bytes);
   expect(prepared.owners).toEqual(names);
   expect(prepared.missing.length).toBe(missing);
   w.move(w.pointer);
   expect(await readClosureRepairV1({...basic,fs:w.fs,pointer:w.pointer})).toEqual(prepared);
  }else if(longNames){
   expect(bytes).toBeGreaterThan(CLOSURE_REPAIR_POLICY_V1.maximumReceiptBytes);
   w.move(w.pointer);
   expect(await recoverClosureRepairV1({...basic,fs:w.fs,pointer:w.pointer,verifyClosure})).toBeUndefined();
   expect(w.writes).toEqual([]);
  }
 }
});

test("a quarantined parent of another implementation stays a valid resume base: the repaired candidate is verified by content, never started cold",async()=>{
 const w=world();w.move(w.pointer);
 const moved={...basic,implementation:hash("moved implementation")},verifyClosure=async(ref:typeof input|typeof lock)=>ref.namespace===lock.namespace;
 const recovered=await recoverClosureRepairV1({...moved,fs:w.fs,pointer:w.pointer,verifyClosure});
 expect(recovered?.owners).toEqual(["a","b"]);
 expect(recovered?.parent).toBe(w.expected.manifest);
 expect(await recoverClosureRepairV1({...moved,identity:hash("another registry"),fs:w.fs,pointer:w.pointer,verifyClosure})).toBeUndefined();
});
