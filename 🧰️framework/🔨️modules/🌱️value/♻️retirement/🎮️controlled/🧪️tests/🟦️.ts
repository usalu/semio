import Ajv from "ajv";
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve, join, dirname } from "node:path";
import { applyPatch as applyJsonPatch, type Operation } from "fast-json-patch";
import { Database } from "bun:sqlite";
import { inspectRustPathMounts } from "@semio-tech/framework-compiler-rs/syntax/module-tree";
import { between, readRetirementCopyDemandLaw, readRetirementPhysicalWorkLaw, retirementCopyDemand } from "../../🧪️tests/🧫️fixtures/🟦️.ts";

const owner = resolve(import.meta.dir, "../../..");
const CONTRACT_ROOT = join(owner, "🧬️retained-clone/🧬️contract/🦀️.rs");

/** 🧬️ The retained clone contract root together with the cursor modules it mounts from its own folder (sibling owners and tests excluded). */
function contractSource(): string {
  const root = readFileSync(CONTRACT_ROOT, "utf8");
  return [root, ...inspectRustPathMounts(root).filter(mount => !mount.path.startsWith("../") && !mount.path.startsWith("🧪️tests")).map(mount => readFileSync(join(dirname(CONTRACT_ROOT), mount.path), "utf8"))].join("\n");
}

/** 🩹 Applies a validated JSON patch to a copy and hands back the patched document as untrusted data. */
function patched(document: Readonly<Record<string, unknown>>, operations: readonly Operation[]): Record<string, unknown> {
  return applyJsonPatch<Record<string, unknown>>({ ...document }, [...operations], true, false).newDocument;
}

test("erased controlled retirement preserves the neutral independent-grant contract", () => {
  const law = JSON.parse(readFileSync(join(owner, "♻️retirement/🧪️tests/🧫️fixtures/🎮️erased-controlled/🔣️.json"), "utf8"));
  const text = new TextDecoder("utf-8", { fatal: true }).decode(new TextEncoder().encode(law.text));
  expect(text).toBe(law.text);
  expect(JSON.parse(JSON.stringify({ text }))).toEqual({ text });
  expect(patched({ candidate: { text } }, [{ op: "remove", path: "/candidate" }])).toEqual({});
  expect(law.reservedCapacity).toBeGreaterThan(Buffer.byteLength(text));
  expect(law.maximumCopyBytes).toBe(3);
  expect(law.maximumItems).toBe(1);
  expect(law.unsupportedKind).toBe("unsupportedOwner");
  expect(law.unsupportedEffects).toEqual({bornBytes:0,releasedBytes:0});
});
const validValue = new Ajv({strict:false}).compile(JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8")));

test("controlled paged retirement conserves independently admitted native tree owners", () => {
  const law = JSON.parse(readFileSync(join(owner, "📦️paged/🧪️tests/🧫️fixtures/🎮️native-owner/🔣️.json"), "utf8")).controlledRetirement;
  let value: unknown = null;
  for (let index = 0; index < law.depth; index++) value = {label:law.label,children:[value]};
  expect(validValue(value)).toBe(true);
  expect(JSON.stringify(JSON.parse(JSON.stringify(value)))).toBe(JSON.stringify(value));
  for (const pause of law.pauseAt) {
    const owners = {tree:structuredClone(value),pause};
    expect(patched(owners,[{op:"remove",path:"/tree"}])).toEqual({pause});
  }
  const close = JSON.parse(readFileSync(join(owner, "📦️paged/🧪️tests/🧫️fixtures/🎮️native-owner/🔣️.json"), "utf8")).cloneClose;
  const corpus = JSON.parse(readFileSync(join(owner, "📦️paged/🧪️tests/🧫️fixtures/🎮️native-owner/🔣️.json"), "utf8"));
  const label = corpus.prefix.repeat(close.textRepeat);
  const record = {tree:value,optional:[label,close.optionalValues],objects:{[label]:close.objectValue}};
  expect(validValue(record)).toBe(true);
  expect(JSON.parse(JSON.stringify(record))).toEqual(record);
  for (const pause of close.pauseAt) expect(patched({candidate:structuredClone(record),pause},[{op:"remove",path:"/candidate"}])).toEqual({pause});
  expect(close.zeroGrantInert).toBe(true);
  expect(close.maximumBytes).toBe(law.maximumBytes);
  expect(law.maximumItems).toBe(1);
  expect(law.maximumBytes).toBe(4096);
});


test("controlled paged retirement closes appended UTF8 without unadmitted scaffold births",()=>{
 const corpus=JSON.parse(readFileSync(join(owner,"📦️paged/🧪️tests/🧫️fixtures/🎮️native-owner/🔣️.json"),"utf8"));const law=corpus.appendClose,text=corpus.prefix.repeat(corpus.prefixRepeat),value=law.destinationPrefix+text;
 expect(validValue(value)).toBe(true);expect(new TextDecoder("utf-8",{fatal:true}).decode(new TextEncoder().encode(value))).toBe(value);expect(JSON.parse(JSON.stringify(value))).toBe(value);
 for(const pause of law.cancelAt)expect(patched({destination:value,pending:text,pause},[{op:"remove",path:"/pending"}])).toEqual({destination:value,pause});
 expect(law.constructorAllocationBytes).toBe(0);expect(law.maximumBytes).toBe(4096);expect(law.zeroGrantInert).toBe(true);
 const source=readFileSync(join(owner,"📦️paged/🎮️append/🦀️.rs"),"utf8");expect(source).toContain("pub fn close_step(");expect(source).toContain("self.controlled_close.next_owner_demands::<String>");expect(source).toContain("self.controlled_close.begin_granted(&mut self.pending, grant)");
});

test("controlled paged retirement exposes separate append birth and release demand",()=>{
 const corpus=JSON.parse(readFileSync(join(owner,"📦️paged/🧪️tests/🧫️fixtures/🎮️native-owner/🔣️.json"),"utf8"));const law=corpus.appendClose.demand;
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE grants (bytes INTEGER PRIMARY KEY)");law.grants.forEach((bytes:number)=>database.query("INSERT INTO grants VALUES (?)").run(bytes));
  expect(database.query("SELECT bytes FROM grants WHERE bytes>=7 ORDER BY bytes").all()).toEqual([{bytes:7},{bytes:4096}]);
  expect(database.query("SELECT MIN(bytes) AS bytes FROM grants WHERE bytes>=?").get(law.nonemptyByteOwnerBodyGrant)).toEqual({bytes:1});
  const pending={field:corpus.prefix};expect(validValue(pending)).toBe(true);
  expect(patched(pending,[{op:"move",from:"/field",path:"/retirement"},{op:"remove",path:"/retirement"}])).toEqual({});
 }finally{database.close();}
 const source=readFileSync(join(owner,"📦️paged/🎮️append/🦀️.rs"),"utf8");
 expect(source).toContain("pub fn next_close_capacity_byte_demand(");expect(source).toContain("pub fn next_close_release_byte_demand(");
});

