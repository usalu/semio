import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {acquireResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {artifactScratchResourceV1} from "../../🟦️.ts";

const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const lease=()=>{const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(parent,{recursive:true});const root=mkdtempSync(join(realpathSync(parent),"sc-"));roots.push(root);return root;};

/** 🧪️ Two activations that build the same artifact each get a private scratch directory, so their claims must not collide (the failure was a hold-and-wait refusal after 15 s on one shared claim). */
test("concurrent scratch directories of one artifact hold their own claims at the same time, the same directory does not",async()=>{
 const directory=lease(),layout="/store/m/artifacts/os-plugin-describe-rs/prototype",first=artifactScratchResourceV1(layout+"/tmp/aaaaaaaaaaaaaaaa"),second=artifactScratchResourceV1(layout+"/tmp/bbbbbbbbbbbbbbbb");
 expect(first).toMatch(/^artifact-scratch-private:[a-f0-9]{64}$/u);expect(second).not.toBe(first);expect(artifactScratchResourceV1(layout+"/tmp/aaaaaaaaaaaaaaaa")).toBe(first);
 const one=await acquireResourceLease({directory,resource:first,mode:"exclusive",timeoutMs:0,signal:new AbortController().signal}),two=await acquireResourceLease({directory,resource:second,mode:"exclusive",timeoutMs:0,signal:new AbortController().signal});
 await expect(acquireResourceLease({directory,resource:first,mode:"exclusive",timeoutMs:0,signal:new AbortController().signal})).rejects.toThrow("timed out");
 one.release();two.release();
});
