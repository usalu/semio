import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {REPOSITORY_LEASE_DIRECTORY_V1,readLeaseAdmissionV1,runLeasesCliV1} from "../🟦️.ts";

const root=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"lc-"));
afterAll(()=>rmSync(root,{recursive:true,force:true}));
const environment=(language:string)=>({...process.env,SEMIO_LANGUAGE:language,SEMIO_LEASE_REGISTRY_DIRECTORY:join(root,"registry"),SEMIO_HOST_CAPACITY_DIRECTORY:join(root,"capacity")}) as NodeJS.ProcessEnv;

test("leases status for a checkout reads its lease store, appends the machine admission queue, and speaks English and German",async()=>{
 mkdirSync(join(root,REPOSITORY_LEASE_DIRECTORY_V1),{recursive:true});
 const english:string[]=[],german:string[]=[];
 expect(await runLeasesCliV1(root,["status"],environment("en"),line=>english.push(line))).toBe(0);
 expect(await runLeasesCliV1(root,["status"],environment("de"),line=>german.push(line))).toBe(0);
 expect(english[0]).toContain("Leases in");expect(english.join("\n")).toContain("No process holds or awaits a lease");expect(english.join("\n")).toContain("Machine admission:");
 expect(german[0]).toContain("Leases in");expect(german.join("\n")).toContain("Kein Prozess hält oder erwartet");expect(german.join("\n")).toContain("Rechner-Zulassung:");
});

test("the JSON report carries the admission section and a closed report shape",async()=>{
 const lines:string[]=[];
 expect(await runLeasesCliV1(root,["status","--json"],environment("en"),line=>lines.push(line))).toBe(0);
 const report=JSON.parse(lines.join("\n"));
 expect(report).toMatchObject({version:1,holders:[],waiters:[],waitFor:[],cycles:[]});
 expect(report.admission.floorMegabytes).toBeGreaterThan(0);expect(report.admission.grants).toEqual([]);
});

test("an unknown subcommand or option is refused",async()=>{
 await expect(runLeasesCliV1(root,["nonsense"],environment("en"))).rejects.toThrow("Expected repo leases status");
 await expect(runLeasesCliV1(root,["status","--bogus"],environment("en"))).rejects.toThrow("Unknown leases option");
 await expect(runLeasesCliV1(root,["status","--violations=-1"],environment("en"))).rejects.toThrow("non-negative integer");
});

test("an unreadable admission gate omits the section instead of failing the command",async()=>{
 expect(await readLeaseAdmissionV1({...process.env,SEMIO_HOST_CAPACITY_DIRECTORY:"/dev/null/nope"} as NodeJS.ProcessEnv,"en")).toBeUndefined();
});
