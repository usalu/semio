import{test,expect}from"bun:test";
import Ajv from"ajv";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{parse}from"jsonc-parser";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};

const validate=new Ajv({strict:true,allowUnionTypes:true}).compile(schema.definitions.Environment);
test("editor runtime startup cache environment agrees with neutral Ajv and SQLite corpus",()=>{
 const database=new Database(":memory:");try{const oracle=database.query("select coalesce(json_extract(?1,'$.BUN_RUNTIME_TRANSPILER_CACHE_PATH')='0' and json_extract(?1,'$.NODE_DISABLE_COMPILE_CACHE')='1' and not exists(select 1 from json_each(?1) where type not in ('text','null')),0) accepted");for(const row of corpus.cases){expect(validate(row.environment)).toBe(row.accepted);expect(validateJsonSchemaSubset(schema.definitions.Environment,row.environment,schema).length===0).toBe(row.accepted);expect(Boolean((oracle.get(JSON.stringify(row.environment))as{accepted:number}).accepted)).toBe(row.accepted);}}finally{database.close();}
});
test("editor executable catalogs disable implicit caches before runtime startup",()=>{
 const database=new Database(":memory:");try{database.exec("create table launches(catalog text,name text,environment text)");const insert=database.query("insert into launches values(?,?,?)");let count=0;for(const file of schema.definitions.Protocol.const.catalogs){const errors:import("jsonc-parser").ParseError[]=[],document=parse(readFileSync(join(process.cwd(),file),"utf8"),errors)as{configurations:unknown[]};expect(errors).toEqual([]);let selected=0;for(const value of document.configurations){if(!value||typeof value!=="object")continue;const row=value as Record<string,unknown>;if(row.type!==schema.definitions.Protocol.const.type||row.request!==schema.definitions.Protocol.const.request||typeof row.command!=="string"&&typeof row.runtimeExecutable!=="string")continue;insert.run(file,String(row.name),JSON.stringify(row.env??{}));selected++;}expect(selected).toBeGreaterThan(0);count+=selected;}const invalid=database.query("select catalog,name from launches where coalesce(json_extract(environment,'$.BUN_RUNTIME_TRANSPILER_CACHE_PATH')='0' and json_extract(environment,'$.NODE_DISABLE_COMPILE_CACHE')='1',0)=0").all();expect(invalid.length).toBe(0);for(const row of database.query("select environment from launches").all()as{environment:string}[])expect(validate(JSON.parse(row.environment))).toBe(true);}finally{database.close();}
});
