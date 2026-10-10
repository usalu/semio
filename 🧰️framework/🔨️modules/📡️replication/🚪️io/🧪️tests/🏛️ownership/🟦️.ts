import { test, expect, describe, it } from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/🟦️.ts";
/** 🏛️ Verifies the shared representation boundary against an independent path oracle. */
import assert from "node:assert/strict";
import { readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";
import { minimatch } from "minimatch";

const owner = resolve(import.meta.dir, "../../..");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🏛️ownership/🔣️.json"), "utf8"));
assert.equal(new Set(fixture.traits.map((row: {name: string}) => row.name)).size, 5);
const semantic = readFileSync(resolve(owner, fixture.semanticOwner), "utf8");
for (const row of fixture.traits as {name: string; owner: string}[]) {
  assert.equal(row.owner.split("/")[0], "🚪️io");
  assert.equal(minimatch(row.owner, "🚪️io/**/*.rs"), true);
  assert.match(readFileSync(resolve(owner, row.owner), "utf8"), new RegExp(`pub trait ${row.name}\\b`));
  assert.doesNotMatch(semantic, new RegExp(`(?:pub trait|pub use[^;]*)\\s${row.name}\\b`));
}
assert.equal(existsSync(resolve(owner, fixture.binaryOwner)), true);
assert.equal(minimatch(fixture.binaryOwner, "🚪️io/💾️binary/**/*.rs"), true);
assert.equal(existsSync(resolve(owner, "🎮️mutation/📦️bytes")), false);
const canonical = fixture.canonical;
const canonicalSemantic = readFileSync(resolve(owner, canonical.semanticOwner), "utf8");
assert.doesNotMatch(canonicalSemantic, /impl[^\n]*ArtifactCanonicalJsonTree/);
assert.equal(existsSync(resolve(owner, canonical.oldSealOwner)), false);
for (const path of [canonical.textOwner, canonical.sealOwner]) {
  assert.equal(minimatch(path, "🚪️io/📝️text/**/*.rs"), true);
  assert.equal(existsSync(resolve(owner, path)), true);
}
const canonicalText = readFileSync(resolve(owner, canonical.textOwner), "utf8");
for (const type of canonical.borrowedTrees as string[]) assert.match(canonicalText, new RegExp(`ArtifactCanonicalJsonTree for ${type}\\b`));
assert.match(readFileSync(resolve(owner, canonical.sealOwner), "utf8"), /struct ArtifactCanonicalEditSealCursor/);
assert.doesNotMatch(semantic, /pub use[^;]*ArtifactCanonicalEditSealCursor/);
console.log(`[DEBUG] replication-io-ownership traits=5 canonicalTrees=${canonical.borrowedTrees.length} boundary=io oracle=minimatch`);

for(const key of ["typescriptBootstrap","typescriptWire"]as const)test(`Original ${key} Implementation Is Owned by IO`,()=>{
 const declared=fixture[key],component=readFileSync(resolve(owner,declared.semanticOwner),"utf8");
 for(const name of declared.functions)assert.equal(new RegExp(`(?:export )?(?:async )?function ${name}\\b`).test(component),false,name);
 for(const name of declared.semanticTypes)assert.match(component,new RegExp(`export type ${name}\\b`));
 assert.equal(minimatch(declared.owner,"🚪️io/**/*.ts"),true);
 const implementation=readFileSync(resolve(owner,declared.owner),"utf8");
 for(const name of declared.functions)assert.match(implementation,new RegExp(`(?:export )?(?:async )?function ${name}\\b`));
 for(const [kind,names]of [["class",declared.classes??[]],["type",declared.types??[]],["interface",declared.interfaces??[]],["const",declared.constants]]as const)for(const name of names){assert.doesNotMatch(component,new RegExp(`(?:export )?${kind} ${name}\\b`));assert.match(implementation,new RegExp(`(?:export )?${kind} ${name}\\b`));}
 console.log(`[DEBUG] ${key} functions=${declared.functions.length} actualOwner=io originalSemanticTypesRemain=true independentPathOracle=minimatch`);
});

test("Original Bootstrap Corpus Has a Closed Schema and Independent SHA-256",async()=>{
 const corpus=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🚀️artifact-bootstrap/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(resolve(owner,"🚪️io/📥️bootstrap/🧬️schema/🔣️.json"),"utf8")),oracle=new Ajv2020({strict:true}).compile(schema);
 const foreign=structuredClone(corpus);foreign.artifact.sourceAuthority=1;
 for(const [value,accepted]of [[corpus,true],[foreign,false]]as const){expect(validateJsonSchemaSubset(schema,value).length===0).toBe(accepted);expect(Boolean(oracle(value))).toBe(accepted);}
 const {artifactBootstrapSha256,artifactBootstrapAggregateHash}=await import("../../../🟦️.ts"),{createHash}=await import("node:crypto");
 const pack=Buffer.from(corpus.payload.packHex,"hex"),spr=Buffer.from(corpus.payload.sprHex,"hex");
 for(const [bytes,expected]of [[pack,corpus.payload.packHash],[spr,corpus.payload.sprHash],[Buffer.concat([pack,spr]),corpus.payload.aggregateHash]]as const){expect(createHash("sha256").update(bytes).digest("hex")).toBe(expected);expect(Buffer.from(await artifactBootstrapSha256(bytes)).toString("hex")).toBe(expected);}
 expect(Buffer.from(await artifactBootstrapAggregateHash(pack,spr)).toString("hex")).toBe(corpus.payload.aggregateHash);
 console.log("[DEBUG] original bootstrap closed first-party/Ajv corpus and independent Node SHA-256 match pack/SPR/aggregate bytes");
});

