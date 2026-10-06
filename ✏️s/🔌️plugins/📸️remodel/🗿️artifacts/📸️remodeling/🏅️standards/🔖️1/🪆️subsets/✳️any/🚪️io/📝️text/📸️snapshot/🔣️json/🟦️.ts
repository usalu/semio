/** 🔣️ Remodeling physical JSON scalars, defaults and text codec. */
import {RemodelingCodecError,camelOf,REMODELING_SNAPSHOT_SPEC,type RemodelingSnapshot,type RecordSpec,type ValueSpec,type Float32Buffer} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
const fail=(path:string,message:string):never=>{throw new RemodelingCodecError(path,message)};
import {binary32,binary64,parseBinary32,parseBinary64,type Binary32,type Binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
const isPlainObject = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);
const finiteNumber = (value: unknown, path: string): number => (typeof value === "number" && Number.isFinite(value) ? value : (fail(path, `expected a finite number, got ${JSON.stringify(value)}`) as never));
const wholeNumber = (value: unknown, path: string, signed: boolean): number => {
  const number = finiteNumber(value, path);
  if (!Number.isSafeInteger(number) || (!signed && number > 4294967295)) fail(path, "integer width exceeded");
  if (!Number.isInteger(number)) fail(path, `expected an integer, got ${number}`);
  if (!signed && number < 0) fail(path, `expected an unsigned integer, got ${number}`);
  return number;
};

function exactFloat(value:unknown,width:32|64,path:string):Binary32|Binary64 {
 if(typeof value==="number")return width===32?binary32(finiteNumber(value,path)):binary64(finiteNumber(value,path));
 if(!isPlainObject(value)||Object.keys(value).length!==1||!Object.hasOwn(value,"bits"))return fail(path,"expected one exact IEEE word");
 if(typeof value.bits!=="string"||!(width===32?/^[0-9a-f]{8}$/:/^[0-9a-f]{16}$/).test(value.bits))return fail(path,"expected lowercase IEEE word");return width===32?{bits:Number.parseInt(value.bits,16)}:{bits:BigInt("0x"+value.bits)}
}

function exactInteger(value:unknown,signed:boolean,path:string):bigint {
 let word:bigint;
 if(typeof value==="number"&&Number.isSafeInteger(value))word=BigInt(value);else if(typeof value==="string"&&(signed?/^(0|-[1-9][0-9]*|[1-9][0-9]*)$/:/^(0|[1-9][0-9]*)$/).test(value)&&value.length<=20)word=BigInt(value);else return fail(path,"expected canonical integer decimal");
 return word>=(signed?-9223372036854775808n:0n)&&word<=(signed?9223372036854775807n:18446744073709551615n)?word:fail(path,"integer width exceeded");
}

/** 🧩️ Decode the declared JSON scalar representation into owned values. */
export function decodeValue(value:unknown,spec:ValueSpec,path:string):unknown {
 switch(spec.k){
 case "text":return typeof value==="string"?value:fail(path,"expected TEXT");
 case "bool":return typeof value==="boolean"?value:fail(path,"expected Boolean");
 case "uint":return wholeNumber(value,path,false);
 case "int":return exactInteger(value,true,path);
 case "u64":return exactInteger(value,false,path);
 case "f64":return exactFloat(value,64,path);
 case "f32":return exactFloat(value,32,path);
 case "bytes":{if(!Array.isArray(value))return fail(path,"expected an octet array");return Uint8Array.from(value.map((v,i)=>{const n=wholeNumber(v,path+"["+i+"]",false);return n<=255?n:fail(path,"octet width exceeded")}))}
 case "floatBuffer":{if(!isPlainObject(value))return fail(path,"expected a float buffer");if(value.kind==="inline"){if(Object.keys(value).length!==2||!Array.isArray(value.values))return fail(path,"expected inline values");return{kind:"inline",values:value.values.map((v,i)=>exactFloat(v,32,path+".values["+i+"]"))}}if(value.kind==="content"){if(Object.keys(value).length!==3||typeof value.contentId!=="string")return fail(path,"expected content reference");return{kind:"content",contentId:value.contentId,chunkCount:exactInteger(value.chunkCount,false,path+".chunkCount")}}return fail(path,"unknown buffer variant")}
 case "enum":return typeof value==="string"&&spec.of.includes(value)?value:fail(path,"unknown enum");
 case "tuple":{if(!Array.isArray(value)||value.length!==spec.len)return fail(path,"tuple width mismatch");return value.map((v,i)=>exactFloat(v,spec.w,path+"["+i+"]"))}
 case "list":{if(!Array.isArray(value))return fail(path,"expected a list");return value.map((v,i)=>decodeValue(v,spec.of,path+"["+i+"]"))}
 case "map":{if(!isPlainObject(value))return fail(path,"expected a map");const out:Record<string,unknown>={};for(const key of Object.keys(value).sort())Object.defineProperty(out,key,{value:decodeValue(value[key],spec.of,path+"."+key),enumerable:true,writable:true,configurable:true});return out}
 case "rec":return decodeRecord(value,spec.of(),path);
 case "opt":return value===null||value===undefined?null:decodeValue(value,spec.of,path);
 }
}


