import {boundedServicePayloadV1,admitDocumentServiceDeclarationV1,InstalledServiceRegistryV1,parseInstalledServiceOperationV1,parseInstalledServiceStatusV1} from "../../🟦️.ts";
import {admitDocumentServiceWireDeclarationV1,encodeDocumentServiceWireDeclarationV1,documentServiceRequestV1,admitDocumentServiceResponseV1,decodeDocumentServiceSchemaV1} from "../../🚪️io/🟦️.ts";
import ts from "typescript";
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🧱️io-boundary.json";
import {verifyInstalledServiceLawsV1} from "../🟦️.ts";
import {build} from "esbuild";
import {resolve,join} from "node:path";
import ownership from "../🧫️fixtures/🧱️source-ownership.json";

test("intrinsic service laws bundle with every specific owner unavailable",async()=>{
  const workspace=resolve(import.meta.dir,"../../../../../../");
  const refused=ownership.removedOwners.map(owner=>resolve(workspace,owner).replaceAll("\\","/")+"/");
  const result=await build({entryPoints:[join(workspace,ownership.owner,ownership.entry)],bundle:true,platform:"node",format:"esm",external:["bun:test"],write:false,metafile:true,logLevel:"silent",plugins:[{name:"removed-service-consumer",setup(builder){builder.onLoad({filter:/.*/},input=>refused.some(prefix=>input.path.replaceAll("\\","/").startsWith(prefix))?{errors:[{text:"Intrinsic service law requires removed owner: "+input.path}]}:undefined);}}]});
  const specificSources=Object.keys(result.metafile!.inputs).filter(path=>ownership.removedOwners.some(owner=>path.replaceAll("\\","/").startsWith(owner+"/")));
  expect(result.outputFiles).toHaveLength(1);expect(specificSources).toHaveLength(ownership.expectedSpecificSources);
  console.log("[DEBUG] Intrinsic installed service removal proof",{sourceCount:Object.keys(result.metafile!.inputs).length,specificSources:specificSources.length});
},15_000);

test("existing neutral service operations, lifecycle and independent Ajv laws replay",()=>{
  const counts=verifyInstalledServiceLawsV1();expect(counts.envelopeVectors).toBe(4);
  console.log("[DEBUG] Neutral installed service lifecycle/envelopes and independent Ajv replay",counts);
});

test("service semantic owner retains no physical JSON schema or request body",()=>{
  expect(new Ajv({strict:true}).validate({type:"object",required:["schema","semanticOwner","wireOwner","limits"],properties:{schema:{const:"semio.test.service-io-boundary/v1"},semanticOwner:{const:"inference/service"},wireOwner:{const:"inference/service/io"},limits:{type:"object"}}},fixture)).toBe(true);
  const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8");
  expect(source).not.toContain("JSON.parse(");expect(source).not.toContain("JSON.stringify(");
  expect(source).not.toContain("inputSchema: string");expect(source).not.toContain("function documentServiceRequestV1(");
  console.log("[DEBUG] Pure installed service value admission/declaration has explicit physical IO separation");
});

const declaration=()=>admitDocumentServiceDeclarationV1("neutral",{schema:"semio.os.document-http-port/v1",owner:"neutral",serviceId:"neutral.service",operations:[{action:"run",method:"POST",route:["run"],sendBody:true,cursorField:null,requestMaxBytes:16384,responseMaxBytes:16384,inputSchema:{$id:"neutral:request",type:"object",required:["value"],additionalProperties:false,properties:{value:{type:"string"}}},outputSchema:{$id:"neutral:reply",type:"object",required:["value"],additionalProperties:false,properties:{value:{type:"string"}}}}]});

test("semantic value ownership preserves signed zero and independently cloned values",()=>{
  const input={value:-0,nested:[{label:"東😀",enabled:true}],empty:null};
  const owned=boundedServicePayloadV1(input),reference=structuredClone(input);
  expect(owned).toEqual(reference);expect(Object.is((owned as {value:number}).value,-0)).toBe(fixture.preserveNegativeZero);
  expect(owned).not.toBe(input);expect(Object.isFrozen(owned)).toBe(fixture.frozenCopy);
  expect(Object.isFrozen((owned as {nested:readonly unknown[]}).nested)).toBe(true);
  input.nested[0]!.label="changed";expect((owned as {nested:readonly {label:string}[]}).nested[0]!.label).toBe(reference.nested[0]!.label);
  let read=0;const getter={get value(){read++;return 7;}};expect(()=>boundedServicePayloadV1(getter)).toThrow();expect(read).toBe(0);
  const cyclic:Record<string,unknown>={};cyclic.self=cyclic;expect(()=>boundedServicePayloadV1(cyclic)).toThrow();
  expect(()=>boundedServicePayloadV1([,])).toThrow();expect(()=>boundedServicePayloadV1(Object.assign([1],{foreign:1}))).toThrow();
  console.log("[DEBUG] Direct semantic ownership agrees with structuredClone while preserving signed zero/frozen copies and refusing getter/cycle payloads");
});

