import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{argumentClassesV1,differingComponentsV1,unitKeyComponentsV1,unitKeyV1}from"../../🟦️.ts";
import Ajv from"ajv";
import schema from"../../../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};

/** 🧬️ The key of a unit lookup is explained by named components, each an individual digest: changing exactly one input moves exactly its component (and the key), and an independent Python classifier computes the same digests. */
const python=Bun.which("python3")??Bun.which("python"),input=fixture.input as Parameters<typeof unitKeyV1>[0];
const variantOf=(row:(typeof fixture.variants)[number])=>({...input,...("set"in row?row.set:{}),...("from"in row?{normalized:input.normalized.map(argument=>argument===row.from?row.to:argument)}:{})}) as Parameters<typeof unitKeyV1>[0];
const named=(value:Parameters<typeof unitKeyV1>[0],extra:Record<string,string>)=>unitKeyComponentsV1(value,extra).components;

test("exactly the changed input moves exactly its component, and the key moves with it",()=>{
 const base=named(input,fixture.extra);
 for(const row of fixture.variants){
  const extra=("extra"in row?row.extra:fixture.extra) as Record<string,string>,changed=named(variantOf(row),extra);
  expect(differingComponentsV1([base,changed]),row.name).toEqual(row.changes);
  if(row.changes[0]!=="platform"&&row.changes[0]!=="roots")expect(unitKeyV1(variantOf(row)),row.name).not.toBe(unitKeyV1(input));
 }
 expect(named(input,fixture.extra)).toEqual(base);
 for(const digest of Object.values(base))expect(digest).toMatch(/^[0-9a-f]{16}$/u);
});

test("every argument class is named and the dependencies stay out of the argument classes",()=>{
 const classes=argumentClassesV1(input.normalized);
 expect([...classes.keys()].sort()).toEqual(["cfg","codegen","edition","features","kind","lints","link","metadata","remap","diagnostics","source","unstable"].sort());
 expect(classes.get("features")).toEqual(['--cfg=feature="std"']);
 expect(JSON.stringify([...classes.values()])).not.toContain("#0123");
});

test("the component record is a valid Lookup of the closed schema",()=>{
 const ajv=new Ajv({strict:false});ajv.addSchema(schema);
 const validate=ajv.getSchema(`${schema.$id}#/definitions/Lookup`)!,made=unitKeyComponentsV1(input,fixture.extra);
 const row={version:1,atMs:1,lane:"d",pid:1,crate:"libc",key:"a".repeat(64),remote:"b".repeat(64),outcome:"miss",components:made.components,values:made.values};
 expect(validate(row),JSON.stringify(validate.errors)).toBe(true);
 expect(validate({...row,components:{...made.components,"bogus":"0".repeat(16)}})).toBe(false);
 expect(validate({...row,components:{...made.components,toolchain:"short"}})).toBe(false);
 expect(validate({...row,extra:1})).toBe(false);
});

test("Python computes the same component digests",()=>{
 expect(python,"an independent Python is required").toBeTruthy();
 const directory=mkdtempSync(join(tmpdir(),"rf-components-")),file=join(directory,"fixture.json");writeFileSync(file,JSON.stringify(fixture));
 const out=spawnSync(python!,["-I",join(import.meta.dir,"🐍️.py"),file],{encoding:"utf8"});rmSync(directory,{recursive:true,force:true});
 expect(out.status,out.stderr).toBe(0);
 expect(JSON.parse(out.stdout)).toEqual({base:named(input,fixture.extra),variants:fixture.variants.map(row=>named(variantOf(row),("extra"in row?row.extra:fixture.extra) as Record<string,string>))});
});
