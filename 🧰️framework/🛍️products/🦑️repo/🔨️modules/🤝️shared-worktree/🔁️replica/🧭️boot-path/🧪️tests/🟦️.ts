import {expect,test} from "bun:test";
import {existsSync} from "node:fs";
import {join,resolve} from "node:path";
import {bootModuleRelativeV1} from "../🟦️.ts";
import {bootModuleRelativeV1 as supervisorCopyV1} from "../../🛟️supervisor/🟦️.ts";

test("the boot module path is one value in the neutral module and the standalone supervisor, and names this module",()=>{
 expect(bootModuleRelativeV1).toBe(supervisorCopyV1);
 const root=resolve(import.meta.dirname,"../../../../../../../..");
 expect(existsSync(join(root,...bootModuleRelativeV1.split("/"),"🔁️replica","🧭️boot-path","🟦️.ts"))).toBe(true);
});