const originalProtocol=await import("../../../🟦️.ts"),{registerTests1}=await import("../../../🧪️tests/🧪️artifact-bootstrap-protocol/🟦️.ts");
await registerTests1({describe,expect,it}as never,originalProtocol,{directory:owner,url:new URL("../../../🟦️.ts",import.meta.url).href});

test("Original Client and Server Frame Bytes Agree With Independent Protobuf Bodies",async()=>{
 const codec=await import("../../💾️binary/📡️wire/🟦️.ts"),protobuf=await import("protobufjs");
 const rows=[["🎟️client-credit-grant","client",5],["📣️client-preview-publish","client",3],["🙋️client-presence","client",4],["🎫️server-credit-grant","server",7],["🚨️server-error","server",8],["🪪️server-session","server",9]]as const;
 for(const [name,side,tag]of rows){
  const bytes=readFileSync(resolve(owner,`🧫️fixtures/📡️wire/${name}/💾️.bin`)),reader=protobuf.Reader.create(bytes),writer=protobuf.Writer.create().uint32(bytes[0]).uint32(tag);
  expect(bytes[1]).toBe(tag);reader.pos=2;let frame:any;
  if(tag===5||tag===7){const n=reader.uint64().toNumber();frame={CreditGrant:{n}};writer.uint64(n);}
  else if(tag===3){const key=reader.string(),seq=reader.uint64().toNumber(),payload=Array.from(reader.bytes());frame={PreviewPublish:{key,seq,payload}};writer.string(key).uint64(seq).bytes(payload);}
  else if(tag===4){const peer=Array.from(reader.bytes());frame={Presence:{peer}};writer.bytes(peer);}
  else if(tag===8){const code=reader.string(),message=reader.string();frame={Error:{code,message}};writer.string(code).string(message);}
  else{const actor=reader.string(),color=reader.buf[reader.pos++];frame={Session:{actor,color}};writer.string(actor).uint32(color);}
  const lane=bytes[0]===0?"command":"preview",decoded=side==="client"?codec.decodeClientFrame(bytes):codec.decodeServerFrame(bytes),encoded=side==="client"?codec.encodeClientFrame(frame,lane):codec.encodeServerFrame(frame,lane);
  expect(reader.pos).toBe(bytes.length);expect(decoded).toEqual({lane,frame});expect(Buffer.from(encoded).toString("hex")).toBe(bytes.toString("hex"));expect(Buffer.from(writer.finish()).toString("hex")).toBe(bytes.toString("hex"));
 }
 console.log(`[DEBUG] original frame fixtures=${rows.length} clientAndServer=true independentProtobufDecodeAndEncode=true byteExact=true actualWireIO=true`);
});

