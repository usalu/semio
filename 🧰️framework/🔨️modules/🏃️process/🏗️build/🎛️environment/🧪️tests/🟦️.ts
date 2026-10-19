import{test,expect}from"bun:test";
import Ajv from"ajv";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{typeScriptCheckEnvironmentV1}from"../../🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};

const validate=new Ajv({strict:true,allowUnionTypes:true}).compile(schema.definitions.Environment);
test("owned runtime startup cache environment agrees with neutral Ajv and SQLite corpus",()=>{
 const database=new Database(":memory:");try{const oracle=database.query("select coalesce(json_extract(?1,'$.BUN_RUNTIME_TRANSPILER_CACHE_PATH')='0' and json_extract(?1,'$.NODE_DISABLE_COMPILE_CACHE')='1' and not exists(select 1 from json_each(?1) where type not in ('text','null')),0) accepted");for(const row of corpus.cases){expect(validate(row.environment)).toBe(row.accepted);expect(validateJsonSchemaSubset(schema.definitions.Environment,row.environment,schema).length===0).toBe(row.accepted);expect(Boolean((oracle.get(JSON.stringify(row.environment))as{accepted:number}).accepted)).toBe(row.accepted);}}finally{database.close();}
});
test("dashboard runtime environment verification has no IDE catalog authority",()=>{
 expect(Object.keys(schema.definitions)).toEqual(["Environment"]);
 const root=process.cwd(),owner=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🏃️process/📋️project.json"),"utf8"));
 expect(owner.targets["test-runtime-environment"]).toMatchObject({executor:"repo:script",options:{cwd:"🧰️framework/🔨️modules/🏃️process",command:"bun ./📜️script.ts test runtime-environment"}});
 const route=readFileSync(join(root,"🧰️framework/🔨️modules/🏃️process/📜️script.ts"),"utf8");expect(route).toContain('"runtime-environment": { source: "🏗️build/🎛️environment/🧪️tests/🟦️.ts"');
});
test("actual compiler environment disables implicit caches for every dashboard build mode",()=>{
 const database=new Database(":memory:");try{
  const oracle=database.query("select json_set(?1,'$.BUN_RUNTIME_TRANSPILER_CACHE_PATH','0','$.NODE_DISABLE_COMPILE_CACHE','1','$.SEMIO_BUILD_MODE',?2) environment");
  for(const mode of ["prototype","release"]as const)for(const row of corpus.cases){
   const inherited={...row.environment,NODE_COMPILE_CACHE:"unowned-cache",CUSTOM:"retained"}as Record<string,string|undefined>,before={...inherited},actual=typeScriptCheckEnvironmentV1(mode,inherited),expected=JSON.parse((oracle.get(JSON.stringify(inherited),mode)as{environment:string}).environment);delete expected.NODE_COMPILE_CACHE;
   expect(actual).toEqual(expected);expect(actual).not.toHaveProperty("NODE_COMPILE_CACHE");expect(inherited).toEqual(before);expect(Object.isFrozen(actual)).toBe(true);
   expect(validate(actual)).toBe(!Object.values(actual).some(value=>value!==null&&typeof value!=="string"));
   expect(validateJsonSchemaSubset(schema.definitions.Environment,actual,schema).length===0).toBe(validate(actual));
  }
  console.log(`[DEBUG] Owned compiler startup verified ${corpus.cases.length*2} runtime environments against SQLite and Ajv without IDE catalogs`);
 }finally{database.close();}
});
