import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {join} from "node:path";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "../../../🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import * as leases from "../../🟦️.ts";
declareBuildTestEnvelopeV1({owner:"lease-terminal-release",maximumElapsedMilliseconds:60000,capabilities:{artifacts:true}});
test("terminal release is idempotent after the original body already released the accepted lease",()=>withBuildTestInvocationV1(async original=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR!;mkdirSync(base,{recursive:true});const clean=mkdtempSync(join(base,"terminal-body-released-"));
 const control=createIssuedResourceControlV1(original,"native:terminal-body-released",0),resource="native-fixed-image:"+clean,lease=await leases.acquireResourceLease({directory:clean,resource,mode:"shared",signal:control.signal,control}),db=new Database(leases.resourceLeasePathV1(clean,resource)),release=lease.release;let calls=0;lease.release=()=>{calls++;release.call(lease);};const token=leases.acceptResourceLeaseTerminalV1(lease,clean,resource,control);
 try{lease.release();expect(calls).toBe(1);expect(()=>leases.releaseResourceLeaseTerminalV1(token)).not.toThrow();expect(()=>leases.releaseResourceLeaseTerminalV1(token)).not.toThrow();expect(calls).toBe(1);db.exec("pragma busy_timeout=0");db.exec("begin exclusive");db.exec("rollback");}finally{db.close();rmSync(clean,{recursive:true,force:true});}
}));
