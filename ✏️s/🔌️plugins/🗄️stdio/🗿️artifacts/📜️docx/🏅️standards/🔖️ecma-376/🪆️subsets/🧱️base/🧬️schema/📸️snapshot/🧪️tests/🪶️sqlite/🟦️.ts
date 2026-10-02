/** 🧫️ Complete OPC/XML owned state checked with independent SQLite and schema admission. */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import fixture from"../../🧫️fixtures/🪶️sqlite/🔣️.json";
import schema from"../../🔣️.json";
import xmlSchema from"../../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json";
import{DOCX_SQLITE_SCHEMA,docxSnapshotToSqliteDatabase,docxSnapshotFromSqliteDatabase}from"../../../../../../../../🟦️.ts";
import type{DocxSnapshot}from"../../../../../../../../🟦️.ts";
import{exportSqliteDatabase,importSqliteDatabase}from"@semio-tech/framework";
import{parseXmlDocument}from"../../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import profiles from"../../🧫️fixtures/🛡️profile/🔣️.json";
import profilesSchema from"../../🧫️fixtures/🛡️profile/🧬️schema/🔣️.json";
import Ajv2020 from"ajv/dist/2020";
import{validateDocxSnapshotProfile}from"../../../../../../../../🟦️.ts";
import{createHash}from"node:crypto";
import{Buffer}from"node:buffer";
import definition from"../../../../../../../../📜️artifact-definition.json";
test("DOCX native factory hash identifies the authored complete literal protocol",async()=>{const bytes=await Bun.file(new URL("../../💾️binary/📡️.protocol.semio",import.meta.url)).arrayBuffer();const measured=createHash("sha256").update(Buffer.from(bytes)).digest("hex");expect(new Bun.CryptoHasher("sha256").update(bytes).digest("hex")).toBe(measured);expect(definition.codecs[0]!.native_factory!.pack_schema_hash).toBe(measured);});
function profileSnapshot(document:unknown,relationshipBase:string):DocxSnapshot{return{schema:"literal retained schema",opc:{parts:[],contentTypes:{defaults:[],overrides:[]},relationships:{"":[{id:"main",relType:relationshipBase+"/officeDocument",target:"word/document.xml",targetMode:"internal"}]},comment:""},xmlParts:[{path:"word/document.xml",contentType:"application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",document:parseXmlDocument(document)}]};}
test("DOCX exact profile policies inspect typed namespace entities independently",async()=>{
 expect(new Ajv2020({strict:true}).validate(profilesSchema,profiles)).toBe(true);
 for(const item of profiles.cases){const snapshot=profileSnapshot(item.document,item.relationshipBase);const result=await validateDocxSnapshotProfile(snapshot,item.subset as"strict"|"transitional");expect(result.map(item=>item.code)).toEqual(item.codes);expect(result.filter(item=>item.severity==="warning").length).toBe(item.warnings);const db=Database.deserialize(await exportSqliteDatabase(await docxSnapshotToSqliteDatabase(snapshot)));try{expect(db.query("SELECT value FROM docx_xml_attribute WHERE name='xmlns:w'").all().length).toBe(1);expect(db.query("SELECT target FROM docx_relationship").get()).toEqual({target:"word/document.xml"});}finally{db.close();}}
});
const input={...fixture,xmlParts:fixture.xmlParts.map(part=>({...part,document:parseXmlDocument(part.document)}))} as unknown as DocxSnapshot;
test("DOCX typed profile path ownership is bounded and interior cancellable",async()=>{
 const first=profiles.cases[0]!;const snapshot=profileSnapshot(first.document,first.relationshipBase);const path="x".repeat(profiles.largeTextCharacters);snapshot.opc.relationships[""]![0]!.target=path;snapshot.xmlParts[0]!.path=path;
 const controller=new AbortController();let reached=false;
 await expect(validateDocxSnapshotProfile(snapshot,"strict",{signal:controller.signal,onProgress(event){if(event.total===path.length&&event.completed>=profiles.cancelAfter&&event.completed<event.total){reached=true;controller.abort();}}})).rejects.toThrow("cancelled");expect(reached).toBe(true);
 await expect(validateDocxSnapshotProfile(snapshot,"strict",{maxValueBytes:profiles.smallValueBudget})).rejects.toThrow("limit");
});
test("DOCX all owned OPC/XML fields expose independent relational identities",async()=>{
 const ajv=new Ajv({strict:false}).addSchema(xmlSchema);const admitted=ajv.validate(schema,fixture);expect(admitted,JSON.stringify(ajv.errors)).toBe(true);expect(DOCX_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text());
 const db=Database.deserialize(await exportSqliteDatabase(await docxSnapshotToSqliteDatabase(input)));try{
 expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT ordinal,path,content_type,data FROM docx_binary_part ORDER BY ordinal").all()).toEqual(input.opc.parts.map((part,ordinal)=>({ordinal,path:part.path,content_type:part.contentType,data:new Uint8Array(part.bytes)})));expect(db.query("SELECT owner_path FROM docx_relationship_owner ORDER BY owner_path").all()).toEqual(Object.keys(input.opc.relationships).sort().map(owner_path=>({owner_path})));expect(await docxSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(input);
 db.query("UPDATE docx_xml_text SET text=? WHERE node_id=(SELECT node_id FROM docx_xml_document_misc WHERE xml_document_id=1 AND position='prolog' AND ordinal=0)").run("independent SQL 世界");const restored=await docxSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));expect(restored.xmlParts[0]!.document.prolog[0]).toEqual({kind:"text",text:"independent SQL 世界"});expect(restored.opc).toEqual(input.opc);
 }finally{db.close();}
});
test("DOCX aggregate domain rows include package and XML ownership exactly",async()=>{const db=await docxSnapshotToSqliteDatabase(input);const count=db.tables.reduce((total,table)=>total+table.rows.length,0);expect(await docxSnapshotFromSqliteDatabase(await docxSnapshotToSqliteDatabase(input,{maxRows:count}),{maxRows:count})).toEqual(input);await expect(docxSnapshotToSqliteDatabase(input,{maxRows:count-1})).rejects.toThrow();});

test("DOCX dangling references and shared OPC/XML owners are refused",async()=>{const database=await docxSnapshotToSqliteDatabase(input);for(const[name,row,column,value]of[["docx_document",0,2,2n],["docx_xml_part",1,4,1n],["docx_relationship",0,1,99n],["docx_default_content_type",0,2,99n],["docx_relationship_owner",1,2,""]]as const){const edited={tables:database.tables.map(table=>table.name!==name?table:{...table,rows:table.rows.map((entry,index)=>index!==row?entry:{...entry,values:entry.values.map((cell,at)=>at===column?value:cell)})})};await expect(docxSnapshotFromSqliteDatabase(edited),name).rejects.toThrow();}});
test("DOCX intrinsic large-part copies have interior ownership cancellation",async()=>{const snapshot=structuredClone(input);snapshot.opc.parts[0]!.bytes=new Array(100000).fill(0x97);const database=await docxSnapshotToSqliteDatabase(snapshot);for(const phase of["projectSnapshot","reconstructSnapshot"]as const){const controller=new AbortController();let reached=false;const options={signal:controller.signal,onProgress:(event:{phase:string;completed:number;total:number})=>{if(event.phase===phase&&event.total===100000&&event.completed>=65536&&event.completed<event.total){reached=true;controller.abort();}}};await expect(phase==="projectSnapshot"?docxSnapshotToSqliteDatabase(snapshot,options):docxSnapshotFromSqliteDatabase(database,options)).rejects.toHaveProperty("name","AbortError");expect(reached).toBe(true);}});
