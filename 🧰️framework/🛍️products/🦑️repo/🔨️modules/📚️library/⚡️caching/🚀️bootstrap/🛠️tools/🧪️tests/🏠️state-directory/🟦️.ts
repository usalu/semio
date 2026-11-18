import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {classifyManagedNxActivationLinkV1,readManagedNxActivationTargetV1} from "../../🔗️activation/🟦️.ts";
import activation from "../../🔗️activation/🧬️schema/🔣️.json" with {type:"json"};

const withState=<T>(state:string|undefined,run:()=>T):T=>{
 const previous=process.env.SEMIO_MACHINE_STATE_DIRECTORY;
 if(state===undefined)delete process.env.SEMIO_MACHINE_STATE_DIRECTORY;else process.env.SEMIO_MACHINE_STATE_DIRECTORY=state;
 try{return run();}finally{if(previous===undefined)delete process.env.SEMIO_MACHINE_STATE_DIRECTORY;else process.env.SEMIO_MACHINE_STATE_DIRECTORY=previous;}
};
const digest="c".repeat(64),root="/work/checkout",state="/work/state",generations=join(state,"b","t","m","nx"),inCheckout=join(root,".cache","b","t","m","nx");

/** 🏠️ The generations of the Nx tooling live below the declared machine state directory, and the activation reads them there: a generation under the state is owned, a generation under the checkout's own `.cache` is a relocated one, a path elsewhere is refused. */
test("the managed Nx generation store follows the declared machine state directory",()=>{
 expect(activation.$defs.Protocol.const.store.join("/")).toBe(".cache/b/t/m/nx");
 expect(withState(state,()=>classifyManagedNxActivationLinkV1(root,join(generations,digest),"posix"))).toBe("owned");
 expect(withState(state,()=>readManagedNxActivationTargetV1(root,join(generations,digest),"posix"))).toBe(join(generations,digest));
 expect(withState(state,()=>classifyManagedNxActivationLinkV1(root,join(inCheckout,digest),"posix"))).toBe("relocated");
 expect(()=>withState(state,()=>readManagedNxActivationTargetV1(root,join(inCheckout,digest),"posix"))).toThrow("Exact canonical managed Nx generation target");
 expect(()=>withState(state,()=>classifyManagedNxActivationLinkV1(root,join("/elsewhere","nx",digest),"posix"))).toThrow("Exact canonical managed Nx generation target");
});

test("without a declared state the store is the checkout's, as before, and the windows spelling ignores the declaration",()=>{
 expect(withState(undefined,()=>classifyManagedNxActivationLinkV1(root,join(inCheckout,digest),"posix"))).toBe("owned");
 expect(()=>withState(undefined,()=>classifyManagedNxActivationLinkV1(root,join(generations,digest),"posix"))).toThrow("Exact canonical managed Nx generation target");
 const windows=`C:\\work\\checkout\\.cache\\b\\t\\m\\nx\\${digest}`;
 expect(withState("/work/state",()=>classifyManagedNxActivationLinkV1("C:\\work\\checkout",windows,"windows"))).toBe("owned");
});

/** 🧭️ The two joins of the tooling module that name the generation store go through the seam; a bare `join(root,STORE)` would read the checkout while the build writes below the state (wall 95: `Native storage path escapes enforcing boundary`). */
test("the tooling module names its generation store only through the machine state seam",()=>{
 const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8");
 expect(source).toContain('const STORE = ".cache/b/t/m/nx";');
 const uses=[...source.matchAll(/\bSTORE\b/gu)].length;
 const seamed=[...source.matchAll(/machinePathV1\(root,STORE\)/gu)].length;
 expect(uses-1).toBe(seamed);
 expect(source).not.toMatch(/join\(root,\s*STORE\)/u);
});
