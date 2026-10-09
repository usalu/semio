import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {applyPatch} from "fast-json-patch";
test("owned member batch retains exact ordered source across refusal and cancellation",()=>{
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const source=structuredClone(fixture.values);
 for(const stop of [0,1,2,3]){
  let state={n:0};for(const value of source.slice(0,stop))state=applyPatch(state,[{op:"replace",path:"/n",value}],true,false).newDocument;
  expect(source).toEqual(fixture.values);expect(JSON.parse(JSON.stringify(source))).toEqual(fixture.values);
  if(stop===source.length)expect(state.n).toBe(fixture.expectedAfter);
 }
 expect(new TextEncoder().encode(fixture.metadata.actor).length).toBeLessThanOrEqual(fixture.grant.maximumCopyBytes);
 expect(fixture.grant.maximumItems).toBe(1);expect(fixture.grant.maximumReleaseBytes).toBe(4096);
 expect(fixture.transport).toEqual({apply:"typed",preview:"wire",applyingRequestsWireCodec:false});
 let typed={n:0};for(const value of source)typed=applyPatch(typed,[{op:"replace",path:"/n",value}],true,false).newDocument;
 const wire=JSON.parse(JSON.stringify(source));let preview={n:0};for(const value of wire)preview=applyPatch(preview,[{op:"replace",path:"/n",value}],true,false).newDocument;
 expect(typed).toEqual(preview);expect(typed.n).toBe(fixture.expectedAfter);
 console.log("[DEBUG] Owned member batch JSON Patch oracle preserves ordered original source through four admission/cancellation positions");
});

test("owned source census advances one exact ordered row and preserves cancellation prefix",()=>{
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 for(const count of fixture.census.rowCounts){
  const rows=Array.from({length:count},(_,index)=>fixture.values[index%fixture.values.length]);
  let state={n:0};let visited=0;
  for(const value of rows){state=applyPatch(state,[{op:"replace",path:"/n",value}],true,false).newDocument;visited+=fixture.census.maximumPreflightsPerTurn;expect(visited).toBeLessThanOrEqual(count);}
  expect(state.n).toBe(rows.at(-1));expect(JSON.parse(JSON.stringify(rows))).toEqual(rows);expect(visited).toBe(count);
 }
 console.log("[DEBUG] owned source JSON Patch census oracle: 1/3/257 exact forward rows, one row per turn and every serialized source retained");
});

test("owned scalar source declares copy work independently from physical release",()=>{
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 expect(Int32Array.BYTES_PER_ELEMENT).toBe(fixture.copyDemand.minimumCopyBytes);
 for(const count of fixture.copyDemand.rowCounts){
  const values=Array.from({length:count},(_,index)=>fixture.values[index%fixture.values.length]);
  const oracle=Int32Array.from(values);
  expect(oracle.byteLength).toBe(count*fixture.copyDemand.minimumCopyBytes);
  expect(Array.from(oracle)).toEqual(values);
  expect(fixture.copyDemand.zeroCopyReleasesBytes).toBe(0);
 }
 expect(readFileSync(resolve(import.meta.dir,"../🦀️.rs"),"utf8").includes("pub fn next_copy_byte_demand")).toBe(true);
 console.log("[DEBUG] independent Int32Array 0/1/65/257 ordered typed sources require4 bytes of logical work per scalar and zero physical release during that work");
});

import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { existsSync } from "node:fs";
import { join } from "node:path";

test("owned member batch retains canonical fallible grant axes and denied original vectors", () => {
  const owner = resolve(import.meta.dir, "..");
  let root = owner;
  while (!existsSync(join(root, "bun.lock"))) { const parent = resolve(root, ".."); if (parent === root) throw Error("Repository root missing"); root = parent; }
  const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(root, "🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json"), "utf8"));
  const valid = new Ajv({ strict: false }).compile({ ...schema, $ref: "#/$defs/Demand" });
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE axes (axis TEXT PRIMARY KEY, value INTEGER)");
    for (const row of fixture.constructorRefusals) database.query("INSERT INTO axes VALUES (?,?)").run(row.axis, row.value);
    expect(database.query("SELECT axis FROM axes WHERE value=0 ORDER BY axis").all()).toEqual([{ axis: "maximumCapacityBytes" }, { axis: "maximumDepth" }, { axis: "maximumItems" }]);
    for (const row of fixture.constructorRefusals) {
      const original = fixture.values.map((value: number) => value);
      expect(JSON.parse(JSON.stringify(original))).toEqual(fixture.values);
      expect(Buffer.from(new Int32Array(original).buffer).length).toBe(original.length * 4);
      expect(row.value).toBe(0);
    }
    expect(valid({ copyBytes: 4, capacityBytes: 128, releaseBytes: 0, depth: 2 })).toBe(true);
    expect(valid({ copyBytes: 4, capacityBytes: 128, releaseBytes: 0 })).toBe(false);
  } finally { database.close(); }
  const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
  expect(source).toContain("fn next_copy_byte_demand(&self) -> Result<usize, ValueError>");
  expect(source).toContain("maximum_body_bytes: usize");
  expect(source).toContain("next_depth_demand()");
  expect(source).toContain("grant.maximum_depth == 0");
  expect(source).toContain("admit_typed_controlled_retirement");
  console.log("[DEBUG] canonical production demand/Ajv, independent SQLite denied axes and ordered scalar bytes; native allocator and original pointer proof separately required");
});