test("semantic node depth capacity and physical UTF8 byte budgets have distinct neutral laws",()=>{
  expect(()=>boundedServicePayloadV1(Array(fixture.limits.nodes-1).fill(null))).not.toThrow();
  expect(()=>boundedServicePayloadV1(Array(fixture.limits.nodes).fill(null))).toThrow();
  let depth:unknown=null;for(let index=0;index<fixture.limits.depth;index++)depth=[depth];expect(()=>boundedServicePayloadV1(depth)).not.toThrow();depth=[depth];expect(()=>boundedServicePayloadV1(depth)).toThrow();
  expect(()=>boundedServicePayloadV1("x".repeat(fixture.limits.capacity-1))).not.toThrow();expect(()=>boundedServicePayloadV1("x".repeat(fixture.limits.capacity))).toThrow();
  const payload={value:fixture.unicode.value.repeat(fixture.unicode.count)};
  expect(()=>boundedServicePayloadV1(payload)).not.toThrow();expect(()=>documentServiceRequestV1(declaration(),{spaceId:"space",documentId:"document"},"run",payload)).toThrow("installed-service.bounds");
  console.log("[DEBUG] Neutral semantic4096node/depth32/capacity16384 and independent UTF8 wire byte refusal are distinct");
});

test("semantic dispatch and schema compilation execute with JSON codecs disabled",()=>{
  const originalParse=JSON.parse,originalStringify=JSON.stringify;
  let calls=0;try{
    JSON.parse=()=>{throw new Error("unexpected physical decoder");};JSON.stringify=()=>{throw new Error("unexpected physical encoder");};
    const model=declaration();expect(typeof model.operations[0]!.inputSchema).toBe("object");
    const registry=new InstalledServiceRegistryV1();registry.install({owner:"neutral",serviceId:"neutral.service",dispatch(operation){calls++;expect(Object.isFrozen(operation.payload)).toBe(true);},retire(){},sessionRetired(){},documentClosed(){},documentRebootstrapped(){},documentMounted(){}});
    const operation=parseInstalledServiceOperationV1({kind:"service-operation",owner:"neutral",serviceId:"neutral.service",action:"run",operationEpoch:1,payload:{value:"test"}});
    expect(registry.dispatch(operation)).toBe(true);expect(parseInstalledServiceStatusV1({owner:"neutral",serviceId:"neutral.service",payload:{value:"test"}}).payload).toEqual({value:"test"});
  }finally{JSON.parse=originalParse;JSON.stringify=originalStringify;}
  expect(calls).toBe(1);console.log("[DEBUG] Pure installed service declaration/admission/dispatch/status executed with all JSON codecs disabled");
});

test("raw native declaration strings are admitted only through owned IO",()=>{
  const typed=declaration(),wire=encodeDocumentServiceWireDeclarationV1(typed);
  expect(typeof typed.operations[0]!.inputSchema).toBe("object");expect(typeof wire.operations[0]!.inputSchema).toBe("string");
  expect(admitDocumentServiceWireDeclarationV1("neutral",wire)).toEqual(typed);
  expect(()=>admitDocumentServiceDeclarationV1("neutral",wire)).toThrow();expect(()=>admitDocumentServiceWireDeclarationV1("neutral",typed)).toThrow();
  for(const operation of wire.operations)new Ajv({strict:true}).compile(JSON.parse(operation.inputSchema));
  const model=declaration();const body=JSON.stringify({value:"reply"});expect(admitDocumentServiceResponseV1(model,"run",body)).toEqual(JSON.parse(body));expect(()=>admitDocumentServiceResponseV1(model,"run",'{"value":7}')).toThrow();
  expect(()=>decodeDocumentServiceSchemaV1('{"format":"unknown"}')).toThrow();expect(decodeDocumentServiceSchemaV1('{\n"$id":"neutral:pretty","type":"string"\n}')).toEqual({$id:"neutral:pretty",type:"string"});
  console.log("[DEBUG] Neutral native declarations roundtrip explicit IO; semantic record-only admission and independent Ajv hold");
});

test("owned semantic value/compiler sources have explicit ownership and TS syntax",()=>{
  for(const path of ["../../🧬️schema/🧺️value/🟦️.ts","../../🌐️document-http/🧬️schema/🟦️.ts"]){const source=readFileSync(new URL(path,import.meta.url),"utf8");expect(source).not.toContain("JSON.parse(");expect(source).not.toContain("JSON.stringify(");}
  for(const path of ["../../🟦️.ts","../../🚪️io/🟦️.ts","../../🌐️document-http/🧬️schema/🟦️.ts","../../🧬️schema/🧺️value/🟦️.ts"]){const source=ts.createSourceFile(path,readFileSync(new URL(path,import.meta.url),"utf8"),ts.ScriptTarget.Latest,true);expect((source as unknown as {parseDiagnostics:readonly unknown[]}).parseDiagnostics.length).toBe(0);}
  console.log("[DEBUG] Current semantic value/schema compiler bodies contain no JSON codec; service, io, compiler and value syntax accepted");
});

