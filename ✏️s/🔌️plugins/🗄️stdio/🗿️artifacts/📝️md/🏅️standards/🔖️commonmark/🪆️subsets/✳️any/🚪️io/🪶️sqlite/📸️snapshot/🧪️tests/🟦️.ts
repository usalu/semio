import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import schema from "../../../../🧬️schema/📸️snapshot/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import cohort from "../🧫️fixtures/📏️cohort/🔣️.json";

import nativeControl from "../🧫️fixtures/🛬️controlled.json";

import type {MdSnapshot,MdBlock,MdInline} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {parseMdSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {MD_SQLITE_SCHEMA,mdSnapshotToSqliteDatabase,mdSnapshotFromSqliteDatabase,validateMdSnapshotSqliteDialect} from "../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";



test("CommonMark independent consistent structural renumbering preserves every literal owner",async()=>{
 const snapshot=fixture as MdSnapshot,oracle=Database.deserialize(await exportSqliteDatabase(await mdSnapshotToSqliteDatabase(snapshot)));
 try{oracle.exec("PRAGMA foreign_keys=OFF");for(const{name}of oracle.query("SELECT name FROM sqlite_schema WHERE type='table'").all()as{name:string}[]){const key=(oracle.query(`PRAGMA table_info(${name})`).all()as{name:string;pk:number}[]).find(column=>column.pk===1)!.name;const links=oracle.query(`PRAGMA foreign_key_list(${name})`).all()as{from:string}[];oracle.exec(`UPDATE ${name} SET ${[`${key}=${key}+${cohort.renumberOffset}`,...links.filter(link=>link.from!==key).map(link=>`${link.from}=${link.from}+${cohort.renumberOffset}`)].join(",")}`);}expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);const database=await importSqliteDatabase(oracle.serialize());expect(await mdSnapshotFromSqliteDatabase(database)).toEqual(snapshot);expect(await validateMdSnapshotSqliteDialect(snapshot,{artifactKind:"s.stdio.md",standard:"commonmark",subset:"*"},database)).toEqual([]);}finally{oracle.close();}
});



test("CommonMark cancellation occurs inside known UTF text ownership",async()=>{
 const snapshot:MdSnapshot={schema:cohort.unicodeUnit.repeat(cohort.unicodeRepeat),blocks:[]};
 const oracle=Database.deserialize(await exportSqliteDatabase(await mdSnapshotToSqliteDatabase(snapshot)));
 try{const row=oracle.query("SELECT length(CAST(schema AS BLOB)) AS n FROM md_document").get() as {n:number};expect(row.n).toBe(Buffer.byteLength(snapshot.schema,"utf8"));}finally{oracle.close();}
 const cancel=new AbortController();let interior=false;
 await expect(mdSnapshotToSqliteDatabase(snapshot,{signal:cancel.signal,onProgress:event=>{if(event.phase==="projectSnapshot"&&event.completed>=cohort.cancelAfter&&event.completed<event.total){interior=true;cancel.abort();}}})).rejects.toMatchObject({name:"ValueError",kind:"canceled"});expect(interior).toBe(true);
});

test("CommonMark wide empty list items remain independent ordered entities",async()=>{
 const snapshot:MdSnapshot={schema:"literal wide list",blocks:[{kind:"list",ordered:true,start:0,tight:false,items:Array.from({length:cohort.wideEmptyItems},()=>[])}]},rows=cohort.wideEmptyItems+4;
 expect(new Ajv({strict:false}).validate(schema,snapshot)).toBe(true);const oracle=Database.deserialize(await exportSqliteDatabase(await mdSnapshotToSqliteDatabase(snapshot,{maxRows:rows})));
 try{expect(oracle.query("SELECT COUNT(*) AS n, MIN(ordinal) AS first, MAX(ordinal) AS last FROM md_list_item").get()).toEqual({n:cohort.wideEmptyItems,first:0,last:cohort.wideEmptyItems-1});expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(await mdSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).toEqual(snapshot);}finally{oracle.close();}await expect(mdSnapshotToSqliteDatabase(snapshot,{maxRows:rows-1})).rejects.toThrow();
});

