import "../🛬️decoding/🟦️.ts";
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import TOML from "@iarna/toml";
import { Database } from "bun:sqlite";
import { ValueError, type ValueRefusalKind } from "../../../../🌱️value/⚠️refusal/🟦️.ts";

const owner = resolve(import.meta.dir, "../.."), read = (path: string): string => readFileSync(resolve(owner, path), "utf8");

test("encoded record primitive ownership has independent closed cardinalities and SQLite fields",async()=>{
 const {default:Ajv}=await import("ajv/dist/2020.js"),fixture=JSON.parse(read("🧫️fixtures/💰️record-backing/🔣️.json")) as {cardinalities:number[];primitive:{fieldIdStart:number;integerStart:number};policies:Record<string,string>;descriptor:Record<string,boolean>};
 const validate=new Ajv({strict:true}).compile(JSON.parse(read("🧫️fixtures/💰️record-backing/🧬️schema/🔣️.json")));expect(validate(fixture)).toBe(true);expect(validate({...fixture,cardinalities:[0,1,3]})).toBe(false);expect(validate({...fixture,policies:{...fixture.policies,requestAuthority:"logicalCount"}})).toBe(false);expect(validate({...fixture,unknown:0})).toBe(false);
 const oracle=new Database(":memory:");try{for(const count of fixture.cardinalities){const expected=oracle.query("WITH RECURSIVE field(i) AS (SELECT 0 WHERE ?1>0 UNION ALL SELECT i+1 FROM field WHERE i+1<?1) SELECT i+?2 AS fieldId,i+?3 AS integerValue FROM field ORDER BY i").all(count,fixture.primitive.fieldIdStart,fixture.primitive.integerStart);const fields=Array.from({length:count},(_,index)=>({fieldId:index+fixture.primitive.fieldIdStart,integerValue:index+fixture.primitive.integerStart}));expect<unknown>(fields).toEqual(expected);expect(new Set(fields.map(field=>field.fieldId)).size).toBe(count);for(const field of fields){const bytes=Buffer.alloc(10);bytes.writeUInt16LE(field.fieldId);bytes.writeBigInt64LE(BigInt(field.integerValue),2);const view=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength);expect(view.getUint16(0,true)).toBe(field.fieldId);expect(view.getBigInt64(2,true)).toBe(BigInt(field.integerValue));}}}finally{oracle.close();}
});

test("actual generic record metadata has one independent canonical package below the container", () => {
  const path = "📦️packages/🦀️rust/Cargo.toml";
  expect(existsSync(resolve(owner, path)), "actual record provider").toBe(true);
  const source = read(path), native = Bun.TOML.parse(source) as {package: {name: string}; lib: {name: string; path: string}; dependencies: Record<string, unknown>};
  expect(native).toEqual(TOML.parse(source) as typeof native);
  expect(native.package.name).toBe("semio-framework-dsl-record");
  expect(native.lib).toEqual({name: "semio_framework_dsl_record", path: "../../🦀️.rs"});
  expect(Object.keys(native.dependencies).sort()).toEqual(["semio-framework-diagnostic", "semio-framework-dsl", "semio-framework-pack-json", "semio-framework-value"]);
});

test("record model and binding definitions directly select the actual neutral value and JSON floors", () => {
  for (const path of ["🦀️.rs", "🪆️binding/🦀️.rs", "🏭️producer/🦀️.rs", "🛬️decoding/🦀️.rs", "🛫️encoding/🦀️.rs", "🛫️encode/🦀️.rs"]) {
    const source = read(path);
    for (const spelling of ["protocol::value::", "protocol::bytes::", "crate::os_pack::", "use crate::os_dsl::"]) expect(source.includes(spelling), `${path}: ${spelling}`).toBe(false);
  }
  expect(read("🦀️.rs").includes("pub struct RecordSpec")).toBe(true);
  expect(read("🪆️binding/🦀️.rs").includes("pub trait DslField")).toBe(true);
  expect(read("🪆️binding/🦀️.rs").includes("pub trait DslVariants")).toBe(true);
  expect(read("🦀️.rs").includes("semio_framework_pack_json::")).toBe(true);
});

