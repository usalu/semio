/** 🎟️ Actual guest context keeps original input through funded root and payload admission before borrowing Reactor. */
import{test,expect}from"bun:test";import Ajv from"ajv";import{Database}from"bun:sqlite";import{readFileSync,existsSync}from"node:fs";
test("original source epoch is supplied by verified actor input and cannot be rebound by children",async()=>{
 const fixture=await Bun.file(new URL("🧫️fixtures/🎟️epoch.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🎟️epoch.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE turns(operation INTEGER,generation INTEGER,epoch INTEGER CHECK(epoch>0),PRIMARY KEY(operation,generation,epoch))");for(const epoch of fixture.epochs)db.query("INSERT INTO turns VALUES(?,?,?)").run(fixture.operation,fixture.generation,epoch);expect(db.query("SELECT epoch FROM turns ORDER BY epoch").all()).toEqual(fixture.epochs.map((epoch:number)=>({epoch})));expect(()=>db.query("INSERT INTO turns VALUES(?,?,?)").run(fixture.operation,fixture.generation,fixture.refusedEpoch)).toThrow();}finally{db.close();}
 const actor=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8"),job=readFileSync(new URL("../../../../🧵️job/🦀️.rs",import.meta.url),"utf8");expect(actor).toContain("cx.bind_original_source_epoch(retained.epoch)?");expect(job).toContain("pub fn original_source_epoch(");expect(job).toContain("pub fn bind_original_source_epoch(");
});
test("original actor context owner retains denied input through separate paid authorities and funded closure",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE original_phase(phase TEXT PRIMARY KEY,items INTEGER,ready INTEGER)");for(const row of law.phases)db.run("INSERT INTO original_phase VALUES(?,?,?)",row.phase,row.items,Number(row.ready));expect(db.query("SELECT sum(items) AS items,sum(ready) AS ready FROM original_phase").get()).toEqual({items:4,ready:1});}finally{db.close();}
 const context=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");for(const text of ['ManuallyDrop<Option<T>>','CancelToken::admit_root(','JobPayloadAuthority::admit(','source.borrow_context(','pub fn take_input(','pub fn close_step(','retained.epoch<=self.epoch','!subset(budget.retained,retained.grant)','CancelTokenRetirement::from_token('])expect(context.includes(text)).toBe(true);expect(context.includes('root_now(')).toBe(false);
});
test("original actor context keeps input and same admitted authorities through denied phases and original context borrowing",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE original_phase(phase TEXT PRIMARY KEY,items INTEGER,ready INTEGER)");for(const row of law.phases)db.run("INSERT INTO original_phase VALUES(?,?,?)",row.phase,row.items,Number(row.ready));expect(db.query("SELECT sum(items) AS items,sum(ready) AS ready FROM original_phase").get()).toEqual({items:4,ready:1});expect(()=>db.run("INSERT INTO original_phase VALUES('root',1,0)")).toThrow();expect(law.admissionGrant[1]).toBe(0);expect(law.originalInputCapacity).toBe(97);}finally{db.close();}
 const path=new URL("../🦀️.rs",import.meta.url);expect(existsSync(path)).toBe(true);const context=readFileSync(path,"utf8");for(const text of ['pub struct OriginalActorContext','CancelToken::admit_root(','JobPayloadAuthority::admit(','source.borrow_context(','ManuallyDrop<Option<T>>','fn take_input('])expect(context.includes(text)).toBe(true);
});