test("CommonMark wide empty list cancellation uses the known item frontier",async()=>{
 const snapshot:MdSnapshot={schema:"literal wide list",blocks:[{kind:"list",ordered:false,tight:true,items:Array.from({length:cohort.wideEmptyItems},()=>[])}]},cancel=new AbortController();let interior=false;
 await expect(mdSnapshotToSqliteDatabase(snapshot,{signal:cancel.signal,onProgress:event=>{if(event.phase==="projectSnapshot"&&event.total===cohort.wideEmptyItems&&event.completed>=cohort.cancelAfter&&event.completed<event.total){interior=true;cancel.abort();}}})).rejects.toMatchObject({name:"ValueError",kind:"canceled"});expect(interior).toBe(true);
});

test("CommonMark exact owned admission refuses independently edited permitted literal state",async()=>{
 const snapshot=fixture as MdSnapshot,oracle=Database.deserialize(await exportSqliteDatabase(await mdSnapshotToSqliteDatabase(snapshot)));
 try{oracle.run("UPDATE md_text SET text='different permitted literal' WHERE id=(SELECT inline_id FROM md_block_inline WHERE block_id=(SELECT block_id FROM md_document_block WHERE ordinal=0) AND ordinal=0)");expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);const database=await importSqliteDatabase(oracle.serialize());expect(await mdSnapshotFromSqliteDatabase(database)).not.toEqual(snapshot);await expect(validateMdSnapshotSqliteDialect(snapshot,{artifactKind:"s.stdio.md",standard:"commonmark",subset:"*"},database)).rejects.toThrow("identity");}finally{oracle.close();}
});

test("CommonMark complete neutral tree exposes independent domain joins and edits",async()=>{
 expect(new Ajv({strict:false}).validate(schema,fixture)).toBe(true);expect(MD_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());const snapshot=structuredClone(fixture) as MdSnapshot;const database=await mdSnapshotToSqliteDatabase(snapshot);expect(await mdSnapshotFromSqliteDatabase(database)).toEqual(snapshot);
 const db=Database.deserialize(await exportSqliteDatabase(database));try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT h.level,t.text FROM md_document_block b JOIN md_heading h ON h.id=b.block_id JOIN md_block_inline i ON i.block_id=h.id JOIN md_text t ON t.id=i.inline_id WHERE b.ordinal=0 AND i.ordinal=0").get()).toEqual({level:255,text:"Alpha 世界\0"});expect(db.query("SELECT start FROM md_list WHERE start IS NOT NULL").get()).toEqual({start:4294967295});db.run("UPDATE md_text SET text='edited 世界' WHERE id=(SELECT inline_id FROM md_block_inline WHERE block_id=(SELECT block_id FROM md_document_block WHERE ordinal=0) AND ordinal=0)");db.run("UPDATE md_heading SET level=0");const heading=snapshot.blocks[0];if(heading?.kind!=="heading"||heading.inlines[0]?.kind!=="text")throw Error("fixture shape");heading.level=0;heading.inlines[0].text="edited 世界";expect(await mdSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(snapshot);}finally{db.close();}
});

test("CommonMark preserves empty documents and every empty optional state",async()=>{for(const snapshot of [{schema:"",blocks:[]},fixture] as MdSnapshot[])expect(await mdSnapshotFromSqliteDatabase(await importSqliteDatabase(await exportSqliteDatabase(await mdSnapshotToSqliteDatabase(snapshot))))).toEqual(snapshot);});

test("CommonMark schema-first owned parsers cover full neutral shapes and scalar widths",()=>{expect(parseMdSnapshot(fixture)).toEqual(fixture as MdSnapshot);for(const block of [{kind:"heading",level:256,inlines:[]},{kind:"list",ordered:true,tight:true,start:4294967296,items:[]},{kind:"paragraph",inlines:[{kind:"unknown"}]}]){const value={schema:"",blocks:[block]};expect(new Ajv({strict:false}).validate(schema,value)).toBe(false);expect(()=>parseMdSnapshot(value)).toThrow();}});

test("CommonMark refuses broken domain ownership ordinals subtype and numeric widths",async()=>{const database=await mdSnapshotToSqliteDatabase(fixture as MdSnapshot);for(const sql of ["UPDATE md_heading SET level=256","UPDATE md_block SET kind='paragraph' WHERE id=1","UPDATE md_document_block SET ordinal=99 WHERE id=1","UPDATE md_block_inline SET block_id=999 WHERE id=1","DELETE FROM md_list_item WHERE id=1","DELETE FROM md_document_block WHERE id=1","UPDATE md_list SET start=-1 WHERE start IS NOT NULL","INSERT INTO md_text VALUES(999,'stranded')"]){const db=Database.deserialize(await exportSqliteDatabase(database));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(mdSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();}finally{db.close();}}await expect(mdSnapshotToSqliteDatabase(fixture as MdSnapshot,{maxRows:1})).rejects.toThrow();await expect(mdSnapshotFromSqliteDatabase(database,{maxValueBytes:0})).rejects.toThrow();const cancel=new AbortController();await expect(mdSnapshotFromSqliteDatabase(database,{signal:cancel.signal,onProgress:()=>cancel.abort()})).rejects.toMatchObject({name:"ValueError",kind:"canceled"});});