test("schema constructor refusal corpus preserves all eight semantic kinds at every owned boundary", async () => {
  const {default: Ajv}=await import("ajv/dist/2020.js");
  const schema=JSON.parse(read("🧬️schema/⚠️refusal/🔣️.json"));
  const fixture=JSON.parse(read("🧫️fixtures/⚠️refusal/🔣️.json"));
  const validate=new Ajv({strict:true}).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(new Set(fixture.cases.map((row: {id:string})=>row.id)).size).toBe(24);
  for(const row of fixture.cases)expect(JSON.parse(JSON.stringify({kind:row.kind,message:row.boundary+"."+row.message}))).toEqual(row.expected);
});

test("controlled field refusal matrix has an independent schema and path output authority", async () => {
  const { default: Ajv } = await import("ajv/dist/2020.js");
  const schema = JSON.parse(read("🧬️schema/🪆️refusal/🔣️.json"));
  const fixture = JSON.parse(read("🧫️fixtures/🪆️refusal/🔣️.json")) as {
    kinds: string[];
    paths: { method: string; boxed: boolean; message: string; under: string; expectedMessage: string }[];
    derivedPaths: { method: string; boxed: boolean; message: string; under: string; expectedMessage: string }[];
  };
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  const kinds: Record<string, ValueRefusalKind> = {
    InvalidValue: "invalidValue", Canceled: "canceled", OwnershipLimit: "ownershipLimit", AllocationFailed: "allocationFailed",
    WorkLimit: "workLimit", DepthLimit: "depthLimit", UnsupportedOwner: "unsupportedOwner", InvariantViolated: "invariantViolated",
  };
  const database = new Database(":memory:");
  try {
    const reference = database.query("SELECT lower(substr(k.value,1,1))||substr(k.value,2) AS kind, json_extract(p.value,'$.method') AS method, json_extract(p.value,'$.boxed') AS boxed, json_extract(p.value,'$.under')||'.'||json_extract(p.value,'$.message') AS message FROM json_each(?1,'$.kinds') k CROSS JOIN json_each(?1,'$.paths') p ORDER BY k.key,p.key").all(JSON.stringify({...fixture,paths:fixture.paths.concat(fixture.derivedPaths)}));
    const actual = fixture.kinds.flatMap(kind => fixture.paths.concat(fixture.derivedPaths).map(path => {
      const error = new ValueError(kinds[kind], path.message).under(path.under);
      expect(error.message).toBe(path.expectedMessage);
      return { kind: error.kind, method: path.method, boxed: Number(path.boxed), message: error.message };
    }));
    expect(actual).toHaveLength(80);
    expect<unknown>(actual).toEqual(reference);
  } finally {
    database.close();
  }
});


