import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import {readFileSync} from "node:fs";
import {createHash} from "node:crypto";
test("original paged binary payload agrees with independent Buffer and SHA-256",()=>{const law=JSON.parse(readFileSync(new URL("🔣️.json",import.meta.url),"utf8"));const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));expect(new Ajv2020({strict:true}).compile(schema)(law)).toBe(true);const original=Buffer.from(Array.from({length:law.repeats},()=>law.seed).flat());expect(original.length).toBeGreaterThan(2*law.maximumPhysicalBytes);expect(createHash("sha256").update(original).digest("hex")).toBe(law.sha256);for(const body of law.bodyBytes){const output=Buffer.concat(Array.from({length:Math.ceil(original.length/body)},(_,index)=>original.subarray(index*body,(index+1)*body)));expect(output).toEqual(original);}});
