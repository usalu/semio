/** 🧫️ Complete OPC/XML owned state checked with independent SQLite and schema admission. */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import fixture from"../../🧫️fixtures/🪶️sqlite/🔣️.json";
import schema from"../../🔣️.json";
import xmlSchema from"../../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json";
import{XLSX_SQLITE_SCHEMA,xlsxSnapshotToSqliteDatabase,xlsxSnapshotFromSqliteDatabase}from"../../../../../../../../🟦️.ts";
import type{OpcPackage,XlsxSnapshot}from"../../../../../../../../🟦️.ts";
import{exportSqliteDatabase,importSqliteDatabase}from"@semio-tech/framework";
import{parseXmlDocument}from"../../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import{validateXlsxSnapshotProfile}from"../../../../../../../../🟦️.ts";
import profiles from"../../🧫️fixtures/🛡️profile/🔣️.json";
import profileSchema from"../../🧫️fixtures/🛡️profile/🧬️schema/🔣️.json";
const input={...fixture,xmlParts:fixture.xmlParts.map(part=>({...part,document:parseXmlDocument(part.document)}))} as unknown as XlsxSnapshot;
function profileSnapshot(plan:typeof profiles.cases[number]):XlsxSnapshot{
 const attrs=[{name:"xmlns",value:plan.xmlns},{name:"xmlns:r",value:plan.relationshipsNamespace},...(plan.conformance===null?[]:[{name:"conformance",value:plan.conformance}])];
 return{schema:"arbitrary owned profile",opc:{parts:plan.vml?[{path:"drawing.vml",contentType:"application/vnd.openxmlformats-officedocument.vmlDrawing",bytes:[]}]:[],contentTypes:{defaults:[],overrides:[]},relationships:{"":[{id:"main",relType:"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",target:"xl/workbook.xml",targetMode:"internal"}],"xl/workbook.xml":plan.missingWorksheet?[{id:"sheet",relType:"http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",target:"../missing.xml",targetMode:"internal"}]:[]},comment:""},xmlParts:[{path:"xl/workbook.xml",contentType:"application/xml",document:parseXmlDocument({root:{kind:"element",name:"workbook",attrs,children:[]},prolog:[],epilog:[]})}]};
}
test("XLSX exact profile policies match independently queried namespaces and relationships",async()=>{
 expect(new Ajv2020({strict:true}).validate(profileSchema,profiles)).toBe(true);
 for(const plan of profiles.cases){const snapshot=profileSnapshot(plan);const diagnostics=await validateXlsxSnapshotProfile(snapshot,plan.subset as "strict"|"transitional");expect(diagnostics.map(d=>d.code)).toEqual(plan.expected.map(code=>`stdio.xlsx.${plan.subset}.${code}`));expect(diagnostics.map(d=>d.severity)).toEqual(plan.expected.map(code=>code==="worksheet-content-type-missing"?"warning":"error"));
 const sql=Database.deserialize(await exportSqliteDatabase(await xlsxSnapshotToSqliteDatabase(snapshot)));try{expect(sql.query("SELECT name,value FROM xlsx_xml_attribute ORDER BY ordinal").all()).toEqual(snapshot.xmlParts[0]!.document.root?.kind==="element"?snapshot.xmlParts[0]!.document.root.attrs.map(attr=>({name:attr.name,value:attr.value})):[]);expect(sql.query("SELECT count(*) AS n FROM xlsx_relationship WHERE target='../missing.xml'").get()).toEqual({n:plan.missingWorksheet?1:0});}finally{sql.close();}}
});
test("XLSX exact profile diagnostics expose long-field interior cancellation and caller bounds",async()=>{
 const snapshot=profileSnapshot(profiles.cases[0]!);if(snapshot.xmlParts[0]!.document.root?.kind!=="element")throw new Error("workbook");snapshot.xmlParts[0]!.document.root.attrs[0]!.value="x".repeat(profiles.largeTextCharacters);
 const controller=new AbortController();let reached=false;await expect(validateXlsxSnapshotProfile(snapshot,"strict",{signal:controller.signal,onProgress:event=>{if(event.total>=profiles.largeTextCharacters&&event.completed>=profiles.cancelAfter&&event.completed<event.total){reached=true;controller.abort();}}})).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);await expect(validateXlsxSnapshotProfile(snapshot,"strict",{maxValueBytes:profiles.smallValueBudget})).rejects.toThrow();
});
test("XLSX all owned OPC/XML fields expose independent relational identities",async()=>{
 const ajv=new Ajv({strict:false}).addSchema(xmlSchema);const admitted=ajv.validate(schema,fixture);expect(admitted,JSON.stringify(ajv.errors)).toBe(true);expect(XLSX_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text());
 const db=Database.deserialize(await exportSqliteDatabase(await xlsxSnapshotToSqliteDatabase(input)));try{
 expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT ordinal,path,content_type,data FROM xlsx_binary_part ORDER BY ordinal").all()).toEqual(input.opc.parts.map((part,ordinal)=>({ordinal,path:part.path,content_type:part.contentType,data:new Uint8Array(part.bytes)})));expect(db.query("SELECT owner_path FROM xlsx_relationship_owner ORDER BY owner_path").all()).toEqual(Object.keys(input.opc.relationships).sort().map(owner_path=>({owner_path})));expect(await xlsxSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(input);
 db.query("UPDATE xlsx_xml_text SET text=? WHERE node_id=(SELECT node_id FROM xlsx_xml_document_misc WHERE xml_document_id=1 AND position='prolog' AND ordinal=0)").run("independent SQL 世界");const restored=await xlsxSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));expect(restored.xmlParts[0]!.document.prolog[0]).toEqual({kind:"text",text:"independent SQL 世界"});expect(restored.opc).toEqual(input.opc);
 }finally{db.close();}
});
test("XLSX aggregate domain rows include package and XML ownership exactly",async()=>{const db=await xlsxSnapshotToSqliteDatabase(input);const count=db.tables.reduce((total,table)=>total+table.rows.length,0);expect(await xlsxSnapshotFromSqliteDatabase(await xlsxSnapshotToSqliteDatabase(input,{maxRows:count}),{maxRows:count})).toEqual(input);await expect(xlsxSnapshotToSqliteDatabase(input,{maxRows:count-1})).rejects.toThrow();});

