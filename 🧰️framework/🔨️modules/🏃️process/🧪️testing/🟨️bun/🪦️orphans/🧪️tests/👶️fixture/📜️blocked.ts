import{test}from"bun:test";
import{spawnSync}from"node:child_process";
test("blocked in a synchronous child",()=>{spawnSync("sleep",["40"]);},60000);
