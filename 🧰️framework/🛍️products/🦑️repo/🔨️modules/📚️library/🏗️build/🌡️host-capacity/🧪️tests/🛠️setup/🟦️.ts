import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import corpus from"./📥️.json" with{type:"json"};
import schema from"./🔣️.json" with{type:"json"};
import{buildCapacityAskV1}from"../../🟦️.ts";

test("selective tools setup requests ordinary host capacity while tool queries stay unreserved",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const db=new Database(":memory:");try{for(const row of corpus.cases){const oracle=db.query("select case when not exists(select 1 from json_each(?1) where value in ('--help','-h')) and (json_extract(?1,'$[0]')='preflight' or (json_extract(?1,'$[0]')='tools' and json_extract(?1,'$[1]') in ('exec','setup'))) then 'command' else null end operation").get(JSON.stringify(row.args)) as{operation:string|null};expect(oracle.operation).toBe(row.operation);expect(buildCapacityAskV1(row.args,{})?.operation??null).toBe(oracle.operation);}}finally{db.close();}
});