test("Causal Representation Ownership Uses Original Domain Types and Actual Wire Consumers", () => {
const causal = fixture.causal;
const causalSemantic = readFileSync(resolve(owner, causal.semanticOwner), "utf8");
for (const type of causal.semanticTypes as string[]) assert.match(causalSemantic, new RegExp(`pub struct ${type}\\b`));
for (const name of causal.functions as string[]) assert.equal(new RegExp(`(?:pub )?fn ${name}\\b`).test(causalSemantic), false, name);
assert.equal(minimatch(causal.binaryOwner, "🚪️io/💾️binary/**/*.rs"), true);
const causalBinary = readFileSync(resolve(owner, causal.binaryOwner), "utf8");
for (const name of causal.functions as string[]) assert.match(causalBinary, new RegExp(`(?:pub )?fn ${name}\\b`));
assert.doesNotMatch(causalSemantic, /pub use[^;]*(?:binary::causal|encode_envelope|decode_envelope)/);
for (const path of causal.consumers as string[]) {
  const consumer = readFileSync(resolve(owner, path), "utf8");
  assert.doesNotMatch(consumer, /crate::causal::(?:encode|decode)_/);
  assert.match(consumer, /crate::io::binary::causal::(?:encode|decode)_/);
}
assert.equal(minimatch(causal.textOwner, "🚪️io/📝️text/**/*.rs"), true);
for (const type of causal.valueTypes as string[]) for (const trait of ["ToValue", "FromValue"]) assert.equal(new RegExp(`impl crate::value::${trait} for ${type}\\b`).test(causalSemantic), false, `${trait} ${type}`);
const causalText = readFileSync(resolve(owner, causal.textOwner), "utf8");
for (const type of causal.valueTypes as string[]) for (const trait of ["ToValue", "FromValue"]) assert.equal(new RegExp(`impl crate::value::${trait} for ${type}\\b`).test(causalText), true, `${trait} ${type}`);
console.log(`[DEBUG] causal-codec-ownership functions=${causal.functions.length} semanticTypes=${causal.semanticTypes.length} actualConsumer=true oracle=minimatch`);
});

test("Neutral Replication Representation Ownership Is Closed and Agrees With Ajv", () => {
 const schema = JSON.parse(readFileSync(resolve(owner, "🚪️io/🧬️schema/🏛️ownership/🔣️.json"), "utf8"));
 const oracle = new Ajv2020({ strict: true }).compile(schema);
 const wrongOwner = structuredClone(fixture); wrongOwner.causal.binaryOwner = fixture.causal.semanticOwner;
 const duplicate = structuredClone(fixture); duplicate.causal.functions[1] = duplicate.causal.functions[0];
 const foreign = { ...fixture, legacyAlias: true };
 for (const [candidate, accepted] of [[fixture, true], [wrongOwner, false], [duplicate, false], [foreign, false]] as const) {
  expect(validateJsonSchemaSubset(schema, candidate).length === 0).toBe(accepted);
  expect(Boolean(oracle(candidate))).toBe(accepted);
 }
 console.log("[DEBUG] replication ownership neutral positive and three hostile domains agree first-party/Ajv; native wire behavior remains separately required");
});

test("Original TypeScript Task Inputs Include Every Extracted IO Source Owner",()=>{
 const declared=fixture.typescriptInputs,project=JSON.parse(readFileSync(resolve(owner,declared.projectOwner),"utf8"));
 expect(project.namedInputs.default).toContain(declared.requiredSourceInput);
 const pattern=declared.requiredSourceInput.replace("{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/","");
 for(const path of [fixture.primitives.binaryOwner,fixture.presence.binaryOwner,fixture.typescriptCausal.binaryOwner,fixture.typescriptBackbone.binaryOwner,fixture.typescriptBootstrap.owner,fixture.typescriptWire.owner])expect(minimatch(path,pattern)).toBe(true);
 console.log("[DEBUG] original TypeScript Nx input covers all six actual IO runtime owners independentPathOracle=minimatch");
});