test("controlled paged retirement exposes typed child metadata and exact scaffold demand",()=>{
 const path=join(owner,"🧬️retained-clone/🧪️tests/🧫️fixtures/📏️release-authority"),corpus=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8"));
 const law=corpus.closeDemand,database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE ownership (phase TEXT PRIMARY KEY,capacity INTEGER,release INTEGER)");
  law.phases.forEach((phase:{name:string;capacityBytes:number;releaseBytes:number})=>database.query("INSERT INTO ownership VALUES (?,?,?)").run(phase.name,phase.capacityBytes,phase.releaseBytes));
  expect(database.query("SELECT phase AS name FROM ownership WHERE release>? ORDER BY phase").all(corpus.oneBelowReleaseBytes)).toEqual([{name:"terminalChild"}]);
  expect(database.query("SELECT SUM(capacity) AS bytes FROM ownership").get()).toEqual({bytes:0});
  expect(database.query("SELECT SUM(release) AS bytes FROM ownership").get()).toEqual({bytes:corpus.physicalCursorBytes});
  expect(patched({child:corpus.inputByte,backing:corpus.physicalCursorBytes},[{op:"remove",path:"/child"}])).toEqual({backing:corpus.physicalCursorBytes});
 }finally{database.close();}
 const trait=contractSource(),field=readFileSync(join(owner,"🧬️retained-clone/📋️field/🦀️.rs"),"utf8");
 expect(between(trait,"pub trait RetainedCloneCursor","pub struct RetainedCloneClose")).toContain("fn next_close_capacity_byte_demand(");
 expect(field).toContain("fn next_close_release_byte_demand(");expect(field).toContain("nested_retained_clone_close_demands::<T>(child.as_ref(),body)");expect(field).toContain("Ok(self.close_demands(body)?.capacity_bytes)");
});

