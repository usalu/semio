import {expect,test} from "bun:test";
import {minimatch} from "minimatch";
import {flatFixtureRole,nestedFixtureRole} from "@semio-tech/framework/schema/oracles/document";

const flat=["k/✅apply/🔣️.before.json","k/✅apply/🔣️.after.json","k/✅apply/🔣️.diff.json","k/✅apply/🔣️.mutation.json","k/✅apply/🔣️.outcome.json","k/🔣️.json","🔣️.before.json","k/✅apply/📸️snapshot/⬅️before/🔣️.json"];
const nested=["k/✅apply/📸️snapshot/⬅️before/🔣️.json","k/✅apply/📸️snapshot/➡️after/🔣️.json","k/✅apply/🔺️diff/🔣️.json","k/✅apply/🦠️mutation/🔣️.json","k/✅apply/🎯️outcome/🔣️.json","k/✅apply/🔣️.before.json"];
const snapshot=(layout:"flat"|"nested")=>layout==="flat"?"**/🔣️.{before,after}.json":"**/📸️snapshot/{⬅️before,➡️after}/🔣️.json";
const diff=(layout:"flat"|"nested")=>layout==="flat"?"**/🔣️.diff.json":"**/🔺️diff/🔣️.json";

test("flat fixture roles agree with independent minimatch",()=>{
 for(const path of [...flat,...nested]){const expected=minimatch(path,snapshot("flat"))?"snapshot":minimatch(path,diff("flat"))?"diff":undefined;expect(flatFixtureRole(path),path).toBe(expected);}
});

test("nested fixture roles agree with independent minimatch",()=>{
 for(const path of [...flat,...nested]){const expected=minimatch(path,snapshot("nested"))?"snapshot":minimatch(path,diff("nested"))?"diff":undefined;expect(nestedFixtureRole(path),path).toBe(expected);}
});

test("the flat bundle path is 22 bytes shorter than the nested one per role file",()=>{
 const bytes=(text:string)=>Buffer.byteLength(text,"utf8");
 expect(bytes("/📸️snapshot/⬅️before/🔣️.json")-bytes("/🔣️.before.json")).toBe(22);
 expect(bytes("/📸️snapshot/➡️after/🔣️.json")-bytes("/🔣️.after.json")).toBe(22);
});