test("TypeScript Binary Primitives Are Owned by IO and Real Frame Consumers Import Them", () => {
 const primitive = fixture.primitives;
 const component = readFileSync(resolve(owner, primitive.semanticOwner), "utf8");
 for (const name of primitive.functions as string[]) assert.equal(new RegExp(`export function ${name}\\b`).test(component), false, name);
 assert.equal(primitive.binaryOwner.split("/")[0], "🚪️io");
 assert.equal(minimatch(primitive.binaryOwner, "🚪️io/💾️binary/**/*.ts"), true);
 const binary = readFileSync(resolve(owner, primitive.binaryOwner), "utf8");
 for (const name of primitive.functions as string[]) assert.equal(new RegExp(`export function ${name}\\b`).test(binary), true, name);
 assert.match(readFileSync(resolve(owner,fixture.typescriptWire.owner),"utf8"), /import \{[^}]+writeVarintU64[^}]+\} from "\.\.\/🟦️\.ts"/);
 console.log(`[DEBUG] actual TypeScript binary primitives=${primitive.functions.length} owner=io frameConsumer=true independentPathOracle=minimatch`);
});


test("Presence Binary IO Implements Original Semantic Types Outside the Component", () => {
 const presence = fixture.presence;
 const component = readFileSync(resolve(owner, presence.semanticOwner), "utf8");
 for (const name of presence.functions as string[]) assert.equal(new RegExp(`export function ${name}\\b`).test(component), false, name);
 for (const name of presence.semanticTypes as string[]) assert.match(component, new RegExp(`export type ${name}\\b`));
 assert.equal(minimatch(presence.binaryOwner, "🚪️io/💾️binary/**/*.ts"), true);
 const binary = readFileSync(resolve(owner, presence.binaryOwner), "utf8");
 for (const name of presence.functions as string[]) assert.match(binary, new RegExp(`export function ${name}\\b`));
 assert.doesNotMatch(component, /class PresencePeerReader\b/);
 assert.match(binary, /class PresencePeerReader\b/);
 assert.match(binary, /import type \{[^}]*ArtifactPresencePeer[^}]*\} from "\.\.\/\.\.\/\.\.\/🟦️\.ts"/);
 console.log(`[DEBUG] presence binary functions=${presence.functions.length} originalSemanticTypes=${presence.semanticTypes.length} boundary=io oracle=minimatch`);
});


test("Original Binary Primitive Bytes Match the Neutral Corpus and Protobufjs", async () => {
 const binary = await import("../../💾️binary/🟦️.ts");
 const protobuf = await import("protobufjs");
 const corpus = JSON.parse(readFileSync(resolve(owner, "🚪️io/💾️binary/🧫️fixtures/🔣️.json"), "utf8"));
 const schema = JSON.parse(readFileSync(resolve(owner, "🚪️io/💾️binary/🧬️schema/🔣️.json"), "utf8"));
 expect(validateJsonSchemaSubset(schema, corpus)).toHaveLength(0);
 expect(new Ajv2020({ strict: true }).compile(schema)(corpus)).toBe(true);
 for (const row of corpus.cases) {
  const output: number[] = [], writer = protobuf.Writer.create();
  const position: [number] = [0];
  let decoded: unknown, independent: unknown, oracle: Uint8Array;
  switch (row.kind) {
   case "u64": binary.writeVarintU64(output, row.value); oracle = writer.uint64(row.value).finish(); decoded = binary.readVarintU64(Uint8Array.from(output), position); independent = protobuf.Reader.create(oracle).uint64().toNumber(); break;
   case "u64Exact": binary.writeVarintU64Exact(output, BigInt(row.value)); oracle = writer.uint64(row.value).finish(); decoded = binary.readVarintU64Exact(Uint8Array.from(output), position).toString(); independent = protobuf.Reader.create(oracle).uint64().toString(); break;
   case "text": binary.writeStr(output, row.value); oracle = writer.string(row.value).finish(); decoded = binary.readStr(Uint8Array.from(output), position); independent = protobuf.Reader.create(oracle).string(); break;
   case "bytes": binary.writeBytes(output, row.value); oracle = writer.bytes(row.value).finish(); decoded = binary.readBytes(Uint8Array.from(output), position); independent = Array.from(protobuf.Reader.create(oracle).bytes()); break;
   case "hash": binary.writeHash32(output, row.value); oracle = writer.bytes(row.value).finish().subarray(1); decoded = binary.readHash32(Uint8Array.from(output), position); independent = Array.from(protobuf.Reader.create(Uint8Array.from([32, ...oracle])).bytes()); break;
   case "bool": binary.writeBool(output, row.value); oracle = writer.bool(row.value).finish(); decoded = binary.readBool(Uint8Array.from(output), position); independent = protobuf.Reader.create(oracle).bool(); break;
   case "f64": binary.writeF64(output, row.value); oracle = writer.double(row.value).finish(); decoded = binary.readF64(Uint8Array.from(output), position); independent = protobuf.Reader.create(oracle).double(); break;
   case "vectorBytes": binary.writeVecBytes(output, row.value); writer.uint32(row.value.length); for (const bytes of row.value) writer.bytes(bytes); oracle = writer.finish(); decoded = binary.readVecBytes(Uint8Array.from(output), position); const reader = protobuf.Reader.create(oracle); independent = Array.from({length: reader.uint32()}, () => Array.from(reader.bytes())); break;
   default: throw Error("Unknown original primitive corpus kind");
  }
  expect(Buffer.from(output).toString("hex")).toBe(row.hex);
  expect(Buffer.from(oracle).toString("hex")).toBe(row.hex);
  expect(decoded).toEqual(row.value);
  expect(independent).toEqual(row.value);
  expect(position[0]).toBe(output.length);
 }
 console.log(`[DEBUG] actual binary primitive corpus=${corpus.cases.length} byteExact=true decodePositionExact=true independentProtobufjs=true independentAjv=true`);
});


