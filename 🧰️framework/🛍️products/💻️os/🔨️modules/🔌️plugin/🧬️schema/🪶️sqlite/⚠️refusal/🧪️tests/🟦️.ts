import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import Parser from "web-tree-sitter";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import request from "../../🧫️fixtures/🔣️.json";
import requestSchema from "../../🧬️schema/🔣️.json";

test("guest SQLite rejection carries all eight intrinsic causes independently of prose and VM cancellation", () => {
  const ajv = new Ajv({ strict: true });
  const validate = ajv.compile(schema.$defs.rejection);
  const diagnostics = Buffer.from(fixture.diagnostic.message, "utf8");
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE refusal(kind TEXT PRIMARY KEY, message TEXT NOT NULL, diagnostics BLOB NOT NULL)");
    for (const kind of fixture.kinds) {
      const carrier = { kind, message: fixture.message, diagnostics: [...diagnostics] };
      expect(validate(carrier)).toBe(true);
      expect(validate({ ...carrier, kind: "OwnershipLimit" })).toBe(false);
      expect(validate({ message: carrier.message, diagnostics: carrier.diagnostics })).toBe(false);
      expect(validate({ ...carrier, guessedKind: kind })).toBe(false);
      database.query("INSERT INTO refusal VALUES (?, ?, ?)").run(kind, fixture.message, diagnostics);
    }
    const rows = database.query("SELECT kind, message, diagnostics FROM refusal ORDER BY rowid").all() as { kind: string; message: string; diagnostics: Uint8Array }[];
    expect(rows.map(row => row.kind)).toEqual(fixture.kinds);
    for (const row of rows) {
      expect(row.message).toBe(fixture.message);
      expect(Buffer.from(row.diagnostics)).toEqual(diagnostics);
    }
  } finally { database.close(); }
  const wit = readFileSync(resolve(import.meta.dir, "../../../📜️.wit"), "utf8");
  const cases = wit.match(/enum value-refusal-kind\s*\{([^}]+)\}/)?.[1].split(",").map(value => value.trim()).filter(Boolean);
  expect(cases).toEqual(fixture.kinds.map(kind => kind.replace(/[A-Z]/g, letter => `-${letter.toLowerCase()}`)));
  expect(wit.match(/record snapshot-rejection\s*\{([^}]+)\}/)?.[1]).toMatch(/kind:\s*value-refusal-kind/);
  expect(fixture.providerCancellation).toBe("canceled");
  expect(fixture.vmCancellation).toBe("turnFaultCancelled");
});


