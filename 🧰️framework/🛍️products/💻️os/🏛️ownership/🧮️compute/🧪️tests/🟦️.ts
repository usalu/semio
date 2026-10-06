/** 🏛️ Preserves the OS-owned facade retirement assertion. */
import {expect,test} from "bun:test";

import {readFileSync,existsSync} from "node:fs";
import {join,resolve} from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";
const root=resolve(import.meta.dir,"../../../../../..");

test("OS consumes the neutral compute owner without a generic facade",()=>{const source=readFileSync(join(root,fixture.source),"utf8");expect(source).not.toMatch(/pub mod os_engine|pub use crate::os_engine/);expect(existsSync(join(root,fixture.removed))).toBe(false);});