test("rejected presence retains admitted ownership and separate physical retirement axes", () => {
  let root = import.meta.dir;
  while (!existsSync(join(root, "bun.lock"))) root = resolve(root, "..");
  const schema = JSON.parse(readFileSync(join(root, "🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json"), "utf8"));
  const valid = new Ajv({ strict: false }).compile({ ...schema, $ref: "#/$defs/Demand" });
  const actor = "Üser🙂";
  const octets = Buffer.from(actor, "utf8");
  expect(octets.length).toBe(new TextEncoder().encode(actor).length);
  expect(valid({ copyBytes: 0, capacityBytes: 0, releaseBytes: octets.length, depth: 1 })).toBe(true);
  const source = readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🚫️rejection/🦀️.rs"), "utf8");
  expect(source).toContain("grant: RetainedCloneGrant");
  expect(source).toContain("admit_artifact_retirement");
  expect(source).toContain("original_arc");
  expect(source).toContain("FactoryAuthority");
  expect(source).not.toContain("actor.truncate");
  expect(source).not.toContain("maximum_items: usize, maximum_bytes: usize");
  console.log("[DEBUG] independent Ajv Demand and UTF8 octets; source-only full-grant rejection boundary, native original/frame accounting still required");
});

import { spawnSync } from "node:child_process";
test("owned durable and presence Rust sources parse under the installed independent Rust grammar", () => {
  let root = import.meta.dir;
  while (!existsSync(join(root, "bun.lock"))) root = resolve(root, "..");
  const version = spawnSync("rustfmt", ["--version"], { encoding: "utf8", timeout: 1000 });
  expect(version.status).toBe(0);
  const base = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
  for (const relative of ["🦀️.rs", "♻️retirement/📦️backing/🦀️.rs", "../../../../🔨️modules/📡️replication/🔗️causal/🦀️.rs", "🧩️composition/🗄️durable-group/🦀️.rs", "🧵️canonical-edit/🦀️.rs", "🧵️canonical-edit/📖️reader/🦀️.rs", "🧵️canonical-edit/🧵️borrowed/🦀️.rs", "📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs", "👥️presence/🚫️rejection/🦀️.rs", "👥️presence/♻️retirement/🦀️.rs", "🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs", "🫧️ephemeral/📢️publication/🔁️transfer/🦀️.rs", "🧩️composition/📨️emission/📦️owned/🦀️.rs", "🧩️composition/📬️publication/🤝️group/🦀️.rs", "🔗️read/♻️retirement/🦀️.rs"]) {
    const source = readFileSync(join(base, relative), "utf8");
    const result = spawnSync("rustfmt", ["--edition", "2024", "--emit", "stdout", "--config", "skip_children=true"], { input: source, encoding: "utf8", maxBuffer: 8 * 1024 * 1024, timeout: 2000 });
    if (result.status !== 0) throw Error(`${relative}: ${result.stderr}`);
    expect(result.stdout.length).toBeGreaterThan(0);
  }
  console.log(`[DEBUG] ${version.stdout.trim()}: owned source syntax only; type, allocator, pointer and runtime custody proofs remain separately native`);
});


