import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import {readFileSync} from "node:fs";
test("borrowed paged byte fragments agree with Buffer",()=>{const law=JSON.parse(readFileSync(new URL("🔣️.json",import.meta.url),"utf8"));const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));expect(new Ajv2020({strict:true}).compile(schema)(law)).toBe(true);const original=Buffer.from(Array.from({length:law.repeats},()=>law.seed).flat());for(const row of law.cases)expect(original.subarray(row.offset,row.offset+row.width).toString("hex")).toBe(row.hex);});
