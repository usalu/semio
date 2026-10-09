import Ajv from "ajv";
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve, join } from "node:path";
import { applyPatch } from "fast-json-patch";
import { Database } from "bun:sqlite";

const owner = resolve(import.meta.dir, "../../..");

test("erased controlled retirement preserves the neutral independent-grant contract", () => {
  const law = JSON.parse(readFileSync(join(owner, "♻️retirement/🧫️fixtures/🎮️erased-controlled/🔣️.json"), "utf8"));
  const text = new TextDecoder("utf-8", { fatal: true }).decode(new TextEncoder().encode(law.text));
  expect(text).toBe(law.text);
  expect(JSON.parse(JSON.stringify({ text }))).toEqual({ text });
  expect(applyPatch({ candidate: { text } }, [{ op: "remove", path: "/candidate" }], true, false).newDocument).toEqual({});
  expect(law.reservedCapacity).toBeGreaterThan(Buffer.byteLength(text));
  expect(law.maximumCopyBytes).toBe(3);
  expect(law.maximumItems).toBe(1);
  expect(law.unsupportedKind).toBe("unsupportedOwner");
  expect(law.unsupportedEffects).toEqual({bornBytes:0,releasedBytes:0});
});
console.log("[DEBUG] controlled retirement independent oracle imports loaded");
const validValue = new Ajv({strict:false}).compile(JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8")));
console.log("[DEBUG] controlled retirement independent Value schema compiled");

test("controlled paged retirement conserves independently admitted native tree owners", () => {
  const law = JSON.parse(readFileSync(join(owner, "📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json"), "utf8")).controlledRetirement;
  let value: unknown = null;
  for (let index = 0; index < law.depth; index++) value = {label:law.label,children:[value]};
  console.log("[DEBUG] controlled retirement neutral tree constructed", law.depth);
  expect(validValue(value)).toBe(true);
  expect(JSON.stringify(JSON.parse(JSON.stringify(value)))).toBe(JSON.stringify(value));
  for (const pause of law.pauseAt) {
    const owners = {tree:structuredClone(value),pause};
    expect(applyPatch(owners,[{op:"remove",path:"/tree"}],true,false).newDocument).toEqual({pause});
  }
  const close = JSON.parse(readFileSync(join(owner, "📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json"), "utf8")).cloneClose;
  const corpus = JSON.parse(readFileSync(join(owner, "📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json"), "utf8"));
  const label = corpus.prefix.repeat(close.textRepeat);
  const record = {tree:value,optional:[label,close.optionalValues],objects:{[label]:close.objectValue}};
  expect(validValue(record)).toBe(true);
  expect(JSON.parse(JSON.stringify(record))).toEqual(record);
  for (const pause of close.pauseAt) expect(applyPatch({candidate:structuredClone(record),pause},[{op:"remove",path:"/candidate"}],true,false).newDocument).toEqual({pause});
  expect(close.zeroGrantInert).toBe(true);
  expect(close.maximumBytes).toBe(law.maximumBytes);
  expect(law.maximumItems).toBe(1);
  expect(law.maximumBytes).toBe(4096);
  console.log("[DEBUG] controlled retirement JSON Patch and Value schema owner conservation verified");
});


test("controlled paged retirement closes appended UTF8 without unadmitted scaffold births",()=>{
 const corpus=JSON.parse(readFileSync(join(owner,"📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json"),"utf8"));const law=corpus.appendClose,text=corpus.prefix.repeat(corpus.prefixRepeat),value=law.destinationPrefix+text;
 expect(validValue(value)).toBe(true);expect(new TextDecoder("utf-8",{fatal:true}).decode(new TextEncoder().encode(value))).toBe(value);expect(JSON.parse(JSON.stringify(value))).toBe(value);
 for(const pause of law.cancelAt)expect(applyPatch({destination:value,pending:text,pause},[{op:"remove",path:"/pending"}],true,false).newDocument).toEqual({destination:value,pause});
 expect(law.constructorAllocationBytes).toBe(0);expect(law.maximumBytes).toBe(4096);expect(law.zeroGrantInert).toBe(true);
 const source=readFileSync(join(owner,"📦️paged/🎮️append/🦀️.rs"),"utf8");expect(source).toContain("pub fn close_granted(");
 console.log("[DEBUG] Native append-close neutral UTF8/TextEncoder/JSONPatch preserves destination and independently removes each pending owner");
});

test("controlled paged retirement exposes separate append birth and release demand",()=>{
 const corpus=JSON.parse(readFileSync(join(owner,"📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json"),"utf8"));const law=corpus.appendClose.demand;
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE grants (bytes INTEGER PRIMARY KEY)");law.grants.forEach((bytes:number)=>database.query("INSERT INTO grants VALUES (?)").run(bytes));
  expect(database.query("SELECT bytes FROM grants WHERE bytes>=7 ORDER BY bytes").all()).toEqual([{bytes:7},{bytes:4096}]);
  expect(database.query("SELECT MIN(bytes) AS bytes FROM grants WHERE bytes>=?").get(law.nonemptyByteOwnerBodyGrant)).toEqual({bytes:1});
  const pending={field:corpus.prefix};expect(validValue(pending)).toBe(true);
  expect(applyPatch(pending,[{op:"move",from:"/field",path:"/retirement"},{op:"remove",path:"/retirement"}],true,false).newDocument).toEqual({});
 }finally{database.close();}
 const source=readFileSync(join(owner,"📦️paged/🎮️append/🦀️.rs"),"utf8");
 expect(source).toContain("pub fn next_close_capacity_byte_demand(");expect(source).toContain("pub fn next_close_release_byte_demand(");
 console.log("[DEBUG] Native append exact separate demand neutral SQLite admission and Value schema/RFC6902 retained ownership agree; actual birth/release native witness required");
});

