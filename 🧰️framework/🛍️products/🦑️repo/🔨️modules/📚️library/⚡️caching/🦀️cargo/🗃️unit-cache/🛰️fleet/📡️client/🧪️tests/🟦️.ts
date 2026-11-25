/** 🧬️ Tests for the instance (namespace) that the unit cache writes to. */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fleetCacheLocalPathV1} from "../../../../../../🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";
import {readFleetSettingsV1} from "../🟦️.ts";

test("a leftover instance of the environment never wins over the instance the fleet facts name, the operator's override does, and no facts leave the environment and the default as they were",()=>{
 const root=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"unit-instance-")));
 try{
  const base={SEMIO_REMOTE_CACHE_ENDPOINT:"http://10.0.0.5:6094",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_UNIT_CACHE_WORKSPACE_ROOT:root};
  expect(readFleetSettingsV1(root,{...base,SEMIO_REMOTE_CACHE_INSTANCE:"semio"})!.instance,"no facts: the environment").toBe("semio");
  expect(readFleetSettingsV1(root,base)!.instance,"no facts, no instance: the policy default").toBe("main");
  const path=fleetCacheLocalPathV1(root);mkdirSync(join(path,".."),{recursive:true});
  writeFileSync(path,JSON.stringify({version:1,fetchedAtMs:Date.now(),checkedAtMs:Date.now(),selectedEndpoint:"http://10.0.0.5:6094",facts:{version:1,group:"g1",generation:1,publishedAtMs:1,publisher:{directory:"/var/lib/semio-remote-cache",port:6094,host:"h1"},cache:{endpoints:["http://10.0.0.5:6094"],instance:"main",access:"read-write",protocols:["bazel-http","nx-self-hosted"],protocolVersion:1},auth:{scheme:"semio-fleet-v1",roles:["cache-read","cache-write","submit","worker","admin"]}}}));
  expect(readFleetSettingsV1(root,{...base,SEMIO_REMOTE_CACHE_INSTANCE:"semio"})!.instance,"a leftover profile value loses").toBe("main");
  expect(readFleetSettingsV1(root,base)!.instance,"facts without an instance of the environment").toBe("main");
  expect(readFleetSettingsV1(root,{...base,SEMIO_REMOTE_CACHE_INSTANCE:"semio",SEMIO_REMOTE_CACHE_ENDPOINT_OVERRIDE:"1"})!.instance,"the operator's override").toBe("semio");
 }finally{rmSync(root,{recursive:true,force:true});}
});
