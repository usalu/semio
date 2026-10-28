import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{unitKeyPartsV1,unitKeyV1}from"../../🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};

/** 🧩️ A unit-cache miss names the parts of its first-level key (toolchain, arguments, source, externs, environment) by short digests, so two compilations of one crate that disagree on the key are explained by the one part that differs; Python hashes the same parts. */
const python=Bun.which("python3")??Bun.which("python");
test("exactly the changed part of the key changes, and Python gives the same part digests",()=>{
 const base=unitKeyPartsV1(fixture.input as any);
 for(const row of fixture.variants){
  const parts=unitKeyPartsV1({...fixture.input,...row.set} as any).split(" "),was=base.split(" ");
  expect(parts.map((part,index)=>part===was[index]?"":part.split(":")[0]).filter(Boolean),row.name).toEqual([row.changes]);
  expect(unitKeyV1({...fixture.input,...row.set} as any),row.name).not.toBe(unitKeyV1(fixture.input as any));
 }
 expect(unitKeyPartsV1(fixture.input as any)).toBe(base);
 expect(python,"an independent Python is required").toBeTruthy();
 const directory=mkdtempSync(join(tmpdir(),"rf-parts-")),file=join(directory,"fixture.json");writeFileSync(file,JSON.stringify(fixture));
 const out=spawnSync(python!,["-I",join(import.meta.dir,"🐍️.py"),file],{encoding:"utf8"});rmSync(directory,{recursive:true,force:true});
 expect(out.status,out.stderr).toBe(0);
 expect(JSON.parse(out.stdout)).toEqual({base,variants:fixture.variants.map(row=>unitKeyPartsV1({...fixture.input,...row.set} as any))});
});