test("XLSX dangling references and shared OPC/XML owners are refused",async()=>{const database=await xlsxSnapshotToSqliteDatabase(input);for(const[name,row,column,value]of[["xlsx_document",0,2,2n],["xlsx_xml_part",1,4,1n],["xlsx_relationship",0,1,99n],["xlsx_default_content_type",0,2,99n],["xlsx_relationship_owner",1,2,""]]as const){const edited={tables:database.tables.map(table=>table.name!==name?table:{...table,rows:table.rows.map((entry,index)=>index!==row?entry:{...entry,values:entry.values.map((cell,at)=>at===column?value:cell)})})};await expect(xlsxSnapshotFromSqliteDatabase(edited),name).rejects.toThrow();}});
test("XLSX intrinsic large-part copies have interior ownership cancellation",async()=>{const snapshot=structuredClone(input);snapshot.opc.parts[0]!.bytes=new Array(100000).fill(0x97);const database=await xlsxSnapshotToSqliteDatabase(snapshot);for(const phase of["projectSnapshot","reconstructSnapshot"]as const){const controller=new AbortController();let reached=false;const options={signal:controller.signal,onProgress:(event:{phase:string;completed:number;total:number})=>{if(event.phase===phase&&event.total===100000&&event.completed>=65536&&event.completed<event.total){reached=true;controller.abort();}}};await expect(phase==="projectSnapshot"?xlsxSnapshotToSqliteDatabase(snapshot,options):xlsxSnapshotFromSqliteDatabase(database,options)).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);}});

