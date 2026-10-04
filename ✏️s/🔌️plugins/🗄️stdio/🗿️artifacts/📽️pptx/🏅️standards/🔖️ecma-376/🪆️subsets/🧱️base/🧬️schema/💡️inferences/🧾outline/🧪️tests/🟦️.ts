import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import * as outlineOwner from "../🟦️.ts";
import pptxSchema from "../../../📸️snapshot/🔣️.json";
import xmlSchema from "../../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json";
test("PPTX outline reads actual OPC XML and refuses missing retained authority",async()=>{
 const fixture=await Bun.file(new URL("../🧫️fixtures/🔣️.json",import.meta.url)).json();
 const schema=await Bun.file(new URL("../🧫️fixtures/🧬️schema/🔣️.json",import.meta.url)).json();
 expect(new Ajv({strict:false}).addSchema(xmlSchema).addSchema(pptxSchema).validate(schema,fixture)).toBe(true);
 const compute=(outlineOwner as unknown as{computePptxOutline?:(snapshot:unknown)=>unknown}).computePptxOutline;expect(compute).toBeTypeOf("function");
 const sql=new Database(":memory:");try{
  sql.exec("CREATE TABLE nodes(id INTEGER PRIMARY KEY,part TEXT,parent INTEGER,name TEXT,text TEXT);CREATE TABLE attrs(owner INTEGER,name TEXT,value TEXT);CREATE TABLE rels(owner TEXT,id TEXT,kind TEXT,target TEXT)");
  let next=0;const walk=(part:string,node:any,parent:number|null)=>{const id=++next;sql.query("INSERT INTO nodes VALUES(?,?,?,?,?)").run(id,part,parent,node.name??"",node.text??"");for(const attr of node.attrs??[])sql.query("INSERT INTO attrs VALUES(?,?,?)").run(id,attr.name,attr.value);for(const child of node.children??[])walk(part,child,id);};
  for(const part of fixture.snapshot.xmlParts)walk(part.path,part.document.root,null);
  for(const[owner,relations]of Object.entries(fixture.snapshot.opc.relationships))for(const relation of relations as any[])sql.query("INSERT INTO rels VALUES(?,?,?,?)").run(owner,relation.id,relation.relType,relation.target);
  expect(sql.query("SELECT count(*) AS n FROM rels WHERE owner='' AND kind LIKE '%/officeDocument'").get()).toEqual({n:1});
  expect(sql.query("SELECT count(*) AS n FROM nodes WHERE name='p:sld' AND parent IS NULL").get()).toEqual({n:fixture.expected.slideCount});
  expect(sql.query("SELECT count(*) AS n FROM nodes n JOIN nodes p ON p.id=n.parent WHERE p.name='p:spTree' AND n.name IN('p:sp','p:pic')").get()).toEqual({n:fixture.expected.shapeCount});
  expect(sql.query("SELECT sum(length(trim(text))-length(replace(trim(text),' ',''))+1) AS n FROM nodes WHERE text<>''").get()).toEqual({n:fixture.expected.wordCount});
  expect(compute!(fixture.snapshot)).toEqual(fixture.expected);
  for(const refusal of fixture.refusals){const snapshot=structuredClone(fixture.snapshot);if(refusal.role==="missingRelationship")snapshot.opc.relationships={};else if(refusal.role==="missingXmlPart")snapshot.xmlParts.pop();else snapshot.xmlParts[0].document.root.attrs[0].value="foreign";expect(()=>compute!(snapshot)).toThrow();try{compute!(snapshot);}catch(error){expect(error).toHaveProperty("kind",refusal.kind);}}
 }finally{sql.close();}
});
