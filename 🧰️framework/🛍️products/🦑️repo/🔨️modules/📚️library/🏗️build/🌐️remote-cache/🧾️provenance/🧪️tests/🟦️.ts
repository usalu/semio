import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {encodeProtobufV1,REAPI_V1} from "../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {parseRemoteCacheStableResultV1} from "../../🔬️action-result/🟦️.ts";
import {openProvenanceV1,PROVENANCE_VOLATILE_OUTPUTS_V1,provenanceFingerprintV1} from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

/** 🧾️ The fingerprint ignores the manifest names that carry timestamps, and the consumption record counts who read whose results. The fingerprint cases are one shared fixture checked by an independent Python implementation. */
const here=fileURLToPath(new URL(".",import.meta.url)),scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const directory=()=>{const path=mkdtempSync(join(tmpdir(),"prov-"));scratch.push(path);return path;};

test("the fingerprint of an action result leaves out the volatile manifest and equals the independent Python computation on every fixture case",()=>{
 expect(PROVENANCE_VOLATILE_OUTPUTS_V1).toEqual(fixture.volatile);
 for(const row of fixture.cases){
  const bytes=encodeProtobufV1(REAPI_V1.actionResult,{output_files:row.outputs.map(output=>({path:output.path,digest:{hash:output.hash,size_bytes:3}})),exit_code:0});
  const parsed=parseRemoteCacheStableResultV1(bytes,PROVENANCE_VOLATILE_OUTPUTS_V1);
  expect(provenanceFingerprintV1([...parsed.stable,...parsed.trees].map(digest=>digest.hash)),row.name).toBe(row.fingerprint);
  expect(parsed.manifest,row.name).toBe(row.manifest);
 }
 const python=spawnSync("python3",["-I",resolve(here,"🐍️.py"),resolve(here,"🧫️fixtures/🔣️.json")],{encoding:"utf8"});
 if(python.error===undefined)expect(python.stdout.trim(),python.stderr).toBe("ok "+fixture.cases.length);
});

test("consumption counts producer>consumer pairs with the compiled units among them, survives a restart and stays bounded",async()=>{
 const root=directory();let now=1000;
 const first=await openProvenanceV1(root,{now:()=>now});
 first.record("main","ac","unit","g1-m01","f1",true);first.record("main","ac","record","g1-m02","f2");
 first.consume("main","ac","unit","g1-m02");first.consume("main","ac","unit","g1-m02");first.consume("main","ac","unit","g1-m01");
 first.consume("main","ac","record","g1-m01");first.consume("main","ac","gone","static");
 expect(first.consumption()).toEqual({since:1000,total:5,pairs:{"g1-m01>g1-m02":{hits:2,units:2},"g1-m01>g1-m01":{hits:1,units:1},"g1-m02>g1-m01":{hits:1,units:0},"unknown>static":{hits:1,units:0}}});
 await first.flush();
 expect(existsSync(join(root,"fleet","consumption.json"))).toBe(true);expect(JSON.parse(readFileSync(join(root,"fleet","consumption.json"),"utf8")).total).toBe(5);
 now=2000;const second=await openProvenanceV1(root,{now:()=>now});
 expect(second.consumption()).toEqual(first.consumption());
 second.consume("main","ac","unit","g1-m02");expect(second.consumption().pairs["g1-m01>g1-m02"]).toEqual({hits:3,units:3});expect(second.consumption().since).toBe(1000);
 for(let index=0;index<1100;index++)second.consume("main","ac","unit","c"+index);
 const pairs=Object.keys(second.consumption().pairs);expect(pairs.length).toBeLessThanOrEqual(1025);expect(second.consumption().pairs["*>*"]!.hits).toBeGreaterThan(0);
 expect(second.consumption().total).toBe(5+1+1100);
 await second.flush();
});
