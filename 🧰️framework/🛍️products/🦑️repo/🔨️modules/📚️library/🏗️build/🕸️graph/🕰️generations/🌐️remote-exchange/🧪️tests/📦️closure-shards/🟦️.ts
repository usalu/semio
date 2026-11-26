import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {brotliCompressSync} from "node:zlib";
import Ajv from "ajv";
import {GENERATION_POLICY_V1,createFrontierV1,createGenerationStoreV1} from "../../../🟨️.mjs";
import {createMemoryFilePort} from "../../../🧪️tests/🏃️world/🟦️.ts";
import {generationExchangeKeyV1,publishGenerationV1,restoreGenerationV1,type ClosurePortV1} from "../../🟦️.ts";
import {closureEntryV1,encodeClosureMemoV1,inventoryFragmentClosuresV1,readClosureMemoV1} from "../../🧲️closure-shards/🟦️.ts";
import exchange from "../../🧬️schema/🔣️.json";
import {parseRemoteCacheActionResultV1,parseRemoteCacheTreeV1,validateRemoteCacheActionResultV1} from "../../../../../🌐️remote-cache/🔬️action-result/🟦️.ts";
import {REAPI_V1,decodeProtobufV1,encodeProtobufV1} from "../../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {RemoteExecutionContentErrorV1,type RemoteExecutionContentV1} from "../../../../../🌐️remote-execution/🗄️content/🟦️.ts";
import type {ClosureNamespaceV1} from "../../../../🩹️closure-repair/🟨️.mjs";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const sha=(bytes:string|Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
const location=(root:string)=>({root,workspace:root,files:createMemoryFilePort().port,platform:"darwin-arm64",instance:"main"});
const keyspace=(leaves=new Map<string,Uint8Array>()):ClosurePortV1=>({
 directory:namespace=>"/memory/"+namespace,
 size:(namespace,key)=>leaves.get(closureEntryV1(namespace,key))?.length,
 read:async(namespace,key)=>leaves.get(closureEntryV1(namespace,key)),
 put:async(namespace,key,bytes)=>{leaves.set(closureEntryV1(namespace,key),bytes);},
});

test("an archived-size generation publishes and restores every closure within the authenticated ActionResult budget",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const work=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"gt-"));
 try{
  const source=location(join(work,"src")),reader=location(join(work,"dst")),store=createGenerationStoreV1({root:source.root,fs:source.files,holds:async()=>true,resources:{facts:"facts",builder:"builder"}});
  const identity=sha("sized-fixture"),implementation=sha("sized-code"),key=generationExchangeKeyV1(identity,source.platform,"local"),names=fixture.fragments-1,sourceLeaves=keyspace(),receiveLeaves=keyspace(),cas=new Map<string,Uint8Array>();
  const leaves=Array.from({length:fixture.uniqueClosures},(_,index)=>{
   const lock=index%17===0,namespace:ClosureNamespaceV1=lock?"build-lock-closures":"build-input-closures",payload=Buffer.from(JSON.stringify({fixture:index})),hash=sha(payload);
   return{namespace,payload,hash,token:(lock?"SEMIO_CARGO_LOCK_":"SEMIO_SOURCE_CLOSURE_")+hash};
  });
  for(const row of leaves)await sourceLeaves.put(row.namespace,row.hash,row.payload);
  const owners=Array.from({length:names},(_,index)=>{
   const start=Math.floor(index*fixture.ownerLinks/names),end=Math.floor((index+1)*fixture.ownerLinks/names),name="owner-"+index;
   return{name,root:name,kind:"project" as const,node:{name,refs:Array.from({length:end-start},(_,offset)=>leaves[(start+offset)%leaves.length]!.token)},edges:[],targets:[],rows:[]};
  });
  await store.refresh({identity,implementation,membership:async()=>({digest:sha("membership"),frontier:createFrontierV1({directories:[],members:[],capturedAt:0}),changes:[]}),compute:async()=>({owners,env:{digest:sha("env"),rows:[]}})});
  let action:Uint8Array|undefined,attempts=0;
  const content:RemoteExecutionContentV1={
   readActionResult:async(_instance,slot)=>slot===key?action:undefined,
   has:async(_instance,hash)=>cas.has(hash),
   missing:async(_instance,rows)=>rows.filter(row=>!cas.has(row.hash)),
   read:async(_instance,row)=>cas.get(row.hash),
   stream:async(_instance,row)=>{const bytes=cas.get(row.hash);return bytes===undefined?undefined:(async function*(){yield bytes;})();},
   write:async(_instance,row,bytes)=>{if(!(bytes instanceof Uint8Array)||sha(bytes)!==row.hash||bytes.length!==row.size)throw Error("Invalid fixture CAS write");cas.set(row.hash,bytes);},
   writeActionResult:async(_instance,_key,bytes)=>{
    const parsed=parseRemoteCacheActionResultV1(bytes);
    if(++attempts===1){cas.delete(parsed.trees[0]!.hash);cas.delete(leaves[0]!.hash);}
    const verdict=await validateRemoteCacheActionResultV1(bytes,"complete",{has:hash=>cas.has(hash),read:async hash=>cas.get(hash)});
    if(!verdict.valid)throw new RemoteExecutionContentErrorV1(3,verdict.reason,/[a-f0-9]{64}/u.exec(verdict.reason)?.[0]);
    action=bytes;
   },
  };
  const sent=await publishGenerationV1({...source,content,key,closures:sourceLeaves,stale:0});
  expect(sent).toMatchObject({status:"uploaded",closures:{named:fixture.uniqueClosures,missing:[]}});
  expect(attempts).toBe(2);
  let repeatFragmentReads=0;
  const secondKey=generationExchangeKeyV1(identity,source.platform,"peer"),firstAction=action;
  const observedFiles={...source.files,read:async(path:string)=>{if(path.startsWith(join(source.root,GENERATION_POLICY_V1.fragments)))repeatFragmentReads++;return source.files.read(path);}};
  expect(await publishGenerationV1({...source,files:observedFiles,content,key:secondKey,origin:"peer",closures:sourceLeaves,stale:0})).toMatchObject({status:"uploaded",closures:{named:fixture.uniqueClosures,missing:[]}});
  expect(repeatFragmentReads).toBe(0);
  action=firstAction;
  const sealed=action!,parsed=parseRemoteCacheActionResultV1(sealed);
  expect(sealed.length).toBeLessThanOrEqual(fixture.actionBytesCeiling);
  expect(parsed.trees.length).toBe(fixture.shards);
  const portable=decodeProtobufV1(REAPI_V1.actionResult,sealed),inline=encodeProtobufV1(REAPI_V1.actionResult,{output_files:[...portable.output_files,...leaves.map(row=>({path:"c/"+closureEntryV1(row.namespace,row.hash),digest:{hash:row.hash,size_bytes:row.payload.length}}))],stdout_raw:portable.stdout_raw});
  expect(sealed.length).toBeLessThan(inline.length/4);
  using oracle=new Database(":memory:");
  oracle.exec("CREATE TABLE leaves(name TEXT PRIMARY KEY, digest TEXT); CREATE TABLE shipped(digest TEXT);");
  const insertLeaf=oracle.query("INSERT INTO leaves VALUES (?,?)"),insertSent=oracle.query("INSERT INTO shipped VALUES (?)");
  for(const row of leaves)insertLeaf.run(closureEntryV1(row.namespace,row.hash),row.hash);
  const actual=parsed.trees.map(tree=>{const rows=parseRemoteCacheTreeV1(cas.get(tree.hash)!);for(const row of rows)insertSent.run(row.hash);return rows.length;});
  const expected=oracle.query("SELECT COUNT(*) AS count FROM (SELECT (ROW_NUMBER() OVER (ORDER BY name) - 1) / ? AS shard FROM leaves) GROUP BY shard ORDER BY shard").all(exchange.definitions.Policy.const.closureShardEntries) as {count:number}[];
  expect(actual).toEqual(expected.map(row=>row.count));
  expect(oracle.query("SELECT COUNT(*) AS absent FROM (SELECT digest FROM leaves EXCEPT SELECT digest FROM shipped)").get()).toEqual({absent:0});
  expect(owners.reduce((total,row)=>total+row.node.refs.length,0)).toBe(fixture.ownerLinks);

  const last=parsed.trees.at(-1)!,lastBytes=cas.get(last.hash)!,leaf=leaves[0]!;
  for(const row of fixture.cases){
   action=sealed;cas.set(last.hash,lastBytes);cas.set(leaf.hash,leaf.payload);
   if(row.condition==="missing shard")cas.delete(last.hash);
   if(row.condition==="omitted shard")action=encodeProtobufV1(REAPI_V1.actionResult,{...portable,output_directories:portable.output_directories.slice(0,-1)});
   if(row.condition==="missing leaf")cas.delete(leaf.hash);
   const result=await restoreGenerationV1({...reader,content,key,closures:receiveLeaves,identity,implementation,requireComplete:true});
   expect<unknown>(result.status,row.condition).toBe(row.head);
   expect(await reader.files.read(join(reader.root,GENERATION_POLICY_V1.pointer))!==undefined,row.condition).toBe(row.head==="restored");
  }
 }finally{rmSync(work,{recursive:true,force:true});}
},120000);

