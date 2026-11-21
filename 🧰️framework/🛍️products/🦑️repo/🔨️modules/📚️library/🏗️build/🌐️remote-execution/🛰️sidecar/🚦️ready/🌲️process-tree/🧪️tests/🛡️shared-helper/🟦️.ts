import {expect,test} from "bun:test";
import {readyKillPlanV1,readyParseUnixTableV1,READY_TREE_POLICY_V1} from "../../🟦️.ts";

/** 🛡️ A disk image the first stage attached is served by one `diskimages-helper` of the system for every image of the machine; it is a descendant of the stage that attached it, but ending it with the stage cuts the partitions of the worker (the lab saw a half-copied toolchain after the helper of the first image was ended by the runner of the next stage). */
const table=readyParseUnixTableV1(["100 1 Sat Nov  7 08:50:07 2026 bun","101 100 Sat Nov  7 08:50:17 2026 hdiutil","102 101 Sat Nov  7 08:50:18 2026 /System/Library/PrivateFrameworks/DiskImages.framework/Resources/diskimages-helper","103 100 Sat Nov  7 08:50:27 2026 go","104 102 Sat Nov  7 08:50:37 2026 helper-child"].join("\n"));
test("a stage's kill plan never contains the shared disk image helper but everything else below the stage",()=>{
 expect(READY_TREE_POLICY_V1.sharedSystemProcesses).toContain("diskimages-helper");
 const root={pid:100,startMs:table.find(row=>row.pid===100)!.startMs},plan=readyKillPlanV1({table,root,known:[],self:1});
 expect(plan).not.toContain(102);
 expect([...plan].sort((a,b)=>a-b)).toEqual([100,101,103,104]);
});
test("a process of the same name that the table does not name stays in the plan, and without the helper in the table nothing changes",()=>{
 const bare=readyParseUnixTableV1(["100 1 Sat Nov  7 08:50:07 2026 bun","101 100 Sat Nov  7 08:50:17 2026 hdiutil","103 100 Sat Nov  7 08:50:27 2026 go"].join("\n")),root={pid:100,startMs:bare[0]!.startMs};
 expect([...readyKillPlanV1({table:bare,root,known:[],self:1})].sort((a,b)=>a-b)).toEqual([100,101,103]);
});
