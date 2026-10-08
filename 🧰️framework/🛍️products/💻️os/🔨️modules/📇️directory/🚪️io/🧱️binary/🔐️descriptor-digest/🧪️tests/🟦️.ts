import {admitCheckpointSelectionV1,decodeCanonicalCheckpointPairV1} from "../../🪢️checkpoint-pair/🟦️.ts";
import {admitCanonicalCheckpointPairV1} from "../../../../🧬️schema/🟦️.ts";
import {artifactHashHex,parseArtifactHashHex} from "../../🪪️artifact-hash/🟦️.ts";
import Parser from "web-tree-sitter";
import {dirname,join} from "node:path";
import {validateDocumentDescriptorV1, type DocumentDescriptor } from "../../../../🧬️schema/🟦️.ts";
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import owner from "../🧫️fixtures/🔣️.json";
import {descriptorDigestEncodingV1,descriptorDigestV1} from "../🟦️.ts";

const directory=new URL("../../../../",import.meta.url);
const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/📇️directory/🛡️artifact-authority.json",directory),"utf8")) as {descriptor:DocumentDescriptor;descriptorEncodingHex:string;descriptorDigestV1:number[]};

test("descriptor canonical bytes and digests match independent language-neutral crypto witnesses",async()=>{
  const bytes=descriptorDigestEncodingV1(fixture.descriptor);
  expect(Buffer.from(bytes).toString("hex")).toBe(fixture.descriptorEncodingHex);
  expect([...createHash("sha256").update(bytes).digest()]).toEqual(fixture.descriptorDigestV1);
  expect([...await descriptorDigestV1(fixture.descriptor)]).toEqual(fixture.descriptorDigestV1);
  expect(()=>descriptorDigestEncodingV1({...fixture.descriptor,spaceId:""})).toThrow();
  expect(()=>descriptorDigestEncodingV1({...fixture.descriptor,packSchemaHash:"AA".repeat(32)})).toThrow();
  console.log("[DEBUG] Directory descriptor canonical framing and WebCrypto SHA256 match authored fixture and independent node crypto");
});

test("descriptor binary bodies have explicit IO ownership",()=>{
  expect(new Ajv({strict:true}).validate({type:"object",additionalProperties:false,required:["schema","owner","nativeLaw","physicalBodies","typescriptBodies","refusals","hashOwner","hashPhysicalBodies","hashVectors","rustReceivers"],properties:{rustReceivers:{type:"array",items:{type:"string"}},hashVectors:{type:"array",minItems:4,items:{type:"object",required:["text","bytes","admitted"],properties:{text:{type:"string"},bytes:{type:"array",minItems:32,maxItems:32,items:{type:"integer",minimum:0,maximum:255}},admitted:{type:"boolean"}}}},hashOwner:{const:"os_directory::io::binary::artifact_hash"},hashPhysicalBodies:{type:"array",minItems:5,items:{type:"string"}},refusals:{type:"array",minItems:5,items:{type:"string"}},schema:{const:"directory.descriptor-digest.io-owner/v1"},owner:{const:"os_directory::io::binary::descriptor_digest"},nativeLaw:{type:"string",minLength:1},physicalBodies:{type:"array",minItems:5,items:{type:"string"}},typescriptBodies:{type:"array",minItems:5,items:{type:"string"}}}},owner)).toBe(true);
  const semanticRust=readFileSync(new URL("🧬️schema/🦀️.rs",directory),"utf8"),semanticTs=readFileSync(new URL("🧬️schema/🟦️.ts",directory),"utf8");
  for(const name of owner.physicalBodies)expect(semanticRust).not.toContain(`fn ${name}(`);
  for(const name of owner.typescriptBodies)expect(semanticTs).not.toContain(`function ${name}(`);
  const native=readFileSync(new URL("🚪️io/🧱️binary/🔐️descriptor-digest/🦀️.rs",directory),"utf8"),typescript=readFileSync(new URL("🚪️io/🧱️binary/🔐️descriptor-digest/🟦️.ts",directory),"utf8");
  for(const name of owner.physicalBodies)expect(native).toContain(`fn ${name}(`);
  for(const name of owner.typescriptBodies)expect(typescript).toContain(`function ${name}(`);
  expect(semanticRust).toContain("pub enum DescriptorDigestError");
  console.log("[DEBUG] Directory native descriptor framing/digest body ownership and pure schema refusal type agree");
});

test("pure descriptor metadata refusals agree with the physical admission boundary",()=>{
  const variants:Record<string,DocumentDescriptor>={
    emptyText:{...fixture.descriptor,spaceId:""},
    zeroHash:{...fixture.descriptor,packSchemaHash:"0".repeat(64)},
    uppercaseHash:{...fixture.descriptor,packSchemaHash:"AA".repeat(32)},
    zeroVersion:{...fixture.descriptor,bootstrapVersion:0},
    commitBeyondHead:{...fixture.descriptor,bootstrapFrontier:{...fixture.descriptor.bootstrapFrontier,commitSeq:fixture.descriptor.bootstrapFrontier.headSeq+1}},
  };
  expect(()=>validateDocumentDescriptorV1(fixture.descriptor)).not.toThrow();
  for(const refusal of owner.refusals){expect(()=>validateDocumentDescriptorV1(variants[refusal]!)).toThrow();expect(()=>descriptorDigestEncodingV1(variants[refusal]!)).toThrow();}
  console.log("[DEBUG] Five neutral descriptor metadata refusals agree before any physical encoding");
});

