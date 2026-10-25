import {expect,test} from "bun:test";
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
