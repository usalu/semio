/** 🧾️ The actual Store returned snapshot reader mounts the neutral shared factory retirement owner. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
test("Store returned snapshot reads use the neutral shared factory retirement owner",()=>{
 const store=readFileSync(new URL("../../../../🦀️.rs",import.meta.url),"utf8");
 expect(store.includes('use semio_framework_value::retirement::shared::FactorySharedRetirement as ReturnedSnapshotReadRetirement;')).toBe(true);
 console.log("[DEBUG] actual Store returned snapshot read shared factory retirement mount confirmed");
});
