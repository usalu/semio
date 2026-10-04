/** 🧹️ Shared lifecycle literals stay typed and independently queryable in actual HTML SQLite. */
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import neutral from "../../../🧫️fixtures/🪶️sqlite/🧹️lifecycle/🔣️.json";
import schema from "../../../🧫️fixtures/🪶️sqlite/🧹️lifecycle/🧬️schema/🔣️.json";
import {type HtmlNode,parseHtmlSnapshot} from "../../../🟦️.ts";
import {htmlSnapshotToSqliteDatabase,htmlSnapshotFromSqliteDatabase} from "../../../🪶️sqlite/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
test("HTML late-copy refusal has an independently valid interior UTF8 witness",()=>{
 const bytes=Buffer.from(neutral.lateTextUnit.repeat(neutral.lateTextRepeat),"utf8"),decoder=new TextDecoder("utf-8",{fatal:true});
 expect(decoder.decode(bytes.subarray(0,neutral.cancelUtf8ByteBoundary))).toBe(neutral.lateTextUnit.repeat(neutral.cancelUtf8ByteBoundary/Buffer.byteLength(neutral.lateTextUnit,"utf8")));
 expect(()=>decoder.decode(bytes.subarray(0,neutral.cancelAfterBytes))).toThrow();
 expect(neutral.cancelAfterBytes-neutral.cancelUtf8ByteBoundary).toBeGreaterThan(0);expect(neutral.cancelAfterBytes-neutral.cancelUtf8ByteBoundary).toBeLessThan(4);
 expect(neutral.cancelUtf8ByteBoundary).toBeLessThan(bytes.length);expect(decoder.decode(bytes)).toBe(neutral.lateTextUnit.repeat(neutral.lateTextRepeat));
});
test("HTML lifecycle neutral deep and wide literals retain every owned field in independent SQLite",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(neutral)).toBe(true);
 let root:HtmlNode={kind:"element",name:neutral.elementName,attributes:neutral.attributes,children:neutral.leaves as HtmlNode[]};
 for(let depth=0;depth<neutral.oracleDepth;depth++)root={kind:"element",name:neutral.elementName,attributes:neutral.attributes,children:[root]};
 const value=parseHtmlSnapshot({schema:neutral.schema,doctype:neutral.doctype,root}),db=Database.deserialize(await exportSqliteDatabase(await htmlSnapshotToSqliteDatabase(value)));
 try{
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(db.query("SELECT COUNT(*) AS n FROM html_node").get()).toEqual({n:neutral.oracleDepth+1+neutral.leaves.length});
  expect(db.query("WITH RECURSIVE depth(id,n) AS (SELECT root_node_id,0 FROM html_document UNION ALL SELECT child.child_node_id,depth.n+1 FROM html_child child JOIN depth ON child.parent_element_node_id=depth.id) SELECT MAX(n) AS n FROM depth").get()).toEqual({n:neutral.oracleDepth+1});
  expect(db.query("SELECT name,value FROM html_attribute WHERE element_node_id=1 ORDER BY ordinal").all()).toEqual(neutral.attributes.map(value=>({name:value.name,value:"value"in value?value.value:null})));
  expect(db.query("SELECT schema,doctype FROM html_document").get()).toEqual({schema:neutral.schema,doctype:neutral.doctype});
  expect(await htmlSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(value);
 }finally{db.close();}
 const late=neutral.lateTextUnit.repeat(neutral.lateTextRepeat),bytes=new TextEncoder().encode(late).byteLength;expect(Buffer.byteLength(late,"utf8")).toBe(bytes);expect(bytes).toBeGreaterThan(neutral.cancelAfterBytes);
 const lateDb=Database.deserialize(await exportSqliteDatabase(await htmlSnapshotToSqliteDatabase({schema:late,doctype:late,root:{kind:"text",text:"owned leaf"}})));
 try{expect(lateDb.query("SELECT length(CAST(schema AS BLOB)) AS schemaBytes,length(CAST(doctype AS BLOB)) AS doctypeBytes FROM html_document").get()).toEqual({schemaBytes:bytes,doctypeBytes:bytes});}finally{lateDb.close();}
 console.log("[DEBUG] HTML lifecycle neutral fields, known late-copy frontier and independent recursive SQL ownership executed");
});
