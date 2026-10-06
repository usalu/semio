import {expect,test} from "bun:test";

import {readFileSync} from "node:fs";
import fixture from "../🧫️fixtures/🔣️.json";
test("command transport exposes authored finite admission, cancellation and progress before external operations",()=>{
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 expect(fixture.readPageBytes).toBe(65536);
 expect(fixture.cancelCommand).toBe("cancel");
 expect(Buffer.from(fixture.payload).toString("utf8")).toBe(fixture.payload);
 expect(source).toContain("cancellation:CancelToken");
 expect(source).toContain("observer:F");
 expect(source).toContain("usize::try_from(maximum_transport_bytes)");
 expect(source).toContain("TransportContextRefusalCause::Value(error)=>PackError::Refusal(PackRefusal::ValueRefusal(error))");
 expect(source).toContain("self.cancellation.is_cancelled_now()");
 console.log("[DEBUG] neutral caller fixture and actual native transport source retain cancellation, finite admission and observer contracts");
});

test("independent fatal UTF8 decoder preserves the full page-crossing scalar and rejects an incomplete scalar",()=>{
 const recipe=fixture.textTransfer;const payload=recipe.prefix.repeat(recipe.prefixRepeat)+recipe.tail;const bytes=Buffer.from(payload);expect(bytes.length).toBe(65540);expect(new TextDecoder("utf-8",{fatal:true}).decode(bytes)).toBe(payload);expect(()=>new TextDecoder("utf-8",{fatal:true}).decode(Uint8Array.from(recipe.invalid))).toThrow();expect(Buffer.from(recipe.prefix.repeat(recipe.prefixRepeat)).length).toBe(recipe.pageBytes-1);console.log("[DEBUG] independent fatal UTF8 decoder retained complete multibyte input across the authored page boundary");
});