test("installed store and composed member publish full fallible retirement demand", () => {
  let root = import.meta.dir;
  while (!existsSync(join(root, "bun.lock"))) root = resolve(root, "..");
  const source = readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"), "utf8");
  expect(source).toContain("fn close_owned_demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>");
  expect(source).toContain("fn close_owned_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>");
  const start = source.indexOf("pub trait ArtifactStoreOwnedDisposer");
  const end = source.indexOf("#[derive(Clone, Copy, Debug, PartialEq, Eq)]", start);
  const trait = source.slice(start, end);
  expect(trait).toContain("fn demands(&self, store: &ArtifactStore<P, Mutation>, maximum_body_bytes: usize)");
  expect(trait).toContain("grant: RetainedCloneGrant");
  expect(trait).not.toContain("maximum_bytes: usize");
  const semio = readFileSync(join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs"), "utf8");
  expect(semio.includes("SemioStoreOwnedDisposer")).toBe(false);
  expect(semio.includes("enum SemioStoreClosePhase")).toBe(false);
  expect(semio).toContain("dsl::ArtifactStoreCursorDisposer::<Self, subsets::$module::schema::mutations::$mutation>::new()");
  console.log("[DEBUG] installed Store/SpaceMember and exact Stdio Semio catalog source contract; independent Demand oracle above, native custody proof pending");
});


test("current original Plugin and Stdio close sources parse under independent Rust grammar", () => {
  let root = import.meta.dir;
  while (!existsSync(join(root, "bun.lock"))) root = resolve(root, "..");
  const paths = [
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪆️child/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🧪️tests/🦀️.rs",
    "🧰️framework/🔨️modules/📡️replication/🔗️causal/🧪️tests/🔬️unit/🦀️.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs"
  ];
  for (const relative of paths) {
    const result = spawnSync("rustfmt", ["--edition", "2024", "--emit", "stdout", "--config", "skip_children=true"], { input: readFileSync(join(root, relative), "utf8"), encoding: "utf8", maxBuffer: 8 * 1024 * 1024, timeout: 2000 });
    if (result.status !== 0) throw Error(`${relative}: ${result.stderr}`);
    expect(result.stdout.length).toBeGreaterThan(0);
  }
  console.log("[DEBUG] independent Rust grammar7 current Plugin/Stdio/native-law sources; no native compiler or runtime proof");
});


test("mounted original publication keeps independent retirement currencies and context progress", () => {
  let root = import.meta.dir;
  while (!existsSync(join(root, "bun.lock"))) root = resolve(root, "..");
  const path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs";
  const source = readFileSync(join(root, path), "utf8");
  const owner = source.slice(source.indexOf("impl<M:SpaceMember+MemberFactory> MountedPrivateChildGroup"), source.indexOf("impl<M:SpaceMember+MemberFactory> Drop"));
  expect(owner.includes("fn retirement_demands")).toBe(true);
  expect(owner.includes("batch.next_demands(maximum_body_bytes)?")).toBe(true);
  expect(owner.includes("group.retirement_demands(maximum_body_bytes)?")).toBe(true);
  expect(owner.includes("context.next_close_capacity_byte_demand(maximum_body_bytes)?")).toBe(true);
  expect(owner.includes("InteractiveJobCloseStep::Complete{progress}")).toBe(true);
  expect(owner.includes(".admit(child_grant,context.terminal_is_empty())")).toBe(true);
  expect(owner.includes("step=>step")).toBe(true);
  expect(owner.includes("capacity.max(release)")).toBe(false);
  const schema = JSON.parse(readFileSync(join(root, "🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json"), "utf8"));
  const valid = new Ajv({strict:false}).compile({...schema, $ref:"#/$defs/Demand"});
  const demand = {copyBytes:Int32Array.BYTES_PER_ELEMENT,capacityBytes:128,releaseBytes:0,depth:3};
  expect(valid(demand)).toBe(true);
  expect(valid({...demand,depth:-1})).toBe(false);
  expect(demand.copyBytes).toBe(Buffer.from(new Int32Array([1]).buffer).length);
  const grammar = spawnSync("rustfmt", ["--edition","2024","--emit","stdout","--config","skip_children=true"], {input:source,encoding:"utf8",maxBuffer:8*1024*1024,timeout:2000});
  if(grammar.status!==0)throw Error(grammar.stderr);
  console.log("[DEBUG] mounted source full currencies plus canonical Demand/Ajv/Int32 bytes and independent Rust grammar; native originals/accounting proof pending");
});


test("mounted actual codec, fixed causal owners and original native laws parse under independent Rust grammar", () => {
  let root=import.meta.dir;
  while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");
  const mounted="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/";
  const paths=[
    "🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs",
    "🧰️framework/🔨️modules/🌱️value/📋️list/🧪️tests/🔬️counter/🦀️.rs",
    "🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🦀️.rs",
    ...["🧾️receipt/🦀️.rs","🧾️receipt/📦️group/🦀️.rs","🧾️receipt/📦️group/👤️member/🦀️.rs","📚️members/🦀️.rs","📦️owner/🦀️.rs","📦️owner/🧪️tests/🦀️.rs","🧾️receipt/📦️group/📚️command/🦀️.rs","🧾️receipt/📦️group/📚️command/📄️entry/🧪️tests/🦀️.rs"].map(path=>mounted+path)
  ];
  for(const path of paths){
    const result=spawnSync("rustfmt",["--edition","2024","--emit","stdout","--config","skip_children=true"],{input:readFileSync(join(root,path),"utf8"),encoding:"utf8",maxBuffer:8*1024*1024,timeout:2000});
    if(result.status!==0)throw Error(`${path}: ${result.error?.message??result.stderr}`);
    expect(result.stdout.length).toBeGreaterThan(0);
  }
  console.log("[DEBUG]12 actual codec/page/receipt/native-law sources parsed by independent Rust grammar; native type, blocked ledger and exact physical allocator proof pending");
});


test("Services original backing and five-axis retirement use genuine Job contracts",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");
 const base=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services");
 const cases=JSON.parse(readFileSync(join(base,"🚪️native-io/🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json"),"utf8"));
 const valid=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/Demand"});
 for(const row of cases.retirement){const physical=Buffer.alloc(row.capacityBytes);expect(physical.byteLength).toBe(row.capacityBytes);expect(valid({copyBytes:row.copyBytes,capacityBytes:row.birthBytes,releaseBytes:physical.byteLength,depth:row.depth})).toBe(true);expect(physical.byteLength).toBeGreaterThan(row.logicalBytes);}
 const native=readFileSync(join(base,"🚪️native-io/🦀️.rs"),"utf8");
 expect(native).toContain("fn next_close_capacity_byte_demand");expect(native).toContain("grant: RetainedCloneGrant");expect(native).toContain("path.capacity()");expect(native).not.toContain("error.clear()");expect(native).not.toContain("JobPayloadCloseStep");expect(native).not.toContain("close_step(1,");expect(native).toContain("Result<RetirementDemand, ValueError>");expect(native).toContain("bytes.close_step(child)");expect(native).toContain("writer.close_step(child)");
 const pool=readFileSync(join(base,"🦀️.rs"),"utf8");expect(pool).toContain("schedule_rejected_compute_job");expect(pool).not.toContain("while !rejected.terminal_is_empty()");expect(pool).toContain("close_step_outcome_slot(&mut state.retained_outcome");expect(pool).not.toContain("outcome.close_step(1,");
 for(const path of ["🚪️native-io/🦀️.rs","🦀️.rs","🧪️tests/🔬️native-io-unit/🦀️.rs","🧪️tests/🔬️component-unit/🦀️.rs"]){const grammar=spawnSync("rustfmt",["--edition","2024","--emit","stdout","--config","skip_children=true"],{input:readFileSync(join(base,path),"utf8"),encoding:"utf8",maxBuffer:8*1024*1024,timeout:2000});expect(grammar.status,`${path}: ${grammar.stderr}`).toBe(0);}

 console.log("[DEBUG] Services plain physical-capacity cases agree with independent Buffer and canonical Ajv Demand; native original identity and allocator receipts remain separate");
});


