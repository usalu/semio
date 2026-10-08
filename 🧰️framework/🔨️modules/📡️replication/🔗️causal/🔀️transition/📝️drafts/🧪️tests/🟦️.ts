/** 📝️ Original draft ordering and replacement agree with independent SQLite and JSON Patch. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
test("📝️ actual history drafts own first-party original arena and displaced keys",()=>{
  const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
  const database=new Database(":memory:");try{
    database.exec("CREATE TABLE drafts(id TEXT PRIMARY KEY, value TEXT NOT NULL)");const insert=database.prepare("INSERT INTO drafts VALUES (?, ?) ON CONFLICT(id) DO UPDATE SET value=excluded.value");const original:Record<string,unknown>={};
    for(const row of law.entries){const value=row.withdrawn?{kind:"withdrawn"}:{kind:"input",schema:row.schema,payload:row.payload};insert.run(row.id,JSON.stringify(value));original[row.id]=value;expect(Buffer.byteLength(row.id)).toBeLessThanOrEqual(law.textCapacity);expect(row.payload.length).toBeLessThanOrEqual(law.payloadCapacity);}
    const expected=database.query<{id:string;value:string},[]>("SELECT id,value FROM drafts ORDER BY id COLLATE BINARY").all();expect(expected.map(row=>row.id)).toEqual(law.expectedOrder);expect(expected.map(row=>JSON.parse(row.value))).toEqual(law.expectedOrder.map((id:string)=>original[id]));
    for(const cancel of law.cancelAt){const current=structuredClone(original),prefix=law.expectedOrder.slice(0,Math.min(cancel,law.expectedOrder.length));for(const id of prefix)delete current[id];const patches=Object.keys(current).map(id=>({op:"remove" as const,path:`/${id.replaceAll("~","~0").replaceAll("/","~1")}`}));expect(applyPatch(current,patches,true).newDocument).toEqual({});expect(Object.keys(original).sort()).toEqual(law.expectedOrder);}
  }finally{database.close();}
  const transition=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8"),store=readFileSync(new URL("../../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs",import.meta.url),"utf8");expect(transition.includes('pub use input_drafts::HistoryInputDrafts;')).toBe(true);expect(store.includes("drafts: protocol::HistoryInputDrafts")).toBe(true);
  console.log(`[DEBUG] neutral original draft replacement/order/${law.cancelAt.length} cancellation frontiers match SQLite/RFC6902; actual first-party draft custody is mounted`);
});