test("TypeScript Causal Representation Uses IO With Original Semantic Contracts", () => {
 const causal = fixture.typescriptCausal;
 const component = readFileSync(resolve(owner, causal.semanticOwner), "utf8");
 for (const name of causal.functions as string[]) assert.equal(new RegExp(`(?:export )?function ${name}\\b`).test(component), false, name);
 for (const name of causal.semanticTypes as string[]) assert.match(component, new RegExp(`export type ${name}\\b`));
 assert.doesNotMatch(component, new RegExp(`export interface ${causal.codecInterface}\\b`));
 assert.equal(minimatch(causal.binaryOwner, "🚪️io/💾️binary/**/*.ts"), true);
 const binary = readFileSync(resolve(owner, causal.binaryOwner), "utf8");
 for (const name of causal.functions as string[]) assert.match(binary, new RegExp(`(?:export )?function ${name}\\b`));
 assert.match(binary, new RegExp(`export interface ${causal.codecInterface}\\b`));
 assert.match(readFileSync(resolve(owner,fixture.typescriptWire.owner),"utf8"), /import \{[^}]*encodeExactEnvelope[^}]*\} from "\.\.\/🔗️causal\/🟦️\.ts"/);
 console.log(`[DEBUG] original TS causal codecs=${causal.functions.length} domainTypes=${causal.semanticTypes.length} independentIOPath=minimatch nativeRuntime=false`);
});

test("Strict Document Backbone Bytes Are Owned by IO and Retention Stays Semantic", () => {
 const backbone=fixture.typescriptBackbone, component=readFileSync(resolve(owner,backbone.semanticOwner),"utf8");
 for(const name of backbone.functions)assert.equal(new RegExp(`(?:export )?function ${name}\\b`).test(component),false,name);
 for(const name of backbone.semanticTypes)assert.match(component,new RegExp(`export type ${name}\\b`));
 for(const name of backbone.semanticConstants)assert.match(component,new RegExp(`export const ${name}\\b`));
 assert.equal(minimatch(backbone.binaryOwner,"🚪️io/💾️binary/**/*.ts"),true);
 const binary=readFileSync(resolve(owner,backbone.binaryOwner),"utf8");
 for(const name of backbone.functions)assert.match(binary,new RegExp(`(?:export )?function ${name}\\b`));
 for(const name of backbone.codecTypes)assert.match(binary,new RegExp(`export type ${name}\\b`));
 for(const name of backbone.codecErrors)assert.match(binary,new RegExp(`export class ${name}\\b`));
 for(const name of backbone.codecConstants)assert.match(binary,new RegExp(`export const ${name}\\b`));
 assert.match(binary,/import type \{ ExactWireMutationEnvelope \} from "\.\.\/\.\.\/\.\.\/\.\.\/🟦️\.ts"/);
 assert.match(readFileSync(resolve(owner,fixture.typescriptWire.owner),"utf8"),/import \{[^}]*readDocumentBackboneEnvelopeBatchExact[^}]*\} from "\.\.\/🔗️causal\/🧮️backbone\/🟦️\.ts"/);
 console.log(`[DEBUG] original strict backbone functions=${backbone.functions.length} byteOwner=io retentionOwner=semantic independentPathOracle=minimatch`);
});

