import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import ts from"typescript";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import corpus from"./🔣️.json";
import{requireBrowserGeometrySessionV1}from"../../🟦️.ts";
import{BrowserSession}from"../../📦️packages/🦀️rust/🕸️bindings/semio_session.js";
/** 🧬️ Current first-party method contract refuses incomplete generated implementations without adopting a fallback. */
test("neutral geometry binding admission requires every current method before use",()=>{
 const validate=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["methods"],properties:{methods:{type:"array",minItems:11,maxItems:11,uniqueItems:true,items:{type:"string"}}}}),db=new Database(":memory:");
 try{expect(validate(corpus)).toBe(true);db.exec("create table methods(name text primary key)");const current=Object.fromEntries(corpus.methods.map(name=>[name,()=>{}]));expect(Object.is(requireBrowserGeometrySessionV1(current),current)).toBe(true);for(const name of corpus.methods){db.query("insert into methods values(?)").run(name);const incomplete={...current};delete incomplete[name];expect(()=>requireBrowserGeometrySessionV1(incomplete)).toThrow(/regenerate/);}expect(db.query("select count(*) count from methods").get()).toEqual({count:11});const source=ts.createSourceFile("binding.ts",readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true),contract=source.statements.find(node=>ts.isInterfaceDeclaration(node)&&node.name.text==="BrowserGeometrySession");if(!contract||!ts.isInterfaceDeclaration(contract))throw Error("Actual geometry interface required");expect(contract.members.map(member=>member.name?.getText(source)).sort()).toEqual([...corpus.methods].sort());const generated=BrowserSession.prototype;if(corpus.methods.every(name=>typeof Reflect.get(generated,name)==="function"))expect(Object.is(requireBrowserGeometrySessionV1(generated),generated)).toBe(true);else expect(()=>requireBrowserGeometrySessionV1(generated)).toThrow(/current-browser-binding-required/);}finally{db.close();}
});
