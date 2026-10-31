import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "../../../../🔌️ports/🧰️native-tools/🧬️schema/🔣️.json";
test("native fixture profiles require only their declared qualified tools",async()=>{
 const ports=await import("../🟨️.mjs");expect(typeof ports.readNativeFixtureV1).toBe("function");const database=new Database(":memory:"),validate=new Ajv({strict:true}).compile(schema);
 try{for(const row of corpus){
  const value=row.value,absolute=(path:string|null)=>path!==null&&!path.includes("\0")&&(row.platform==="win32"?(/^[A-Za-z]:/u.test(path)&&[47,92].includes(path.charCodeAt(2))||path.charCodeAt(0)===92&&path.charCodeAt(1)===92):path.startsWith("/")),oracle=database.query("select ?1 AND ?2 AND ((?3='lifecycle' AND ?4 IS NULL) OR (?3='execution' AND ?5)) accepted").get(Number(validate(value)),Number(absolute(value.watcher)),value.profile,value.executionTool,Number(absolute(value.executionTool))) as {accepted:number};
  expect(Boolean(oracle.accepted),row.name).toBe(row.accepted);if(row.accepted)expect(ports.readNativeFixtureV1(value,row.platform as "linux"|"darwin"|"win32")).toEqual(value);else expect(()=>ports.readNativeFixtureV1(value,row.platform as "linux"|"darwin"|"win32")).toThrow();
 }}finally{database.close();}
});

test("lifecycle profile refuses execution instead of manufacturing an execution tool",async()=>{const {createLifecycleExecutionPortV1}=await import("../../🔌️ports/🟨️.mjs");await expect(createLifecycleExecutionPortV1().run()).rejects.toThrow("Lifecycle fixture cannot execute tasks");});