test("controlled paged retirement observes retained scalar and UTF8 leaf birth demand",()=>{
 const path=join(owner,"🧬️retained-clone/🧪️tests/🧫️fixtures/📏️release-authority"),corpus=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8")),law=corpus.leafCloseDemand;

 for(const value of [law.scalar,law.text]){
  expect(validValue(value)).toBe(true);
  const moved=patched({pending:value,backing:128},[{op:"move",from:"/pending",path:"/retirement"}]);
  expect(moved).toEqual({retirement:value,backing:128});
  expect(patched(moved,[{op:"remove",path:"/retirement"}])).toEqual({backing:128});
 }
 expect(new TextDecoder("utf-8",{fatal:true}).decode(new TextEncoder().encode(law.text))).toBe(law.text);
 const database=new Database(":memory:");try{
  database.exec("CREATE TABLE grants(bytes INTEGER)");database.query("INSERT INTO grants VALUES(?)").run(corpus.oneBelowReleaseBytes);database.query("INSERT INTO grants VALUES(?)").run(corpus.physicalCursorBytes);
  expect(database.query("SELECT MIN(bytes) AS bytes FROM grants WHERE bytes>=?").get(corpus.physicalCursorBytes)).toEqual({bytes:128});
  database.exec("CREATE TABLE body_grants(bytes INTEGER); INSERT INTO body_grants VALUES(0),(1)");
  expect(database.query("SELECT MIN(bytes) AS bytes FROM body_grants WHERE bytes>=?").get(law.minimumBodyCopyBytes)).toEqual({bytes:1});
 }finally{database.close();}
 const source=contractSource(),scalar=between(source,"RetainedCloneCursor<T> for ScalarCursor","macro_rules! retained_clone_scalar"),text=between(source,"RetainedCloneCursor<String> for StringCursor","impl RetainedClone for String");
 expect(scalar).toContain("fn next_close_capacity_byte_demand(");expect(text).toContain("fn next_close_release_byte_demand(");
 const retirement=readFileSync(join(owner,"♻️retirement/🦀️.rs"),"utf8"),leaf=between(retirement,"RetirementCursor for Leaf<T>","pub fn leaf<");
 expect(leaf).toContain("fn next_close_byte_demand(");
});

test("controlled paged retirement observes one native paged and derived close frontier",()=>{
 const path=join(owner,"🧬️retained-clone/🧪️tests/🧫️fixtures/📏️release-authority"),corpus=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8")),law=corpus.compositeCloseDemand;
 expect(validValue(law.value)).toBe(true);
 const database=new Database(":memory:");try{
  database.exec("CREATE TABLE fields(ordinal INTEGER PRIMARY KEY,name TEXT)");law.fields.forEach((name:string,ordinal:number)=>database.query("INSERT INTO fields VALUES(?,?)").run(ordinal,name));
  expect(database.query("SELECT name FROM fields ORDER BY ordinal").all()).toEqual(law.fields.map((name:string)=>({name})));
  let remaining=structuredClone(law.value);
  for(const {name} of database.query("SELECT name FROM fields ORDER BY ordinal").all() as {name:string}[]){
   const result=patched(remaining,[{op:"move",from:`/${name}`,path:"/retirement"}]);
   expect(result.retirement).toEqual(law.value[name]);remaining=patched(result,[{op:"remove",path:"/retirement"}]);
  }
  expect(remaining).toEqual({});
 }finally{database.close();}
 for(const path of ["🧬️retained-clone/📋️paged-list/🦀️.rs","🧬️retained-clone/📦️paged/🦀️.rs"]){const source=readFileSync(join(owner,path),"utf8");expect(source).toContain("fn next_close_capacity_byte_demand(");expect(source).toContain("fn next_close_release_byte_demand(");}
 const derive=readFileSync(join(owner,"✨️derive/🧬️retained-clone/🦀️.rs"),"utf8"),methods=derive.slice(derive.indexOf("fn close_demand_methods"),derive.indexOf("fn expand_struct"));
 for(const[method,axis]of[["next_close_capacity_byte_demand","capacity_bytes"],["next_close_release_byte_demand","release_bytes"]])expect(methods).toContain(`("${method}","${axis}")`);
 expect(methods).toContain('if axis=="capacity_bytes" {(quote!(,maximum_body_bytes:usize),quote!(maximum_body_bytes))}');
 expect(methods).toContain("fn #method(&self #argument)");expect(methods).toContain("RetainedCloneCursor::close_demands(self,#hint)?.#axis");
 expect(methods).toContain("nested_retained_clone_close_demands::<#ty>(&self.#cursor,maximum_body_bytes)");
});

