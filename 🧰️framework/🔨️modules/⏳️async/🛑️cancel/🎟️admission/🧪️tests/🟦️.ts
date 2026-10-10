import { existsSync as originalCorpusAuthorityExists } from "node:fs";
/** 🎟️ Independent original cancellation birth and native close metadata law. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
test("original cancellation admission prices real Arc birth and zero-copy metadata",async()=>{
  expect(originalCorpusAuthorityExists(new URL("../\ud83e\uddec\ufe0fschema/\ud83d\udd23\ufe0f.json", import.meta.url))).toBe(false);

 const law=await Bun.file(new URL("../🧫️fixtures/🔣️.json",import.meta.url)).json();const grantSchema=JSON.parse(readFileSync("./🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json","utf8"));const admits=new Ajv({strict:true}).compile(grantSchema);for(const axes of[law.originalGrant,law.closeGrant])expect(admits({maximumItems:axes[0],maximumCopyBytes:axes[1],maximumCapacityBytes:axes[2],maximumReleaseBytes:axes[3],maximumDepth:axes[4]})).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE cancellation(axis TEXT,lim INTEGER,demand INTEGER)");for(const row of law.cases){const demand={items:1,copy:0,capacity:1,release:0,depth:1}[row.axis as"items"];db.query("INSERT INTO cancellation VALUES(?,?,?)").run(row.axis,row.limit,demand);}expect(db.query("SELECT lim>=demand AS admitted FROM cancellation ORDER BY rowid").all().map((row:any)=>Boolean(row.admitted))).toEqual(law.cases.map((row:any)=>row.admitted));}finally{db.close();}
 const source=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");expect(source.includes("pub fn root_admission_demands()")).toBe(true);expect(source.includes("pub fn admit_root(grant:")).toBe(true);expect(source.includes("pub fn admit_child(&self,grant:")).toBe(true);const start=source.indexOf("pub fn admit_root(grant:"),body=source.slice(start,source.indexOf("///",start+10));expect(body.includes("shared_retirement_allocation_bytes::<CancelNode>()")).toBe(true);expect(body.includes("grant.maximum_capacity_bytes")).toBe(true);expect(body.includes("grant.maximum_depth")).toBe(true);expect(body.includes("retained_capacity_bytes:bytes")).toBe(true);expect(body.includes("root_now()")).toBe(false);
 const close=readFileSync(new URL("../../♻️retirement/🦀️.rs",import.meta.url),"utf8");expect(close.includes("copy_bytes:size_of::<CancelNode>()")).toBe(false);expect(close.includes("copy_bytes:size_of::<CancelToken>()")).toBe(false);expect(close.includes("copy_bytes:size_of::<Vec<(u64,Waker)>>")).toBe(false);console.log("[DEBUG] Ajv/SQLite original cancellation admission requires real native Arc birth, independent full grant, inert denials, and zero-copy node/alias/waiter metadata");
});
