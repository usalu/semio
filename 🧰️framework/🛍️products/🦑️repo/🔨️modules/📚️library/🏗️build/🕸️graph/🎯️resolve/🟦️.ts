import type {BuildExecutionV1,BuildGraphV1,BuildProjectV1,BuildTargetV1} from "../📋️model/🟦️.ts";
import {join} from "node:path";
import {requireBuildConfigurationV1} from "../📜️contract/🟦️.ts";


/** 🎚️ Resolves one configuration with environment inherited from the declared execution. */
export function resolveBuildTargetV1(target:BuildTargetV1,configuration?:string):BuildTargetV1 {
  if(!configuration)return target;
  const configured=target.configurations?.[configuration];
  if(!configured)throw Error(`Unknown build configuration: ${configuration}`);
  if(Object.keys(configured).some(key=>key!=="execution")||!configured.execution)throw Error("Build configuration execution options required");
  const overrides=configured.execution as Partial<BuildExecutionV1>;requireBuildConfigurationV1(target.execution,overrides);
  return {...target,execution:{...target.execution,...overrides,env:{...target.execution.env,...overrides.env}}};
}



/** 🎯️ Expands owned named inputs while preserving explicit dependency and runtime declarations. */
export function resolveBuildInputsV1(graph:BuildGraphV1,project:string|BuildProjectV1,inputs:readonly unknown[]):unknown[] {
  const owner=typeof project==="string"?graph.projects[project]:project;
  if(!owner)throw Error(`Unknown build input owner: ${project}`);
  const named={...graph.namedInputs,...owner.namedInputs as Record<string,unknown[]>|undefined},active=new Set<string>();
  const expand=(values:readonly unknown[]):unknown[]=>values.flatMap(value=>{
    if(typeof value!=="string"||!Object.hasOwn(named,value))return [value];
    if(active.has(value))throw Error(`Build named input cycle: ${[...active,value].join(" -> ")}`);
    active.add(value);const result=expand(named[value]!);active.delete(value);return result;
  });
  return expand(inputs);
}
