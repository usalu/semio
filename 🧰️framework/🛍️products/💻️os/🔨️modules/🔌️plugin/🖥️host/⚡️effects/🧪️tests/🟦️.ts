import {test,expect} from "bun:test";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
test("router original source fixture preserves UTF8 bytes and explicit unqualified factory birth",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.sources){expect([...new TextEncoder().encode(row.text)]).toEqual([...Buffer.from(row.text,"utf8")]);expect(Buffer.byteLength(row.text)).toBeLessThanOrEqual(row.capacity);}
 expect(fixture.expectedDropReleaseBytes).toBe(0);expect(fixture.wrapper.normalFactoryBirthQualified).toBe(false);
 const source=readModuleTreeSource(new URL("../🦀️.rs",import.meta.url)),native=readFileSync(new URL("./🔬️unit/🦀️.rs",import.meta.url),"utf8");
 expect(source).toContain("cx.retirement_should_yield()");expect(source).toContain("cx.with_retained_loan(child");expect(source).toContain("maximum_depth:grant.maximum_depth-1");expect(native).toContain("turn_with_decoder");expect(native).toContain("with_original_retirement");expect(native).toContain("original_retirement_loan_closes_router_after_semantic_deadline");expect(native).not.toContain("InteractiveJobCloseStep::Refused");
});
