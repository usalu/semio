import { test, expect } from "bun:test";
import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv/dist/2020";
import {spawnSync} from "node:child_process";
import {parseArtifactPreparedOperationOutput,parseArtifactPreparedOperations} from "../📦️output/🧬️schema/🟦️.ts";
const base = join(import.meta.dir, "..");
const fixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/🔣️.json"), "utf8"));
test("borrowed operation wire neutral UTF8/ordered JSON and bounded chunk oracle", () => {
  for (const row of fixture.cases) {
    const bytes = Buffer.concat([Buffer.from(row.header), Buffer.from(JSON.stringify(row.body))]);
    expect(Array.from(new TextEncoder().encode(JSON.stringify(row.body)))).toEqual(Array.from(bytes.subarray(row.header.length)));
    for (const grant of fixture.grants.filter((value: number) => value > 0)) {
      const chunks = [];
      for (let offset = 0; offset < bytes.length; offset += Math.min(grant, fixture.copyBytes)) chunks.push(bytes.subarray(offset, offset + Math.min(grant, fixture.copyBytes)));
      expect(chunks.every(chunk => chunk.length <= grant && chunk.length <= 64)).toBe(true);
      expect(Buffer.concat(chunks)).toEqual(bytes);
      expect(JSON.parse(bytes.subarray(row.header.length).toString("utf8"))).toEqual(row.body);
    }
  }
});
test("installed borrowed operation cursor retains no owner copy or whole encode fallback", () => {
  const path = join(base, "🦀️.rs");
  expect(existsSync(path)).toBe(true);
  const source = readFileSync(path, "utf8");
  for (const name of ["ArtifactPreparedOperationSource", "ArtifactPreparedOperationCursor", "maximum_copy_bytes", "source_identity", "ArtifactCanonicalJsonCursor"]) expect(source).toContain(name);
  expect(source).not.toContain("encode_op(");
  expect(source).not.toContain("unsafe");
});

test("lowercase JSON hex preserves original UTF8 bytes and nibble continuation", () => {
  for (const row of fixture.cases) {
    const raw = Buffer.from(JSON.stringify(row.body));
    const hex = Buffer.from(raw.toString("hex"));
    expect(Buffer.from(hex.toString(), "hex")).toEqual(raw);
    const chunks = Array.from({length: hex.length}, (_, index) => hex.subarray(index, index + 1));
    expect(Buffer.concat(chunks)).toEqual(hex);
  }
  expect(readFileSync(join(base, "🦀️.rs"), "utf8")).toContain("HexJson");
});

test("borrowed textual operation neutral ordered tuple and UTF8 hex oracle", () => {
  const encode = (node: any): string => node.kind === "hex" ? Buffer.from(node.text).toString("hex") : node.kind === "sequence" ? node.open + node.children.map(encode).join(node.separator) + node.close : node.text;
  for (const row of fixture.textCases) {
    const body = encode(row.node);
    expect(Buffer.from(new TextEncoder().encode(body))).toEqual(Buffer.from(body));
    const full = Buffer.concat([Buffer.from(row.header), Buffer.from(body)]);
    for (const grant of fixture.grants.filter((value: number) => value > 0)) {
      const parts = [];
      for (let index = 0; index < full.length; index += Math.min(grant,64)) parts.push(full.subarray(index,index + Math.min(grant,64)));
      expect(Buffer.concat(parts)).toEqual(full);
    }
  }
  expect(readFileSync(join(base, "🦀️.rs"), "utf8")).toContain("ArtifactOperationTextCursor");
});

test("retained canonical Pack operation reports distinct copied, capacity and release authority", () => {
  const source = readFileSync(join(base,"🦀️.rs"),"utf8");
  for (const name of ["BorrowedProjectedPackCursor", "Pack {", "copied_bytes", "retained_capacity_bytes", "released_bytes"]) expect(source).toContain(name);
  expect(source).not.toContain("encode_record_body(");
});