test("private actual member publication forwards five supplied OneItem axes",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");
 const base=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group");
 const source=readFileSync(join(base,"📦️owner/🦀️.rs"),"utf8");
 expect(source).not.toContain("maximum_bytes:");expect(source).not.toContain("grant.maximum_bytes");expect(source).toContain("next_open_retirement_demands");expect(source).not.toContain("open.next_close_byte_demand()");
 const native=readFileSync(join(base,"🪟️mounted/📦️owner/🦀️.rs"),"utf8");expect(native).toContain("next_open_retirement_demands");
 const schema=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json"),"utf8"));const valid=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/Demand"});
 const demand={copyBytes:Int32Array.BYTES_PER_ELEMENT,capacityBytes:Buffer.alloc(128).byteLength,releaseBytes:Buffer.alloc(256).byteLength,depth:3};expect(valid(demand)).toBe(true);expect(Object.values(demand)).toEqual([4,128,256,3]);
 console.log("[DEBUG] independent Ajv/Int32/Buffer discriminate member copy, birth, release and depth; actual OneItem field forwarding and canonical open demand source law, native credit separate");
});


test("Store returned-read ingress consumes the supplied grant before original custody transfer",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");
 const source=readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"),"utf8");
 const ingress=source.slice(source.indexOf("    pub fn take_returned_snapshot_read_retirement("),source.indexOf("    fn close_cursor_demands("));
 expect(ingress).toContain("grant: RetainedCloneGrant");expect(ingress).toContain("try_admit_one_returned::<P, _>(grant");expect(ingress).not.toContain("try_take_one_returned");expect(ingress).not.toContain("for _ in");expect(ingress).not.toContain("Box::new");
 const schema=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json"),"utf8"));const valid=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/Demand"});
 const demand={copyBytes:0,capacityBytes:Buffer.alloc(128).byteLength,releaseBytes:0,depth:1};expect(valid(demand)).toBe(true);expect(valid({...demand,capacityBytes:-1})).toBe(false);
 const database=new Database(":memory:");try{database.exec("CREATE TABLE leases(id INTEGER PRIMARY KEY, original TEXT)");database.query("INSERT INTO leases VALUES(?,?)").run(1,"same-original");database.exec("BEGIN");database.query("DELETE FROM leases WHERE id=?").run(1);database.exec("ROLLBACK");expect(database.query("SELECT original FROM leases WHERE id=1").get()).toEqual({original:"same-original"});}finally{database.close();}
 console.log("[DEBUG] canonical Demand/Ajv and independent SQLite refusal restoration; actual Store granted ingress remains subject to native original-pointer and allocator law");
});


test("ephemeral publication retires original preparation frames and capabilities under independent axes",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");
 const source=readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"),"utf8");
 const start=source.indexOf("impl<P, Mutation> ArtifactEphemeralOneItemPublication<P, Mutation>");const end=source.indexOf("impl<P, Mutation> Drop for ArtifactEphemeralOneItemPublication",start);const owner=source.slice(start,end);
 expect(owner).toContain("pub fn retirement_demands(&self");expect(owner).toContain("Result<RetainedCloneStep, ValueError>");expect(owner).not.toContain("SnapshotRetirementStep");expect(owner).not.toContain("fault.pop()");expect(owner).toContain("fault.capacity()");expect(owner).toContain("size_of_val(owner.as_ref())");expect(owner).toContain("factory_retirement");expect(owner).toContain("grant.maximum_depth - 1");
 const schema=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json"),"utf8"));const valid=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/Demand"});
 for(const releaseBytes of [0,128,16384]){const backing=Buffer.alloc(releaseBytes);const demand={copyBytes:0,capacityBytes:0,releaseBytes:backing.byteLength,depth:1};expect(valid(demand)).toBe(true);expect(demand.releaseBytes).toBe(releaseBytes);expect(valid({...demand,depth:-1})).toBe(false);}
 console.log("[DEBUG] independent Buffer and canonical Ajv Demand preserve whole release independent of copy/birth; ephemeral actual native physical owners remain required");
});