/** 🧱 Decodes one record, rejecting unknown keys and keys that carry no serde default. */
export function decodeRecord(value: unknown, spec: RecordSpec, path: string): Record<string, unknown> {
  if (!isPlainObject(value)) fail(path, `expected a ${spec.title} object, got ${JSON.stringify(value)}`);
  const source = value as Record<string,unknown>;
  const known = new Set(spec.fields.map((field) => camelOf(field.name)));
  for (const key of Object.keys(source)) if (!known.has(key)) fail(`${path}.${key}`, `unknown key for ${spec.title} (known: ${[...known].join(", ")})`);
  const out: Record<string, unknown> = {};
  for (const field of spec.fields) {
    const key = camelOf(field.name);
    if (!(key in source)) {
      if (!spec.serdeDefault && !field.jsonOptional) fail(`${path}.${key}`, `missing required key for ${spec.title}`);
      out[key] = field.dflt();
      continue;
    }
    out[key] = decodeValue(source[key], field.spec, `${path}.${key}`);
  }
  return out;
}


/** 📸️ Decodes a parsed RFC 8259 value into a validated `RemodelingSnapshot`. */
export const decodeRemodelingSnapshot = (json: unknown): RemodelingSnapshot => decodeRecord(json, REMODELING_SNAPSHOT_SPEC, "") as unknown as RemodelingSnapshot;


/** 🔢 Shortest decimal lexeme that round-trips through the given float width — `ryu`'s rule.
 *
 *  ⚠️ A `format: float` FIELD is emitted at width 64, not 32, and that is not a mistake: since the
 *  serde-elimination sweep this type graph is written through `pack::json` over `dsl::ToValue`, and
 *  `impl ToValue for f32` widens with `*self as f64` (`🌱️value/🔁️codec/🦀️.rs:114`) before
 *  `pack::json`'s f64 shortest-round-trip writer sees it. `0.42f32` therefore reaches the wire as
 *  `0.41999998688697815`, not as `0.42` the way `serde_json`'s `serialize_f32` used to write it.
 *  The width-32 rule stays available because it is still the right answer for anything that reaches
 *  the wire as a real `f32`. */
export function floatLexeme(value: number, width: 32 | 64): string {
  const narrow = width === 32 ? Math.fround : (n: number) => n;
  const target = narrow(value);
  if (Number.isInteger(target) && Math.abs(target) < 1e16) return `${target}.0`;
  for (let digits = 1; digits <= 17; digits += 1) {
    const candidate = target.toPrecision(digits);
    if (narrow(Number(candidate)) === target) return Number(candidate).toString().replace("e+", "e");
  }
  return target.toString().replace("e+", "e");
}


const indentOf = (depth: number): string => "  ".repeat(depth);

const prettyJson=(value:unknown,depth:number):string=>JSON.stringify(value,null,2).split("\n").map((line,index)=>index===0?line:indentOf(depth)+line).join("\n");


