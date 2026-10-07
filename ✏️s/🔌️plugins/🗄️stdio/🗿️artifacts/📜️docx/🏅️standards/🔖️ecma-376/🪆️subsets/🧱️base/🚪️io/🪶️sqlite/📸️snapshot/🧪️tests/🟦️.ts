/** 🧫️ Complete OPC/XML owned state checked with independent SQLite and schema admission. */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import fixture from"../🧫️fixtures/🔣️.json";
import schema from"../../../../🧬️schema/📸️snapshot/🔣️.json";
import xmlSchema from"../../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json";
import{DOCX_SQLITE_SCHEMA,docxSnapshotToSqliteDatabase,docxSnapshotFromSqliteDatabase}from"../../../../../../../../🟦️.ts";
import type{DocxSnapshot}from"../../../../../../../../🟦️.ts";
import{exportSqliteDatabase,importSqliteDatabase}from"@semio-tech/framework";
import{materializeRetainedXmlDocument,parseRetainedXmlDocument,parseXmlDocument,retainXmlDocument}from"../../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import profiles from"../../../../🧬️schema/📸️snapshot/🧫️fixtures/🛡️profile/🔣️.json";

import Ajv2020 from"ajv/dist/2020";
import{validateDocxSnapshotProfile}from"../../../../../../../../🟦️.ts";
import{createHash}from"node:crypto";
import{Buffer}from"node:buffer";
import definition from"../../../../../../../../📜️artifact-definition.json";
test("DOCX native factory hash identifies the authored complete literal protocol",async()=>{const bytes=await Bun.file(new URL("../../../💾️binary/📸️snapshot/📡️.protocol.semio",import.meta.url)).arrayBuffer();const measured=createHash("sha256").update(Buffer.from(bytes)).digest("hex");expect(new Bun.CryptoHasher("sha256").update(bytes).digest("hex")).toBe(measured);expect(definition.codecs[0]!.native_factory!.pack_schema_hash).toBe(measured);});
function profileSnapshot(document:unknown,relationshipBase:string):DocxSnapshot{return{schema:"literal retained schema",opc:{parts:[],contentTypes:{defaults:[],overrides:[]},relationships:{"":[{id:"main",relType:relationshipBase+"/officeDocument",target:"word/document.xml",targetMode:"internal"}]},comment:""},xmlParts:[{path:"word/document.xml",contentType:"application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",document:retainXmlDocument(parseXmlDocument(document))}]};}
test("DOCX exact profile policies inspect typed namespace entities independently",async()=>{
 
 for(const item of profiles.cases){const snapshot=profileSnapshot(item.document,item.relationshipBase);const result=await validateDocxSnapshotProfile(snapshot,item.subset as"strict"|"transitional");expect(result.map(item=>item.code)).toEqual(item.codes);expect(result.filter(item=>item.severity==="warning").length).toBe(item.warnings);const db=Database.deserialize(await exportSqliteDatabase(await docxSnapshotToSqliteDatabase(snapshot)));try{expect(db.query("SELECT value FROM docx_xml_attribute WHERE name='xmlns:w'").all().length).toBe(1);expect(db.query("SELECT target FROM docx_relationship").get()).toEqual({target:"word/document.xml"});}finally{db.close();}}
});
const input={...fixture,xmlParts:fixture.xmlParts.map(part=>({...part,document:parseRetainedXmlDocument(part.document)}))} as unknown as DocxSnapshot;
test("DOCX typed profile path ownership is bounded and interior cancellable",async()=>{
 const first=profiles.cases[0]!;const snapshot=profileSnapshot(first.document,first.relationshipBase);const path="x".repeat(profiles.largeTextCharacters);snapshot.opc.relationships[""]![0]!.target=path;snapshot.xmlParts[0]!.path=path;
 const controller=new AbortController();let reached=false;
 await expect(validateDocxSnapshotProfile(snapshot,"strict",{signal:controller.signal,onProgress(event){if(event.total===path.length&&event.completed>=profiles.cancelAfter&&event.completed<event.total){reached=true;controller.abort();}}})).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);
 await expect(validateDocxSnapshotProfile(snapshot,"strict",{maximumBytes:profiles.smallValueBudget})).rejects.toThrow("limit");
});
test("DOCX all owned OPC/XML fields expose independent relational identities",async()=>{
 const ajv=new Ajv({strict:false}).addSchema(xmlSchema);const admitted=ajv.validate(schema,fixture);expect(admitted,JSON.stringify(ajv.errors)).toBe(true);expect(DOCX_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());
 const db=Database.deserialize(await exportSqliteDatabase(await docxSnapshotToSqliteDatabase(input)));try{
 expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT ordinal,path,content_type,data FROM docx_binary_part ORDER BY ordinal").all()).toEqual(input.opc.parts.map((part,ordinal)=>({ordinal,path:part.path,content_type:part.contentType,data:new Uint8Array(part.bytes)})));expect(db.query("SELECT owner_path FROM docx_relationship_owner ORDER BY owner_path").all()).toEqual(Object.keys(input.opc.relationships).sort().map(owner_path=>({owner_path})));expect(await docxSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(input);
 db.query("UPDATE docx_xml_text SET text=? WHERE node_id=(SELECT node_id FROM docx_xml_document_misc WHERE xml_document_id=1 AND position='prolog' AND ordinal=0)").run("independent SQL 世界");const restored=await docxSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));expect(materializeRetainedXmlDocument(restored.xmlParts[0]!.document).prolog[0]).toEqual({kind:"text",text:"independent SQL 世界"});expect(restored.opc).toEqual(input.opc);
 }finally{db.close();}
});
test("DOCX aggregate domain rows include package and XML ownership exactly",async()=>{const db=await docxSnapshotToSqliteDatabase(input);const count=db.tables.reduce((total,table)=>total+table.rows.length,0);expect(await docxSnapshotFromSqliteDatabase(await docxSnapshotToSqliteDatabase(input,{maxRows:count}),{maxRows:count})).toEqual(input);await expect(docxSnapshotToSqliteDatabase(input,{maxRows:count-1})).rejects.toThrow();});

