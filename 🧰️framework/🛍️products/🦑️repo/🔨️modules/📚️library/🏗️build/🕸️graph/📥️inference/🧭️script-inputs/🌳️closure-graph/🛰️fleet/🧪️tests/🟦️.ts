/** 🛰️ Contract: the module closure graph of one machine travels as ONE fleet object (`../🟦️.ts`): its chunks as blobs, its head as the result of a key from the head digest, declared kind `graph`; offering it again writes nothing; a second machine fetches it (copying the chunks it already has), confirms its own tree by digests without walking (a node whose file bytes differ here is left out, every other node is adopted and stamped for this machine), and a pass over the adopted graph visits exactly the nodes left out and answers the digests a pass from scratch on that machine answers; a damaged blob or a foreign head is a fault, never a graph. */
import{expect,setDefaultTimeout,test}from"bun:test";
import{createHash}from"node:crypto";
import{cpSync,mkdirSync,mkdtempSync,realpathSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{REAPI_V1,decodeProtobufV1}from"../../../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import{createClosureGraphV1,saveClosureGraphV1}from"../../🟨️.mjs";
import{globNodesV1}from"../../🍃️leaves/🟨️.mjs";
import{cargoCrateNodesV1,cargoOwnerRootV1}from"../../🦀️cargo/🟨️.mjs";
import{adoptClosureGraphV1,closureGraphFleetKeyV1,fetchClosureGraphV1,offerClosureGraphV1}from"../🟦️.ts";

setDefaultTimeout(180000);
const sha=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex");
const TREE:Record<string,string>={
 "package.json":'{"name":"closure-fleet-fixture"}\n',
 "entry.ts":'import "./a.ts";import "./b.ts";export {};\n',"a.ts":'import "./leaf.ts";export const a=1;\n',"b.ts":'import "./leaf.ts";export const b=2;\n',"leaf.ts":'export const leaf=1;\n',"other.ts":'export const other=1;\n',
 "crate/Cargo.toml":'[package]\nname="crate"\nversion="0.1.0"\nedition="2021"\n[workspace]\n',"crate/src/lib.rs":"pub fn f(){}\n","crate/Cargo.lock":"# lock\n",
 "assets/one.json":"{}\n",
};
const write=(root:string)=>{for(const[name,text]of Object.entries(TREE)){mkdirSync(dirname(join(root,name)),{recursive:true});writeFileSync(join(root,name),text);}const at=new Date(Date.now()-3600000);for(const name of Object.keys(TREE))utimesSync(join(root,name),at,at);};
const memoryFleet=()=>{
 const blobs=new Map<string,Uint8Array>(),results=new Map<string,{bytes:Uint8Array;slot:string}>(),count={requests:0};
 const client:any={settings:{upload:true},usable:()=>true,
  async getResult(key:string){count.requests++;return results.get(key)?.bytes??"miss";},
  async getBlob(hash:string){count.requests++;return blobs.get(hash);},
  async hasBlob(hash:string){count.requests++;return blobs.has(hash);},
  async putBlob(hash:string,bytes:Uint8Array){count.requests++;if(sha(bytes)!==hash)return{stored:false,refusal:{text:"digest"}};blobs.set(hash,bytes);return{stored:true};},
  async putResult(key:string,bytes:Uint8Array,slot:string){count.requests++;results.set(key,{bytes,slot});return{stored:true};}};
 return{client,blobs,results,count};
};

test("one machine offers its graph once, another adopts it by digests and walks only what its own tree changed",async()=>{
 const inference=await import("../../../../🟨️.mjs") as any;await inference.libraryBootstrap;
 const scripts=await import("../../../🟨️.mjs") as any;
 const a=realpathSync(mkdtempSync(join(tmpdir(),"closure-fleet-a-"))),b=realpathSync(mkdtempSync(join(tmpdir(),"closure-fleet-b-"))),storeA=join(a,".g"),storeB=join(b,".g");
 try{
  write(a);cpSync(a,b,{recursive:true,preserveTimestamps:true});
  writeFileSync(join(b,"b.ts"),'import "./leaf.ts";export const b=3;\n');writeFileSync(join(b,"crate/src/lib.rs"),"pub fn g(){}\n");
  const past=new Date(Date.now()-3600000);for(const name of["b.ts","crate/src/lib.rs"])utimesSync(join(b,name),past,past);
  await Bun.sleep(2100);
  const questions=(root:string)=>[{roots:[join(root,"entry.ts")]},{roots:[join(root,"a.ts")]},{roots:[join(root,"other.ts")]}];
  const fresh=(root:string,graph?:any)=>{const cache=inference.createScriptInputCache({workspaceRoot:root});cache.pass={};cache.generatedOutput=()=>undefined;if(graph)cache.closureGraph=graph;return cache;};
  const extras=(root:string,graph:any)=>{const nodes=new Map<string,any>();cargoCrateNodesV1(root,"crate",{nodes});for(const node of globNodesV1(root,"assets","*.json"))nodes.set(node.key,node);graph.update(nodes.values());};
  const cacheA=fresh(a),graphA=createClosureGraphV1();await scripts.closureGraphPassV1(cacheA,a,questions(a),{graph:graphA,witness:true});extras(a,graphA);
  await saveClosureGraphV1(graphA,storeA);
  const fleet=memoryFleet(),offered=await offerClosureGraphV1({client:fleet.client,directory:storeA,scope:"fixture"});
  expect(offered.stored).toBe(true);expect(offered.uploaded).toBe(offered.chunks);
  const stored=decodeProtobufV1(REAPI_V1.actionResult,fleet.results.get(closureGraphFleetKeyV1(offered.head!))!.bytes) as any,manifest=stored.output_files.find((file:any)=>file.path==="stage.json");
  expect(JSON.parse(Buffer.from(fleet.blobs.get(manifest.digest.hash)!).toString("utf8")).retention.kind).toBe("graph");
  const again=await offerClosureGraphV1({client:fleet.client,directory:storeA,scope:"fixture"});
  expect(again).toMatchObject({stored:true,reason:"current",uploaded:0});expect(again.requests).toBe(1);
  const before=fleet.count.requests,graphB=createClosureGraphV1(),adopted=await adoptClosureGraphV1({client:fleet.client,head:offered.head!,directory:storeB,workspaceRoot:b,graph:graphB});
  expect(adopted.outcome).toBe("adopted");expect(fleet.count.requests-before).toBe(adopted.requests);expect(adopted.downloaded).toBe(offered.chunks);
  const refusedNames=[...graphA.records()].filter(record=>!graphB.has(record.key)).map(record=>record.name).sort();
  expect(refusedNames).toEqual(["{workspaceRoot}/b.ts","{workspaceRoot}/crate","{workspaceRoot}/crate/src/lib.rs"]);
  const cacheB=fresh(b,graphB);
  const answered:string[]=[];for(const question of questions(b))answered.push((await scripts.closureGraphAnswerV1(cacheB,b,question)).digest);
  const built=[...cacheB.walkNodes.values()].filter((entry:any)=>entry.node?.kind!==undefined).map((entry:any)=>entry.node.name).sort();
  expect(built).toEqual(["{workspaceRoot}/b.ts"]);
  const scratch=await scripts.closureGraphPassV1(fresh(b),b,questions(b));
  expect(answered).toEqual(scratch.digests);
  const cacheAgain=fresh(a);const fromA=await scripts.closureGraphPassV1(cacheAgain,a,questions(a));expect(answered,"a body edit on this machine is visited again and moves no digest").toEqual(fromA.digests);
  const nodesB=new Map<string,any>();cargoCrateNodesV1(b,"crate",{nodes:nodesB,held:key=>graphB.get(key)});graphB.update(nodesB.values());
  const scratchCargo=createClosureGraphV1(),fromScratch=new Map<string,any>();cargoCrateNodesV1(b,"crate",{nodes:fromScratch});scratchCargo.update(fromScratch.values());
  expect(graphB.closureDigest([cargoOwnerRootV1(b,"crate")])).toBe(scratchCargo.closureDigest([cargoOwnerRootV1(b,"crate")]));
  const storeC=join(b,".c");mkdirSync(join(storeC,"c"),{recursive:true});cpSync(join(storeA,"c"),join(storeC,"c"),{recursive:true});
  const copied=await fetchClosureGraphV1({client:fleet.client,head:offered.head!,directory:join(b,".copy"),known:storeC});
  expect(copied).toMatchObject({outcome:"fetched",downloaded:0,requests:1});
  const[first]=[...fleet.blobs.keys()].filter(hash=>stored.output_files.some((file:any)=>file.digest.hash===hash&&file.path!=="stage.json"));fleet.blobs.set(first!,Buffer.from("damaged"));
  expect(await fetchClosureGraphV1({client:fleet.client,head:offered.head!,directory:join(b,".damaged")})).toBe("fault");
  expect(await fetchClosureGraphV1({client:fleet.client,head:sha("other"),directory:join(b,".other")})).toBe("miss");
 }finally{rmSync(a,{recursive:true,force:true});rmSync(b,{recursive:true,force:true});}
});