import opcNative from "../../../../../../../../../🎒️zip/📦️opc/🧩️native/🧫️fixtures/🔣️.json";
import opcNativeSchema from "../../../../../../../../../🎒️zip/📦️opc/🧩️native/🧫️fixtures/🧬️schema/🔣️.json";
import Ajv2020 from "ajv/dist/2020";
import {Buffer}from"node:buffer";
import nativeLiterals from"../../🧫️fixtures/🧩️native-literals/🔣️.json";
import nativeLiteralsSchema from"../../🧫️fixtures/🧩️native-literals/🧬️schema/🔣️.json";
import artifactDefinition from"../../../../../../../../📜️artifact-definition.json";
import{createHash}from"node:crypto";
test("XLSX declared factory identity matches its explicit literal protocol",async()=>{
 const protocol=await Bun.file(new URL("../../💾️binary/📡️.protocol.semio",import.meta.url)).arrayBuffer();const hash=createHash("sha256").update(Buffer.from(protocol)).digest("hex");expect(new Bun.CryptoHasher("sha256").update(protocol).digest("hex")).toBe(hash);expect(artifactDefinition.codecs[0]!.native_factory.pack_schema_hash).toBe(hash);
});
test("XLSX complete native literal fixture has independent envelope and empty package ownership",async()=>{
 expect(new Ajv2020({strict:true}).validate(nativeLiteralsSchema,nativeLiterals)).toBe(true);expect(new Ajv({strict:false}).addSchema(xmlSchema).validate(schema,nativeLiterals.snapshot)).toBe(true);
 const text=`semio stdio.xlsx.dsl v1\n[${Buffer.from(nativeLiterals.snapshot.schema).toString("hex")},[[],[],[],[],],[]]`;expect(text).toBe(nativeLiterals.text);const token=Buffer.from("stdio.xlsx.pack v1");const width=Buffer.alloc(4);width.writeUInt32LE(token.length);const binary=Buffer.concat([Buffer.from([137,83,69,77,13,10,26,10]),width,token,Buffer.from([1,Buffer.byteLength(nativeLiterals.snapshot.schema)]),Buffer.from(nativeLiterals.snapshot.schema),Buffer.alloc(6)]);expect([...binary]).toEqual(nativeLiterals.binary);
 const database=await xlsxSnapshotToSqliteDatabase(nativeLiterals.snapshot as unknown as XlsxSnapshot,{maxRows:nativeLiterals.rows});expect(database.tables.reduce((sum,table)=>sum+table.rows.length,0)).toBe(nativeLiterals.rows);const sql=Database.deserialize(await exportSqliteDatabase(database));try{expect(sql.query("SELECT schema FROM xlsx_document").get()).toEqual({schema:nativeLiterals.snapshot.schema});expect(sql.query("SELECT comment FROM xlsx_package").get()).toEqual({comment:""});}finally{sql.close();}
});
test("OPC native component fixture exposes literal duplicated defaults and independent octets",async()=>{
 expect(new Ajv2020({strict:true}).validate(opcNativeSchema,opcNative)).toBe(true);
 const p=opcNative.package as unknown as OpcPackage;const hex=(text:string)=>Buffer.from(text,"utf8").toString("hex");const list=(items:readonly string[])=>`[${items.join(",")}]`;const text=list([list(p.parts.map(part=>list([hex(part.path),hex(part.contentType),Buffer.from(part.bytes).toString("hex")]))),list(p.contentTypes.defaults.map(([key,value])=>list([hex(key!),hex(value!)]))),list(p.contentTypes.overrides.map(([key,value])=>list([hex(key!),hex(value!)]))),list(Object.entries(p.relationships).map(([owner,relationships])=>list([hex(owner),list(relationships.map(rel=>list([hex(rel.id),hex(rel.relType),hex(rel.target),rel.targetMode==="external"?"1":"0"])))]))),hex(p.comment)]);expect(text).toBe(opcNative.text);
 const bytes:number[]=[];const scalar=(input:Uint8Array)=>{expect(input.length).toBeLessThan(128);bytes.push(input.length,...input);};bytes.push(p.parts.length);for(const part of p.parts){scalar(Buffer.from(part.path));scalar(Buffer.from(part.contentType));scalar(Buffer.from(part.bytes));}for(const pairs of[p.contentTypes.defaults,p.contentTypes.overrides]){bytes.push(pairs.length);for(const[key,value]of pairs){scalar(Buffer.from(key!));scalar(Buffer.from(value!));}}bytes.push(Object.keys(p.relationships).length);for(const[owner,relationships]of Object.entries(p.relationships)){scalar(Buffer.from(owner));bytes.push(relationships.length);for(const rel of relationships){scalar(Buffer.from(rel.id));scalar(Buffer.from(rel.relType));scalar(Buffer.from(rel.target));bytes.push(rel.targetMode==="external"?1:0);}}scalar(Buffer.from(p.comment));expect(bytes).toEqual(opcNative.binary);
 const model={schema:"literal OPC cursor",opc:p,xmlParts:[]}as unknown as XlsxSnapshot;const database=await xlsxSnapshotToSqliteDatabase(model);expect(database.tables.reduce((sum,table)=>sum+table.rows.length,0)).toBe(opcNative.rows+1);const sql=Database.deserialize(await exportSqliteDatabase(database));try{expect(sql.query("SELECT extension,content_type FROM xlsx_default_content_type ORDER BY ordinal").all()).toEqual([{extension:"XML",content_type:"a"},{extension:"XML",content_type:"b"}]);expect(sql.query("SELECT data AS bytes FROM xlsx_binary_part").get()).toEqual({bytes:Uint8Array.of(0,255)});}finally{sql.close();}
});