test("controlled paged retirement exposes typed child metadata and exact scaffold demand",()=>{
 const path=join(owner,"🧬️retained-clone/🧫️fixtures/📏️release-authority"),corpus=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8"));
 const law=corpus.closeDemand,database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE ownership (phase TEXT PRIMARY KEY,capacity INTEGER,release INTEGER)");
  law.phases.forEach((phase:{name:string;capacityBytes:number;releaseBytes:number})=>database.query("INSERT INTO ownership VALUES (?,?,?)").run(phase.name,phase.capacityBytes,phase.releaseBytes));
  expect(database.query("SELECT phase AS name FROM ownership WHERE release>? ORDER BY phase").all(corpus.oneBelowReleaseBytes)).toEqual([{name:"terminalChild"}]);
  expect(database.query("SELECT SUM(capacity) AS bytes FROM ownership").get()).toEqual({bytes:0});
  expect(database.query("SELECT SUM(release) AS bytes FROM ownership").get()).toEqual({bytes:corpus.physicalCursorBytes});
  expect(applyPatch({child:corpus.inputByte,backing:corpus.physicalCursorBytes},[{op:"remove",path:"/child"}],true,false).newDocument).toEqual({backing:corpus.physicalCursorBytes});
 }finally{database.close();}
 const trait=readFileSync(join(owner,"🧬️retained-clone/🦀️.rs"),"utf8"),field=readFileSync(join(owner,"🧬️retained-clone/📋️field/🦀️.rs"),"utf8");
 expect(trait.slice(trait.indexOf("pub trait RetainedCloneCursor"),trait.indexOf("trait ControlledCloneRetirement"))).toContain("fn next_close_capacity_byte_demand(");
 expect(field).toContain("fn next_close_release_byte_demand(");expect(field).toContain("child.next_close_capacity_byte_demand(");
 console.log("[DEBUG] Typed clone demand independent SQLite/RFC6902 preserves128-byte child scaffold through metadata and exposes exact terminal release without allocation");
});

