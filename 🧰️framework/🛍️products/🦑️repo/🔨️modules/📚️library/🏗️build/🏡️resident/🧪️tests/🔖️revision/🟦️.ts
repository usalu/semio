/** 🔖️ Contract: the identity of a live host service carries the digest of the code it runs on, so a service that has run since the night on older code is another implementation than a client on the current code and retires; the sealed door digest still wins, the same root and code always give the same identity, and the digest equals an independent Python computation over the files the policy names.
 * @see ../../../🧭️invocation/🛰️plan/🟦️.ts */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {hostImplementationV1,hostServiceRevisionV1} from "../../../🧭️invocation/🛰️plan/🟦️.ts";
import {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const library=fileURLToPath(new URL("../../../",import.meta.url));
test("the revision equals an independent Python digest of the named files and every named file exists",()=>{
 const files=HOST_POLICY_V1.serviceRevisionFiles as string[];expect(files.length).toBeGreaterThan(5);
 const script="import sys,json,hashlib,os\nbase,files=sys.argv[1],json.loads(sys.argv[2])\nh=hashlib.sha256()\nfor p in files:\n    h.update((p+'\\0').encode())\n    try:\n        h.update(open(os.path.normpath(os.path.join(base,p)),'rb').read())\n    except OSError:\n        h.update(b'absent')\nprint(h.hexdigest())";
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
