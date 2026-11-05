import { decodeShellAppCommand } from "@semio-tech/framework/mcp/channel";
import { Database } from "bun:sqlite";
import { test, expect } from "bun:test";
import Ajv from "ajv/dist/2020";
import { OS_MCP_SCHEMA_DOCUMENT, parseActionPrepareInput, parseContextResolveInput } from "../../🧬️schema/🟦️";
import cases from "./🔣️.json";

test("public target and genuine caller context agree with AJV",()=>{
 const shape=OS_MCP_SCHEMA_DOCUMENT.$defs.ActionPrepareInput;
 expect(shape).toBeDefined();
 const valid=new Ajv({strict:false}).compile(shape);
 for(const value of cases.valid) { expect(valid(value)).toBe(true); expect(()=>parseActionPrepareInput(value)).not.toThrow(); }
 for(const value of cases.invalid) { expect(valid(value)).toBe(false); expect(()=>parseActionPrepareInput(value)).toThrow(); }
});

test("target ownership neutral corpus agrees with an independent SQLite relational oracle",()=>{
 const db=new Database(":memory:");
 db.run("CREATE TABLE instances(pluginId TEXT,appId TEXT,instanceId TEXT,artifactRef TEXT,windowIds TEXT)");
 const insert=db.prepare("INSERT INTO instances VALUES (?,?,?,?,?)");
 for(const entry of cases.entries)insert.run(entry.pluginId,entry.appId,entry.instanceId,entry.artifactRef,JSON.stringify(entry.windowIds));
 for(const row of cases.selectionCases){
  const selector=row.target as Record<string,string>;
  const matches=db.query("SELECT instanceId FROM instances WHERE pluginId='test' AND appId='editor' AND (? IS NULL OR instanceId=?) AND (? IS NULL OR pluginId=?) AND (? IS NULL OR appId=?) AND (? IS NULL OR artifactRef=?) AND (? IS NULL OR EXISTS(SELECT 1 FROM json_each(windowIds) WHERE value=?))").all(...["instanceId","pluginId","appId","artifactRef","windowId"].flatMap(key=>[selector[key]??null,selector[key]??null])) as {instanceId:string}[];
  expect(matches.length===1?matches[0].instanceId:null).toBe(row.selected);
 }
 db.close();
});

test("context resolution has no invented language",()=>{
 const valid=new Ajv({strict:false}).compile(OS_MCP_SCHEMA_DOCUMENT.$defs.ContextResolveInput);
 for(const locale of ["en","de"]){expect(valid({locale})).toBe(true);expect(()=>parseContextResolveInput({locale})).not.toThrow();}
 for(const value of [{},{locale:""},{locale:"fr"}]){expect(valid(value)).toBe(false);expect(()=>parseContextResolveInput(value)).toThrow();}
});

test("private command codec preserves genuine locale and terminology without inventing context",()=>{
 for(const row of cases.valid){
  const encoded=new TextEncoder().encode(JSON.stringify({version:1,kind:"pureCommand",capabilityId:row.capabilityId,input:{},context:row.context}));
  const decoded=decodeShellAppCommand(encoded);
  expect(decoded.kind).toBe("pureCommand");
  if(decoded.kind!=="pureCommand")throw new Error("wrong command");
  expect(decoded.context).toEqual(row.context);
 }
 for(const row of cases.invalid.filter(row=>"context" in row)){
  expect(()=>decodeShellAppCommand(new TextEncoder().encode(JSON.stringify({version:1,kind:"pureCommand",capabilityId:row.capabilityId,input:{},context:row.context})))).toThrow();
 }
 const absent=decodeShellAppCommand(new TextEncoder().encode(JSON.stringify({version:1,kind:"pureCommand",capabilityId:"test.command",input:{}})));
 expect("context" in absent).toBe(false);
});
