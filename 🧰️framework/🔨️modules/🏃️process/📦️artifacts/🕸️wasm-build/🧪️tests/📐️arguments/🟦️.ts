import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import corpus from"./🔣️.json"with{type:"json"};
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
/** 📐️ Pure command DATA is independently checked without manufacturing a compiler or paid storage owner. */
test("neutral consumed WASM command plan preserves exact compiler and wasm-pack selections",async()=>{
 const api=await import("../../🟦️.ts"),plan=Reflect.get(api,"wasmCompilerSelectionArgumentsV1"),validate=new Ajv({strict:true}).compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/CompilerSelection"}),database=new Database(":memory:");expect(typeof plan).toBe("function");if(typeof plan!=="function")throw Error("Actual neutral WASM compiler selection owner required");try{for(const row of corpus.cases){expect(validate(row.request)).toBe(true);const actual=plan(row.request)as{cargo:readonly string[];wasmPackCargo:readonly string[]};expect(actual.cargo).toEqual(row.cargo);expect(actual.wasmPackCargo).toEqual(row.wasmPackCargo);expect(database.query("select count(*) count from json_each(?1) where value=?2").get(JSON.stringify(actual.cargo),row.request.selectors[0]!)).toEqual({count:1});expect(Object.isFrozen(actual)&&Object.isFrozen(actual.cargo)).toBe(true);expect(()=>plan({...row.request,selectors:[row.request.selectors[0]!.replace(row.request.mode,row.request.mode==="release"?"prototype":"release")]})).toThrow();expect(()=>plan({...row.request,features:["../unowned"]})).toThrow();}}finally{database.close();}
});
/** 📦️ Package ownership contains only portable source/mode DATA; no escaped checkout projection is accepted. */
test("neutral consumed WASM package owner schema has no obsolete free output path",async()=>{
 const api=await import("../../🟦️.ts"),validate=new Ajv({strict:true}).compile(schema.definitions.PackageOwner);for(const row of corpus.owners){expect(validate(row.value)).toBe(row.accept);if(row.accept)expect(JSON.stringify(api.readWasmPackageOwnerV1(row.value))).toBe(JSON.stringify(row.value));else expect(()=>api.readWasmPackageOwnerV1(row.value)).toThrow();}
});