test("current Rust descriptor owner, validator, and native witnesses parse",async()=>{
  await Parser.init(); const parser=new Parser();
  try{
    parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",import.meta.dir)),"out/tree-sitter-rust.wasm")));
    for(const path of ["🧬️schema/🦀️.rs","🦀️.rs","🚪️io/🧱️binary/🦀️.rs","🚪️io/🧱️binary/🔐️descriptor-digest/🦀️.rs","🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🦀️.rs"]){
      const tree=parser.parse(readFileSync(new URL(path,directory),"utf8"));try{expect(tree.rootNode.hasError()).toBe(false);}finally{tree.delete();}
    }
  }finally{parser.delete();}
  console.log("[DEBUG] Current Rust descriptor physical owner, pure validator, and native witness syntax accepted");
});

test("Directory hash transport and typed checkpoint gates retain no schema codec",()=>{
  const rust=readFileSync(new URL("🧬️schema/🦀️.rs",directory),"utf8"),ts=readFileSync(new URL("🧬️schema/🟦️.ts",directory),"utf8"),pair=readFileSync(new URL("🧬️schema/🪢️canonical-checkpoint-pair-v1/🦀️.rs",directory),"utf8");
  expect(rust).not.toContain("pub fn parse_hex(");expect(rust).not.toContain("pub fn hex(");expect(rust).not.toContain("pub fn hex_lower(");
  expect(rust).not.toContain("pub fn artifact_frontier(");expect(rust).not.toContain("pub fn of_artifact_frontier(");
  expect(ts).not.toContain("function canonicalCheckpointPairHexV1(");expect(pair).not.toContain(".hex()");
  expect(pair).toContain("expected: &AdmittedCheckpointSelectionV1");
  const native=readFileSync(new URL("🚪️io/🧱️binary/🪪️artifact-hash/🦀️.rs",directory),"utf8");for(const name of owner.hashPhysicalBodies)expect(native).toContain(`fn ${name}(`);
  console.log("[DEBUG] Directory hash IO ownership and typed checkpoint pure admission bodies accepted");
});

test("semantic hash values roundtrip only through explicit native text IO",()=>{
  for(const vector of owner.hashVectors){
    expect(artifactHashHex(vector.bytes)).toBe(Buffer.from(vector.bytes).toString("hex"));
    const admitted=parseArtifactHashHex(vector.text);
    expect(admitted!==undefined).toBe(vector.admitted);
    if(admitted)expect(admitted).toEqual([...Buffer.from(vector.text,"hex")]);
  }
  console.log("[DEBUG] Four neutral hash text vectors agree with independent Buffer encoding/admission witnesses");
});

test("current native hash receiving imports and typed pair gates parse",async()=>{
  await Parser.init();const parser=new Parser();
  try{
    parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",import.meta.dir)),"out/tree-sitter-rust.wasm")));
    const repo=new URL("../../../../../",directory);
    const errors=(node:Parser.SyntaxNode):string[]=>node.type==="ERROR"?[`${node.startPosition.row+1}:${node.startPosition.column+1} ${node.text.slice(0,500)}`]:node.children.flatMap(errors);
    for(const path of owner.rustReceivers){const tree=parser.parse(readFileSync(new URL(path,repo),"utf8").replace(/\bunsafe(\s+extern\s+"[^"]+")/g,"      $1"));try{if(tree.rootNode.hasError())throw new Error(`Rust receiving grammar: ${path} ${errors(tree.rootNode).join("; ")}`);expect(tree.rootNode.hasError()).toBe(false);}finally{tree.delete();}}
  }finally{parser.delete();}
  console.log("[DEBUG] Current native hash receiving imports and typed pair gate grammar accepted; Rust2024 unsafe extern modifier normalized only for older Tree-sitter grammar");
});

test("typed checkpoint selection schema and pure pair gate preserve authored identity refusals",()=>{
  const pairFixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/📇️directory/🪢️canonical-checkpoint-pair-v1.json",directory),"utf8"));
  const selectionSchema=JSON.parse(readFileSync(new URL("🧬️schema/🪢️canonical-checkpoint-pair-v1/🔣️.json",directory),"utf8"));
  const validate=new Ajv({strict:true}).compile(selectionSchema);
  for(const admission of pairFixture.admissions){
    const row=pairFixture.pairs.find((pair:{id:string})=>pair.id===admission.pair);
    const pair=decodeCanonicalCheckpointPairV1(new Uint8Array(Buffer.from(row.bodyHex,"hex")));
    const selection=admitCheckpointSelectionV1(admission.expected);
    expect(validate(selection)).toBe(true);
    expect(selection.checkpointId).toEqual([...Buffer.from(admission.expected.checkpointId,"hex")]);
    const decision=()=>admitCanonicalCheckpointPairV1(pair,admission.scope,selection);
    if(admission.refusal===null)expect(decision).not.toThrow();else expect(decision).toThrow(admission.refusal);
  }
  console.log("[DEBUG] Typed checkpoint selection Ajv schema and pure identity gate replayed authored refusal corpus");
});
