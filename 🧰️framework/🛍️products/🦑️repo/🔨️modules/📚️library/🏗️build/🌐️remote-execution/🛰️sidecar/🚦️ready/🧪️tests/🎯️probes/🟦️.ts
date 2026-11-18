import {describe,expect,test} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {READY_PROBES_V1,READY_STAGES_V1} from "../../🟦️.ts";

/** 🎯️ Readiness proves one target of a small declared probe set, never the whole workspace: the `workspace` stage reads only the closure of the probes, which are real owners of the registry with the probed target. */
const root=(()=>{let at=import.meta.dir;while(!existsSync(join(at,"📋️build-owners.json")))at=dirname(at);return at;})();
const registry=JSON.parse(readFileSync(join(root,"📋️build-owners.json"),"utf8")) as {owners:{name:string;manifest:string}[]};
const stage=READY_STAGES_V1.find(row=>row.name==="workspace")!;
describe("ready probes",()=>{
 test("the workspace stage runs the declared probes and not the task whose closure is the whole workspace",()=>{
  expect(stage.arguments).not.toContain("workspace:setup");
  expect(stage.arguments).toEqual(["run-many",`--projects=${READY_PROBES_V1.projects.map(row=>row.name).join(",")}`,`--targets=${READY_PROBES_V1.target}`]);
 });
 test("a handful of probes of different kinds, each a literal project name",()=>{
  expect(READY_PROBES_V1.projects.length).toBeGreaterThanOrEqual(3);
  expect(READY_PROBES_V1.projects.length).toBeLessThanOrEqual(8);
  expect(new Set(READY_PROBES_V1.projects.map(row=>row.kind)).size).toBeGreaterThanOrEqual(3);
  for(const row of READY_PROBES_V1.projects)expect(row.name).toMatch(/^[@\w][\w\-./@]*$/u);
 });
 test("every probe is an owner of the registry that declares the probed target",()=>{
  for(const row of READY_PROBES_V1.projects){
   const owner=registry.owners.find(candidate=>candidate.name===row.name);
   expect(owner,row.name).toBeDefined();
   expect(Object.keys(JSON.parse(readFileSync(join(root,owner!.manifest),"utf8")).targets),row.name).toContain(READY_PROBES_V1.target);
  }
 });
 test("the stages before the workspace stage are unchanged",()=>{
  expect(READY_STAGES_V1.map(row=>row.name)).toEqual(["native","repo-mcp","dependencies","tools","workspace"]);
 });
});
