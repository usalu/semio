/** 🧪️ Tests for build host-capacity partial. */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readHostCapacityPolicyV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {runCapacityCommandV1} from "../../⌨️command/🟦️.ts";

/** 🪟️ Contract: `repo capacity status` says in the selected language when the host counters are partial (a Windows probe failed), and prints nothing extra for a complete sample. */
const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const policy=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,sampleMaximumAgeMilliseconds:0}});
const host=(over:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:Date.now(),platform:"win32",cores:8,totalMegabytes:16384,availableMegabytes:9000,pressure:"warn",swapUsedMegabytes:null,swapOutMegabytesPerSecond:null,load:null,openFiles:null,openFilesMaximum:null,attachedImages:null,...over});
const status=async(sample:HostSampleV1,language:string,args:string[]=["status"])=>{
 const directory=mkdtempSync(join(tmpdir(),"hcp-"));roots.push(directory);const lines:string[]=[];
 expect(await runCapacityCommandV1(args,{environment:{LANG:language},write:line=>{lines.push(line);},directory,policy,sample:async()=>({host:sample,swappedOutMegabytes:null})})).toBe(0);
 return lines;
};

test("a partial sample adds one line in the selected language and the JSON snapshot carries the marker",async()=>{
 const english=await status(host({counters:"partial"}),"en_US.UTF-8"),german=await status(host({counters:"partial"}),"de_DE.UTF-8");
 expect(english.length).toBe(2);expect(english[1]).toMatch(/win32 memory counters could not be read \(probe failed\)/u);
 expect(german[1]).toMatch(/Speicherzähler von win32 konnten nicht gelesen werden \(Probe fehlgeschlagen\)/u);
 expect(JSON.parse((await status(host({counters:"partial"}),"en_US.UTF-8",["status","--json"]))[0]!).host.counters).toBe("partial");
});

test("a complete sample prints the status line alone",async()=>{
 expect((await status(host(),"en_US.UTF-8")).length).toBe(1);
});