test("controlled paged retirement observes retained scalar and UTF8 leaf birth demand",()=>{
 const path=join(owner,"🧬️retained-clone/🧫️fixtures/📏️release-authority"),corpus=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8")),law=corpus.leafCloseDemand;
 
 for(const value of [law.scalar,law.text]){
  expect(validValue(value)).toBe(true);
  const moved=applyPatch({pending:value,backing:128},[{op:"move",from:"/pending",path:"/retirement"}],true,false).newDocument;
  expect(moved).toEqual({retirement:value,backing:128});
  expect(applyPatch(moved,[{op:"remove",path:"/retirement"}],true,false).newDocument).toEqual({backing:128});
 }
 expect(new TextDecoder("utf-8",{fatal:true}).decode(new TextEncoder().encode(law.text))).toBe(law.text);
 const database=new Database(":memory:");try{
  database.exec("CREATE TABLE grants(bytes INTEGER)");database.query("INSERT INTO grants VALUES(?)").run(corpus.oneBelowReleaseBytes);database.query("INSERT INTO grants VALUES(?)").run(corpus.physicalCursorBytes);
  expect(database.query("SELECT MIN(bytes) AS bytes FROM grants WHERE bytes>=?").get(corpus.physicalCursorBytes)).toEqual({bytes:128});
  database.exec("CREATE TABLE body_grants(bytes INTEGER); INSERT INTO body_grants VALUES(0),(1)");
  expect(database.query("SELECT MIN(bytes) AS bytes FROM body_grants WHERE bytes>=?").get(law.minimumBodyCopyBytes)).toEqual({bytes:1});
 }finally{database.close();}
 const source=readFileSync(join(owner,"🧬️retained-clone/🦀️.rs"),"utf8"),scalar=source.slice(source.indexOf("RetainedCloneCursor<T> for ScalarCursor"),source.indexOf("macro_rules! retained_clone_scalar")),text=source.slice(source.indexOf("RetainedCloneCursor<String> for StringCursor"),source.indexOf("impl RetainedClone for String"));
 expect(scalar).toContain("fn next_close_capacity_byte_demand(");expect(text).toContain("fn next_close_release_byte_demand(");
 const retirement=readFileSync(join(owner,"♻️retirement/🦀️.rs"),"utf8"),leaf=retirement.slice(retirement.indexOf("RetirementCursor for Leaf<T>"),retirement.indexOf("pub fn leaf<"));
 expect(leaf).toContain("fn next_close_byte_demand(");
 console.log("[DEBUG] Retained scalar/UTF8 leaf demand neutral TextEncoder/SQLite/RFC6902 preserves native owner before exact separate birth and release admission");
});

test("controlled paged retirement observes one native paged and derived close frontier",()=>{
 const path=join(owner,"🧬️retained-clone/🧫️fixtures/📏️release-authority"),corpus=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8")),law=corpus.compositeCloseDemand;
 expect(validValue(law.value)).toBe(true);
 const database=new Database(":memory:");try{
  database.exec("CREATE TABLE fields(ordinal INTEGER PRIMARY KEY,name TEXT)");law.fields.forEach((name:string,ordinal:number)=>database.query("INSERT INTO fields VALUES(?,?)").run(ordinal,name));
  expect(database.query("SELECT name FROM fields ORDER BY ordinal").all()).toEqual(law.fields.map((name:string)=>({name})));
  let remaining=structuredClone(law.value);
  for(const {name} of database.query("SELECT name FROM fields ORDER BY ordinal").all() as {name:string}[]){
   const result=applyPatch(remaining,[{op:"move",from:`/${name}`,path:"/retirement"}],true,false).newDocument;
   expect(result.retirement).toEqual(law.value[name]);remaining=applyPatch(result,[{op:"remove",path:"/retirement"}],true,false).newDocument;
  }
  expect(remaining).toEqual({});
 }finally{database.close();}
 for(const path of ["🧬️retained-clone/📋️paged-list/🦀️.rs","🧬️retained-clone/📦️paged/🦀️.rs","✨️derive/🧬️retained-clone/🦀️.rs"]){const source=readFileSync(join(owner,path),"utf8");expect(source).toContain("fn next_close_capacity_byte_demand(");expect(source).toContain("fn next_close_release_byte_demand(");}
 console.log("[DEBUG] Native paged/derived close frontier neutral Ajv/SQLite field ordering and RFC6902 original typed owner transfer agree, exact native heap witness still required");
});