test("controlled retirement keeps logical UTF8 and ordered scalar work distinct from physical releases",()=>{
 const law=readRetirementPhysicalWorkLaw();

 for(const row of law.cases){
  expect(JSON.parse(JSON.stringify(row.value))).toEqual(row.value);
  expect(retirementCopyDemand(row).totalCopyBytes).toBe(row.processedBytes);
  expect(patched({owner:row.value},[{op:"remove",path:"/owner"}])).toEqual({});
 }
 expect(law.maximumCopyBytes).toBe(64);expect(law.maximumReleaseBytes).toBe(4096);expect(law.physicalReleaseMatchesAllocator).toBe(true);
});

test("controlled paged retirement exposes the next independent payload work demand",()=>{
 const law=readRetirementCopyDemandLaw();

 const database=new Database(":memory:");try{
  database.exec("CREATE TABLE grants(bytes INTEGER PRIMARY KEY)");law.grants.forEach((bytes:number)=>database.query("INSERT INTO grants VALUES(?)").run(bytes));
  for(const row of law.cases){
   expect(retirementCopyDemand(row)).toEqual({minimumCopyBytes:row.minimumCopyBytes,totalCopyBytes:row.totalCopyBytes});
   expect(database.query("SELECT MIN(bytes) AS bytes FROM grants WHERE bytes>=?").get(row.minimumCopyBytes)).toEqual({bytes:row.minimumCopyBytes});
   expect(database.query("SELECT bytes FROM grants WHERE bytes<? ORDER BY bytes").all(row.minimumCopyBytes).every((grant:any)=>grant.bytes<row.minimumCopyBytes)).toBe(true);
   expect(patched({owner:row.value,backing:128},[{op:"remove",path:"/owner"}])).toEqual({backing:128});
  }
 }finally{database.close();}
 expect(law.demandObservationAllocatesBytes).toBe(0);expect(law.copyTurnReleasesBytes).toBe(0);
 expect(readFileSync(join(owner,"♻️retirement/🎮️controlled/🦀️.rs"),"utf8").includes("pub fn next_copy_byte_demand(")).toBe(true);
});

test("controlled paged retirement preserves copy demand through native clone close frontiers",()=>{
 const law=readRetirementCopyDemandLaw();

 const database=new Database(":memory:");try{
  database.exec("CREATE TABLE frontier(ordinal INTEGER PRIMARY KEY,kind TEXT,work INTEGER,backing INTEGER)");
  law.cases.forEach((row,ordinal:number)=>database.query("INSERT INTO frontier VALUES(?,?,?,128)").run(ordinal,row.kind,row.totalCopyBytes));
  for(const row of law.cases){
   const first=database.query("SELECT kind,work,backing FROM frontier ORDER BY ordinal LIMIT 1").get();
   expect(first).toEqual({kind:row.kind,work:row.totalCopyBytes,backing:128});
   expect(patched({pending:row.value,backing:128},[{op:"move",from:"/pending",path:"/retirement"}])).toEqual({retirement:row.value,backing:128});
   database.query("DELETE FROM frontier WHERE ordinal=(SELECT MIN(ordinal) FROM frontier)").run();
  }
  expect(database.query("SELECT COUNT(*) AS count FROM frontier").get()).toEqual({count:0});
 }finally{database.close();}
 const derive=readFileSync(join(owner,"✨️derive/🧬️retained-clone/🦀️.rs"),"utf8"),methods=derive.slice(derive.indexOf("fn close_demand_methods"),derive.indexOf("fn expand_struct"));
 expect(methods).toContain('("next_close_copy_byte_demand","copy_bytes")');expect(methods).toContain("fn #method(&self #argument)");expect(methods).toContain("RetainedCloneCursor::close_demands(self,#hint)?.#axis");
 for(const path of ["🧬️retained-clone/🧬️contract/🦀️.rs","🧬️retained-clone/📋️field/🦀️.rs","🧬️retained-clone/📋️paged-list/🦀️.rs","🧬️retained-clone/📦️paged/🦀️.rs","📦️paged/🎮️append/🦀️.rs"]){
  expect((join(owner,path)===CONTRACT_ROOT?contractSource():readFileSync(join(owner,path),"utf8")).includes("fn next_close_copy_byte_demand(")).toBe(true);
 }
});


