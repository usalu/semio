import Parser from "web-tree-sitter";
import { dirname, join } from "node:path";
import { buildSchema } from "graphql";
import { parse as parseProtobuf } from "protobufjs";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { isDeepStrictEqual } from "node:util";
import fixture from "../🧫️fixtures/🔣️.json";
import replay from "../🧫️fixtures/🪢️replay/🔣️.json";
import schema from "../🔣️.json";
import valueSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json";
import snapshotSchema from "../../📸️snapshot/🔣️.json";
import { parsePdfAdmittedStreamRole, resolveIdentity, validateRoleInputs, validateRoleTransition, pageContent } from "../🟦️.ts";
import type { PdfIndirectObject } from "../../📸️snapshot/🟦️.ts";

test("PDF roles admit the same logical values as independent JSON Schema", () => {
  const independent = new Ajv({ strict: false }).addSchema(valueSchema).addSchema(snapshotSchema).compile(schema);
  for (const input of Object.values(fixture)) {
    expect(independent(input)).toBe(true);
    const admitted = parsePdfAdmittedStreamRole(input);
    expect(isDeepStrictEqual(admitted, input)).toBe(true);
  }
  const invalid = structuredClone(fixture.sampledRole); invalid.value.samples[0] = 4294967296;
  expect(independent(invalid)).toBe(false);
  expect(() => parsePdfAdmittedStreamRole(invalid)).toThrow();
  const foreign = {...fixture.fontRole,value:{...fixture.fontRole.value,program:{...fixture.fontRole.value.program,data:[0,255]}}};
  expect(independent(foreign)).toBe(false);expect(()=>parsePdfAdmittedStreamRole(foreign)).toThrow();
  expect(Object.keys(fixture)).toHaveLength(12);
  console.log("[DEBUG] PDF twelve admitted semantic roles agree with Ajv and refuse native font-byte leakage");
});

test("PDF role identities distinguish generations and changed inputs require semantic payloads", () => {
  const role = parsePdfAdmittedStreamRole(fixture.role);
  const base: PdfIndirectObject[] = [
    { id: role.dependencies[1]!.owner, value: { kind: "stream", dict: [], data: [113, 32, 81], filters: [] } },
    { id: role.identity.owner, value: { kind: "dict", value: [{ key: "Contents", value: { kind: "ref", ...role.dependencies[1]!.owner } }] } }
  ];
  expect(resolveIdentity(base, { owner: { num: 7, gen: 3 }, path: [] })).toBeUndefined();
  expect(isDeepStrictEqual(pageContent([role], role.identity.owner), fixture.role.value.content)).toBe(true);
  const next = structuredClone(base); if (next[0]!.value.kind !== "stream") throw new Error("fixture stream"); next[0]!.value.data.push(113);
  expect(() => validateRoleInputs(base, next, [role], [])).toThrow();
  expect(() => validateRoleInputs(base, next, [role], [role])).not.toThrow();
  const omitted = structuredClone(role); omitted.dependencies = [omitted.identity];
  expect(() => validateRoleInputs(base, next, [role], [omitted])).toThrow("omits");
  const wrong = {...role,value:{kind:"metadataText" as const,text:"q Q"}};
  expect(() => validateRoleInputs(base, next, [role], [wrong])).toThrow("requires");
  expect(() => validateRoleInputs(base, next, [role], [role,role])).toThrow("duplicated");
  next[0]!.id.gen = 3;
  expect(() => validateRoleInputs(base, next, [role], [role])).toThrow();
  console.log("[DEBUG] PDF pure identity lookup and explicit semantic role refusal confirmed");
});


test("PDF role GraphQL and protobuf facets retain typed word and reference contracts",async()=>{
  const graphql=buildSchema(await Bun.file(new URL("../../📸️snapshot/🔗️.graphql",import.meta.url)).text());
  expect(graphql.getType("PdfAdmittedStreamRole")).toBeDefined();
  expect(graphql.getType("PdfAdmittedStreamRoleInput")).toBeDefined();
  const root=parseProtobuf(await Bun.file(new URL("../../📸️snapshot/🛰️.proto",import.meta.url)).text()).root;
  root.resolveAll();
  const word=root.lookupType("semio.s_stdio_pdf_1_7.snapshot.PdfSampledWordRole");
  const input=fixture.sampledRole.value;const encoded=word.encode(word.create({samples:input.samples})).finish();
  expect(word.toObject(word.decode(encoded)).samples).toEqual(input.samples);
  const reference=root.lookupType("semio.s_stdio_pdf_1_7.snapshot.PdfArtifactRef");
  const artifact=fixture.referenceRole.value.reference;
  expect(reference.toObject(reference.decode(reference.encode(reference.create(artifact)).finish()))).toEqual(artifact);
  console.log("[DEBUG] PDF GraphQL role input/output and independent protobuf exact u32/artifact-reference laws completed");
});