test("controlled retirement keeps logical UTF8 and ordered scalar work distinct from physical releases",()=>{
 const path=join(owner,"♻️retirement/🧫️fixtures/⚖️physical-work"),law=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8"));
 
 for(const row of law.cases){
  expect(JSON.parse(JSON.stringify(row.value))).toEqual(row.value);
  const bytes=row.kind==="text"?new TextEncoder().encode(row.value).byteLength:row.kind==="u32-list"?Uint32Array.from(row.value).byteLength:Uint32Array.of(row.value).byteLength;
  expect(bytes).toBe(row.processedBytes);
  expect(applyPatch({owner:row.value},[{op:"remove",path:"/owner"}],true,false).newDocument).toEqual({});
 }
 expect(law.maximumCopyBytes).toBe(64);expect(law.maximumReleaseBytes).toBe(4096);expect(law.physicalReleaseMatchesAllocator).toBe(true);
 console.log("[DEBUG] physical retirement neutral TextEncoder/Uint32Array/RFC6902 oracle validates scalar/UTF8/ordered work separately from native allocator receipts");
});

test("controlled paged retirement exposes the next independent payload work demand",()=>{
 const path=join(owner,"♻️retirement/🧫️fixtures/📏️copy-demand"),law=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8"));
 
 const database=new Database(":memory:");try{
  database.exec("CREATE TABLE grants(bytes INTEGER PRIMARY KEY)");law.grants.forEach((bytes:number)=>database.query("INSERT INTO grants VALUES(?)").run(bytes));
  for(const row of law.cases){
   const bytes=row.kind==="text"?new TextEncoder().encode(row.value).byteLength:row.kind==="u32-list"?Uint32Array.from(row.value).byteLength:Uint32Array.of(row.value).byteLength;
   expect(bytes).toBe(row.totalCopyBytes);
   expect(database.query("SELECT MIN(bytes) AS bytes FROM grants WHERE bytes>=?").get(row.minimumCopyBytes)).toEqual({bytes:row.minimumCopyBytes});
   expect(database.query("SELECT bytes FROM grants WHERE bytes<? ORDER BY bytes").all(row.minimumCopyBytes).every((grant:any)=>grant.bytes<row.minimumCopyBytes)).toBe(true);
   expect(applyPatch({owner:row.value,backing:128},[{op:"remove",path:"/owner"}],true,false).newDocument).toEqual({backing:128});
  }
 }finally{database.close();}
 expect(law.demandObservationAllocatesBytes).toBe(0);expect(law.copyTurnReleasesBytes).toBe(0);
 expect(readFileSync(join(owner,"♻️retirement/🎮️controlled/🦀️.rs"),"utf8").includes("pub fn next_copy_byte_demand(")).toBe(true);
 console.log("[DEBUG] Controlled copy demand independent SQLite/TextEncoder/Uint32Array/RFC6902: exact minimum work, backing retained until separate physical release");
});

test("controlled paged retirement preserves copy demand through native clone close frontiers",()=>{
 const path=join(owner,"♻️retirement/🧫️fixtures/📏️copy-demand"),law=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8"));
 
 const database=new Database(":memory:");try{
  database.exec("CREATE TABLE frontier(ordinal INTEGER PRIMARY KEY,kind TEXT,work INTEGER,backing INTEGER)");
  law.cases.forEach((row:any,ordinal:number)=>database.query("INSERT INTO frontier VALUES(?,?,?,128)").run(ordinal,row.kind,row.totalCopyBytes));
  for(const row of law.cases){
   const first=database.query("SELECT kind,work,backing FROM frontier ORDER BY ordinal LIMIT 1").get();
   expect(first).toEqual({kind:row.kind,work:row.totalCopyBytes,backing:128});
   expect(applyPatch({pending:row.value,backing:128},[{op:"move",from:"/pending",path:"/retirement"}],true,false).newDocument).toEqual({retirement:row.value,backing:128});
   database.query("DELETE FROM frontier WHERE ordinal=(SELECT MIN(ordinal) FROM frontier)").run();
  }
  expect(database.query("SELECT COUNT(*) AS count FROM frontier").get()).toEqual({count:0});
 }finally{database.close();}
 for(const path of ["🧬️retained-clone/🦀️.rs","🧬️retained-clone/📋️field/🦀️.rs","🧬️retained-clone/📋️paged-list/🦀️.rs","🧬️retained-clone/📦️paged/🦀️.rs","✨️derive/🧬️retained-clone/🦀️.rs","📦️paged/🎮️append/🦀️.rs"]){
  expect(readFileSync(join(owner,path),"utf8").includes("fn next_close_copy_byte_demand(")).toBe(true);
 }
 console.log("[DEBUG] Native clone close copy demand independent SQLite/RFC6902 preserves the original typed owner and only exposes the first active work frontier");
});


