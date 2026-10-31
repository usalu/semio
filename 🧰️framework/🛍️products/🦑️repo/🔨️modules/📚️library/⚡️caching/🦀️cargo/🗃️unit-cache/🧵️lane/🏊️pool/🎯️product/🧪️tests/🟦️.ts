import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {planPoolAcquireV1} from "../../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {join,resolve} from "node:path";
import {tmpdir} from "node:os";
import {acquireProductLaneV1} from "../🟦️.ts";
import {acquirePooledLaneV1,trimPoolV1} from "../../🟦️.ts";
import {recordLaneSizeV1} from "../../../🟦️.ts";
import {compilerProductKeyV1,type CompilerProductIntentV1} from "../../../../../../💾️storage/🔒️enforcement/🏗️compiler/🎯️product/🟦️.ts";
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";

const scratch=()=>{const base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir());mkdirSync(base,{recursive:true});return mkdtempSync(join(base,"product-"));},owner={session:"0123456789ab.p",agent:"product-tests"},key="a".repeat(64),request=(key:string)=>({key,crates:["same"],quotaBytes:Number.MAX_SAFE_INTEGER,owner});

test("ordinary acquisitions never repurpose a managed product namespace",()=>{
 const database=new Database(":memory:");
 try{for(const row of fixture.cases){
  const oracle=database.query("select case when exists(select 1 from json_each($lanes) where not coalesce(json_extract(value,'$.product'),0) and not json_extract(value,'$.claimed') and json_extract(value,'$.crateKey')=$key) then 'warm' when json_array_length($lanes)<$maximum and coalesce((select sum(json_extract(value,'$.bytes')) from json_each($lanes)),0)<=$quota then 'new' when exists(select 1 from json_each($lanes) where not coalesce(json_extract(value,'$.product'),0) and not json_extract(value,'$.claimed')) then 'lru' when json_array_length($lanes)<$live then 'new' else 'busy' end as kind").get({$lanes:JSON.stringify(row.lanes),$key:row.crateKey,$maximum:fixture.bounds.maximumLanes,$quota:fixture.bounds.quotaBytes,$live:fixture.bounds.maximumLiveLanes}) as {kind:string};
  expect(oracle.kind,row.id).toBe(row.kind);expect(String(planPoolAcquireV1(row.lanes,{crateKey:row.crateKey},fixture.bounds).kind),row.id).toBe(oracle.kind);
 }}finally{database.close();}
});

test("bounded exact product facts match an independent framework hash and preserve path and selection distinctions",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"test:compiler-product-facts",maximumElapsedMilliseconds:30000},{workspaceRoot:process.cwd()},Date.now()),async original=>{
 const control=createIssuedResourceControlV1(original,"test:product-facts",0),keys:string[]=[];
 for(const raw of[fixture.intent,...fixture.intentVariants.map(row=>({...fixture.intent,...row}))]){
  const intent=raw as CompilerProductIntentV1,actual=await compilerProductKeyV1(intent,control),preimage="semio.cargo.product/v1\n"+[intent.version,intent.root,intent.manifest,intent.mode,intent.command,intent.producer,intent.toolchain,...intent.arguments].map(value=>JSON.stringify(value)+"\n").join("");
  expect(actual).toBe(new Bun.CryptoHasher("sha256").update(preimage).digest("hex"));keys.push(actual);
 }
 expect(new Set(keys).size).toBe(keys.length);expect(await compilerProductKeyV1(fixture.intent as CompilerProductIntentV1,control)).toBe(keys[0]!);
}));

test("exact product claims serialize and cancel only the waiting caller while other products proceed",async()=>{
 const root=scratch(),signal=new AbortController().signal;
 try{
  const first=await acquireProductLaneV1(root,request(key),{signal}),cancelled=new AbortController(),reason=Error("owned waiter cancelled"),waiting=acquireProductLaneV1(root,request(key),{signal:cancelled.signal});
  cancelled.abort(reason);await expect(waiting).rejects.toBe(reason);expect(existsSync(first.paths.build)).toBe(true);
  const other=await acquireProductLaneV1(root,request("b".repeat(64)),{signal});expect(other.paths.build).not.toBe(first.paths.build);other.release();
  const next=acquireProductLaneV1(root,request(key),{signal});first.release();const acquired=await next;expect(acquired.paths).toEqual(first.paths);acquired.release();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("short namespace collisions refuse the different full digest without rewriting its receipt",async()=>{
 const root=scratch(),signal=new AbortController().signal;
 try{
  const first=await acquireProductLaneV1(root,request(key),{signal}),receipt=join(first.paths.base,"product.json"),before=readFileSync(receipt,"utf8");first.release();
  await expect(acquireProductLaneV1(root,request(key.slice(0,11)+"b".repeat(53)),{signal})).rejects.toThrow("identity collision");expect(readFileSync(receipt,"utf8")).toBe(before);
  const again=await acquireProductLaneV1(root,request(key),{signal});again.release();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("ordinary acquisition cannot repurpose product paths and idle product bytes remain subject to retirement",async()=>{
 const root=scratch(),signal=new AbortController().signal;
 try{
  const product=await acquireProductLaneV1(root,request(key),{signal});product.release();
  const ordinary=await acquirePooledLaneV1(root,{crates:["same"],quotaBytes:Number.MAX_SAFE_INTEGER});expect(ordinary.paths.id).not.toBe(product.paths.id);
  recordLaneSizeV1(root,product.paths.id,4096,0,Date.now());const result=trimPoolV1(root,{maximumLanes:16,maximumLiveLanes:32,quotaBytes:0},Date.now());expect(result.removed).toBe(1);expect(existsSync(product.paths.base)).toBe(false);expect(existsSync(ordinary.paths.base)).toBe(true);ordinary.release();
  const recreated=await acquireProductLaneV1(root,request(key),{signal});expect(recreated.paths).toEqual(product.paths);recreated.release();
 }finally{rmSync(root,{recursive:true,force:true});}
});
