import {expect,test} from "bun:test";
import {agentSessionRegistryRootV1} from "../../../../🪪️agent-sessions/🗄️store/🟦️.ts";
import {hostCapacityDirectoryV1} from "../../../../🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts";
import {testPythonV1} from "../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {leaseRegistryRootV1} from "../../🗂️registry/🟦️.ts";

/** 🪟️ The per-user registry root follows each platform's cache convention: `~/Library/Caches` on macOS, `%LOCALAPPDATA%` (else `~/AppData/Local`) on Windows, `$XDG_CACHE_HOME` (else `~/.cache`) on Linux, and an explicit override wins everywhere. The vectors are authored from the Apple File System Programming Guide, the Windows known folder `LocalAppData` and the XDG Base Directory Specification; Node's path joining runs for the host, so separators are compared after normalisation. */
const normalize=(path:string)=>path.replaceAll("\\","/");
const vectors:{name:string;platform:NodeJS.Platform;home:string;environment:Record<string,string>;expected:string}[]=[
 {name:"macOS cache folder",platform:"darwin",home:"/Users/ada",environment:{},expected:"/Users/ada/Library/Caches/semio/lease-registry"},
 {name:"macOS ignores XDG_CACHE_HOME",platform:"darwin",home:"/Users/ada",environment:{XDG_CACHE_HOME:"/elsewhere"},expected:"/Users/ada/Library/Caches/semio/lease-registry"},
 {name:"Windows LocalAppData",platform:"win32",home:"C:/Users/ada",environment:{LOCALAPPDATA:"D:/Local"},expected:"D:/Local/semio/lease-registry"},
 {name:"Windows without LocalAppData",platform:"win32",home:"C:/Users/ada",environment:{},expected:"C:/Users/ada/AppData/Local/semio/lease-registry"},
 {name:"Windows ignores XDG_CACHE_HOME",platform:"win32",home:"C:/Users/ada",environment:{XDG_CACHE_HOME:"/elsewhere"},expected:"C:/Users/ada/AppData/Local/semio/lease-registry"},
 {name:"Linux XDG_CACHE_HOME",platform:"linux",home:"/home/ada",environment:{XDG_CACHE_HOME:"/var/cache/ada"},expected:"/var/cache/ada/semio/lease-registry"},
 {name:"Linux default",platform:"linux",home:"/home/ada",environment:{},expected:"/home/ada/.cache/semio/lease-registry"},
 {name:"Linux ignores LOCALAPPDATA",platform:"linux",home:"/home/ada",environment:{LOCALAPPDATA:"D:/Local"},expected:"/home/ada/.cache/semio/lease-registry"},
 {name:"override beats every platform",platform:"win32",home:"C:/Users/ada",environment:{SEMIO_LEASE_REGISTRY_DIRECTORY:"/fixed/registry",LOCALAPPDATA:"D:/Local"},expected:"/fixed/registry"},
];

test("the registry root follows the cache convention of each platform",()=>{
 for(const row of vectors)expect(normalize(leaseRegistryRootV1(row.environment,row.platform,row.home)),row.name).toBe(row.expected);
});

/** 🐍️ An empty or relative `XDG_CACHE_HOME` is no cache directory (XDG Base Directory Specification, `$XDG_CACHE_HOME`): the three per-user roots of the lease registry, the agent sessions and the capacity gate then fall back to `~/.cache` together, so no working directory owns a registry of its own. The oracle is Python's `os.path.isabs` reading of the rule. */
const spellings:readonly(string|null)[]=[null,"","relative/cache",".","./cache","~/cache","/var/cache/ada"];
test("an empty or relative XDG_CACHE_HOME is ignored by every per-user root, equal to the independent Python reading",()=>{
 const python=testPythonV1();
 if(python===null)throw Error("python is required for the XDG oracle");
 const reading=Bun.spawnSync([python,"-c","import json,os,sys\nrows=json.loads(sys.argv[1])\nprint(json.dumps([('/home/ada/.cache' if v is None or not os.path.isabs(v) else v) for v in rows]))",JSON.stringify(spellings)],{stdout:"pipe",stderr:"pipe"});
 expect(reading.exitCode,reading.stderr.toString()).toBe(0);
 const bases=JSON.parse(reading.stdout.toString()) as string[];
 spellings.forEach((value,index)=>{
  const environment:Record<string,string>=value===null?{}:{XDG_CACHE_HOME:value},base=bases[index]!;
  expect(normalize(leaseRegistryRootV1(environment,"linux","/home/ada")),String(value)).toBe(normalize(base)+"/semio/lease-registry");
  expect(normalize(agentSessionRegistryRootV1(environment,"linux","/home/ada")).startsWith(normalize(base)+"/semio/"),String(value)).toBe(true);
  expect(normalize(hostCapacityDirectoryV1(environment,"linux","/home/ada")),String(value)).toBe(normalize(base)+"/semio/host-capacity");
 });
});