test("PDF graph mutation protobuf facets carry exact admitted role words",async()=>{
  const root=parseProtobuf(await Bun.file(new URL("../../📸️snapshot/🛰️.proto",import.meta.url)).text()).root;
  for(const path of ["../../🔺️diff/🛰️.proto","../../🧬️mutations/📦️insert-object/🛰️.proto","../../🧬️mutations/🧹️remove-object/🛰️.proto","../../🧬️mutations/🔧️set-object-value/🛰️.proto"])parseProtobuf(await Bun.file(new URL(path,import.meta.url)).text(),root);
  root.resolveAll();
  for(const name of ["insert_object.InsertObjectMutation","remove_object.RemoveObjectMutation","set_object_value.SetObjectValueMutation"]){
    const type=root.lookupType(`semio.s_stdio_pdf_1_7.mutation.${name}`);
    const input={id:{num:9,gen:1},admittedStreamRoles:{modified:[{index:0,value:{identity:{owner:{num:9,gen:1}},dependencies:[{owner:{num:9,gen:1}}],value:{sampledWords:{samples:fixture.sampledRole.value.samples}}}}]}};
    const output=type.toObject(type.decode(type.encode(type.create(input)).finish()),{longs:Number}) as typeof input;
    expect(output.admittedStreamRoles.modified[0]!.value.value.sampledWords.samples).toEqual(fixture.sampledRole.value.samples);
    expect(output.id).toEqual(input.id);
  }
  console.log("[DEBUG] PDF insert/remove/set-object protobuf payloads preserve exact admitted role words");
});

test("PDF neutral multiobject replay law has current role-aware receiving syntax",async()=>{

  expect(new Ajv({strict:true}).validate({type:"object",additionalProperties:false,required:["owners","texts","law","expectedRetainedOwner","physicalByteEditRefused","nativeOperatorNames"],properties:{owners:{type:"array",minItems:2,maxItems:2,items:{type:"object",additionalProperties:false,required:["num","gen"],properties:{num:{type:"integer",minimum:1},gen:{type:"integer",minimum:0,maximum:65535}}}},texts:{type:"array",items:{type:"string"},minItems:2,maxItems:2},law:{const:"pdf_stream_roles_multi_object_reorder_delete_and_path_refusal"},expectedRetainedOwner:{type:"object"},physicalByteEditRefused:{const:true},nativeOperatorNames:{type:"array",minItems:2,maxItems:2,items:{type:"string"}}}},replay)).toBe(true);
  const root=new URL("../../../../../../../../../../../../",import.meta.url).pathname;
  await Parser.init();const parser=new Parser();
  try{
    parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",root)),"out/tree-sitter-rust.wasm")));
    for(const path of ["./🦀️.rs","../🦀️.rs","../../🧬️mutations/🦀️.rs","../../🔺️diff/🦀️.rs","../../../🔮️oracles/🦀️.rs"]){
      const source=await Bun.file(new URL(path,import.meta.url)).text();const tree=parser.parse(source);
      try{expect(tree.rootNode.hasError()).toBe(false);if(path==="./🦀️.rs"){const law=tree.rootNode.namedChildren.find(node=>node.type==="function_item"&&node.childForFieldName("name")?.text===replay.law);expect(law).toBeDefined();expect(law!.text).toContain("special_edit");expect(law!.text).toContain("admitted_stream_roles:Some");expect(law!.text).toContain("PdfDiff::between");}}finally{tree.delete();}
    }
  }finally{parser.delete();}
  expect(replay.nativeOperatorNames).toEqual(fixture.role.value.content.map(operator=>operator.op==="save"?"q":"Q"));
  const retained=replay.owners.filter(owner=>owner.num!==replay.owners[0]!.num);expect(retained).toEqual([replay.expectedRetainedOwner]);
  console.log("[DEBUG] PDF neutral replay contract, independently parsed Rust receiving bodies and logical deletion witness verified; native execution pending");
});

test("PDF graph role deletion cannot erase native replacement obligations",()=>{
  const roles=replay.owners.map((owner,index)=>parsePdfAdmittedStreamRole({identity:{owner,path:[]},dependencies:[{owner,path:[]}],value:{kind:"metadataText",text:replay.texts[index]}}));
  const objects:PdfIndirectObject[]=replay.owners.map((id,index)=>({id,value:{kind:"stream",dict:[],filters:[],data:Array.from(new TextEncoder().encode(replay.texts[index]!))}}));
  expect(()=>validateRoleTransition(objects,[objects[1]!],roles,[roles[1]!],[])).not.toThrow();
  expect(()=>validateRoleTransition(objects,[],roles,[],[])).not.toThrow();
  expect(()=>validateRoleTransition(objects,[objects[1]!],roles,[roles[1]!,roles[0]!],[])).toThrow();
  const changed=structuredClone(objects);if(changed[0]!.value.kind!=="stream")throw Error("fixture");changed[0]!.value.data.push(0);
  expect(()=>validateRoleTransition(objects,changed,roles,[roles[1]!],[])).toThrow("replacement");
  expect(()=>validateRoleTransition(objects,changed,roles,roles,[roles[0]!])).not.toThrow();
  const consumed=parsePdfAdmittedStreamRole(fixture.role);
  const native:PdfIndirectObject={id:consumed.dependencies[1]!.owner,value:{kind:"stream",dict:[],filters:[],data:[113,32,81]}};
  const page:PdfIndirectObject={id:consumed.identity.owner,value:{kind:"dict",value:[{key:"Contents",value:{kind:"ref",...native.id}}]}};
  const removedConsumption:PdfIndirectObject={id:page.id,value:{kind:"dict",value:[]}};
  expect(()=>validateRoleTransition([native,page],[native,removedConsumption],[consumed],[],[])).not.toThrow();
  const changedNative=structuredClone(native);if(changedNative.value.kind!=="stream")throw Error("fixture");changedNative.value.data.push(0);
  expect(()=>validateRoleTransition([native,page],[changedNative,removedConsumption],[consumed],[],[])).toThrow("replacement");
  console.log("[DEBUG] PDF actual role-transition implementation permits owner+role deletion and refuses changed native bytes hidden by role removal");
});