test("CommonMark iterative native domain mirrors preserve1024levels with independent recursive SQL",async()=>{const vector=await Bun.file(new URL("../🧫️fixtures/📏️depth/🔣️.json",import.meta.url)).json();let inline:MdInline={kind:"text",text:vector.leaf};for(let depth=0;depth<vector.depth;depth++)inline={kind:"strong",inlines:[inline]};let block:MdBlock={kind:"paragraph",inlines:[inline]};for(let depth=0;depth<vector.depth;depth++)block={kind:"blockQuote",blocks:[block]};const snapshot:MdSnapshot={schema:"deep tree",blocks:[block]};const database=await mdSnapshotToSqliteDatabase(snapshot);const db=Database.deserialize(await exportSqliteDatabase(database));try{expect(db.query("WITH RECURSIVE depth(id,n) AS (SELECT block_id,0 FROM md_document_block UNION ALL SELECT b.block_id,d.n+1 FROM depth d JOIN md_quote_block b ON b.quote_id=d.id) SELECT MAX(n) AS n FROM depth").get()).toEqual({n:vector.depth});expect(db.query("SELECT COUNT(*) AS n FROM md_strong").get()).toEqual({n:vector.depth});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);const restored=await mdSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));let block=restored.blocks[0]!;for(let depth=0;depth<vector.depth;depth++){if(block.kind!=="blockQuote")throw Error("quote depth lost");block=block.blocks[0]!;}if(block.kind!=="paragraph")throw Error("paragraph lost");let inline=block.inlines[0]!;for(let depth=0;depth<vector.depth;depth++){if(inline.kind!=="strong")throw Error("strong depth lost");inline=inline.inlines[0]!;}expect(inline).toEqual({kind:"text",text:vector.leaf});}finally{db.close();}});

test("CommonMark exact four-argument owned admission retains schema identity and cancellation",async()=>{const coordinates=await Bun.file(new URL("../🧫️fixtures/🧭️dialects/🔣️.json",import.meta.url)).json();const snapshot=fixture as MdSnapshot;const database=await mdSnapshotToSqliteDatabase(snapshot);expect(await validateMdSnapshotSqliteDialect(snapshot,coordinates.sqliteDialect,database)).toEqual([]);for(const dialect of coordinates.invalidSqliteDialects)await expect(validateMdSnapshotSqliteDialect(snapshot,dialect,database)).rejects.toThrow();const db=Database.deserialize(await exportSqliteDatabase(database));try{db.run("UPDATE md_document SET schema='different owned schema'");await expect(validateMdSnapshotSqliteDialect(snapshot,coordinates.sqliteDialect,await importSqliteDatabase(db.serialize()))).rejects.toThrow();}finally{db.close()}const cancel=new AbortController();await expect(validateMdSnapshotSqliteDialect(snapshot,coordinates.sqliteDialect,database,{signal:cancel.signal,onProgress:()=>cancel.abort()})).rejects.toMatchObject({name:"ValueError",kind:"canceled"});});

import NormSemanticAjv from "ajv/dist/2020.js";
import normSemanticContract from "../🧫️fixtures/🎛️semantic.json";

import {mdSnapshotToSqliteDatabase as normSemanticProject,mdSnapshotFromSqliteDatabase as normSemanticRestore} from "../🟦️.ts";
import {exportSqliteDatabase as normSemanticExport,importSqliteDatabase as normSemanticImport} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {independentSqliteExtent as normIndependentExtent} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts";


