import {expect,test} from "bun:test";
import {existsSync} from "node:fs";

test("scalar source examples carry no whole-trial schema authority",()=>{
 expect(existsSync(new URL("../../🧬️schema/🔣️.json",import.meta.url))).toBe(false);
 console.log("[DEBUG] Scalar declaration examples have no whole-trial schema export");
});
