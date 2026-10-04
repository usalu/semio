import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import draft7 from "ajv/dist/refs/json-schema-draft-07.json";
import canonicalIoSchema from "../../🔣️.json";
import {IoError as ExposedIoError} from "../../🟦️.ts";
import {IoError,encodeIoErrorControlled,decodeIoErrorControlled} from "../🟦️.ts";
import {ValueError,type ValueRefusalKind} from "../../../../🌱️value/⚠️refusal/🟦️.ts";
import {NativeDecodeControl} from "../../../../🌱️value/🛬️decode/🟦️.ts";
import {TextError} from "../../../../⚠️diagnostic/🚧️text-error/🟦️.ts";
import {encodeDiagnosticControlled,decodeDiagnosticControlled} from "../../../../⚠️diagnostic/🎛️controlled/🟦️.ts";
import type {IntrinsicValue} from "../../../../🌱️value/🧬️schema/🌳️intrinsic/🟦️.ts";
import {binary64,binary64Value} from "../../../🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import causeSchema from "../🧬️schema/🔣️.json";
import copySchema from "../🧬️schema/🧵️stress/🔣️.json";
import copyCases from "../🧫️fixtures/🧵️stress/🔣️.json";
import caseSchema from "../🧬️schema/🧫️cases/🔣️.json";
import wireCaseSchema from "../🧬️schema/🔁️wire/🔣️.json";
import cases from "../🧫️fixtures/🔣️.json";
import wireCases from "../🧫️fixtures/🔁️wire/🔣️.json";
import valueSchema from "../../../../🌱️value/⚠️refusal/🧬️schema/🔣️.json";
import codecSchema from "../../../../🌱️value/⚠️refusal/🔁️codec/🧬️schema/🔣️.json";
import diagnosticSchema from "../../../../⚠️diagnostic/🧬️schema/🎛️controlled/🔣️.json";
import diagnosticCases from "../../../../⚠️diagnostic/🧫️fixtures/🎛️controlled/🔣️.json";
const ajv=new Ajv({strict:true}).addMetaSchema(draft7).addKeyword("x-semio-ui").addSchema(valueSchema).addSchema(codecSchema).addSchema(diagnosticSchema).addSchema(canonicalIoSchema);
const wire=ajv.compile(causeSchema);
const canonicalWire=ajv.compile({$ref:canonicalIoSchema.$id+"#/$defs/IoError"});
const kinds:ValueRefusalKind[]=["invalidValue","canceled","ownershipLimit","allocationFailed","workLimit","depthLimit","unsupportedOwner","invariantViolated"];
function intrinsic(value:unknown):IntrinsicValue {
 if(value===null)return{kind:"null"};
 if(typeof value==="string")return{kind:"text",value};
 if(typeof value==="boolean")return{kind:"boolean",value};
 if(typeof value==="number")return Number.isInteger(value)?{kind:value<0?"signed":"unsigned",value:BigInt(value)}:{kind:"float",value:binary64(value)};
 if(Array.isArray(value))return{kind:"array",items:value.map(intrinsic)};
 if(typeof value==="object")return{kind:"object",members:Object.entries(value).map(([name,value])=>({name,value:intrinsic(value)}))};
 throw Error("unsupported development fixture");
}
function output(value:IntrinsicValue):unknown{
 switch(value.kind){case"null":return null;case"text":case"boolean":return value.value;case"unsigned":case"signed":return Number(value.value);case"float":return binary64Value(value.value);case"array":return value.items.map(output);case"object":return Object.fromEntries(value.members.map(member=>[member.name,output(member.value)]));default:throw Error("unexpected development wire fixture");}
}
function control(maximum=1048576,accept=true):NativeDecodeControl{return new NativeDecodeControl(maximum,()=>accept);}
async function refused(action:()=>Promise<unknown>,kind:string):Promise<void>{try{await action();throw Error("expected owned refusal");}catch(error){expect(error).toBeInstanceOf(ValueError);expect(String((error as ValueError).kind)).toBe(kind);}}
test("IO owned cause retains actual ValueError identity and text source data",async()=>{
 expect(ExposedIoError).toBe(IoError);
 expect(ajv.validate(caseSchema,cases)).toBe(true);expect(ajv.validate(copySchema,copyCases)).toBe(true);
 for(const row of cases){const kind=kinds.find(kind=>kind===row.kind);expect(kind).toBeDefined();if(kind===undefined)throw Error(row.id);if(typeof row.expected!=="string")throw Error("fixture requires expected syntax");const c=control(row.operation==="textOwnership"?0:1048576,row.operation!=="textCanceled");
  if(row.operation==="textCanceled"||row.operation==="textOwnership"){if(typeof row.expectedKind!=="string")throw Error("hostile fixture requires refusal kind");await refused(()=>IoError.fromTextErrorControlled(TextError.expected(kind,row.message,row.span,row.expected),c),row.expectedKind);continue;}
  const original=new ValueError(kind,row.message);
  const error=row.operation==="text"?await IoError.fromTextErrorControlled(TextError.expected(kind,row.message,row.span,row.expected),c):IoError.fromValueError(original);
  expect(error.cause.kind).toBe(kind);expect(error.cause.message).toBe(row.message);if(row.operation==="value")expect(error.cause).toBe(original);
  else{expect(error.diagnostics[0]?.span).toEqual(row.span);expect(error.diagnostics[0]?.message).toBe(row.message);expect(error.diagnostics[0]?.expected?.tokens).toEqual([row.expected]);}
  if("expectedWire"in row){expect(wire(row.expectedWire)).toBe(true);expect(canonicalWire(row.expectedWire)).toBe(true);expect(canonicalWire({message:row.message,diagnostics:[]})).toBe(false);expect(canonicalWire({...row.expectedWire,message:row.message})).toBe(false);expect(output(await encodeIoErrorControlled(error,control()))).toEqual(row.expectedWire);}
 }
 for(const row of copyCases){const message=row.unit.repeat(row.repeat),observed:{completed:number;total:number}[]=[];const c=new NativeDecodeControl(row.maximumBytes,event=>{observed.push(event);return row.cancelAfterCompleted===0||event.total<=1||event.completed<row.cancelAfterCompleted;});const source=TextError.expected("invariantViolated",message,{line:7,column:11,length:4},"required");if(row.expectedKind==="success"){const error=await IoError.fromTextErrorControlled(source,c);expect(error.cause.message).toBe(message);expect(error.cause.kind).toBe("invariantViolated");expect(error.diagnostics[0]?.message).toBe(message);}else await refused(()=>IoError.fromTextErrorControlled(source,c),row.expectedKind);if(row.cancelAfterCompleted!==0)expect(observed.some(event=>event.total>1&&event.completed>=row.cancelAfterCompleted)).toBe(true);expect(source.message).toBe(message);console.log(`[DEBUG] IO long source ${row.id} observed ${observed.length} controlled frontiers, UTF8=${Buffer.byteLength(message)}`);}
 console.log("[DEBUG] IO projection 18 shared cases preserve kinds and the actual ValueError pointer");
});
test("IO controlled wire preserves all kinds and refuses closed members and hostile controls",async()=>{
 expect(ajv.validate(wireCaseSchema,wireCases)).toBe(true);
 for(const row of cases){if(!("expectedWire"in row))continue;const decoded=await decodeIoErrorControlled(intrinsic(row.expectedWire),control());expect(output(await encodeIoErrorControlled(decoded,control()))).toEqual(JSON.parse(JSON.stringify(row.expectedWire)));}
 for(const row of wireCases){const value:IntrinsicValue={kind:"object",members:row.members.map(member=>{if(typeof member[0]!=="string")throw Error("fixture member requires a name");return{name:member[0],value:intrinsic(member[1])};})};const c=control(row.maximumBytes,row.accept);await refused(()=>row.operation==="encode"?encodeIoErrorControlled(IoError.fromValueError(new ValueError("workLimit","canceled spoofed prose")),c):decodeIoErrorControlled(value,c),row.expectedKind);}
 console.log("[DEBUG] IO closed wire 16 positive and 7 refusal cases agree with independent JSON and Ajv");
});
test("Diagnostic portable codec replays the original closed owner cases",async()=>{
 for(const row of diagnosticCases.cases){if(row.owner!=="Diagnostic")continue;if(!row.accepted){await refused(()=>decodeDiagnosticControlled(intrinsic(row.input),control()),"invalidValue");continue;}const actual=await decodeDiagnosticControlled(intrinsic(row.input),control());expect(output(await encodeDiagnosticControlled(actual,control()))).toEqual("expected"in row?row.expected:row.input);}
 const spanCases=diagnosticCases.cases.filter(row=>row.owner==="TextSpan");
 for(const row of spanCases){const diagnostic=intrinsic({code:"test",severity:"error",span:row.input,message:"message"});if(row.accepted)expect((await decodeDiagnosticControlled(diagnostic,control())).span as unknown).toEqual("expected"in row?row.expected:row.input);else await refused(()=>decodeDiagnosticControlled(diagnostic,control()),"invalidValue");}
 console.log("[DEBUG] Diagnostic portable closed cases and exact span laws replayed");
});