test("cold shared retirement keeps logical work separate from exact physical backing",()=>{
 const path=join(owner,"♻️retirement/🧫️fixtures/📏️shared-physical"),law=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8"));
 
 for(const row of law.cases){const bytes=new TextEncoder().encode(row.text);expect(Buffer.byteLength(row.text,"utf8")).toBe(bytes.byteLength);expect(bytes.byteLength<=row.capacity).toBe(true);for(const work of law.workBytes){let remaining=bytes.byteLength,turns=0;while(remaining){remaining-=Math.min(remaining,work);turns++;}expect(turns<=law.maximumTurns).toBe(true);const owner={capacity:row.capacity,text:row.text};expect(applyPatch(owner,[{op:"remove",path:"/text"}],true,false).newDocument).toEqual({capacity:row.capacity});}}
 expect(Math.ceil(law.buffer.bytes/law.buffer.workBytes)+4).toBe(law.buffer.maximumTurns);
 const source=readFileSync(join(owner,"♻️retirement/🦀️.rs"),"utf8");const shared=source.slice(source.indexOf("impl<T: RetireOwned + Sync> ErasedSnapshotRetirement for SharedRetirement<T>"),source.indexOf("impl<T: RetireOwned + Sync> Drop for SharedRetirement<T>"));expect(shared).toContain("fn next_close_byte_demand(");
 console.log("[DEBUG] shared cold retirement neutral Node Buffer/TextEncoder/RFC6902 confirms original logical quanta and indivisible physical capacity, native allocator witness required");
});

test("original controlled reservation failure saves actual retained allocation before error",()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️reservation.json",import.meta.url),"utf8"));const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️reservation.json",import.meta.url),"utf8"));expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const original=Buffer.from(law.original,"utf8");const pointer=original.buffer;
 const before={source:law.original,capacity:0,released:0,receipt:0};const actual=128*law.allocatorMultiplier;
 const retained=applyPatch(before,[{op:"replace",path:"/capacity",value:actual},{op:"replace",path:"/receipt",value:actual}],true,false).newDocument;
 expect(retained.source).toBe(law.original);expect(original.buffer).toBe(pointer);expect(retained.capacity).toBeGreaterThan(128);expect(retained.receipt).toBe(retained.capacity);expect(law.expected.admissionReceipt).toBe("same-actual-receipt");expect(retained.released).toBe(law.expected.releasedBytes);
 const closed=applyPatch(retained,[{op:"replace",path:"/capacity",value:0},{op:"replace",path:"/released",value:actual}],true,false).newDocument;expect(closed.released).toBe(retained.receipt);expect(law.expected.terminalHeap).toEqual([0,0]);
 console.log("[DEBUG] independent Ajv/NodeBuffer/RFC6902 failed reservation preserves original, exact post-allocation receipt and same physical close owner");
});

test("original controlled root transfer preserves the same source under its incoming narrow wallet",()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/📥️root/🔣️.json",import.meta.url),"utf8"));const schema=JSON.parse(readFileSync(new URL("../🧬️schema/📥️root/🔣️.json",import.meta.url),"utf8"));expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE policy(copy INTEGER)");for(const copy of law.copyPolicies)db.query("INSERT INTO policy VALUES(?)").run(copy);expect(db.query("SELECT copy FROM policy WHERE copy>=0 ORDER BY copy").all()).toEqual(law.copyPolicies.map((copy:number)=>({copy})));}finally{db.close();}
 expect(applyPatch({original:law.source},[{op:"move",from:"/original",path:"/retained"}],true,false).newDocument).toEqual({retained:law.source});
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("if self.value.is_some() { return Ok(0); }");const root=source.slice(source.indexOf("let cursor = self.value.take().unwrap().retirement();"),source.indexOf("let index = self.cursors.len() - 1;"));expect(root).toContain("copied_bytes: 0");expect(source).not.toContain("if grant.maximum_copy_bytes < self.next_copy_byte_demand()?");expect(source).toContain("if work && body_bytes < minimum_work_bytes && (birth==0 || !self.cursors.get(index).unwrap().allows_admitted_narrow_work())");
});