test("actual Plugin member birth retains nested source depth before original request transfer",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");
 const base=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
 const source=readFileSync(join(base,"🦀️.rs"),"utf8");expect(source).not.toContain("M::open_birth_bytes(request)");expect(source).toContain("fn member_open_birth_demand");expect(source).toContain("demand.depth > 64");
 const group=readFileSync(join(base,"🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs"),"utf8");expect(group).toContain("M::open_birth_demand(request)");expect(group).toContain("next_open_birth_demand");expect(group).toContain("maximum_depth: grant.maximum_depth - 2");expect(group).not.toContain("M::open_birth_bytes(request)");
 const mounted=readFileSync(join(base,"🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs"),"utf8");expect(mounted).toContain("birth.depth>64");
 const cases=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json"),"utf8"));const valid=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/Demand"});
 for(const parents of [0,1,2]){const demand={copyBytes:0,capacityBytes:Buffer.alloc(cases.grant.maximumCapacityBytes).byteLength,releaseBytes:0,depth:parents+1};expect(valid(demand)).toBe(true);expect(demand.capacityBytes).toBe(cases.grant.maximumCapacityBytes);expect(demand.depth).toBe(parents+1);expect(cases.grant.maximumDepth-demand.depth).toBeGreaterThanOrEqual(0);}
 console.log("[DEBUG] original plain grant and canonical Demand agree with independent Buffer/Ajv; member source depth propagation is source proof, native ownership and advancement receipts pending");
});


test("Store live read admission restores the same generation slot before any failed ownership transfer",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");const source=readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"),"utf8");
 expect(source.includes("fn admit_retirement<T: Send + Sync + 'static>")).toBe(true);expect(source.includes("self.owner = Some(original)")).toBe(true);expect(source.includes("snapshot: &mut Option<ErasedSnapshotRead>")).toBe(true);expect(source.includes("SnapshotRetirementRejected")).toBe(false);
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));const database=new Database(":memory:");try{database.exec("CREATE TABLE slots(id INTEGER PRIMARY KEY,generation INTEGER,owner TEXT)");database.query("INSERT INTO slots VALUES(?,?,?)").run(1,17,JSON.stringify(fixture.values));database.exec("BEGIN");database.query("DELETE FROM slots WHERE id=1 AND generation=17").run();database.exec("ROLLBACK");expect(database.query("SELECT generation,owner FROM slots WHERE id=1").get()).toEqual({generation:17,owner:JSON.stringify(fixture.values)});}finally{database.close();}
 console.log("[DEBUG] SQLite transactional original/generation rollback agrees with plain ordered source; actual native pointer/allocator read-admission law remains required");
});


test("Store counted originals consume an explicit independent physical test policy",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");const base=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
 const source=readFileSync(join(base,"🧪️tests/🔬️unit/🦀️.rs"),"utf8");const start=source.indexOf("struct ExactDemoSnapshotRetirement {");const end=source.indexOf("struct DemoInitialSnapshotRetirementFactory;",start);const counted=source.slice(start,end);
 expect(counted.includes("grant: RetainedCloneGrant")).toBe(true);expect(counted.includes("admit_artifact_retirement(value,grant")).toBe(true);expect(counted.includes("owner: Some(Arc::new(value))")).toBe(false);expect(counted.includes("self.completed.take()")).toBe(true);
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));const grant=fixture.physicalCloseGrant;expect(grant.maximumItems).toBe(1);expect(grant.maximumDepth).toBe(64);expect(Buffer.alloc(grant.maximumCapacityBytes).byteLength).toBe(1048576);expect(Buffer.alloc(grant.maximumReleaseBytes).byteLength).toBe(1048576);expect(grant.maximumCopyBytes).toBe(4096);
 const close=source.slice(source.indexOf("fn close_test_store<"),source.indexOf("impl<P, Mutation> Drop",source.indexOf("fn close_test_store<")));const driver=source.slice(source.indexOf("fn drive_retirement_terminal("),source.indexOf("#[semio_framework_async_macros::async_test]",source.indexOf("fn drive_retirement_terminal(")));const readerStart=source.indexOf("fn close_stalls_at_reader_boundary(");const reader=source.slice(readerStart,source.indexOf("pub(super) fn demo_closable_store_owners",readerStart));for(const helper of [close,driver,reader]){expect(helper.includes("maximum_capacity_bytes: demand.capacity_bytes")).toBe(false);expect(helper.includes("physical_test_close_grant()")).toBe(true);}
 const inverseStart=source.indexOf("impl DemoRetainedCloneEditCursor");const inverse=source.slice(inverseStart,source.indexOf("struct ProbedRetainedClonePreparationFactory",inverseStart));expect(inverse.includes("release_bytes: inverse.capacity()")).toBe(true);expect(inverse.includes("drop(self.inverse.take())")).toBe(true);const probeStart=source.indexOf("impl ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation> for ProbedRetainedClonePreparationFactory");const probe=source.slice(probeStart,source.indexOf("impl ArtifactCanonicalJson for DemoMutation",probeStart));expect(probe.includes("checked_add(std::mem::size_of::<ProbedRetainedClonePreparation>())")).toBe(true);expect(probe.includes("demand.admit(grant.retained_grant())")).toBe(true);expect(probe.includes("drop(self.inner.take())")).toBe(true);expect(probe.includes("counter.step(child)")).toBe(true);expect(probe.includes("impl Drop for ProbedRetainedClonePreparation")).toBe(false);
 console.log("[DEBUG] independently allocated Buffer extents match separately authored plain test currencies; counted pointer/physical native law remains unrun");
});


