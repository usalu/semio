/** 🧱️ Contract: a native Nx shard that cannot rewrite its file map next to the old copy (or is nearly full) is relieved by removing the regenerable file map when nobody holds the shard, the shard is named in the refusal that replaces a bare ENOSPC, and a shard a live run holds is never touched. An independent Python model replays the cases.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import fixture from "./🔣️.json" with {type:"json"};
import caseSchema from "./🧬️schema.json" with {type:"json"};
import policySchema from "../🧬️schema/🔣️.json" with {type:"json"};
import {NATIVE_SHARDS_POLICY_V1,nativeShardConsumerV1,nativeShardNamespaceV1,nativeShardShortV1,nativeShardTextV1,planNativeShardReliefV1,readNativeShardsV1,relieveNativeShardsV1,type NativeShardFactV1} from "../🟦️.ts";

const facts=(id:string)=>fixture.cases.find(row=>row.id===id)!.facts as NativeShardFactV1[];

test("the cases obey their schema",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(policySchema);
 expect(ajv.compile(caseSchema)(fixture)).toBe(true);
});

test("planning and naming equal the independent Python model on every case",()=>{
 const python=Bun.spawnSync(["python3","-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))]);
 expect(python.exitCode).toBe(0);
 const expected=JSON.parse(python.stdout.toString()) as Record<string,{removals:{id:string;name:string;bytes:number}[];consumer:string|null}>;
 for(const row of fixture.cases){
  const plan=planNativeShardReliefV1(row.facts as NativeShardFactV1[]).map(item=>({id:item.id,name:item.name,bytes:item.bytes})),consumer=nativeShardConsumerV1(row.facts as NativeShardFactV1[]);
  expect({id:row.id,plan,consumer:consumer?.id??null}).toEqual({id:row.id,plan:expected[row.id]!.removals,consumer:expected[row.id]!.consumer});
 }
});

test("the shard of the 64 MiB image with a 38 MB file map is short, a roomy one is not, a full one without a map is never touched",()=>{
 expect(nativeShardShortV1(facts("coordinator-shard-cannot-rewrite")[0]!)).toBe(true);
 expect(nativeShardShortV1(facts("roomy-shard-is-left-alone")[0]!)).toBe(false);
 expect(nativeShardShortV1(facts("near-full-with-a-small-map")[0]!)).toBe(true);
 expect(planNativeShardReliefV1(facts("full-without-a-regenerable-file"))).toEqual([]);
 expect(nativeShardConsumerV1(facts("full-without-a-regenerable-file"))).toBeUndefined();
});

test("the named consumer is the fullest short shard and its text is bilingual with every placeholder filled",()=>{
 const consumer=nativeShardConsumerV1(facts("ids-in-order-and-fullest-named"))!;
 expect(consumer.id).toBe("21ff94dc4d63aba6596930a7e4dd621fb0c922bf");expect(consumer.path).toBe("/w/y");expect(consumer.file).toBe("nx_files.nxt");
 const values={id:consumer.id.slice(0,12),path:consumer.path,used:Math.round(consumer.usedBytes/1048576),total:Math.round(consumer.totalBytes/1048576),file:consumer.file,map:Math.round(consumer.fileMapBytes/1048576),free:Math.round(consumer.freeBytes/1048576)};
 for(const language of["en","de"] as const){const text=nativeShardTextV1("consumer",language,values);expect(text).not.toMatch(/\{[A-Za-z]+\}|\?/u);expect(text).toContain("nx_files.nxt");expect(text).toContain("/w/y");}
 expect(nativeShardTextV1("consumer","en",values)).not.toBe(nativeShardTextV1("consumer","de",values));
 expect(nativeShardTextV1("freed","both",{id:"a",file:"f",map:1}).split(" / ").length).toBe(2);
});

test("the relief removes the file map of a free shard, skips a held one and frees nothing in a dry run",async()=>{
 const root=mkdtempSync(join(tmpdir(),"nsh-"));
 try{
  const all=facts("ids-in-order-and-fullest-named").map(fact=>{const path=join(root,fact.id);mkdirSync(path,{recursive:true});for(const file of fact.files)writeFileSync(join(path,file.name),"x");return{...fact,path};});
  const dry=await relieveNativeShardsV1({facts:all,hold:async()=>()=>undefined,dryRun:true,language:"en"});
  expect(dry.freedBytes).toBe(74000000);for(const fact of all)expect(existsSync(join(fact.path,"nx_files.nxt"))).toBe(true);
  const busy="21ff94dc4d63aba6596930a7e4dd621fb0c922bf",released:string[]=[];
  const real=await relieveNativeShardsV1({facts:all,hold:async id=>id===busy?undefined:()=>{released.push(id);},language:"en"});
  expect(real.held).toEqual([busy]);expect(real.freedBytes).toBe(36000000);released.sort();expect(released).toEqual(["f0797d0fc5b6e3e6445776aef6e1539e6beb9738"]);
  expect(existsSync(join(all.find(fact=>fact.id===busy)!.path,"nx_files.nxt"))).toBe(true);
  expect(existsSync(join(all.find(fact=>fact.id!==busy)!.path,"nx_files.nxt"))).toBe(false);
  expect(real.lines.some(line=>line.includes("in use"))).toBe(true);
  const again=await relieveNativeShardsV1({facts:all,hold:async()=>()=>undefined,language:"en"});expect(again.freedBytes).toBe(38000000);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a machine without mounted shards has none and a plain directory is no shard",()=>{
 const root=mkdtempSync(join(tmpdir(),"nsh-"));
 try{
  expect(readNativeShardsV1(root)).toEqual([]);
  mkdirSync(join(root,...NATIVE_SHARDS_POLICY_V1.directory.split("/"),"3410c86e36cbdfb40e22aa01a6d35b2124eae49d","native","m"),{recursive:true});
  expect(readNativeShardsV1(root)).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the namespace resource and directory equal the ones the Nx graph owner derives",async()=>{
 const graph=await import("../../../../../⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts"),root=join("/w",...NATIVE_SHARDS_POLICY_V1.directory.split("/")),id="3410c86e36cbdfb40e22aa01a6d35b2124eae49d";
 const owner=graph.nxGraphNamespaceDirectoryOfNativeV1(root,join(root,id,"native")),mine=nativeShardNamespaceV1("/w",id);
 expect(mine.directory).toBe(owner);expect(mine.resource).toBe(graph.nxGraphNamespaceResourceV1(owner));
});
