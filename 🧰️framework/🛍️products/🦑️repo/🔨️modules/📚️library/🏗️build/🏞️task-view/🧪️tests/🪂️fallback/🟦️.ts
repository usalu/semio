import {afterAll,expect,test} from "bun:test";
import {readFileSync,renameSync,writeFileSync,lstatSync} from "node:fs";
import {join} from "node:path";
import {TASK_VIEW_POLICY_V1,TASK_VIEW_PORTS_V1,digestTaskViewInputsV1,openTaskViewV1,type TaskViewPortV1,type TaskViewPolicyV1} from "../../🟦️.ts";
import {growTreeV1,removeV1,scratchV1} from "../🧰️world/🟦️.ts";

const scratch:string[]=[];
const world=(tag:string)=>{const root=scratchV1(tag);scratch.push(root);return root;};
afterAll(()=>{for(const root of scratch)removeV1(root);});
const noClone:TaskViewPortV1={name:"reflink",file:async()=>{throw Object.assign(Error("clone not supported"),{code:"ENOTSUP"});}};
const bounded=(materialise:Partial<TaskViewPolicyV1["materialise"]>):TaskViewPolicyV1=>({...TASK_VIEW_POLICY_V1,materialise:{...TASK_VIEW_POLICY_V1.materialise,...materialise}} as TaskViewPolicyV1);
const ready=(outcome:{kind:string}) =>{expect(outcome.kind,JSON.stringify(outcome)).toBe("ready");return (outcome as {kind:"ready";view:Awaited<ReturnType<typeof openTaskViewV1>> extends infer T?T extends {view:infer V}?V:never:never}).view;};

test("fallback law: without clone support the closure is copied within the size bound and the copy is exactly the start state",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),paths=growTreeV1(live,50),inputs=await digestTaskViewInputsV1(live,paths);
 const view=ready(await openTaskViewV1({source:live,directory,hash:"fb000001",inputs,ports:{reflink:noClone}}));
 expect(view.receipt.strategy).toBe("copy");
 writeFileSync(join(live,paths[4]!),"in place edit");
 expect(readFileSync(join(view.root,paths[4]!),"utf8"),"a copy is independent of the live inode").not.toBe("in place edit");
 expect((await view.verify()).ok).toBe(true);
 expect(lstatSync(join(view.root,paths[4]!)).ino).not.toBe(lstatSync(join(live,paths[4]!)).ino);
});

test("fallback law: beyond the copy bound a hard link farm is used and an in-place edit is detected, a rename-replace is not even visible",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),paths=growTreeV1(live,50),inputs=await digestTaskViewInputsV1(live,paths);
 const policy=bounded({maximumCopyBytes:1});
 const view=ready(await openTaskViewV1({source:live,directory,hash:"fb000002",inputs,ports:{reflink:noClone},policy}));
 expect(view.receipt.strategy).toBe("hardlink");
 const before=readFileSync(join(view.root,paths[1]!),"utf8");
 writeFileSync(join(live,paths[9]!),"edited in place by a local editor");
 const verdict=await view.verify();
 expect(verdict.ok,"the fallback of R188: detect, then rerun").toBe(false);
 if(!verdict.ok)expect(verdict.paths).toEqual([paths[9]!]);
 writeFileSync(join(live,paths[1]!+".new"),"replica applied");renameSync(join(live,paths[1]!+".new"),join(live,paths[1]!));
 expect(readFileSync(join(view.root,paths[1]!),"utf8"),"the replica replaces by rename: the pin keeps the start bytes").toBe(before);
});

test("fallback law: nothing fits (too many files for a link farm, too many bytes for a copy) is a typed refusal in both languages, not a half view",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),paths=growTreeV1(live,30),inputs=await digestTaskViewInputsV1(live,paths);
 const outcome=await openTaskViewV1({source:live,directory,hash:"fb000003",inputs,ports:{reflink:noClone},policy:bounded({maximumCopyBytes:1,maximumHardlinkFiles:5})});
 expect(outcome.kind).toBe("refused");
 if(outcome.kind==="refused"){expect(outcome.code).toBe("too-large");expect(outcome.en).toContain("live tree");expect(outcome.de).toContain("lebenden Baum");}
 const only=await openTaskViewV1({source:live,directory,hash:"fb000004",inputs,ports:{reflink:noClone},strategies:["reflink"]});
 expect(only.kind).toBe("refused");
});

test("fallback law: a real clone is used when the file system has one, and an error that is no capability error is not swallowed",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),paths=growTreeV1(live,5),inputs=await digestTaskViewInputsV1(live,paths);
 const outcome=await openTaskViewV1({source:live,directory,hash:"fb000005",inputs});
 expect(outcome.kind).toBe("ready");
 if(outcome.kind==="ready")expect(["reflink","copy"]).toContain(outcome.view.receipt.strategy);
 const broken:TaskViewPortV1={name:"reflink",file:async()=>{throw Object.assign(Error("disk on fire"),{code:"EIO"});}};
 await expect(openTaskViewV1({source:live,directory,hash:"fb000006",inputs,ports:{reflink:broken}})).rejects.toThrow("disk on fire");
 expect(TASK_VIEW_PORTS_V1.hardlink.name).toBe("hardlink");
});
