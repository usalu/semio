/** 🧪️ Artifact-owned physical transport corpus and independent oracles. */
import {test,expect,afterAll} from "bun:test";
import {testResumableQueryOracle} from "./../🪜️resumable-query/🟦️.ts";
test("Jack typed query ownership and graph mutations retain independent SQLite and JSON1 laws",()=>testResumableQueryOracle(),{timeout:60000});
afterAll(()=>console.log("[DEBUG] Owned transport corpus completed: @semio-tech/trinity-jack-rs"));