test("DOCX dangling references and shared OPC/XML owners are refused",async()=>{const database=await docxSnapshotToSqliteDatabase(input);for(const[name,row,column,value]of[["docx_document",0,2,2n],["docx_xml_part",1,4,1n],["docx_relationship",0,1,99n],["docx_default_content_type",0,2,99n],["docx_relationship_owner",1,2,""]]as const){const edited={tables:database.tables.map(table=>table.name!==name?table:{...table,rows:table.rows.map((entry,index)=>index!==row?entry:{...entry,values:entry.values.map((cell,at)=>at===column?value:cell)})})};await expect(docxSnapshotFromSqliteDatabase(edited),name).rejects.toThrow();}});
test("DOCX intrinsic large-part copies have interior ownership cancellation",async()=>{const snapshot=structuredClone(input);snapshot.opc.parts[0]!.bytes=new Array(100000).fill(0x97);const database=await docxSnapshotToSqliteDatabase(snapshot);for(const phase of["projectSnapshot","reconstructSnapshot"]as const){const controller=new AbortController();let reached=false;const options={signal:controller.signal,onProgress:(event:{phase:string;completed:number;total:number})=>{if(event.phase===phase&&event.total===100000&&event.completed>=65536&&event.completed<event.total){reached=true;controller.abort();}}};await expect(phase==="projectSnapshot"?docxSnapshotToSqliteDatabase(snapshot,options):docxSnapshotFromSqliteDatabase(database,options)).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);}});

import paidOwner from"../🧫️fixtures/💰️backing/🔣️.json";
import requestSettlement from"../🧫️fixtures/💰️backing/🔬️requests/🔣️.json";


import retainedMetadata from"../🧫️fixtures/🗂️retained-metadata/🔣️.json";

