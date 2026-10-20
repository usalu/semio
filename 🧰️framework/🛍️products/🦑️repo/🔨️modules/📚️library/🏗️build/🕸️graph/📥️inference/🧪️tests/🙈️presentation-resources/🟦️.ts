/** 🙈️ Contract: a resource declared presentation-only in the build policy is not part of any command closure, every other resource the entry imports still is. The expectation list is generated from the entry's own import statements by an independent Python scan (`🐍️oracle.py`). */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {existsSync,readFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import policy from "../../../../🔣️policy.json";

const root=process.env.SEMIO_TEST_WORKSPACE_ROOT??(()=>{let path=dirname(fileURLToPath(import.meta.url));while(!existsSync(join(path,"bunfig.toml"))||!existsSync(join(path,"bun.lock")))path=dirname(path);return path;})();
const names=(list:string[])=>list.map(file=>"{workspaceRoot}/"+file);

test("the fixture is closed under its schema and the policy declares exactly the excluded resources",()=>{
 expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);
 expect(policy.presentationResources).toEqual(fixture.excluded);
 for(const file of [...fixture.excluded,...fixture.included])expect(existsSync(join(root,file))).toBe(true);
});

test("the command closure of the entry omits presentation resources and keeps the other imported resources",async()=>{
 const inference=await import("../../🟨️.mjs");await inference.libraryBootstrap;
 const scripts=await import("../../🧭️script-inputs/🟨️.mjs"),cache=inference.createScriptInputCache({workspaceRoot:root});
 const closure=await scripts.relativeScriptInputs([resolve(root,fixture.entry)],root,cache,false);
 for(const file of names(fixture.excluded))expect(closure).not.toContain(file);
 for(const file of names(fixture.included))expect(closure).toContain(file);
 expect(closure).toContain("{workspaceRoot}/"+fixture.entry);
});

test("python scan of the entry's import statements agrees with the fixture",()=>{
 const python=testPythonV1();if(!python)return;
 const result=Bun.spawnSync([python,"-I",fileURLToPath(new URL("./🐍️oracle.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url)),root]);
 expect(result.stdout.toString().trim()).toBe("ok");
});
