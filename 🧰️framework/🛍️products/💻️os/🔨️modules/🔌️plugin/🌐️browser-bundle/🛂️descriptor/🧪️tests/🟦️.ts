import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {types} from "@bytecodealliance/jco";
import ts from "typescript";
import corpus from "../🧫️fixtures/🔣️.json" with {type:"json"};
import {ACTOR_COMPONENT_EXPORTS,parseActorComponentExports} from "../🟦️.ts";

test("actor exports derive from the owned WIT and independent JCO types",async()=>{
 const wit=fileURLToPath(new URL("../../../🧬️schema/📜️.wit",import.meta.url));
 const generated=await types(wit,{name:"actor-contract",worldName:"actor",instantiation:"async",asyncMode:"jspi"});
 const parsed=ts.createSourceFile("actor.d.ts",new TextDecoder().decode(generated["actor-contract.d.ts"]!),ts.ScriptTarget.Latest,true);
 const actor=parsed.statements.find((node)=>ts.isInterfaceDeclaration(node)&&node.name.text==="Actor") as ts.InterfaceDeclaration;
 const expected:Record<string,string[]>={};
 for(const member of actor.members){
  const name=member.name!.getText(parsed);if(!/^[a-z]+$/.test(name))continue;
  const source=ts.createSourceFile(name+".ts",new TextDecoder().decode(generated["interfaces/semio-framework-"+name+".d.ts"]!),ts.ScriptTarget.Latest,true);
  expected[name]=source.statements.filter(ts.isFunctionDeclaration).map(node=>node.name!.text);
 }
 expect(expected).toEqual(corpus.expected);
 expect(ACTOR_COMPONENT_EXPORTS).toEqual(expected);
 expect(parseActorComponentExports(readFileSync(wit,"utf8"))).toEqual(expected);
 console.log("[DEBUG] actor contract: "+Object.keys(expected).length+" interfaces, JCO and native projection agree");
});
for(const row of corpus.cases)test("actor WIT projection "+Object.keys(row.expected).join(","),()=>expect(parseActorComponentExports(row.source)).toEqual(row.expected));