import artifactSchema from"../../../../🧬️schema/🔣️.json";
test("DOCX retained metadata schema preserves named records and duplicate order",async()=>{
 expect(retainedMetadata["entryKind"]).toEqual("orderedNamedRecord");expect(retainedMetadata["entryFields"]).toEqual(["name","contentType"]);expect(retainedMetadata["preservesOrder"]).toEqual(true);expect(retainedMetadata["preservesDuplicates"]).toEqual(true);
 const items=schema.$defs.OpcContentTypes.properties.defaults.items;
 expect(items.type).toBe("object");expect(items.required).toEqual(retainedMetadata.entryFields);
 expect(artifactSchema.$defs.OpcContentTypes.properties.defaults.items.type).toBe("object");
 expect(fixture.opc.contentTypes.defaults).toEqual(retainedMetadata.defaults);expect(fixture.opc.contentTypes.overrides).toEqual(retainedMetadata.overrides);
 const database=await docxSnapshotToSqliteDatabase(input);const sql=Database.deserialize(await exportSqliteDatabase(database));
 try{
  expect(sql.query("SELECT extension AS name,content_type AS contentType FROM docx_default_content_type ORDER BY ordinal").all()).toEqual(retainedMetadata.defaults);
  expect(sql.query("SELECT part_name AS name,content_type AS contentType FROM docx_override_content_type ORDER BY ordinal").all()).toEqual(retainedMetadata.overrides);
  expect(await docxSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(sql.serialize())))).toEqual(input);
 }finally{sql.close();}
});
test("DOCX neutral ownership roles retain independently measured full fields",async()=>{
 expect(paidOwner["fieldOrder"]).toEqual(["schema","opc","xmlParts"]);expect(paidOwner["tableCount"]).toEqual(21);expect(paidOwner["admission"]).toEqual("beforeConcreteBacking");expect(paidOwner["retirementRefund"]).toEqual(false);expect(paidOwner["roles"]).toEqual({"maxValueBytes":"semanticLiteralBytes","maxAllocationBytes":"cumulativeOwnedBacking","encodedBound":"prospectiveWithoutPayload","borrowedPreflight":"paidTraversalFrontiers"});expect(paidOwner["refusalKinds"]).toEqual({"owned":"ownershipLimit","allocator":"allocationFailed","work":"workLimit","cancellation":"canceled"});expect(paidOwner["literal"]).toEqual("Grüße\t\n\u0000🌠");expect(paidOwner["literalUtf8Hex"]).toEqual("4772c3bcc39f65090a00f09f8ca0");
 expect(Object.keys(input)).toEqual(["schema","opc","xmlParts"]);
 const literal="Grüße\t\n\0🌠";expect(Buffer.from(literal,"utf8").toString("hex")).toBe("4772c3bcc39f65090a00f09f8ca0");
 const owned=docxSnapshotToSqliteDatabase;const database=await owned(input);expect(database.tables.length).toBe(21);
 const sql=Database.deserialize(await exportSqliteDatabase(database));try{
 expect(sql.query("SELECT hex(CAST(? AS BLOB)) AS word").get(literal)).toEqual({word:"4772C3BCC39F65090A00F09F8CA0"});
 expect(sql.query("SELECT count(*) AS n FROM docx_xml_part").get()).toEqual({n:input.xmlParts.length});
 expect(sql.query("SELECT schema FROM docx_document").get()).toEqual({schema:input.schema});
 expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
 }finally{sql.close();}
});

test("DOCX full request settlement preserves independently inspected enclosing ownership",async()=>{
 expect(requestSettlement["phases"]).toEqual(["projectSnapshot","reconstructSnapshot"]);expect(requestSettlement["requestExtent"]).toEqual("fullConcreteRequests");expect(requestSettlement["reallocation"]).toEqual("completeReplacement");expect(requestSettlement["exactReplay"]).toEqual(true);expect(requestSettlement["denial"]).toEqual("oneByteBelowObserved");expect(requestSettlement["retirementRefund"]).toEqual(false);expect(requestSettlement["refusalKind"]).toEqual("ownershipLimit");expect(requestSettlement["tableCount"]).toEqual(21);
 const database=await docxSnapshotToSqliteDatabase(input);
 const sql=Database.deserialize(await exportSqliteDatabase(database));
 try{
  expect(sql.query("SELECT count(*) AS n FROM sqlite_schema WHERE type='table'").get()).toEqual({n:21});
  expect(sql.query("SELECT count(*) AS n FROM docx_xml_part").get()).toEqual({n:input.xmlParts.length});
  expect(sql.query("SELECT owner_path FROM docx_relationship_owner ORDER BY owner_path").all()).toEqual(Object.keys(input.opc.relationships).sort().map(owner_path=>({owner_path})));
  expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(await docxSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(sql.serialize())))).toEqual(input);
 }finally{sql.close();}
});
