/** 🧭️ Contract: the policy owner of a repository script is the script itself unless it names an owner module by a relative specifier; the resolution agrees with an independent `path.resolve` model. */
import {expect,test} from "bun:test";
import {dirname,resolve} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";
import {policyOwnerUrl} from "../🟦️.ts";

const script=pathToFileURL(resolve("/workspace/owner/📦️packages/🟦️typescript/📜️script.ts")).href;
const vectors:readonly (readonly [string|undefined,string])[]=[
 [undefined,script],
 ["../../🏗️builder/🛤️task-routes/🟦️.ts",pathToFileURL(resolve(dirname(fileURLToPath(script)),"../../🏗️builder/🛤️task-routes/🟦️.ts")).href],
 ["./🟦️.ts",pathToFileURL(resolve(dirname(fileURLToPath(script)),"./🟦️.ts")).href],
];
for(const [specifier,expected] of vectors)test("policy owner of "+String(specifier),()=>{expect(policyOwnerUrl(script,specifier)).toBe(expected);});
