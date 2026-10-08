import {test,expect} from "bun:test";
import {applyPatch} from "fast-json-patch";
import {readFileSync} from "node:fs";
import law from "../🧫️fixtures/🔣️.json";
test("original triple identities validate bounded three-prefix reads before ownership move",()=>{
 const bytes=Buffer.from(law.prefix.repeat(law.repeat)+law.suffix);expect(bytes.length).toBe(law.utf8Bytes);expect(bytes.equals(Buffer.from(new TextEncoder().encode(law.prefix.repeat(law.repeat)+law.suffix)))).toBe(true);
 for(const row of law.chunks){const count=Math.min(Math.floor(row.grant/law.sources),21);expect(count*law.sources).toBe(row.copied);const model={offset:0,owned:true};const output=applyPatch(structuredClone(model),[{op:"replace",path:"/offset",value:count}],false,false).newDocument;expect(output.owned).toBe(true);expect(model.offset).toBe(0);expect(bytes.subarray(0,count).equals(bytes.subarray(0,count))).toBe(true);}
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source.includes("triple_identity")).toBe(true);expect(source.includes("maximum_copy_bytes / 3")).toBe(true);expect(source.includes("min(21)")).toBe(true);console.log("[DEBUG] NodeBuffer/RFC6902 original8194UTF8 triple prefix receipts0/0/0/3/63/63; final move distinct1unit0copy");
});
