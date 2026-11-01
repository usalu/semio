import {test,expect} from "bun:test";
import {readFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import Ajv from "ajv";
import schema from "./ast.schema.json" with {type:"json"};
import cases from "./native-cases.json" with {type:"json"};
import {compileIfc4x3Express} from "./compiler.ts";
import {decodeJsonPack,encodeJsonPack} from "../../../../../../📚️catalog/🛠️extract/📦️json-pack.ts";

test("native canonical program has complete schema-admitted corpus and current source digest",async()=>{
 const compilation=await compileIfc4x3Express();expect(compilation.diagnostics).toEqual([]);
 const admit=new Ajv({strict:true,allErrors:true}).compile(schema);expect(admit(compilation.program)).toBe(true);
 for(const [name,count] of Object.entries(cases.compiled))expect(compilation.program[name as keyof typeof cases.compiled].length).toBe(count);
 expect(compilation.program.globalRules.map(rule=>rule.name).toSorted()).toEqual(cases.globalRules.toSorted());
 const json=JSON.stringify(compilation.program),digest=createHash("sha256").update(json).digest("hex"),pack=new Uint8Array(await readFile(new URL("🌳️express/program.ifcj",import.meta.url)));
 expect(createHash("sha256").update(JSON.stringify(decodeJsonPack(pack))).digest("hex")).toBe(digest);
 expect(Buffer.from(encodeJsonPack(JSON.parse(json)))).toEqual(Buffer.from(pack));
 expect(await readFile(new URL("🌳️express/program.json",import.meta.url),"utf8")).toBe(json+"\n");
});

test("retained population reports cannot advertise incomplete validation as valid",async()=>{
 const {default:Ajv2020}=await import("ajv"),schema=await Bun.file(new URL("machine.json",import.meta.url)).json(),contract=await Bun.file(new URL("machine-terminal.json",import.meta.url)).json();
 const admit=new Ajv2020({strict:true,allErrors:true}).compile(schema);
 expect(contract.nativeExecution).toBe(false);
 for(const complete of [false,true])for(const valid of [false,true])expect(admit({schema:"IFC4X3_ADD2",complete,valid,results:[],diagnostics:[]})).toBe(complete||!valid);
 for(const row of contract.cases.filter((row:{diagnostics?:number})=>row.diagnostics!==undefined)){
  const diagnostic={owner:null,rule:null,kind:"admission",outcome:"incomplete",code:row.code};
  expect(admit({schema:contract.schema,complete:row.complete,valid:row.valid,results:[],diagnostics:[diagnostic]})).toBe(true);
  for(const lane of ["results","diagnostics"])expect(admit({schema:contract.schema,complete:true,valid:true,results:[],diagnostics:[],[lane]:[diagnostic]})).toBe(false);
 }
});
