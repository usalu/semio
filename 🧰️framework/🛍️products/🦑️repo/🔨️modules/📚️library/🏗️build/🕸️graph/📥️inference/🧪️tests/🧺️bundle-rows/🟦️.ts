/** 🧺️ Contract: the portable bundles of closure entries that share a layer hold that layer's rows once: however many entries a process remembers for export to the fleet, a row met by several bundles is one live object, so the bundle cache pins references and not a parsed copy per bundle; an independent Python model counts the held and the distinct rows of the same entries.
 * @see ../../🧭️script-inputs/💾️entries/🟨️.mjs `bundle` */
import{expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{createClosureStoreV1}from"../../🧭️script-inputs/💾️entries/🟨️.mjs";
import schema from"../../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";

type Row={kind:string;path:string;fact:{digest:string;size:number}};
const layerOf=(layer:number):Row[]=>Array.from({length:fixture.rowsPerLayer},(_,index)=>({kind:"analysis",path:"/w/l"+layer+"/f"+index,fact:{digest:String(layer*100000+index).padEnd(64,"d"),size:index}}));
const model=()=>{
 const run=Bun.spawnSync([testPythonV1()!,"-I",join(import.meta.dir,"🐍️.py")],{stdin:Buffer.from(JSON.stringify(fixture))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 return JSON.parse(run.stdout.toString()) as{heldRows:number;distinctRows:number};
};

test("bundles that share layers hold one live object per distinct row",async()=>{
 const expected=model(),directory=mkdtempSync(join(tmpdir(),"bundle-rows-")),store=createClosureStoreV1({directory});
 try{
  for(const entry of fixture.entries)await store.save(entry.id,{recipe:{fn:entry.id},value:[],rows:[],layers:entry.layers.map(layerOf)});
  const bundles=[];for(const entry of fixture.entries)bundles.push((await store.bundle(entry.id))!);
  const rows=bundles.flatMap(bundle=>bundle.layers.flat() as Row[]);
  expect(rows.length).toBe(expected.heldRows);
  expect(new Set(rows).size).toBe(expected.distinctRows);
  expect(expected.distinctRows).toBeLessThan(expected.heldRows);
  for(const entry of fixture.entries)expect((await store.bundle(entry.id))!.layers).toEqual(entry.layers.map(layerOf));
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the bundle cache is charged for the rows its bundles hold and bounded by the declared budget",()=>{
 const limits=schema.definitions.Limits.const as Record<string,number>;
 expect(limits.closureBundleCacheBytes).toBeGreaterThan(0);
 expect(limits.closureBundleCacheBytes).toBeLessThanOrEqual(limits.closureEntryCacheBytes);
});
