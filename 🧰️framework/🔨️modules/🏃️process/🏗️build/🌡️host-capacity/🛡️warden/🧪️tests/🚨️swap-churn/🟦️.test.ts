import {expect,test} from "bun:test";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {decideHostWardenV1,HOST_WARDEN_POLICY_V1,type HostWardenHostV1,type HostWardenSubjectV1} from "../../🟦️.ts";
import fixture from "./🔣️.json";

test("sustained compression and swap churn reclaim paused scoped work",()=>{
 const host=fixture.host as HostWardenHostV1,subjects=fixture.subjects as HostWardenSubjectV1[],previous={level:"danger" as const,calmTicks:0};
 const decide=(sample:HostWardenHostV1,items=subjects)=>decideHostWardenV1(HOST_WARDEN_POLICY_V1,sample,items,previous);
 const oracle=Bun.spawnSync([testPythonV1()??Bun.which("python3")??"python3","-I",fileURLToPath(new URL("../🐍️.py",import.meta.url)),"--acute"]);
 expect(oracle.exitCode).toBe(0);
 expect(decide(host)).toEqual(JSON.parse(oracle.stdout.toString()));
 expect(decide(host).actions).toContainEqual({action:"cancel",id:"paused",reason:"danger"});
 expect(decide(host).actions.some(action=>action.id==="protected")).toBe(false);
 expect(decide({...host,swapOutMegabytesPerSecond:0}).actions.some(action=>action.action==="cancel")).toBe(false);
 expect(decide({...host,compressedMegabytes:5000}).actions.some(action=>action.action==="cancel")).toBe(false);
 expect(decide(host,[{...subjects[0]!,suspendedSeconds:0},subjects[1]!]).actions.some(action=>action.action==="cancel")).toBe(false);
 expect(decide(host,[{...subjects[0]!,suspended:false,suspendedSeconds:0}]).actions).toContainEqual({action:"suspend",id:"paused",reason:"danger"});
});

test("continuing OS warnings with compression and swap-out release an overdue build", () => {
 const host = fixture.warningHost as HostWardenHostV1, subjects = fixture.subjects as HostWardenSubjectV1[];
 const decide = (sample: HostWardenHostV1, level: "calm" | "guard") => decideHostWardenV1(HOST_WARDEN_POLICY_V1, sample, subjects, {level, calmTicks:0});
 const first = decide(host, "calm");
 expect(first.level).toBe("guard");
 expect(first.actions.some(action => action.action === "cancel")).toBe(false);
 const continuing = decide(host, "guard");
 const oracle = Bun.spawnSync([testPythonV1() ?? Bun.which("python3") ?? "python3", "-I", fileURLToPath(new URL("../🐍️.py", import.meta.url)), "--warning-churn"]);
 expect(oracle.exitCode).toBe(0);
 expect(continuing).toEqual(JSON.parse(oracle.stdout.toString()));
 expect(continuing.level).toBe("danger");
 expect(continuing.actions).toContainEqual({action:"cancel",id:"paused",reason:"danger"});
 expect(continuing.actions.some(action => action.id === "protected")).toBe(false);
 expect(decide({...host,swapOutMegabytesPerSecond:0}, "guard").actions.some(action => action.action === "cancel")).toBe(false);
});
