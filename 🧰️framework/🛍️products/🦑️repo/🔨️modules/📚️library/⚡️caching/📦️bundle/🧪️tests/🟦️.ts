import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {withBuildTestInvocationV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {scriptInvocationEnvironment} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {createBunBundleStreamV1} from "../🟦️bun/📤️stream/🟦️.ts";
import {readFileSync} from "node:fs";
import {withCarriedOrCountedBunCapacity} from "../🟦️.ts";
import {fixedCompilerCapacityRequestV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";

const never=new Proxy({},{get(){throw Error("A carried capacity must not touch the lease layer");}}) as never;

test("the Bun bundler under a carried counted slot requests no lease and sees the carried parallelism in every worker bound",async()=>{
 const slot=fixedCompilerCapacityRequestV1("bun-bundle") as {maximumParallelism:number};
 const seen=await withCarriedOrCountedBunCapacity("/lease-directory-that-must-stay-unused",never,{parallelism:slot.maximumParallelism},async environment=>environment,{SEMIO_COMPILER_CAPACITY_HELD:"true",KEEP:"1"});
 for(const name of["SEMIO_COMPILER_PARALLELISM","GOMAXPROCS","UV_THREADPOOL_SIZE"])expect(seen[name],name).toBe(String(slot.maximumParallelism));
 expect(seen.SEMIO_COMPILER_CAPACITY_HELD).toBeUndefined();expect(seen.KEEP).toBe("1");
});

test("a carried capacity below the bundler's minimum slot is refused instead of requesting a second capacity beneath the caller's scratch",async()=>{
 const slot=fixedCompilerCapacityRequestV1("bun-bundle") as {minimumParallelism:number};
 if(slot.minimumParallelism<2)return;
 await expect(withCarriedOrCountedBunCapacity("/unused",never,{parallelism:slot.minimumParallelism-1},async()=>undefined)).rejects.toThrow("counted slot");
});

test("the bundler takes the counted Bun slot before any store or scratch lease and carries it beneath them",()=>{
 const text=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),start=text.indexOf("export async function bundleVirtualJavaScript"),body=text.slice(start,text.indexOf("\n}\n",start));
 const order=["withCountedBunCapacity(","capacity:{parallelism:Number(slot.SEMIO_COMPILER_PARALLELISM)}","withRepositoryCacheStoreV1(","withArtifactScratchV1(","withBunBundleFiles("].map(needle=>body.indexOf(needle));
 expect(order.every(index=>index>=0)).toBe(true);expect([...order].sort((left,right)=>left-right)).toEqual(order);
 expect(text.slice(text.indexOf("async function withBunBundleFiles"),start)).toContain("withCarriedOrCountedBunCapacity(");
});

test("every option of a JavaScript bundle is either declared by the bundle schema or withheld from its validation",()=>{
 const text=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8")) as {properties:Record<string,unknown>};
 const declared=text.match(/export interface JavaScriptBundle \{([^}]*(?:\{[^}]*\}[^}]*)*)\}/)![1]!,keys=[...declared.matchAll(/readonly (\w+)\??:/g)].map(match=>match[1]!);
 const withheld=JSON.parse(text.match(/Object\.entries\(options\)\.filter\(\(\[key,value\]\)=>!(\[[^\]]*\])\.includes\(key\)/)![1]!) as string[];
 expect(keys.length).toBeGreaterThan(8);
 for(const key of keys)expect(key in schema.properties||withheld.includes(key),key).toBe(true);
});

/** 🏃️ The compile leaf only runs for the Original of the checkout that holds it (a private repository has no leaf of its own), so its success path is exercised here directly under the receiver Original: no store, no scratch, only the leaf's own request and stream. */
test("the Bun compile leaf compiles a module request for the Original of its own checkout and streams the exact bytes",async()=>{
 await withBuildTestInvocationV1(async original=>{
  const authority=readNativeCallerAuthorityV1(original.capabilities),mode=authority.original.buildMode as"prototype"|"release",control=scriptCompilerAdmissionControlV1(original,"test:bun-leaf",120000),request={version:1,mode,kind:"module",entrypoint:import.meta.path,contents:"export const value=7;\n",external:[]},input=JSON.stringify(request),chunks=new Map<string,Buffer[]>(),stream=createBunBundleStreamV1(createHash("sha256").update(input).digest("hex"),event=>{if(event.type==="file")chunks.set(event.name,[]);else if(event.type==="chunk")chunks.get(event.name)!.push(Buffer.from(event.bytes));});
  await runOwnedCommand(process.execPath,[join(import.meta.dirname,"../🟦️bun/📜️script.ts"),"compile"],authority.repositoryRoot,"javascript:bun-worker-test",0,{deadline:control,input,signal:control.signal,stdout:"ignore",output:{maximumBytes:Math.min(738197504,authority.transport.maximumBytes),maximumLines:authority.transport.maximumLines},outputConsumer:(bytes,source)=>{if(source==="stdout")stream.push(bytes);},env:scriptInvocationEnvironment(original,{...process.env,SEMIO_BUILD_MODE:mode,SEMIO_COMPILER_CAPACITY_HELD:undefined,SEMIO_BUILD_PROFILE_INPUTS:"0"})});
  const records=stream.finish(),source=Buffer.concat(chunks.get("source.js")??[]);
  expect(records.map(row=>row.name)).toContain("source.js");
  expect(records.find(row=>row.name==="source.js")!.sha256).toBe(createHash("sha256").update(source).digest("hex"));
  expect(source.toString("utf8")).toMatch(/value\s*=\s*7/u);
 });
},120000);