test("cold shared retirement keeps logical work separate from exact physical backing",()=>{
 const path=join(owner,"♻️retirement/🧪️tests/🧫️fixtures/📏️shared-physical"),law=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8"));

 for(const row of law.cases){const bytes=new TextEncoder().encode(row.text);expect(Buffer.byteLength(row.text,"utf8")).toBe(bytes.byteLength);expect(bytes.byteLength<=row.capacity).toBe(true);for(const work of law.workBytes){let remaining=bytes.byteLength,turns=0;while(remaining){remaining-=Math.min(remaining,work);turns++;}expect(turns<=law.maximumTurns).toBe(true);const owner={capacity:row.capacity,text:row.text};expect(patched(owner,[{op:"remove",path:"/text"}])).toEqual({capacity:row.capacity});}}
 expect(Math.ceil(law.buffer.bytes/law.buffer.workBytes)+4).toBe(law.buffer.maximumTurns);
 const shared=between(readFileSync(join(owner,"♻️retirement/🔗️arc-custody/🦀️.rs"),"utf8"),"impl<T:RetireOwned+Sync> SharedControlledRetirement<T> {","RetireOwned for Arc<T>");expect(shared).toContain("fn next_close_byte_demand(");expect(shared).toContain("Arc::try_unwrap(source)");expect(shared).toContain("owned.step(grant)");
});

test("original controlled reservation failure saves actual retained allocation before error",()=>{
 const law=JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️reservation.json",import.meta.url),"utf8"));const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️reservation.json",import.meta.url),"utf8"));expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const original=Buffer.from(law.original,"utf8");const pointer=original.buffer;
 const before={source:law.original,capacity:0,released:0,receipt:0};const actual=128*law.allocatorMultiplier;
 const retained=patched(before,[{op:"replace",path:"/capacity",value:actual},{op:"replace",path:"/receipt",value:actual}]);
 expect(retained.source).toBe(law.original);expect(original.buffer).toBe(pointer);expect(retained.capacity).toBeGreaterThan(128);expect(retained.receipt).toBe(retained.capacity);expect(law.expected.admissionReceipt).toBe("same-actual-receipt");expect(retained.released).toBe(law.expected.releasedBytes);
 const closed=patched(retained,[{op:"replace",path:"/capacity",value:0},{op:"replace",path:"/released",value:actual}]);expect(closed.released).toBe(retained.receipt);expect(law.expected.terminalHeap).toEqual([0,0]);
});

test("original controlled root transfer preserves the same source under its incoming narrow wallet",()=>{
 const law=JSON.parse(readFileSync(new URL("🧫️fixtures/📥️root/🔣️.json",import.meta.url),"utf8"));const schema=JSON.parse(readFileSync(new URL("../🧬️schema/📥️root/🔣️.json",import.meta.url),"utf8"));expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE policy(copy INTEGER)");for(const copy of law.copyPolicies)db.query("INSERT INTO policy VALUES(?)").run(copy);expect(db.query("SELECT copy FROM policy WHERE copy>=0 ORDER BY copy").all()).toEqual(law.copyPolicies.map((copy:number)=>({copy})));}finally{db.close();}
 expect(patched({original:law.source},[{op:"move",from:"/original",path:"/retained"}])).toEqual({retained:law.source});
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("if self.value.is_some() || self.cursors.is_empty() { return Ok(0); }");const root=source.slice(source.indexOf("let cursor = self.value.take().unwrap().retirement();"),source.indexOf("let index = self.cursors.len() - 1;"));expect(root).toContain("copied_bytes: 0");expect(source).not.toContain("if grant.maximum_copy_bytes < self.next_copy_byte_demand()?");expect(source).toContain("if work && body_bytes < minimum_work_bytes && (birth==0 || !self.cursors.get(index).unwrap().allows_admitted_narrow_work())");
});