test("guest SQLite caller native grant preserves five independent axes and rejects omitted authority",()=>{
  const validate=new Ajv({strict:true}).compile(requestSchema);
  expect(validate(request)).toBe(true);
  const oracle=JSON.parse(JSON.stringify(request));
  expect(oracle.native).toEqual(request.native);
  for(const axis of Object.keys(request.native)){
    const denied={...request,native:{...request.native,[axis]:0}};
    expect(validate(denied)).toBe(true);
    expect(JSON.parse(JSON.stringify(denied)).native[axis]).toBe(0);
    const missing={...request,native:{...request.native}};
    delete (missing.native as Record<string,number>)[axis];
    expect(validate(missing)).toBe(false);
  }
  const {native,...withoutNative}=request;
  expect(validate(withoutNative)).toBe(false);
  expect(native.maximum_copy_bytes).not.toBe(native.maximum_capacity_bytes);
  expect(native.maximum_release_bytes).not.toBe(native.maximum_capacity_bytes);
  const wit=readFileSync(resolve(import.meta.dir,"../../../📜️.wit"),"utf8");
  expect(wit.match(/sqlite-export: async func\([^;]+;/)?.[0]).toContain("native: retained-clone-grant");
  expect(wit.match(/sqlite-import: async func\([^;]+;/)?.[0]).toContain("native: retained-clone-grant");
  console.log("[DEBUG] Guest native grant: five independent zero-preserving axes validated by Ajv and JSON");
});


test("guest snapshot pending custody requires the same instance and original close grant", async () => {
 const law=(await import("../../🧫️fixtures/♻️retirement/🔣️.json")).default;
 const contract=(await import("../../🧬️schema/♻️retirement/🔣️.json")).default;
 expect(new Ajv({strict:true,allErrors:true}).compile(contract)(law)).toBe(true);
 const db=new Database(":memory:");
 try{
  db.run("CREATE TABLE custody(ticket INTEGER, owned INTEGER, output INTEGER, released INTEGER)");
  db.run("INSERT INTO custody VALUES(?,?,1,0)",[law.ticket,law.firstOwnedBytes]);
  db.run("UPDATE custody SET output=0 WHERE ticket<>? AND ?<>0",[law.ticket,law.maximumReleaseBytes]);
  expect(db.query("SELECT ticket,owned,output,released FROM custody").get()).toEqual({ticket:law.ticket,owned:law.firstOwnedBytes,output:1,released:law.refusedReleaseBytes});
  db.run("UPDATE custody SET owned=owned+? WHERE ticket=?",[law.closeCapacityBytes,law.ticket]);
  expect(db.query("SELECT owned,output,released FROM custody").get()).toEqual({owned:law.finalOwnedBytes,output:1,released:law.refusedReleaseBytes});
  expect(JSON.parse(JSON.stringify(law))).toEqual(law);
  console.error("[DEBUG] actual same-instance close neutral contract keeps pending output and conserved original receipt under zero release");
 }finally{db.close();}
});


test("same snapshot receiving phase accounts additional host input separately from guest and return owners", () => {
  const inputFixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../../../🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🧫️fixtures/📥️input/🔣️.json"), "utf8"));
  expect(inputFixture.firstCapacityBytes + inputFixture.resumedCapacityBytes).toBe(inputFixture.receivedInput);
  expect(inputFixture.receivedInput).toBe(inputFixture.sourceOwned);
  const receipt = { maximum: 57, owned: 3, finished: false, reserved_return: 11, phase_start_owned: 7, phase_start_reserved_return: 0, received_output: 5, received_input: 4, received_retirement: 0 };
  const restored = JSON.parse(JSON.stringify(receipt));
  const db = new Database(":memory:");
  db.exec("CREATE TABLE receiving(phase_start INTEGER NOT NULL, guest INTEGER NOT NULL, returned INTEGER NOT NULL, output_copy INTEGER NOT NULL, input_copy INTEGER NOT NULL, ceiling INTEGER NOT NULL)");
  db.prepare("INSERT INTO receiving VALUES(?, ?, ?, ?, ?, ?)").run(restored.phase_start_owned, restored.owned, restored.reserved_return - restored.phase_start_reserved_return, restored.received_output, restored.received_input, restored.phase_start_owned + restored.maximum);
  const actual = db.query("SELECT phase_start + guest + returned + output_copy + input_copy AS source_owned, ceiling AS source_maximum FROM receiving").get();
  expect(actual).toEqual({ source_owned: 30, source_maximum: 64 });
  expect(restored.received_input).toBe(4);
  expect(restored.received_output).toBe(5);
  db.close();
  console.log("[DEBUG] original receiving continuation conserves distinct guest3 return11 output5 input4 under source30 ceiling64");
});

test("host retirement capacity has a distinct conserved original receiving ledger",()=>{
 const base=resolve(import.meta.dir,"../../../../../🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control");
 const law=JSON.parse(readFileSync(resolve(base,"🧫️fixtures/♻️host/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(resolve(base,"🧬️schema/♻️host/🔣️.json"),"utf8"));
 expect(new Ajv({strict:true,allErrors:true}).compile(schema)(law)).toBe(true);
 const receipt=JSON.parse(JSON.stringify(law.scalarReceipt));const db=new Database(":memory:");
 try{db.exec("CREATE TABLE custody(start INTEGER, guest INTEGER, returned INTEGER, output INTEGER, input INTEGER, retirement INTEGER, maximum INTEGER)");db.prepare("INSERT INTO custody VALUES(?,?,?,?,?,?,?)").run(receipt.phase_start_owned,receipt.owned,receipt.reserved_return-receipt.phase_start_reserved_return,receipt.received_output,receipt.received_input,receipt.received_retirement,receipt.maximum);expect(db.query("SELECT start+guest+returned+output+input+retirement AS owned,start+maximum AS maximum FROM custody").get()).toEqual({owned:law.sourceOwned,maximum:law.sourceMaximum});expect(receipt.received_retirement).toBe(13);expect(receipt.received_input).toBe(4);expect(receipt.received_output).toBe(5);expect(receipt.finished).toBe(true);}finally{db.close();}
 console.log("[DEBUG] original source43 ceiling64 distinguishes host retirement13 from input4 output5 guest3 return11");
});

test("actual original scalar metadata is closed and independently reproduces its funded receipt",()=>{
 const law=JSON.parse(readFileSync(resolve(import.meta.dir,"../../🧫️fixtures/📨️metadata/🔣️.json"),"utf8"));const schema=JSON.parse(readFileSync(resolve(import.meta.dir,"../../🧬️schema/📨️metadata/🔣️.json"),"utf8"));expect(new Ajv({strict:true,allErrors:true}).compile(schema)(law)).toBe(true);const restored=JSON.parse(JSON.stringify(law));expect(restored.query.Ok.ticket).toBe(9);expect(restored.close.Ok.retirement).toEqual(restored.query.Ok);expect(restored.close.Ok.progress).toEqual({copied_items:1,copied_bytes:32,retained_capacity_bytes:64,released_bytes:16});expect(restored.deniedCapacityBytes).toBe(0);expect(restored.deniedReleaseBytes).toBe(0);console.log("[DEBUG] independent Ajv/JSON authentic query and close preserve full scalar progress under zero heap metadata parsing");
});


test("original retirement projection measures proposed charge without consuming original receipt",()=>{
 const base=resolve(import.meta.dir,"../../../../../🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control");const read=(path:string)=>JSON.parse(readFileSync(resolve(base,path),"utf8"));const fixture=read("🧫️fixtures/📥️input/🔣️.json");const validate=new Ajv({strict:true,allErrors:true}).compile(read("🧬️schema/📥️input/🔣️.json"));expect(validate(fixture)).toBe(true);for(const name of Object.keys(fixture.checkpointProjection)){const absent=structuredClone(fixture);delete absent.checkpointProjection[name];expect(validate(absent)).toBe(false);}const extra=structuredClone(fixture);extra.checkpointProjection.consumerAuthority=true;expect(validate(extra)).toBe(false);
 const law=fixture.checkpointProjection;const projection={kind:"original-retirement-measurement",maximum:law.originalMaximumBytes,owned:0,finished:false,reserved_return:0,phase_start_owned:0,phase_start_reserved_return:0,received_output:0,received_input:0,received_retirement:law.hostCharge+law.proposedCharge};const check=new Ajv({strict:true,allErrors:true}).compile(read("🔎️measurement/🧬️schema/🔣️.json"));expect(check(projection)).toBe(true);expect(check({...projection,consumerAuthority:true})).toBe(false);expect(new Ajv({strict:true,allErrors:true}).compile(read("🧬️schema/🧾️receipt/🔣️.json"))(projection)).toBe(false);const db=new Database(":memory:");try{db.exec("CREATE TABLE measurement(actual INTEGER,proposed INTEGER)");db.prepare("INSERT INTO measurement VALUES(?,?)").run(law.hostCharge,law.proposedCharge);expect(db.query("SELECT actual,actual+proposed AS projected FROM measurement").get()).toEqual({actual:law.actualRetirement,projected:law.projectedRetirement});}finally{db.close();}
 const source=readFileSync(resolve(base,"🦀️.rs"),"utf8");expect(source).toContain("pub fn project_retirement_charge(");const measured=readFileSync(resolve(base,"🔎️measurement/🦀️.rs"),"utf8");expect(measured).toContain("OriginalOperationProjection<'a>");expect(measured).not.toContain("Deserialize");expect(measured).not.toContain("NativeEncodeControl");console.log("[DEBUG] borrowed checkpoint projection is measurement-only actual3 proposed7 measured10, strict Ajv and independent SQLite preserve actual receipt");
});


test("original snapshot input cancellation contract retains measured backing and canonical prefix",()=>{
 const law=JSON.parse(readFileSync(resolve(import.meta.dir,"../../🧫️fixtures/📥️input/🔣️.json"),"utf8"));const schema=JSON.parse(readFileSync(resolve(import.meta.dir,"../../🧬️schema/📥️input/🔣️.json"),"utf8"));expect(new Ajv({strict:true,allErrors:true}).compile(schema)(law)).toBe(true);const canonical=JSON.stringify(law.input);expect(canonical).toBe(law.canonical);expect(new TextEncoder().encode(canonical).length).toBe(law.capacityBytes);expect(canonical.startsWith(law.partialPrefix)).toBe(true);expect(law.cancelAfterCompleted).toBeLessThan(law.capacityBytes);expect(law.deniedReleaseBytes).toBe(0);const db=new Database(":memory:");try{db.exec("CREATE TABLE input(capacity INTEGER,prefix INTEGER,released INTEGER)");db.prepare("INSERT INTO input VALUES(?,?,?)").run(law.capacityBytes,new TextEncoder().encode(law.partialPrefix).length,law.deniedReleaseBytes);expect(db.query("SELECT capacity,prefix,released FROM input").get()).toEqual({capacity:law.capacityBytes,prefix:12,released:0});}finally{db.close();}console.log("[DEBUG] independent JSON and SQLite retain actual canonical input prefix12 denied release0");
});


test("authored snapshot owner and input transitions remain actual Rust syntax",async()=>{
 await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",import.meta.dir)),"out/tree-sitter-rust.wasm")));const root=resolve(import.meta.dir,"../../../../🖥️host");try{for(const path of ["🦀️.rs","🚪️io/🪶️snapshot/🦀️.rs","🚪️io/🪶️snapshot/📨️metadata/🦀️.rs","🚪️io/🎛️operation/📥️input/🦀️.rs","🧪️tests/🛂️receiving/🦀️.rs","🧪️tests/🔬️owned-instance-open/🦀️.rs","🧪️tests/🔬️owned-instance-open/🪶️lease/🦀️.rs","🧪️tests/🔬️owned-runtime/🦀️.rs","🧪️tests/🔬️mock-guest-runtime/🦀️.rs","📥️ui-patch/🧪️tests/🧪️component/🦀️.rs"]){const tree=parser.parse(readFileSync(resolve(root,path),"utf8"));expect(tree).not.toBeNull();const visit=(node:Parser.SyntaxNode):Parser.SyntaxNode[]=>[node,...node.namedChildren.flatMap(visit)];const errors=visit(tree!.rootNode).filter(node=>node.type==="ERROR"||node.isMissing());for(const error of errors){expect(error.text).toBe("async");console.log(`[DEBUG] original Rust async-closure grammar qualification ${path}:${error.startPosition.row+1}`);}tree!.delete();}}finally{parser.delete();}console.log("[DEBUG] original Host snapshot/input/metadata Rust syntax10 with explicit async-closure grammar qualification; native runtime remains unexecuted");
});


test("actual pending snapshot fuel never refreshes its original ceiling",()=>{const law=JSON.parse(readFileSync(resolve(import.meta.dir,"../../🧫️fixtures/📥️input/🔣️.json"),"utf8")).fuel;const db=new Database(":memory:");try{db.exec("CREATE TABLE fuel(ceiling INTEGER,origin INTEGER,completed INTEGER,pending INTEGER,requested INTEGER)");db.prepare("INSERT INTO fuel VALUES(?,?,?,?,?)").run(law.originalCeiling,law.origin,law.completedOperations.reduce((a:number,b:number)=>a+b,0),law.pendingConsumed,law.requestedTurn);expect(db.query("SELECT ceiling-origin-completed-pending AS remaining,min(requested,ceiling-origin-completed-pending) AS admitted FROM fuel").get()).toEqual({remaining:law.remaining,admitted:law.admittedTurn});expect(JSON.parse(JSON.stringify(law)).originalCeiling).toBe(64);}finally{db.close();}console.log("[DEBUG] independent SQLite conserved original snapshot fuel64 origin7 completed16 pending13 remaining28 admitted20");});


test("real Host tests declare distinct finite Native ceiling and original retirement currencies",()=>{const root=resolve(import.meta.dir,"../../../../🖥️host");const law=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/🛂️receiving/🔣️.json"),"utf8"));const schema=JSON.parse(readFileSync(resolve(root,"🧬️schema/🛂️receiving/🔣️.json"),"utf8"));expect(new Ajv({strict:true,allErrors:true}).compile(schema)(law)).toBe(true);expect(law.maximumNativeBytes).toBeGreaterThan(law.grant.maximumCapacityBytes);expect(law.grant.maximumCopyBytes).not.toBe(law.grant.maximumCapacityBytes);const db=new Database(":memory:");try{db.exec("CREATE TABLE policy(ceiling INTEGER,copy INTEGER,capacity INTEGER,release INTEGER)");db.prepare("INSERT INTO policy VALUES(?,?,?,?)").run(law.maximumNativeBytes,law.grant.maximumCopyBytes,law.grant.maximumCapacityBytes,law.grant.maximumReleaseBytes);expect(db.query("SELECT ceiling,copy,capacity,release FROM policy").get()).toEqual({ceiling:33554432,copy:65536,capacity:2097152,release:33554432});}finally{db.close();}expect(JSON.parse(JSON.stringify(law)).grant).toEqual(law.grant);console.log("[DEBUG] real Host fixture uses explicit32MiB original Native ceiling, distinct copy64KiB capacity2MiB, caller-owned same recipient and observer; native execution pending");});
