import assert from "node:assert/strict";
import {selectPackageManagerV1,packageManagerRegistryV1} from "../../🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";
const javascriptManagerV1=selectPackageManagerV1("javascript"),javascriptLockV1=javascriptManagerV1.primaryLockName();
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{BundleScript,createIssuedResourceControlV1,withRequestedIssuedResourceControlV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl}from"@semio-tech/process/leases";
import { loadBuildGraphV1, workspaceBuildFilesV1, buildGraphMembersV1, neutralBuildTargetV1 } from "../../../🏗️build/🕸️graph/🟦️.ts"; import type { BuildTargetV1 } from "../../../🏗️build/🕸️graph/📋️model/🟦️.ts";
import{readBuildGraphSelectionV1,readBuildOwnerRegistryV1}from"../../../🏗️build/🕸️graph/🚪️admission/🟦️.ts";
import{discoverBuildProjectsV1}from"../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs";

/** 🕸️ Verifies neutral graph edges and authored contributions under the actual incoming lifetime. */
export class GraphScript extends BundleScript{
 async run():Promise<void>{
  const phase=createIssuedResourceControlV1(this.invocation,"cache:graph-verification",0);
  await withRequestedIssuedResourceControlV1(phase,0,async actual=>{
   const graph=await loadBuildGraphV1(this.repoRoot,this.invocation,actual,{selection:readBuildGraphSelectionV1({kind:"all"})});checkResourceLeaseControl(actual);
   assert.ok(graph.dependencies.length>0,"A monorepo graph must contain dependency edges");
   for(const edge of graph.dependencies)assert.ok(graph.projects[edge.target]||graph.externalNodes?.[edge.target],`Unknown dependency ${edge.target}`);
   const registry=readBuildOwnerRegistryV1(JSON.parse(readFileSync(join(this.repoRoot,"📋️build-owners.json"),"utf8")));
   assert.equal(createHash("sha256").update(JSON.stringify(registry)).digest("hex"),graph.admission?.registry,"Current declared registry must match the graph admission");
   checkResourceLeaseControl(actual);const files=buildGraphMembersV1(await workspaceBuildFilesV1(this.repoRoot,this.invocation));checkResourceLeaseControl(actual);
   const declared=await discoverBuildProjectsV1(files.filter(file=>file.endsWith("📋️project.json")||file.endsWith("Cargo.toml")||javascriptManagerV1.lockFileNames().includes(file)||file.endsWith(".patch")),{analyzeLockfile:true,catalogProviders:registry.catalogProviders},{workspaceRoot:this.repoRoot,invocation:this.invocation,phaseControl:actual,registry});checkResourceLeaseControl(actual);
   for(const[,contribution]of declared)for(const[name,project]of Object.entries(contribution.projects)){
    checkResourceLeaseControl(actual);const resolved=graph.projects[name];assert.ok(resolved,`Graph omitted ${name}`);
    for(const[targetName,target]of Object.entries(project.targets)){
     checkResourceLeaseControl(actual);const expected=neutralBuildTargetV1(target,`${name}:${targetName}`),current:BuildTargetV1|undefined=resolved.targets[targetName];assert.ok(current,`Graph omitted ${name}:${targetName}`);
     for(const property of["cache","outputs","dependsOn"]as const)if(Object.hasOwn(expected,property))assert.deepEqual(current[property],expected[property],`Graph precedence changed ${name}:${targetName}.${property}`);
    }
   }
   checkResourceLeaseControl(actual);console.log(`[build-graph] projects=${Object.keys(graph.projects).length} edges=${graph.dependencies.length}`);
  });
 }
}
