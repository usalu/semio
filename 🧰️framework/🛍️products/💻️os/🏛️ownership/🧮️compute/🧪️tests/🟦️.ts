/** 🏛️ Preserves the OS-owned facade retirement assertion. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {readFileSync,existsSync} from "node:fs";
import {join,resolve} from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
const root=resolve(import.meta.dir,"../../../../../..");
test("closed OS compute ownership fixture is exact",()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,extra:true})).toBe(false);});
test("OS consumes the neutral compute owner without a generic facade",()=>{const source=readFileSync(join(root,fixture.source),"utf8");expect(source).not.toMatch(/pub mod os_engine|pub use crate::os_engine/);expect(existsSync(join(root,fixture.removed))).toBe(false);});