test("actual Store lifecycle caller grants have five declared currencies without query-priced authority",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");const base=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");const source=readFileSync(join(base,"🧪️tests/🔬️unit/🦀️.rs"),"utf8");
 expect(source.includes("uniform_one_item_grant")).toBe(false);const start=source.indexOf("fn close_retained_clone_preparation_publication");const owner=source.slice(start,source.indexOf("#[semio_framework_async_macros::async_test]",start));expect(owner.includes("grant: RetainedCloneGrant")).toBe(true);expect(owner.includes("let grant = RetainedCloneGrant")).toBe(false);
 const schema=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json"),"utf8"));const valid=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/Grant"});const lifecycle=JSON.parse(readFileSync(join(base,"🧬️snapshot-clone/🧪️fixtures/📦️lifecycle/🔣️.json"),"utf8"));const latest=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🔗️tool-latest-wins-integration.json"),"utf8"));
 for(const row of [lifecycle.grant,lifecycle.handoff.closeGrant,latest]){const grant=Object.fromEntries(["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"].map(key=>[key,row[key]]));expect(valid(grant)).toBe(true);expect(valid({...grant,maximumDepth:-1})).toBe(false);for(const key of ["maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes"]){expect(Buffer.alloc(grant[key]).byteLength).toBe(grant[key]);}expect(grant.maximumItems).toBe(1);}
 console.log("[DEBUG] actual plain five-field lifecycle Grant projections pass canonical production schema plus independent Ajv/Buffer; no corpus validation, self-priced authority or native runtime credit");
});


test("initialization runtime callers preserve independent physical grant fields and actual receipts", () => {
  let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");
  const fixture=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json"),"utf8"));
  const schema=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json"),"utf8"));
  const valid=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/Grant"});
  expect(valid(fixture.grant)).toBe(true);
  for(const row of fixture.cases){
    const original=Buffer.alloc(row.physicalBytes);
    expect(original.byteLength).toBe(row.physicalBytes);
    expect(row.releasedBytes).toBe(row.callerBytes>=original.byteLength?original.byteLength:0);
    expect(valid({...fixture.grant,maximumReleaseBytes:row.callerBytes})).toBe(true);
  }
  const source=readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"),"utf8");
  const start=source.indexOf("fn initialization_runtime_preserves_physical_backing_and_terminal_cursor_box()");
  const end=source.indexOf("impl ErasedSnapshotRetirement for DemandingBufferRetirement",start);
  const owner=source.slice(start,end);
  expect(owner.includes("maximum_release_bytes: caller")).toBe(true);
  expect(owner.includes("settle_current_retirement_step(grant)")).toBe(true);
  expect(owner.includes("next_close_release_byte_demand().unwrap()")).toBe(true);
  expect(owner.includes("observe_backing")).toBe(true);
  expect(owner.includes("SnapshotRetirementStep")).toBe(false);
  expect(owner.includes("next_close_byte_demand")).toBe(false);
  const vcsStart=source.indexOf("fn vcs_retirement_propagates_nested_physical_allocation_demand()");const vcs=source.slice(vcsStart,source.indexOf("fn fresh_field_release_propagates",vcsStart));expect(vcs.includes("next_close_byte_demand")).toBe(false);expect(vcs.includes("SnapshotRetirementStep")).toBe(false);expect(vcs.includes("maximum_release_bytes: caller")).toBe(true);expect(vcs.includes("closing_factories: std::mem::ManuallyDrop::new([None,None])")).toBe(true);const genesisStart=source.indexOf("fn envelope_and_genesis_preserve_terminal_cursor_physical_grants()");const genesis=source.slice(genesisStart,source.indexOf("fn displaced_store_owner_preserves",genesisStart));expect(genesis.includes("admit_verified_pack")).toBe(true);expect(genesis.includes("next_close_byte_demand")).toBe(false);expect(genesis.includes("SnapshotRetirementStep")).toBe(false);expect(genesis.includes("maximum_release_bytes: caller")).toBe(true);expect(genesis.includes("original")).toBe(true);
  console.log("[DEBUG] authored release frontier rows match independent Buffer/canonical Ajv Grant; actual runtime caller separate currencies/allocator law still requires native execution");
});