import paidOwner from"../../🧫️fixtures/🪶️sqlite/💰️backing/🔣️.json";
import requestSettlement from"../../🧫️fixtures/🪶️sqlite/💰️backing/🔬️requests/🔣️.json";
import requestSettlementSchema from"../../🧫️fixtures/🪶️sqlite/💰️backing/🔬️requests/🧬️schema/🔣️.json";
import paidOwnerSchema from"../../🧫️fixtures/🪶️sqlite/💰️backing/🧬️schema/🔣️.json";
test("XLSX neutral ownership roles retain independently measured full fields",async()=>{
 expect(new Ajv({strict:true}).validate(paidOwnerSchema,paidOwner)).toBe(true);
 expect(Object.keys(input)).toEqual(["schema","opc","xmlParts"]);
 const literal="Grüße\t\n\0🌠";expect(Buffer.from(literal,"utf8").toString("hex")).toBe("4772c3bcc39f65090a00f09f8ca0");
 const owned=xlsxSnapshotToSqliteDatabase;const database=await owned(input);expect(database.tables.length).toBe(21);
 const sql=Database.deserialize(await exportSqliteDatabase(database));try{
 expect(sql.query("SELECT hex(CAST(? AS BLOB)) AS word").get(literal)).toEqual({word:"4772C3BCC39F65090A00F09F8CA0"});
 expect(sql.query("SELECT count(*) AS n FROM xlsx_xml_part").get()).toEqual({n:input.xmlParts.length});
 expect(sql.query("SELECT schema FROM xlsx_document").get()).toEqual({schema:input.schema});
 expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
 }finally{sql.close();}
});

test("XLSX full request settlement preserves independently inspected enclosing ownership",async()=>{
 expect(new Ajv({strict:true}).validate(requestSettlementSchema,requestSettlement)).toBe(true);
 const database=await xlsxSnapshotToSqliteDatabase(input);
 const sql=Database.deserialize(await exportSqliteDatabase(database));
 try{
  expect(sql.query("SELECT count(*) AS n FROM sqlite_schema WHERE type='table'").get()).toEqual({n:21});
  expect(sql.query("SELECT count(*) AS n FROM xlsx_xml_part").get()).toEqual({n:input.xmlParts.length});
  expect(sql.query("SELECT owner_path FROM xlsx_relationship_owner ORDER BY owner_path").all()).toEqual(Object.keys(input.opc.relationships).sort().map(owner_path=>({owner_path})));
  expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(await xlsxSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(sql.serialize())))).toEqual(input);
 }finally{sql.close();}
});