test("Strict Backbone IO Preserves Original Thirty-Case Corpus and Independent Protobuf Bytes", async () => {
 const codec=await import("../../💾️binary/🔗️causal/🧮️backbone/🟦️.ts"),protobuf=await import("protobufjs");
 const corpus=JSON.parse(readFileSync(resolve(owner,"🔗️causal/🧫️fixtures/🧮️document-backbone-batch-v1/🔣️.json"),"utf8"));
 const hex=(bytes:Uint8Array)=>Buffer.from(bytes).toString("hex");
 const project=(envelopes:readonly any[])=>envelopes.map(envelope=>({mutationId:envelope.mutation_id,documentId:envelope.document_id,actor:envelope.actor,dependencies:envelope.dependencies,observed:envelope.observed,target:envelope.target,diff:{schema:envelope.diff.schema,payloadHex:hex(envelope.diff.payload)},inverse:{schema:envelope.inverse.schema,payloadHex:hex(envelope.inverse.payload)},timestamp:{actor:envelope.timestamp.actor.toString(),physicalMs:envelope.timestamp.physical_ms.toString(),logical:envelope.timestamp.logical.toString()},transaction:envelope.transaction,verb:envelope.verb}));
 let acceptedCases=0,deniedCases=0;
 for(const row of corpus.cases){
  const bytes=Buffer.from(row.rawHex,"hex");
  if(row.expect.outcome!=="accepted"){
   let failure:unknown;try{codec.decodeDocumentBackboneEnvelopeBatchExact(bytes,row.limits);}catch(error){failure=error;}
   expect(failure).toBeInstanceOf(codec.DocumentBackboneBatchError);expect((failure as Error&{outcome:string}).outcome).toBe(row.expect.outcome);expect((failure as Error&{reason:string}).reason).toBe(row.expect.reason);
   if(row.expect.reason!=="trailing-bytes"){const position:[number]=[0];expect(()=>codec.readDocumentBackboneEnvelopeBatchExact(bytes,position,row.limits)).toThrow(codec.DocumentBackboneBatchError);expect(position[0]).toBe(0);}
   deniedCases++;continue;
  }
  const decoded=codec.decodeDocumentBackboneEnvelopeBatchExact(bytes,row.limits);expect(project(decoded)).toEqual(row.expect.envelopes);expect(hex(codec.encodeDocumentBackboneEnvelopeBatchExact(decoded))).toBe(row.rawHex);
  const position:[number]=[1],embedded=codec.readDocumentBackboneEnvelopeBatchExact(Buffer.concat([Buffer.from([0xaa]),bytes,Buffer.from([0xbb])]),position,row.limits);expect(position[0]).toBe(1+bytes.length);expect(project(embedded.envelopes)).toEqual(row.expect.envelopes);expect(hex(embedded.bytes)).toBe(row.rawHex);
  const writer=protobuf.Writer.create().uint64(row.expect.envelopes.length);
  for(const envelope of row.expect.envelopes){
   writer.string(envelope.mutationId).string(envelope.documentId).string(envelope.actor).uint64(envelope.dependencies.length);for(const value of envelope.dependencies)writer.string(value);
   writer.bool(envelope.observed!==null);if(envelope.observed!==null)writer.string(envelope.observed);writer.uint64(envelope.target.length);for(const value of envelope.target)writer.string(value);
   writer.string(envelope.diff.schema).bytes(Buffer.from(envelope.diff.payloadHex,"hex")).string(envelope.inverse.schema).bytes(Buffer.from(envelope.inverse.payloadHex,"hex")).uint64(envelope.timestamp.actor).uint64(envelope.timestamp.physicalMs).uint64(envelope.timestamp.logical).uint64((envelope.transaction===null?0:1)|(envelope.verb===null?0:2));
   if(envelope.transaction!==null)writer.string(envelope.transaction.id).string(envelope.transaction.tool);if(envelope.verb!==null)writer.string(envelope.verb);
  }
  expect(hex(writer.finish())).toBe(row.rawHex);acceptedCases++;
 }
 expect(acceptedCases).toBe(10);expect(deniedCases).toBe(20);
 console.log(`[DEBUG] original strict backbone corpus=${corpus.cases.length} accepted=${acceptedCases} denied=${deniedCases} exactU64=true failedEmbeddedCursorUnchanged=true independentProtobufjs=true`);
});