test("original controlled selected constructor quote preserves future UInt128 atoms and independent source values",()=>{
 const law=JSON.parse(readFileSync(new URL("🧫️fixtures/🎯️selected/🔣️.json",import.meta.url),"utf8")),schema=JSON.parse(readFileSync(new URL("../🧬️schema/🎯️selected/🔣️.json",import.meta.url),"utf8"));const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,futureAtomBytes:0})).toBe(false);expect(validate({...law,selectedChildCopyBytes:16})).toBe(false);
 for(const text of law.source){const value=BigInt(text),bytes=new DataView(new ArrayBuffer(law.futureAtomBytes));bytes.setBigUint64(0,value&((1n<<64n)-1n),true);bytes.setBigUint64(8,value>>64n,true);expect((bytes.getBigUint64(0,true)+(bytes.getBigUint64(8,true)<<64n)).toString()).toBe(text);const moved=patched({original:text},[{op:"move",from:"/original",path:"/child"}]);expect(moved).toEqual({child:text});expect(patched(moved,[{op:"remove",path:"/child"}])).toEqual({});}
 expect(law.copyGrants).toEqual([1,3,64]);expect(law.quoteHeap).toEqual([0,0]);
 if(law.frontierBacking){expect(law.frontierBacking).toBe("sameOriginalReserveOneEvent");const db=new Database(":memory:");db.run("CREATE TABLE backing(event TEXT PRIMARY KEY, quoted INTEGER NOT NULL, reserved INTEGER NOT NULL CHECK (quoted=reserved))");db.run("INSERT INTO backing VALUES ('original-full-page',4096,4096)");expect(()=>db.run("INSERT INTO backing VALUES ('partial-quote',16,4096)")).toThrow();db.close();}
});


test("original controlled selected array leaf preserves future Float32 atoms and independent source values",()=>{
 const law=JSON.parse(readFileSync(new URL("🧫️fixtures/🎯️array/🔣️.json",import.meta.url),"utf8")),schema=JSON.parse(readFileSync(new URL("../🧬️schema/🎯️array/🔣️.json",import.meta.url),"utf8"));const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,futureElementBytes:0})).toBe(false);expect(validate({...law,selectedChildCopyBytes:4})).toBe(false);
 for(const row of law.source){const bytes=new DataView(new ArrayBuffer(law.futureArrayBytes));row.forEach((value:number,index:number)=>bytes.setFloat32(index*law.futureElementBytes,value,true));expect(row.map((_:number,index:number)=>bytes.getFloat32(index*law.futureElementBytes,true))).toEqual(row);let original:Record<string,unknown>={array:[...row],leaves:[]};for(let index=row.length-1;index>=0;index--){original=patched(original,[{op:"move",from:`/array/${index}`,path:"/leaves/-"}]);}expect(original.array).toEqual([]);expect(original.leaves).toEqual([...row].reverse());}
 expect(law.copyGrants).toEqual([1,3,64]);expect(law.quoteHeap).toEqual([0,0]);
 if(law.frontierBacking){expect(law.frontierBacking).toBe("sameOriginalReserveOneEvent");const db=new Database(":memory:");db.run("CREATE TABLE backing(event TEXT PRIMARY KEY, quoted INTEGER NOT NULL, reserved INTEGER NOT NULL CHECK (quoted=reserved))");db.run("INSERT INTO backing VALUES ('original-full-page',4096,4096)");expect(()=>db.run("INSERT INTO backing VALUES ('partial-quote',16,4096)")).toThrow();db.close();}
});

test("unsupported original root retains custody across zero-allocation refusal",()=>{
 const law=JSON.parse(readFileSync(new URL("🧫️fixtures/📥️root/🔣️.json",import.meta.url),"utf8"));const schema=JSON.parse(readFileSync(new URL("../🧬️schema/📥️root/🔣️.json",import.meta.url),"utf8"));expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const owner=law.unsupportedOriginal;const db=new Database(":memory:");try{const result=db.query("SELECT ? + ? AS heap, ? = ? AND ? AS exact").get(owner.constructorHeapBytes,owner.refusalHeapBytes,owner.demandRefusal,owner.stepRefusal,Number(owner.sameOriginalRecoverable)) as {heap:number;exact:number};expect(result).toEqual({heap:0,exact:1});}finally{db.close();}const original={text:law.source};const refused={original};expect(refused.original).toBe(original);const recovered=refused.original;expect(recovered).toBe(original);console.log("[DEBUG] retained unsupported root neutral/Ajv/SQLite ownership refusal contract passed");
});