test("controlled numeric refusals use canonical wire identities with an independent schema and JSON authority",async()=>{
 const {default:Ajv}=await import("ajv/dist/2020.js");
 const fixture=JSON.parse(read("🧫️fixtures/🔢️number-refusal/🔣️.json")) as {cases:{id:string;fragment:string;repeat:number;expected:{kind:"workLimit"|"invalidValue";message:string}}[]};
 const schema=JSON.parse(read("🧫️fixtures/🔢️number-refusal/🧬️schema/🔣️.json"));
 const validate=new Ajv({strict:true}).compile(schema);
 expect(validate(fixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  const reference=database.query("SELECT json_extract(value,'$.expected.kind') AS kind,json_extract(value,'$.expected.message') AS message FROM json_each(?1,'$.cases') ORDER BY key").all(JSON.stringify(fixture));
  const actual=fixture.cases.map(row=>{const error=new ValueError(row.expected.kind,row.expected.message);return{kind:error.kind,message:error.message};});
  expect<unknown>(actual).toEqual(reference);
  expect(validate({...fixture,cases:fixture.cases.map(row=>({...row,expected:{...row.expected,kind:"WorkLimit"}}))})).toBe(false);
 }finally{database.close();}
});

test("encoded record denied deep insert has a closed literal state and independent SQLite capacity",async()=>{
 const {default:Ajv}=await import("ajv/dist/2020.js");
 const fixture=JSON.parse(read("🧫️fixtures/💰️record-backing/🛑️rejected-insert/🔣️.json")) as {slots:number;existing:{id:number;integer:number};replacement:{integer:number};rejected:{id:number;depth:number;payloadBytes:number;byte:number};stackBytes:number;expected:{kind:string;length:number;integer:number;reclaimedLeaves:number;reclaimedBlocks:number};policies:Record<string,string>};
 const validate=new Ajv({strict:true}).compile(JSON.parse(read("🧫️fixtures/💰️record-backing/🛑️rejected-insert/🧬️schema/🔣️.json")));
 expect(validate(fixture)).toBe(true);expect(validate({...fixture,rejected:{...fixture.rejected,depth:2047}})).toBe(false);expect(validate({...fixture,unknown:0})).toBe(false);expect(validate({...fixture,expected:{...fixture.expected,kind:"ownershipLimit"}})).toBe(false);
 const db=new Database(":memory:");try{
 db.run("CREATE TABLE slot(id INTEGER PRIMARY KEY CHECK(id="+fixture.existing.id+"), integer_value INTEGER NOT NULL)");db.run("INSERT INTO slot VALUES(?,?)",[fixture.existing.id,fixture.existing.integer]);db.run("UPDATE slot SET integer_value=? WHERE id=?",[fixture.replacement.integer,fixture.existing.id]);
 expect(()=>db.run("INSERT INTO slot VALUES(?,?)",[fixture.rejected.id,0])).toThrow();expect<unknown>(db.query("SELECT id,integer_value AS integerValue FROM slot ORDER BY id").all()).toEqual([{id:fixture.existing.id,integerValue:fixture.expected.integer}]);expect(db.query("SELECT count(*) AS total FROM slot").get()).toEqual({total:fixture.expected.length});
 const leaf=Buffer.alloc(fixture.rejected.payloadBytes,fixture.rejected.byte);expect(leaf.length).toBe(65536);expect(leaf.every(value=>value===120)).toBe(true);const depth=db.query("WITH RECURSIVE path(n) AS(SELECT 0 UNION ALL SELECT n+1 FROM path WHERE n<?) SELECT max(n) AS depth FROM path").get(fixture.rejected.depth);expect(depth).toEqual({depth:fixture.expected.reclaimedBlocks});expect(fixture.expected.reclaimedLeaves).toBe(1);expect(fixture.policies.retirementFrontier).toBe("separateObservedRequests");
 }finally{db.close();}
});

test("borrowed object allocation fixture has a closed canonical schema and independent SQLite values",async()=>{
 const {default:Ajv}=await import("ajv/dist/2020.js"),fixture=JSON.parse(read("🧪️tests/🫳️borrowed-object/🧫️fixtures/🔣️.json")) as {schema:string;depth:number;payloadBytes:number;allocationMultiplier:number;unsigned:string;binary64Bits:string};
 const validate=new Ajv({strict:true}).compile(JSON.parse(read("🧪️tests/🫳️borrowed-object/🧬️schema/🔣️.json")));expect(validate(fixture)).toBe(true);expect(validate({...fixture,schema:"different.owner/v1"})).toBe(false);expect(validate({...fixture,unknown:0})).toBe(false);expect(validate({...fixture,depth:0})).toBe(false);
 const actual={schema:fixture.schema,depth:fixture.depth,payloadBytes:fixture.payloadBytes,allocationMultiplier:fixture.allocationMultiplier,unsigned:fixture.unsigned,binary64Bits:fixture.binary64Bits},database=new Database(":memory:");try{expect<unknown>(actual).toEqual(database.query("SELECT json_extract(?1,'$.schema') AS schema,json_extract(?1,'$.depth') AS depth,json_extract(?1,'$.payloadBytes') AS payloadBytes,json_extract(?1,'$.allocationMultiplier') AS allocationMultiplier,json_extract(?1,'$.unsigned') AS unsigned,json_extract(?1,'$.binary64Bits') AS binary64Bits").get(JSON.stringify(fixture)));}finally{database.close();}
});
