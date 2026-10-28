import {expect,test} from "bun:test";
import {readFile} from "node:fs/promises";
import Ajv from "ajv/dist/2020.js";
import cases from "../../../../../../../../📇️registry/🧬️contract/📐️part21/🔤️lexer-and-parse/alphabets/cases.json" with {type:"json"};
import data from "../../../../../../../../📇️registry/🧬️contract/📐️part21/🔤️lexer-and-parse/alphabets/data.json" with {type:"json"};
import schema from "../../../../../../../../📇️registry/🧬️contract/📐️part21/🔤️lexer-and-parse/alphabets/schema.json" with {type:"json"};
import {parseIfc4x3Step} from "../step.ts";

const envelope=(first:string,second="")=>`ISO-10303-21;HEADER;FILE_DESCRIPTION((''),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;#1=IFCLABEL('${first}'${second});ENDSEC;END-ISO-10303-21;`;

test("all nine official alphabet pages obey the closed schema and independent ICU decoder",()=>{
 expect(new Ajv({strict:true}).validate(schema,data)).toBe(true);expect(data.pages.map(page=>page.selector).join("")).toBe("ABCDEFGHI");let defined=0,undefinedCodes=0;
 for(const page of data.pages)for(let index=0;index<95;index++){const scalar=page.scalars[index],bytes=Uint8Array.of(index+160);if(scalar===null){expect(()=>new TextDecoder(page.encoding,{fatal:true}).decode(bytes)).toThrow();undefinedCodes++;}else{expect(new TextDecoder(page.encoding,{fatal:true}).decode(bytes)).toBe(String.fromCodePoint(scalar!));defined++;}}
 expect(defined+undefinedCodes).toBe(855);expect(undefinedCodes).toBeGreaterThan(0);
});
test.each(data.pages)("the real STEP reader consumes all positions in alphabet $selector",async page=>{
 const rows=cases.cases.filter(row=>row.name.startsWith(page.selector+"-"));expect(cases.cases).toHaveLength(855);expect(rows).toHaveLength(95);for(const row of rows){if(row.expected===null){await expect(parseIfc4x3Step(envelope(row.source))).rejects.toHaveProperty("code","step.string");continue;}const graph=await parseIfc4x3Step(envelope(row.source));expect(graph.entities[0]!.args[0]).toEqual({kind:"string",value:row.expected});}
});
test("alphabet selection resets per string and undefined positions refuse",async()=>{
 const graph=await parseIfc4x3Step(envelope("\\PB\\\\S\\!",",'\\S\\!'") );expect(graph.entities[0]!.args).toEqual([{kind:"string",value:"Ą"},{kind:"string",value:"¡"}]);for(const payload of ["\u007f","\u0080","é"])await expect(parseIfc4x3Step(envelope("\\S\\"+payload))).rejects.toHaveProperty("code","step.string");
});
test("native compiled literals and neutral payload coverage equal every owned declaration",async()=>{
 const source=await readFile(new URL("../../../../../../../../📇️registry/🧬️contract/📐️part21/🔤️lexer-and-parse/alphabets/data.rs",import.meta.url),"utf8"),body=source.split("= [")[1]!;expect(body).toBeDefined();const tokens=body.match(/0x[0-9A-F]+|\b0\b/g)!;expect(tokens).toHaveLength(855);expect(body.replace(/0x[0-9A-F]+|\b0\b/g,"")).toMatch(/^[\s,\[\];]+$/);
 expect(tokens.map(token=>Number(token))).toEqual(data.pages.flatMap(page=>page.scalars.map(value=>value??0)));for(let page=0;page<9;page++)for(let payload=32;payload<=126;payload++){const row=cases.cases[page*95+payload-32]!;expect(row.source).toBe(`\\P${String.fromCharCode(65+page)}\\\\S\\${String.fromCharCode(payload)}`);expect(row.expected).toBe(data.pages[page]!.scalars[payload-32]===null?null:String.fromCodePoint(data.pages[page]!.scalars[payload-32]!));}
});