test("a checksummed closure memo reopens only new fragments and retains both namespaces of a digest",async()=>{
 const key=sha("shared"),first=brotliCompressSync(JSON.stringify([`SEMIO_SOURCE_CLOSURE_${key}`,`SEMIO_CARGO_LOCK_${key}`])),second=brotliCompressSync(JSON.stringify([`SEMIO_CARGO_LOCK_${sha("new")}`]));
 const blobs=new Map([[sha(first),first],[sha(second),second]]),reads:string[]=[];
 const read=async(name:string)=>{reads.push(name);return blobs.get(name);};
 const memo=new Map<string,readonly string[]>(),one=await inventoryFragmentClosuresV1([sha(first)],read,undefined,memo),wire=encodeClosureMemoV1([sha(first)],memo)!;
 const schema=new Ajv({strict:true}).compile(exchange.definitions.ClosureMemo);
 expect(schema(JSON.parse(wire))).toBe(true);
 expect(one.size).toBe(2);
 const known=readClosureMemoV1(Buffer.from(wire)),both=await inventoryFragmentClosuresV1([...blobs.keys()],read,undefined,known);
 expect([...both.keys()]).toEqual([...one.keys(),closureEntryV1("build-lock-closures",sha("new"))]);
 expect(reads).toEqual([...blobs.keys()]);
 const tampered=JSON.parse(wire);tampered.entries[0][1].pop();
 expect(readClosureMemoV1(Buffer.from(JSON.stringify(tampered))).size).toBe(0);
});
