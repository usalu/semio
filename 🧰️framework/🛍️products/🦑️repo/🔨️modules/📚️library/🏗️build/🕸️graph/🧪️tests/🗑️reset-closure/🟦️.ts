import {expect,test} from "bun:test";
import {existsSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {censusNxAuthorizerSourcesV1} from "../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import {BUILD_GRAPH_RESET_SCHEMA_PATH_V1,planBuildGraphInvalidationV1} from "../../📜️contract/🟦️.ts";
import selection from "../../../⌨️command/🧭️selection/🧬️schema/🔣️.json";

/** 🗑️ `repo reset` runs in the runtime kernel, whose source closure is captured before it runs: the schema its graph reset reads has to be part of that closure, or the command is refused as an undeclared physical Source read. The closure of the kernel lists the schema, and the path the code quotes points at the file from the file that quotes it. */
const root=(()=>{let at=dirname(fileURLToPath(import.meta.url));while(!existsSync(join(at,"go.work"))){const up=dirname(at);if(up===at)throw Error("Workspace root required");at=up;}return at;})();
const contract=fileURLToPath(new URL("../../📜️contract/🟦️.ts",import.meta.url)),commandDirectory=fileURLToPath(new URL("../../../⌨️command/",import.meta.url));

test("the quoted schema path resolves from the file that quotes it to the reset schema",()=>{
 const target=resolve(dirname(contract),BUILD_GRAPH_RESET_SCHEMA_PATH_V1);
 expect(existsSync(target)).toBe(true);expect(target.endsWith("🧬️schema/🗑️reset.json")).toBe(true);
});

test("the source closure of the kernel that runs reset carries the reset schema",async()=>{
 const entry=relative(root,join(commandDirectory,selection.properties.kernels.const.runtime)).replaceAll("\\","/"),census=await censusNxAuthorizerSourcesV1(root,entry,AbortSignal.timeout(120000),{installed:true});
 const paths=(census as unknown as {sources:readonly {path:string}[]}).sources.map(row=>typeof row==="string"?row:row.path);
 expect(paths.some(path=>path.endsWith("🏗️build/🧬️schema/🗑️reset.json"))).toBe(true);
},180000);

test("an empty inventory plans nothing and a foreign entry is refused",()=>{
 expect(planBuildGraphInvalidationV1({device:1,entries:[]} as never)).toEqual([]);
 expect(()=>planBuildGraphInvalidationV1({} as never)).toThrow();
});
