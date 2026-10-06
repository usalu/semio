import {test} from "bun:test";
import {existsSync} from "node:fs";
import {dirname,join} from "node:path";
import {testNxCoordinator} from "../🟦️.ts";
test("Nx coordinator owns child cancellation for every malformed process observation",async()=>{let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=dirname(root);await testNxCoordinator(root);});