test("decode retirement separates inline page work from original fixed directory release",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const grant=fixture.physicalCloseGrant;const pageBytes=16384;
 const schema=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json"),"utf8"));const valid=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/Grant"});expect(valid(fixture.decodeCloseGrant)).toBe(true);expect(fixture.decodeCloseGrant.maximumCopyBytes).toBe(pageBytes);expect(fixture.decodeCloseGrant.maximumCapacityBytes).toBe(0);
 for(const pages of [1,2]){const layout=Buffer.alloc(pages*(pageBytes+Uint16Array.BYTES_PER_ELEMENT));expect(layout.byteLength).toBe(pages*(pageBytes+2));expect(grant.maximumReleaseBytes).toBeGreaterThanOrEqual(layout.byteLength);expect(Buffer.alloc(pageBytes).byteLength).toBe(pageBytes);expect(grant.maximumCopyBytes).toBeLessThan(pageBytes);}
 const source=readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"),"utf8");
 const start=source.indexOf("pub struct OwnedSchemaDecodePages");const pages=source.slice(start,source.indexOf("pub struct OwnedSchemaTokenCursor",start));
 expect(pages.includes("self.slots.is_empty()")).toBe(true);expect(pages.includes("fn close_backing_step")).toBe(true);
 const token=source.slice(source.indexOf("impl OwnedSchemaTokenCursor"),source.indexOf("pub struct OwnedSchemaRecordCursor"));
 expect(token.includes("self.pages.close_backing_step(grant)")).toBe(true);expect(pages.includes("grant.maximum_release_bytes<bytes")).toBe(true);
 console.log("[DEBUG] independent Buffer/u16 layout distinguishes inline16KiB copy from fixed directory release; original directory terminal and native allocator assertions remain separate");
});


test("unit factory closure retains inline original witness without ungranted child ticket birth",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");
 const fixture=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🧫️fixtures/🔣️.json"),"utf8"));
 for(const row of fixture.soleSnapshotWeak){const frame=Buffer.alloc(row.frameBytes);expect(frame.byteLength).toBe(row.frameBytes);expect(row.frameBytes).toBe(2*row.pointerBytes+row.snapshotBytes);}
 const source=readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"),"utf8");
 const start=source.indexOf("impl semio_framework_value::FactoryPayloadRetirement for DemoOneItemPreparationFactory");const end=source.indexOf("impl DemoOneItemPreparationFactory",start);const owner=source.slice(start,end);
 expect(owner.includes("DemoOneItemFactoryClose")).toBe(true);expect(owner.includes("fn close_state_birth_bytes(&self) -> usize { 0 }")).toBe(true);expect(owner.includes("preborn_factory_retirement")).toBe(false);expect(owner.includes("fn close_state_demands")).toBe(true);expect(owner.includes("grant: RetainedCloneGrant")).toBe(true);expect(owner.includes("state.published=Some(published_root)")).toBe(true);
 console.log("[DEBUG] independent Buffer frame cases and current unit factory supplied-grant/inline owner contract; native sole-Weak pointer/allocator law remains required");
});


test("durable preparation factories admit original request before actual frame birth",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const grant=fixture.physicalCloseGrant;const original=Buffer.from(fixture.metadata.actor,"utf8");
 expect(original.byteLength).toBe(new TextEncoder().encode(fixture.metadata.actor).byteLength);expect(grant.maximumCopyBytes).toBeGreaterThanOrEqual(original.byteLength);
 const schema=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json"),"utf8"));const valid=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/Grant"});expect(valid(grant)).toBe(true);expect(valid({...grant,maximumCapacityBytes:-1})).toBe(false);
 const source=readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"),"utf8");
 for(const [start,end] of [["impl ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation> for CountingOwnedPreparationFactory","#[semio_framework_async_macros::async_test]"],["impl MemberStoreOneItemWirePreparationFactory<DemoSnapshot, DemoMutation> for DemoMemberWirePreparationFactory","impl DemoMemberWirePreparation"],["impl ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation> for DemoOneItemPreparationFactory","impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for DemoOneItemPreparation"]]){const from=source.indexOf(start);const owner=source.slice(from,source.indexOf(end,from));expect(owner.includes("fn begin_demand")).toBe(true);expect(owner.includes("grant: ArtifactStoreOneItemGrant")).toBe(true);expect(owner.includes("RetainedCloneProgress")).toBe(true);expect(owner.includes("(ValueError, ArtifactStoreOneItemPreparationRequest")).toBe(true);}
 const from=source.indexOf("impl ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation> for DemoOneItemPreparationFactory");const owner=source.slice(from,source.indexOf("impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for DemoOneItemPreparation",from));expect(owner.indexOf("demand.admit(grant.retained_grant())")).toBeLessThan(owner.indexOf("Box::new"));expect(owner.includes("grant.maximum_copy_bytes < copied_bytes")).toBe(true);expect(owner.includes("retained_capacity_bytes")).toBe(true);
 console.log("[DEBUG] original UTF8 identity bytes agree with independent Buffer/TextEncoder and canonical Ajv Grant; source factory birth/refusal order, actual allocator/identity native law unrun");
});


