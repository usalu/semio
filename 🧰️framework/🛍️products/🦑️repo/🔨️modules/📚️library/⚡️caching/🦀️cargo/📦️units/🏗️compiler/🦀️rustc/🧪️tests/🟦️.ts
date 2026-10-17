/** 🧪️ Rustc: 2 tests covering neutral actual Rust compiler closes CPU and jobserver before consumed bytes under retained readers; neutral Rust compiler resource classes preserve semantic profile flags. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import ts from"typescript";
import fixture from"./🔣️.json";
/** 🔬️ Checks the actual direct compiler owners without running a compiler or manufacturing a storage port. */
test("neutral actual Rust compiler closes CPU and jobserver before consumed bytes under retained readers",()=>{
 const path=new URL("../🟦️.ts",import.meta.url),text=readFileSync(path,"utf8"),source=ts.createSourceFile(path.pathname,text,ts.ScriptTarget.Latest,true),arrows=new Map<string,ts.ArrowFunction>();
 const walk=(node:ts.Node)=>{if(ts.isVariableDeclaration(node)&&ts.isIdentifier(node.name)&&node.initializer&&ts.isArrowFunction(node.initializer))arrows.set(node.name.text,node.initializer);ts.forEachChild(node,walk);};walk(source);
 expect(text).toContain(fixture.source.consumer);expect(text).toContain(fixture.source.policy);expect(text).toContain(fixture.source.jobserver);
 const compile=arrows.get(fixture.source.compile),consume=arrows.get(fixture.source.consume);if(!compile||!consume)throw Error("Actual CPU-only compiler and retained consumer owners required");
 const cpu=compile.getText(source),after=consume.getText(source);expect(cpu).toContain("command.cfgArguments");expect(cpu).toContain("command.arguments");expect(cpu).toContain("command.cwd");expect(cpu).toContain("command.program");expect(cpu).toContain("jobs.environment()");expect(cpu).toContain("jobs.acquire(");expect(cpu).toContain("jobs.close()");expect(cpu).not.toContain("readPhysicalCargoSourceV1");expect(cpu).not.toContain("readRustCompilerConsumedBytesV1");expect(after).toContain("withRequestedIssuedResourceControlV1(control,0");expect(after).toContain("readPhysicalCargoSourceV1");expect(after).toContain("readRustCompilerConsumedBytesV1");expect(after).not.toContain("jobs.environment");expect(text).toContain("return()=>consumeCompleted()");expect(text).toContain("withReader(control,");expect(text).toContain("withRustCompilerStateOperationV1(");expect(text).not.toContain('SEMIO_COMPILER_PARALLELISM:"1"');
});

import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{rustCompilerCapacityRequestV1}from"../🎟️capacity/🟦️.ts";
import capacitySchema from"../🎟️capacity/🧬️schema/🔣️.json";
/** 🧾️ Compares closed profile resource classes with independent relational arithmetic. */
test("neutral Rust compiler resource classes preserve semantic profile flags",()=>{const validate=new Ajv({strict:true}).compile(capacitySchema),db=new Database(":memory:");try{for(const row of fixture.profiles){const profile={codegen_backend:row.codegen_backend,codegen_units:row.codegen_units,lto:row.lto},before=JSON.stringify(profile);expect(validate(profile)).toBe(true);const result=rustCompilerCapacityRequestV1(profile),oracle=db.query("select case when ?1 is null and ?2 > 1 and ?3 in ('false','off') then 'jobserver-codegen' else 'whole-host-backend' end as resourceClass").get(profile.codegen_backend,profile.codegen_units,profile.lto)as{resourceClass:string};expect(result.resourceClass).toBe(oracle.resourceClass);expect(result as unknown).toEqual({resourceClass:row.resourceClass,request:row.request});expect(JSON.stringify(profile)).toBe(before);expect(Object.isFrozen(result.request)).toBe(true);}}finally{db.close();}});
