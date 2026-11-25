/** 🔖️ Contract: the identity of a live host service carries the digest of the code it runs on, so a service that has run since the night on older code is another implementation than a client on the current code and retires; the sealed door digest still wins, the same root and code always give the same identity, and the digest equals an independent Python computation over the files the policy names.
 * @see ../../../🧭️invocation/🛰️plan/🟦️.ts */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {existsSync,statSync} from "node:fs";
import {join} from "node:path";
import {hostImplementationV1,hostServiceRevisionPathsV1,hostServiceRevisionV1} from "../../../🧭️invocation/🛰️plan/🟦️.ts";
import {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const library=fileURLToPath(new URL("../../../",import.meta.url));
test("the revision equals an independent Python digest of the named files and folders and every named file and folder exists",()=>{
 const files=HOST_POLICY_V1.serviceRevisionFiles as string[];expect(files.length).toBeGreaterThan(5);
 for(const path of files){const full=join(library,path);expect(existsSync(full),path).toBe(true);expect(statSync(full).isDirectory(),path).toBe(path.endsWith("/"));}
 const script="import sys,json,hashlib,os\nbase,names=sys.argv[1],json.loads(sys.argv[2])\nfiles=[]\nfor n in names:\n    if not n.endswith('/'):\n        files.append(n);continue\n    found=[]\n    for d,ds,fs in os.walk(os.path.join(base,n)):\n        ds[:]=[x for x in ds if x not in ('\u{1F9EA}\uFE0Ftests','\u{1F4D6}\uFE0Ftexts')]\n        rel=os.path.relpath(d,os.path.join(base,n)).replace(os.sep,'/')\n        found+=[n+('' if rel=='.' else rel+'/')+f for f in fs if f.endswith(('.ts','.mjs','.json')) and not os.path.islink(os.path.join(d,f))]\n    files+=sorted(found,key=lambda p:p.encode())\nh=hashlib.sha256()\nfor p in files:\n    h.update((p+'\\0').encode())\n    try:\n        h.update(open(os.path.normpath(os.path.join(base,p)),'rb').read())\n    except OSError:\n        h.update(b'absent')\nprint(h.hexdigest())";
 const run=spawnSync(Bun.which("python3")??"python3",["-I","-c",script,library,JSON.stringify(files)],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 expect(hostServiceRevisionV1()).toBe(run.stdout.trim());
});

test("a live identity is stable, differs by root and seal, includes the code revision and yields to the sealed digest",()=>{
 const live=hostImplementationV1("/r",{});
 expect(hostImplementationV1("/r",{})).toBe(live);expect(hostImplementationV1("/s",{})).not.toBe(live);expect(hostImplementationV1("/r",{SEMIO_REPO_SEAL:"x"})).not.toBe(live);
 expect(live).not.toBe(createHash("sha256").update("live\0/r\0").digest("hex"));
 expect(live).toBe(createHash("sha256").update("live\0/r\0\0"+hostServiceRevisionV1()).digest("hex"));
 const sealed="a".repeat(64);expect(hostImplementationV1("/r",{SEMIO_SEALED_DOOR_DIGEST:sealed})).toBe(sealed);
});

test("a file only a slice worker loads is not part of the service identity: the pool starts a fresh worker with the current code, so an edit of it must not retire the service and cancel the plans it runs",()=>{
 const files=hostServiceRevisionPathsV1();
 expect(files.filter(path=>path.endsWith("slice-pool/\u{1F477}\uFE0Fworker.ts"))).toEqual([]);
 for(const path of files)expect(path.endsWith(".ts")||path.endsWith(".mjs")||path.endsWith(".json")).toBe(true);
});

test("the capacity rules a waiting service decides with are part of its identity: the decision and the gate of the process host capacity folder, without tests or texts",()=>{
 const files=hostServiceRevisionPathsV1(),capacity="../../../../../\u{1F528}\uFE0Fmodules/\u{1F3C3}\uFE0Fprocess/\u{1F3D7}\uFE0Fbuild/\u{1F321}\uFE0Fhost-capacity/";
 for(const member of ["\u{1F4CF}\uFE0Fdecision/\u{1F7E6}\uFE0F.ts","\u{1F3AB}\uFE0Fgate/\u{1F7E6}\uFE0F.ts","\u{1F4E1}\uFE0Fsampling/\u{1F7E6}\uFE0F.ts"])expect(files).toContain(capacity+member);
 expect(files.filter(path=>path.includes("/\u{1F9EA}\uFE0Ftests/")||path.includes("/\u{1F4D6}\uFE0Ftexts/"))).toEqual([]);
 expect(new Set(files).size).toBe(files.length);
});