test("actual actor source parent is paid before lending its original context reference",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();const valid=new Ajv({strict:true}).compile(schema);expect(valid(law)).toBe(true);expect(valid({...law,sourceParentBeforeBorrow:false})).toBe(false);const db=new Database(":memory:");try{db.run("CREATE TABLE source_admission(phase TEXT,items INTEGER,ready INTEGER)");for(const row of law.phases)db.query("INSERT INTO source_admission VALUES(?,?,?)").run(row.phase,row.items,Number(row.ready));expect(db.query("SELECT sum(items) AS items,sum(ready) AS ready FROM source_admission WHERE phase<>'borrow'").get()).toEqual({items:3,ready:0});}finally{db.close()}const actor=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8"),job=readFileSync(new URL("../../../../🧵️job/🦀️.rs",import.meta.url),"utf8");expect(actor.includes("OriginalCallerSourceParent::admit(")).toBe(true);expect(actor.includes("source.borrow_context(")).toBe(true);expect(job.includes("pub fn original_source_parent(")).toBe(true);
});

test("original actor source totals decrease across separate bounded call policies", async () => {
 const law = await Bun.file(new URL("🧫️fixtures/🔣️.json", import.meta.url)).json();
 const schema = await Bun.file(new URL("../🧬️schema/🔣️.json", import.meta.url)).json();
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db = new Database(":memory:");
 try {
  db.exec("CREATE TABLE source(items INTEGER CHECK(items>=0),copy INTEGER CHECK(copy>=0),capacity INTEGER CHECK(capacity>=0),release INTEGER CHECK(release>=0),depth INTEGER,fuel INTEGER CHECK(fuel>=0)); CREATE TABLE policy(items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER,depth INTEGER)");
  db.query("INSERT INTO source VALUES(?,?,?,?,?,?)").run(...law.totalGrant, law.work.fuel);
  db.query("INSERT INTO policy VALUES(?,?,?,?,?)").run(...law.bodyGrant);
  expect(db.query("SELECT count(*) AS count FROM source s,policy p WHERE p.items<=s.items AND p.copy<=s.copy AND p.capacity<=s.capacity AND p.release<=s.release AND p.depth<=s.depth").get()).toEqual({count:1});
  const admission = law.phases.reduce((items:number,row:{items:number}) => items+row.items,0);
  db.query("UPDATE source SET items=items-?,fuel=fuel-?").run(admission,admission);
  expect(db.query("SELECT items,fuel FROM source").get()).toEqual({items:60,fuel:60});
  db.exec("UPDATE source SET items=items-1,copy=copy-41,fuel=fuel-1");
  db.query("UPDATE source SET items=items-1,release=release-?,fuel=fuel-1").run(law.originalInputCapacity);
  expect(db.query("SELECT items,copy,release,fuel FROM source").get()).toEqual({items:58,copy:4055,release:262047,fuel:58});
  expect(() => db.exec("UPDATE source SET items=items-65")).toThrow();
 } finally { db.close(); }
 const context = readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 for(const text of ["physical_spent:RetainedCloneProgress","pub fn remaining_grant(","cx.bind_original_physical_recipient(recipient)?","Self::validate_policy(retained,policy)?"])expect(context).toContain(text);
});

test("actor native receiving retains original decoder and exact child receipt across failure and paid root closure",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🛬️decoder.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🛬️decoder.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE original_turn(id TEXT PRIMARY KEY,capacity TEXT,failure INTEGER,admitted INTEGER)");for(const row of law.cases){const admitted=row.capacity!=="zero";expect(admitted).toBe(row.admitted);db.query("INSERT INTO original_turn VALUES(?,?,?,?)").run(row.id,row.capacity,+row.semanticFailure,+row.admitted);}expect(db.query("SELECT sum(admitted) AS born,sum(admitted*failure) AS failed_but_retained FROM original_turn").get()).toEqual({born:2,failed_but_retained:1});expect(db.query("SELECT count(*) AS denied FROM original_turn WHERE capacity='zero' AND admitted=0").get()).toEqual({denied:2});}finally{db.close();}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8"),native=readFileSync(new URL("🛬️decoder/🦀️.rs",import.meta.url),"utf8");for(const text of ["decoder:ManuallyDrop<OriginalJobDecodeOwner>","OriginalJobDecodeOwner::empty()","with_original_decoder(&mut self.decoder)","self.decoder.retirement_demands(body)","self.decoder.terminal_is_empty()","Original(OriginalTurnFailure)"])expect(source).toContain(text);
 const close=source.slice(source.indexOf("fn close_inner("),source.indexOf("fn subset("));expect(close.indexOf("self.decoder.close_step(grant)")).toBeLessThan(close.indexOf("self.source.as_mut()"));expect(native).toContain("control.receive_shared(");expect(native).toContain("consume_receiving_retained(grant,progress)");expect(native).toContain("OriginalActorCallError::Semantic");expect(native).toContain("observe(||drop(slot))");
});
