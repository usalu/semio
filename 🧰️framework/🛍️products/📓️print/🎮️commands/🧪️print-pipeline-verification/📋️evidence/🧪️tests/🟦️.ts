import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{posix}from"node:path";
import{printPipelineEvidenceV1}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import cases from"./🔣️.json"with{type:"json"};
test("Print evidence finite names and bytes agree with Ajv SQLite and platform paths",()=>{const validate=new Ajv({strict:true}).compile(schema),database=new Database(":memory:");try{database.exec("create table files(name text primary key,bytes integer)");for(const row of cases.valid){expect(validate(row)).toBe(true);expect(JSON.stringify(printPipelineEvidenceV1(row))).toBe(JSON.stringify(row));expect(posix.normalize(row.name)).toBe(row.name);database.query("insert into files values(?,?)").run(row.name,row.bytes);}expect(database.query<{total:number},[]>("select sum(bytes) total from files").get()?.total).toBe(cases.valid.reduce((sum,row)=>sum+row.bytes,0));for(const row of cases.invalid)expect(()=>printPipelineEvidenceV1(row)).toThrow();}finally{database.close();}});
