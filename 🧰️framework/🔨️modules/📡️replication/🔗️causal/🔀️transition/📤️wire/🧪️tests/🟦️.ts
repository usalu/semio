import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import {readFileSync} from "node:fs";
import {createHash} from "node:crypto";
const law=JSON.parse(readFileSync(new URL("🔣️.json",import.meta.url),"utf8"));
function varint(value:number):Buffer{const out:number[]=[];while(value>=128){out.push((value&127)|128);value=Math.floor(value/128);}out.push(value);return Buffer.from(out);}
function text(value:string):Buffer{const bytes=Buffer.from(value,"utf8");return Buffer.concat([varint(bytes.length),bytes]);}
test("original sorted supersession wire matches independent Buffer framing and SHA-256",()=>{expect(new Ajv2020({strict:true}).compile(JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8")))(law)).toBe(true);for(const row of law.cases){const inputs=[...row.inputs].sort((left:any,right:any)=>Buffer.compare(Buffer.from(left.target),Buffer.from(right.target)));const parts=[Buffer.from([6,row.scope===null?0:1]),...(row.scope===null?[]:[text(row.scope)]),varint(inputs.length)];for(const input of inputs){parts.push(text(input.target),Buffer.from([input.withdrawn?1:0]));if(!input.withdrawn){const payload=Buffer.from(Array.from({length:input.payloadLength},(_,index)=>(input.payloadSeed+index*31)&255));parts.push(text(input.schema),varint(payload.length),payload);}}const output=Buffer.concat(parts);expect(output.length).toBe(row.wireBytes);expect(createHash("sha256").update(output).digest("hex")).toBe(row.sha256);}});
