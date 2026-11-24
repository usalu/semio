/** 🐍️ Contract: `repo dependencies ensure` synchronizes the Python environment only while it is missing, `sync --all` always, and neither a scoped sync nor a lock refresh ever; the rows are language-agnostic data.
 * @see ../../🟦️.ts */
import {expect,test} from "bun:test";
import {pythonSyncDueV1} from "../../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

for(const row of fixture.rows)test("python sync due: "+row.name,()=>{
 expect(pythonSyncDueV1(row.mode as "sync"|"lock"|"ensure",{kind:row.selection as "all"|"scope"},row.missing)).toBe(row.due);
});