test("Extracted Presence IO Preserves the Complete Original Strict Corpus and Protobuf Bytes", async () => {
 const codec=await import("../../💾️binary/👥️presence/🟦️.ts"), protobuf=await import("protobufjs");
 const corpus=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/👥️presence-peer-codec-v1/🔣️.json"),"utf8"));
 expect(codec.PRESENCE_PEER_WIRE_LIMITS_V1).toEqual(corpus.limits);
 let independentCases=0;
 for(const row of corpus.cases){
  const bytes=Buffer.concat([Buffer.from(row.prefixHex,"hex"),Buffer.alloc(row.repeatCount,Number.parseInt(row.repeatHex,16)),Buffer.from(row.suffixHex,"hex")]);
  const position:[number]=[0];
  if(!row.accepted){expect(()=>codec.decodePresencePeer(bytes,position)).toThrow(Error);expect(position[0]).toBe(0);continue;}
  const peer=codec.decodePresencePeer(bytes,position);expect(position[0]).toBe(bytes.length);
  expect(Buffer.from(codec.encodePresencePeer(peer)).toString("hex")).toBe(row.canonicalHex);
  const semantic=JSON.parse(JSON.stringify(peer));if(peer.presencePack!==undefined)semantic.presencePack=Buffer.from(peer.presencePack).toString("base64");if(peer.interaction!==undefined){semantic.interaction.appId=peer.interaction.app_id;delete semantic.interaction.app_id;}
  expect(semantic).toEqual(row.expected);
  const reference=protobuf.Reader.create(bytes);expect(reference.string()).toBe(row.expected.actor);const flags=reference.uint64().toNumber();expect(reference.uint64().toNumber()).toBe(row.expected.connectedAtMs);
  if((flags&0b1111111111)!==0)continue;
  const writer=protobuf.Writer.create().string(row.expected.actor).uint64(flags).uint64(row.expected.connectedAtMs);
  if(row.expected.toolRun){const run=row.expected.toolRun;writer.string(run.toolId).uint32(["starting","running","paused","complete","finalizing","finalized","aborting","aborted","faulted"].indexOf(run.state)).uint64(run.stage).uint64(run.completed).bool(run.total!==undefined);if(run.total!==undefined)writer.uint64(run.total);}
  if(row.expected.principalKind!==undefined)writer.uint32(["human","agent"].indexOf(row.expected.principalKind));
  if(row.expected.activeTool!==undefined)writer.string(row.expected.activeTool);
  if(row.expected.historyEdit){const edit=row.expected.historyEdit;writer.string(edit.mutationId).uint32(["editing","replaying","reviewing","choosing","finalizing"].indexOf(edit.stage)).uint64(edit.drafts);}
  if(row.expected.typing){writer.uint64(row.expected.typing.length);for(const run of row.expected.typing)writer.string(run.windowId).string(run.deleted).string(run.insert);}
  expect(Buffer.from(writer.finish()).toString("hex")).toBe(row.canonicalHex);independentCases++;
 }
 expect(independentCases).toBeGreaterThanOrEqual(8);
 console.log(`[DEBUG] original Presence corpus=${corpus.cases.length} allStrictDenialsPreserveCursor=true byteExact=true independentProtobufCases=${independentCases}`);
});

test("Original Presence and Backbone vectors have no whole-trial authority", () => {
 for (const path of ["🚪️io/💾️binary/👥️presence/🧬️schema/🔣️.json", "🚪️io/💾️binary/🔗️causal/🧮️backbone/🧬️schema/🔣️.json"]) expect(existsSync(resolve(owner,path))).toBe(false);
});