test("original operation output uses independent dense-octet and retained-owner authority",()=>{
  const schema=JSON.parse(readFileSync(join(base,"📦️output/🧬️schema/🔣️.json"),"utf8"));
  const rows=JSON.parse(readFileSync(join(base,"📦️output/🧫️fixtures/🔣️.json"),"utf8"));
  const admit=new Ajv({strict:true}).compile(schema);
  for(const row of rows.examples){expect(admit(row.bytes)).toBe(true);expect(parseArtifactPreparedOperationOutput(row.bytes)).toBe(row.bytes);const bytes=Array.from({length:row.repeat??1},()=>row.bytes).flat();expect(Buffer.from(bytes)).toEqual(Buffer.from(row.text.repeat(row.repeat??1)));expect(bytes).toEqual([...new TextEncoder().encode(row.text.repeat(row.repeat??1))]);}
  for(const value of [[256],[-1],[1.5],[null],{}]){expect(admit(value)).toBe(false);expect(()=>parseArtifactPreparedOperationOutput(value)).toThrow();}
  const sparse=new Array(1);expect(()=>parseArtifactPreparedOperationOutput(sparse)).toThrow();
  const source=readFileSync(join(base,"🦀️.rs"),"utf8");expect(source).toContain("ArtifactPreparedOperationOutput");
  const owner=readFileSync(join(base,"📦️output/🦀️.rs"),"utf8");for(const token of ["RetainedCloneGrant","next_reserve_depth_demand","error.allocated_bytes","next_release_depth_demand","with_retained_progress"])expect(owner).toContain(token);
  expect(owner).not.toContain("to_vec");expect(owner).not.toContain("usize::MAX");
});

test("original retained operation output and native allocator law preserve Rust grammar",()=>{
  for(const path of [join(base,"📦️output/🦀️.rs"),join(base,"🧪️tests/🦀️.rs"),join(base,"📝️text/🦀️.rs")]){const parsed=spawnSync("rustfmt",["--edition","2021","--emit","stdout",path],{timeout:5000,stdio:["ignore","ignore","pipe"]});expect(parsed.error).toBeUndefined();expect(parsed.status).toBe(0);}
});

test("original text refusal preserves initialized UTF8 prefix and canonical physical authority",()=>{
  const row=fixture.textFailure;
  const scalar=Buffer.from(row.scalar);const prefix=Buffer.concat([Buffer.from(row.header),scalar.subarray(0,row.initializedScalarBytes)]);
  expect([...prefix]).toEqual(row.expectedPrefix);expect(prefix.length).toBe(row.expectedCopiedBytes);expect([...new TextEncoder().encode(row.scalar)]).toEqual([...scalar]);
  const root=join(base,"../../../../../..");
  const reasonSchema=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/⚠️refusal/🔁️codec/🧬️schema/🔣️.json"),"utf8"));
  const schema=JSON.parse(readFileSync(join(base,"⚠️failure/🧬️schema/🔣️.json"),"utf8"));const ajv=new Ajv({strict:true});ajv.addSchema(reasonSchema);const admit=ajv.compile(schema);
  const value={writtenBytes:prefix.length,reason:{kind:"invariantViolated",message:"original scalar refusal",retainedProgress:{copiedItems:1,copiedBytes:prefix.length,retainedCapacityBytes:0,releasedBytes:0}}};
  expect(admit(value)).toBe(true);expect(admit({...value,writtenBytes:-1})).toBe(false);
  const text=readFileSync(join(base,"📝️text/🦀️.rs"),"utf8");expect(text).toContain("ArtifactPreparedOperationError");expect(text).not.toContain("Result<(usize, bool), String>");expect(text).not.toContain("ValueError::new");
  const wire=readFileSync(join(base,"🦀️.rs"),"utf8");expect(wire).toContain("text_failure");expect(wire).toContain("error.written_bytes");
});

test("original text operation admits its real frontier under supplied parent depth",()=>{
 const encode=(node:any):string=>node.kind==="sequence"?node.open+node.children.map(encode).join(node.separator)+node.close:node.text;
 for(const row of fixture.textDepthRows){const text=encode(row.node);expect(Buffer.from(text)).toEqual(Buffer.from(new TextEncoder().encode(text)));if(row.refused)expect(text.startsWith(row.expectedPrefix)).toBe(true);else expect(text).toBe(row.expectedText);}
 const text=readFileSync(join(base,"📝️text/🦀️.rs"),"utf8");expect(text).toContain("grant: RetainedCloneGrant");expect(text).toContain("grant.maximum_depth");expect(text).toContain("self.depth + 2");const wire=readFileSync(join(base,"🦀️.rs"),"utf8");expect(wire).not.toContain("grant.maximum_depth < super::ARTIFACT_CANONICAL_JSON_DEPTH");expect(wire).toContain("self.text.advance(body, &mut output[written..maximum], child)");expect(wire).not.toContain("ValueError::new");
 console.log("[DEBUG] original Text byte/sequence Unicode oracle retains parent depth2/3 and refuses unadmitted child before output");
});

