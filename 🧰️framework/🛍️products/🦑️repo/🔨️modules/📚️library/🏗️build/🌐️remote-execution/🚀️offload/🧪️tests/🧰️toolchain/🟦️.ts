import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {detectWorkerRustcV1} from "../../🧰️toolchain/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

const shim=`import os, sys
if sys.argv[1:] == ["-vV"]:
    sys.stdout.buffer.write(os.environ["SEMIO_TEST_RUSTC_IDENTITY"].encode())
elif sys.argv[1:] == ["--print", "sysroot"]:
    print(os.environ["SEMIO_TEST_RUSTC_SYSROOT"])
else:
    sys.exit(64)
`;

for(const row of fixture.cases)test(row.name,()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"rc-"));
 try{
  const sysroot=join(root,"sys root"),compiler=join(sysroot,"bin",row.installed),python=testPythonV1()??"python3";
  mkdirSync(dirname(compiler),{recursive:true});writeFileSync(compiler,shim,{mode:0o755});
  for(const target of row.targets)mkdirSync(join(sysroot,"lib","rustlib",target,"lib"),{recursive:true});
  mkdirSync(join(sysroot,"lib","rustlib","components"),{recursive:true});
  const identity=["rustc 1.0.0","commit-hash: "+fixture.commit,"host: "+row.host,""].join(row.lineEnding);
  const environment={...process.env,SEMIO_TEST_RUSTC_IDENTITY:identity,SEMIO_TEST_RUSTC_SYSROOT:sysroot};
  const probe=spawnSync(python,["-I",compiler,"-vV"],{encoding:"utf8",env:environment,timeout:10000});
  expect(probe.status,probe.stderr).toBe(0);expect(probe.stdout).toBe(identity);
  const reported=spawnSync(python,["-I",compiler,"--print","sysroot"],{encoding:"utf8",env:environment,timeout:10000});
  expect(reported.status,reported.stderr).toBe(0);expect(reported.stdout.trim()).toBe(sysroot);
  const offer=detectWorkerRustcV1(root,environment,[python,"-I",compiler]);
  if(!row.offers){expect(offer).toBeUndefined();return;}
  const digest=spawnSync(python,["-I","-c","import hashlib, sys; print(hashlib.sha256(sys.stdin.buffer.read()).hexdigest()[:16])"],{input:identity,encoding:"utf8",timeout:10000});
  expect(digest.status,digest.stderr).toBe(0);
  expect(offer).toEqual({
   tools:{rustc:{path:compiler,readable:[sysroot]}},
   toolchain:{commit:fixture.commit,host:row.host,digest:digest.stdout.trim(),targets:row.targets},
   properties:[{name:"semio-offload-format",value:"argv-1"},...row.targets.map(target=>({name:"semio-rustc-target-"+target,value:"1"}))],
  });
 }finally{rmSync(root,{recursive:true,force:true});}
});
