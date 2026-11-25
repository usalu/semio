import {expect,test} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {planTwins} from "../../🟦️.ts";
import {compare} from "../../⚖️equivalence/🟦️.ts";

test("declared record homes preserve siblings and reject escaping paths",()=>{
 const fixtures=fileURLToPath(new URL("./🧫️fixtures/",import.meta.url));
 const plans=planTwins(join(fixtures,"valid"),["rust"]);
 expect(plans).toHaveLength(1);
 expect(plans[0]!.text).toContain("pub value: String,");
 expect(plans[0]!.text).toContain("pub fn sibling() -> bool { true }");
 expect(planTwins(join(fixtures,"invalid"),["rust"])[0]!.problem).toContain("invalid Rust record file");
});

test("EN 1993 record schemas project to their declared row homes",()=>{
 let root=dirname(fileURLToPath(import.meta.url));
 while(!existsSync(join(root,"✏️s"))&&dirname(root)!==root)root=dirname(root);
 const scope=join(root,"✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
 const plans=planTwins(scope,["rust"]);
 expect(plans).toHaveLength(8);
 expect(plans.every(plan=>plan.problem===undefined&&compare("rust",readFileSync(plan.path,"utf8"),plan.text)!=="drift")).toBe(true);
 expect(plans.filter(plan=>plan.path.includes("🧱️rows"))).toHaveLength(7);
});

test("EN 1999 detail diffs project to their carved home",()=>{
 let root=dirname(fileURLToPath(import.meta.url));
 while(!existsSync(join(root,"✏️s"))&&dirname(root)!==root)root=dirname(root);
 const scope=join(root,"✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff");
 const plans=planTwins(scope,["rust"]);
 expect(plans).toHaveLength(2);
 expect(plans.every(plan=>plan.problem===undefined&&compare("rust",readFileSync(plan.path,"utf8"),plan.text)!=="drift")).toBe(true);
});
