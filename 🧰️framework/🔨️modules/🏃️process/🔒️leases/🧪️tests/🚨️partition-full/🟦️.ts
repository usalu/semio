/** 🚨️ Contract: the refusal of a full directory partition (a Windows partition is a metered directory, its fullness is no ENOSPC of the volume) is recognised as a full lease store, so the lease store retires its idle databases, queues and claims in place and retries instead of failing the whole stage with "Directory partition capacity exceeded"; an ordinary failure is not mistaken for it.
 * @see ../../🟦️.ts `leaseStoreFullV1` */
import{expect,test}from"bun:test";
import{admitNativeDirectoryCapacityV1,nativeDirectoryCapacityRefusalV1}from"../../../📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts";
import{leaseStoreFullV1}from"../../🟦️.ts";

test("the capacity refusal of the metered directory is a full store, with the message unchanged",()=>{
 const refusal=nativeDirectoryCapacityRefusalV1();
 expect(refusal.message).toBe("Directory partition capacity exceeded");
 expect(leaseStoreFullV1(refusal)).toBe(true);
 let thrown:unknown;
 try{admitNativeDirectoryCapacityV1({filesystemBytes:66060288,maximumPhysicalBytes:67108864,reservedBytes:0,usedBytes:67108865,requestBytes:0});}catch(error){thrown=error;}
 expect((thrown as Error).message).toBe("Directory partition capacity exceeded");
 expect(leaseStoreFullV1(thrown)).toBe(true);
 expect(leaseStoreFullV1(Error("Directory partition capacity exceeded"))).toBe(false);
 expect(leaseStoreFullV1(Object.assign(Error("x"),{code:"EPERM"}))).toBe(false);
 const oracle=Bun.spawnSync(["python3","-I","-c","print(67108865>67108864)"]);
 expect(oracle.stdout.toString().trim()).toBe("True");
});