test("durable preparation cancellation retains original candidate and captured issuers",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json"),"utf8"));const valid=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/Demand"});
 for(const value of fixture.values){const buffer=Buffer.alloc(Int32Array.BYTES_PER_ELEMENT);buffer.writeInt32LE(value);const view=new DataView(buffer.buffer,buffer.byteOffset,buffer.byteLength);expect(view.getInt32(0,true)).toBe(value);expect(valid({copyBytes:0,capacityBytes:buffer.byteLength,releaseBytes:0,depth:2})).toBe(true);}
 const source=readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"),"utf8");const start=source.indexOf("impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for DemoOneItemPreparation");const owner=source.slice(start,source.indexOf("struct DemoEphemeralPreparationFactory",start));expect(owner.includes("SnapshotRetirementStep")).toBe(false);expect(owner.includes("prepared.admit_retirement")).toBe(true);expect(owner.includes("try_return_to_registry_witness")).toBe(true);expect(owner.includes("fn next_close_capacity_byte_demand")).toBe(true);expect(owner.includes("self.prepared = Some(original)")).toBe(true);expect(owner.includes("authority.retire(child)")).toBe(true);expect(owner.includes("artifact_retirement_box_close_step")).toBe(true);
 const ephemeralStart=source.indexOf("impl ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation> for DemoEphemeralPreparation");const ephemeral=source.slice(ephemeralStart,source.indexOf("fn close_durable_publication",ephemeralStart));expect(ephemeral.includes("SnapshotRetirementStep")).toBe(false);expect(ephemeral.includes("fn next_close_depth_demand")).toBe(true);expect(ephemeral.includes("ArtifactEphemeralBaseOwner::Transient(original)")).toBe(true);expect(ephemeral.includes("ArtifactEphemeralBaseOwner::Presence(original)")).toBe(true);expect(ephemeral.includes("ArtifactEphemeralBaseOwner::TransientRead(original)")).toBe(true);expect(ephemeral.includes("artifact_retirement_box_close_step")).toBe(true);expect(ephemeral.includes("self.request.take().is_some()")).toBe(false);
 console.log("[DEBUG] original ordered i32 values agree with independent Buffer/DataView and canonical Ajv Demand; current candidate/captured issuer/registry refusal source law, native physical original custody still unrun");
});


test("wire preparation preserves original buffers and captured typed issuer through granted closure",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));const bytes=Buffer.from(fixture.metadata.actor,"utf8");expect(bytes.byteLength).toBe(new TextEncoder().encode(fixture.metadata.actor).byteLength);
 const schema=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json"),"utf8"));const valid=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/Demand"});expect(valid({copyBytes:0,capacityBytes:0,releaseBytes:bytes.byteLength,depth:1})).toBe(true);
 const source=readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"),"utf8");const from=source.indexOf("struct DemoMemberWirePreparationFactory");const owner=source.slice(from,source.indexOf("async fn retained_member_publication",from));
 expect(owner.includes("SnapshotRetirementStep")).toBe(false);expect(owner.includes("grant.maximum_bytes")).toBe(false);expect(owner.includes("wire.bytes.pop()")).toBe(false);expect(owner.includes("wire.schema.pop()")).toBe(false);expect(owner.includes("wire.bytes.capacity()")).toBe(true);expect(owner.includes("wire.schema.capacity()")).toBe(true);expect(owner.includes("typed_request: Option<")).toBe(true);expect(owner.includes("typed_request = Some(original)")).toBe(true);expect(owner.includes("try_return_to_registry_witness")).toBe(true);expect(owner.includes("DemoOneItemPreparationFactory::admissible().begin")).toBe(false);expect(owner.includes("fn next_close_depth_demand")).toBe(true);
 console.log("[DEBUG] plain UTF8 bytes match independent Buffer/TextEncoder and canonical Ajv Demand; wire original backing and granted typed ingress source law, native receipt still required");
});


test("shared Store coverage admits only every independently supplied axis",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=resolve(root,"..");
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));const {demand,denials}=fixture.grantCoverage;const policy=fixture.physicalCloseGrant;
 const schema=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json"),"utf8"));const valid=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/Demand"});expect(valid(demand)).toBe(true);
 const database=new Database(":memory:");try{for(const row of [{axis:null,value:null},...denials]){const g={...policy,...(row.axis?{[row.axis]:row.value}:{})};const own=g.maximumItems>0&&g.maximumCopyBytes>=demand.copyBytes&&g.maximumCapacityBytes>=demand.capacityBytes&&g.maximumReleaseBytes>=demand.releaseBytes&&g.maximumDepth>=demand.depth;const oracle=database.query("SELECT (? > 0 AND ? >= ? AND ? >= ? AND ? >= ? AND ? >= ?) AS covered").get(g.maximumItems,g.maximumCopyBytes,demand.copyBytes,g.maximumCapacityBytes,demand.capacityBytes,g.maximumReleaseBytes,demand.releaseBytes,g.maximumDepth,demand.depth) as {covered:number};expect(own).toBe(Boolean(oracle.covered));expect(own).toBe(row.axis===null);}}finally{database.close();}
 const source=readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"),"utf8");const start=source.indexOf("fn artifact_retirement_grant_covers(");const owner=source.slice(start,source.indexOf("\n}",start));expect(owner).toContain("grant.maximum_depth >= demand.depth");
 console.log("[DEBUG] plain five-axis grant denial boundaries agree with independent SQLite/canonical Ajv; actual shared helper native law remains separately required");
});