/** 🧵 Writes one value as `serde_json`'s pretty printer would, float width taken from the spec. */
export function writeValueJson(value: unknown, spec: ValueSpec, depth: number): string {
  switch (spec.k) {
    case "text":
    case "enum":
      return JSON.stringify(value);
    case "bool":
      return value ? "true" : "false";
    case "uint":return `${value}`;
    case "int":
    case "u64":
      return JSON.stringify((value as bigint).toString());
    case "f64":
      return prettyJson({bits:parseBinary64(value).bits.toString(16).padStart(16,"0")},depth);
    case "f32":
      return prettyJson({bits:parseBinary32(value).bits.toString(16).padStart(8,"0")},depth);
    case "bytes":return prettyJson(Array.from(value as Uint8Array),depth);
    case "floatBuffer":{const v=value as Float32Buffer;return prettyJson(v.kind==="inline"?{kind:"inline",values:v.values.map(x=>({bits:parseBinary32(x).bits.toString(16).padStart(8,"0")}))}:{kind:"content",contentId:v.contentId,chunkCount:v.chunkCount.toString()},depth)}
    case "tuple":return prettyJson((value as unknown[]).map(v=>({bits:spec.w===32?parseBinary32(v).bits.toString(16).padStart(8,"0"):parseBinary64(v).bits.toString(16).padStart(16,"0")})),depth);
    case "list": {
      const items = value as unknown[];
      if (items.length === 0) return "[]";
      return `[\n${items.map((item) => `${indentOf(depth + 1)}${writeValueJson(item, spec.of, depth + 1)}`).join(",\n")}\n${indentOf(depth)}]`;
    }
    case "map": {
      const entries = Object.entries(value as Record<string, unknown>).sort(([left], [right]) => (left < right ? -1 : left > right ? 1 : 0));
      if (entries.length === 0) return "{}";
      return `{\n${entries.map(([key, entry]) => `${indentOf(depth + 1)}${JSON.stringify(key)}: ${writeValueJson(entry, spec.of, depth + 1)}`).join(",\n")}\n${indentOf(depth)}}`;
    }
    case "rec":
      return writeRecordJson(value as Record<string, unknown>, spec.of(), depth);
    case "opt":
      return value === null || value === undefined ? "null" : writeValueJson(value, spec.of, depth);
  }
}


/** 🧱 Writes one record in Rust declaration order — the order `serde` emits. */
export function writeRecordJson(value: Record<string, unknown>, spec: RecordSpec, depth: number): string {
  if (spec.fields.length === 0) return "{}";
  const body = spec.fields.map((field) => {
    const key = camelOf(field.name);
    return `${indentOf(depth + 1)}${JSON.stringify(key)}: ${writeValueJson(value[key], field.spec, depth + 1)}`;
  });
  return `{\n${body.join(",\n")}\n${indentOf(depth)}}`;
}


/** 📸️ Encodes a snapshot into a plain JSON value with camelCase keys in declaration order. */
export const encodeRemodelingSnapshot = (snapshot: RemodelingSnapshot): unknown => JSON.parse(writeRecordJson(snapshot as unknown as Record<string, unknown>, REMODELING_SNAPSHOT_SPEC, 0));


/** 📄️ Encodes a snapshot as `serde_json::to_string_pretty` would render it. */
export const remodelingSnapshotToJsonText = (snapshot: RemodelingSnapshot): string => writeRecordJson(snapshot as unknown as Record<string, unknown>, REMODELING_SNAPSHOT_SPEC, 0);


/** 📄️ Decodes RFC 8259 text into a validated `RemodelingSnapshot`. */
export function remodelingSnapshotFromJsonText(text: string): RemodelingSnapshot {
  let parsed: unknown;
  try {
    parsed = JSON.parse(text);
  } catch (error) {
    throw new RemodelingCodecError("", `not RFC 8259 text: ${(error as Error).message}`);
  }
  return decodeRemodelingSnapshot(parsed);
}