test("original proposal contiguous octets preserve independent payload and funded ownership",()=>{
 const rows=JSON.parse(readFileSync(join(base,"📦️output/🧫️fixtures/🔣️.json"),"utf8"));const contract=JSON.parse(readFileSync(join(base,"📦️output/🧬️schema/🔣️.json"),"utf8"));const admit=new Ajv({strict:true}).compile(contract);
 for(const row of rows.examples){const bytes=Array.from({length:row.repeat??1},()=>row.bytes).flat();expect(admit(bytes)).toBe(true);expect(parseArtifactPreparedOperationOutput(bytes)).toBe(bytes);expect(Buffer.from(bytes)).toEqual(Buffer.from(new TextEncoder().encode(row.text.repeat(row.repeat??1))));}
 const source=readFileSync(join(base,"📦️output/🦀️.rs"),"utf8");for(const method of ["pub fn prepare_contiguous","pub fn take_contiguous","contiguous:ManuallyDrop<Option<Vec<u8>>>","grant.maximum_capacity_bytes<capacity","self.bytes.allocated_bytes().checked_add(capacity)","self.contiguous_complete=true","self.close_paged_one(grant)"])expect(source).toContain(method);
 expect(source).not.toMatch(/\.collect::<Vec|\.to_vec\(|unwrap_or_default\(/);const parsed=spawnSync("rustfmt",["--edition","2021","--emit","stdout","--config","skip_children=true",join(base,"📦️output/🦀️.rs")],{stdio:["ignore","ignore","pipe"],timeout:5000});expect(parsed.status).toBe(0);const native=spawnSync("rustfmt",["--edition","2021","--emit","stdout","--config","skip_children=true",join(base,"🧪️tests/🦀️.rs")],{stdio:["ignore","ignore","pipe"],timeout:5000});expect(native.status).toBe(0);
});

test("original proposal operation collection admits real octet rows and funded handoff",()=>{
 const rows=JSON.parse(readFileSync(join(base,"📦️output/🧫️fixtures/🔣️.json"),"utf8"));const contract=JSON.parse(readFileSync(join(base,"📦️output/🧬️schema/🔣️.json"),"utf8"));const admit=new Ajv({strict:true}).addSchema(contract).compile({$ref:contract.$id+"#/$defs/ArtifactPreparedOperations"});
 for(const row of rows.operationRows){expect(admit(row.values)).toBe(true);expect(parseArtifactPreparedOperations(row.values)).toBe(row.values);expect(JSON.parse(JSON.stringify(row.values))).toEqual(row.expected);for(let index=0;index<row.values.length;index++)expect(Buffer.from(row.values[index])).toEqual(Buffer.from(row.expected[index]));}
 for(const value of [new Array(1),[[256]],[[null]],[{}],{}]){expect(admit(value)).toBe(false);expect(()=>parseArtifactPreparedOperations(value)).toThrow();}
 const path=join(base,"📦️output/📋️operations/🦀️.rs");expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");for(const marker of ["pub struct ArtifactPreparedOperationsOwner","pub fn append_original","pub fn prepare_handoff","pub fn take_prepared","ManuallyDrop<PagedList<Option<Vec<u8>>","grant.maximum_capacity_bytes<capacity","self.values.get_mut(self.transferred)","row.take()","fn close_step"])expect(source).toContain(marker);expect(source).not.toMatch(/\.clone\(|\.to_vec\(|\.collect::<Vec|unwrap_or_default\(/);
 const native=readFileSync(join(base,"🧪️tests/🦀️.rs"),"utf8");expect(native).toContain("fn original_operation_rows_preserve_each_buffer_and_paid_collection_cancel_cut");for(const sourcePath of [path,join(base,"🧪️tests/🦀️.rs")]){const parsed=spawnSync("rustfmt",["--edition","2021","--emit","stdout","--config","skip_children=true",sourcePath],{stdio:["ignore","ignore","pipe"],timeout:5000});expect(parsed.status).toBe(0);}
});

test("original wire source boundaries retain independent rows and reuse paid cursor state",()=>{
 const policy=fixture.sourceBoundaryGrant;expect(policy.maximumItems).toBe(1);expect(policy.maximumCopyBytes).toBe(4096);
 for(const row of fixture.sourceBatches){const actual=row.sources.map((source:any)=>[...Buffer.concat([Buffer.from(source.header),Buffer.from(JSON.stringify(source.body))])]);expect(actual).toEqual(row.expected);for(let index=0;index<row.sources.length;index++)expect(actual[index].slice(row.sources[index].header.length)).toEqual([...new TextEncoder().encode(JSON.stringify(row.sources[index].body))]);}
 const source=readFileSync(join(base,"🦀️.rs"),"utf8");expect(source).toContain("pub fn begin_next_source");expect(source).toContain("self.pack.begin_next_source");expect(source).toContain("self.json.begin_next_source");expect(source).toContain("self.text.begin_next_source");
 console.log("[DEBUG] original wire source rows match immutable Unicode/empty octets independently; paid reset/native System remains required");
});