test("CommonMark native controlled fixture names owned backing separately from semantic values",async()=>{
 expect(nativeControl["depth"]).toEqual(600);expect(nativeControl["maxRows"]).toEqual(10);expect(nativeControl["maxAllocationBytes"]).toEqual(4096);expect(nativeControl["allocationLimitRole"]).toEqual("nativeOwnedBacking");expect(nativeControl["cancelAfter"]).toEqual(256);expect(nativeControl["schemaRepeat"]).toEqual(10000);expect(nativeControl["orderedStart"]).toEqual(4294967295);
 const oldBudget={...nativeControl,maxValueBytes:4096};delete (oldBudget as Record<string,unknown>).maxAllocationBytes;
 
 const ownedSchema="owned logical schema 世界".repeat(nativeControl.schemaRepeat),database=await mdSnapshotToSqliteDatabase({schema:ownedSchema,blocks:[]}),oracle=Database.deserialize(await exportSqliteDatabase(database));
 try{const row=oracle.query("SELECT length(CAST(schema AS BLOB)) AS bytes FROM md_document").get() as {bytes:number};expect(row.bytes).toBe(Buffer.byteLength(ownedSchema,"utf8"));expect(row.bytes).toBeGreaterThan((nativeControl as Record<string,unknown>).maxAllocationBytes as number);expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(await mdSnapshotFromSqliteDatabase(database)).toEqual({schema:ownedSchema,blocks:[]});}finally{oracle.close();}
});

test("CommonMark neutral cohort has exactly its individually authored domain rows",async()=>{
 
 const database=await mdSnapshotToSqliteDatabase(fixture as MdSnapshot,{maxRows:cohort.domainRows});
 const db=Database.deserialize(await exportSqliteDatabase(database));
 try{let total=0;for(const[name,expected]of Object.entries(cohort.tableRowCounts)){const actual=db.query(`SELECT COUNT(*) AS n FROM "${name}"`).get() as {n:number};expect(actual.n).toBe(expected);total+=actual.n;}expect(total).toBe(cohort.domainRows);expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(await mdSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(fixture as MdSnapshot);}finally{db.close();}
 await expect(mdSnapshotToSqliteDatabase(fixture as MdSnapshot,{maxRows:cohort.domainRows-1})).rejects.toThrow();
});

test("MD complete independent cells preserve copied limits and required empty metadata",async()=>{
 expect(normSemanticContract["schemaBytes"]).toEqual(3901);expect(normSemanticContract["tableWidths"]).toEqual({"md_block":2,"md_block_inline":4,"md_block_quote":1,"md_code_block":3,"md_code_span":2,"md_document":2,"md_document_block":4,"md_emphasis":1,"md_emphasis_inline":4,"md_hard_break":1,"md_heading":2,"md_html_block":2,"md_html_inline":2,"md_image":4,"md_inline":2,"md_link":3,"md_link_text":4,"md_list":4,"md_list_item":3,"md_list_item_block":4,"md_paragraph":1,"md_quote_block":4,"md_soft_break":1,"md_strong":1,"md_strong_inline":4,"md_text":2,"md_thematic_break":1});expect(normSemanticContract["cases"]).toEqual([{"id":"full","rows":88,"valueBytes":1879},{"id":"metadataRetainingEmpty","rows":1,"valueBytes":37}]);
 for(const item of normSemanticContract.cases){const source=structuredClone(fixture) as import("../../../../🧬️schema/📸️snapshot/🟦️.ts").MdSnapshot;if(item.id==="metadataRetainingEmpty"){source.blocks=[];}const database=await normSemanticProject(source),bytes=await normSemanticExport(database);expect(normIndependentExtent(bytes)).toEqual({rows:item.rows,valueBytes:item.valueBytes,schemaBytes:normSemanticContract.schemaBytes,tableWidths:normSemanticContract.tableWidths});const limits={maxRows:item.rows,maxValueBytes:item.valueBytes,maxSchemaBytes:normSemanticContract.schemaBytes,maxTables:27,maxColumns:4};expect(await normSemanticProject(source,limits)).toEqual(database);expect(await normSemanticRestore(await normSemanticImport(bytes),limits)).toEqual(source);
  for(const short of[{...limits,maxRows:item.rows-1},{...limits,maxValueBytes:item.valueBytes-1},{...limits,maxSchemaBytes:limits.maxSchemaBytes-1},{...limits,maxTables:26},{...limits,maxColumns:3}]){await expect(normSemanticProject(source,short)).rejects.toThrow();await expect(normSemanticRestore(database,short)).rejects.toThrow();}
 }console.log("[DEBUG] MD complete independent SQLite cells and metadata preserve full and empty semantic limits");
});
