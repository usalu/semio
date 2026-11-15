import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🪢️ The authored platform policy of an Nx task slot.
 * @see ./🧬️schema/🔣️.json */
export const TASK_SLOT_POLICY_V1=schema.definitions.Policy.const;
const policy=TASK_SLOT_POLICY_V1;

/** 🔎️ What the slot of a task depends on: its declared inputs, the named inputs they refer to (project groups over workspace groups) and the execution kind. */
export type TaskSlotInputsV1=Readonly<{inputs?:readonly unknown[]|undefined;namedInputs?:Readonly<Record<string,readonly unknown[]|undefined>>|undefined;executionKind?:string|undefined}>;
/** 🏷️ Whether the hash of a task can differ between platforms, with the first reasons found. */
export type TaskPlatformVerdictV1=Readonly<{platform:boolean;reasons:readonly string[]}>;

/** 🧭️ Decides if a task is platform-bound by expanding its inputs; `^group` and `{input,projects}` references expand like the group they name, which can only make the verdict more cautious. Anything it cannot interpret counts as platform-bound. */
export function taskPlatformDependenceV1(task:TaskSlotInputsV1):TaskPlatformVerdictV1{
 const reasons:string[]=[],named=task.namedInputs??{},seen=new Set<string>();let budget=policy.maximumExpansion;
 if(task.executionKind!==undefined&&(policy.nativeExecutionKinds as readonly string[]).includes(task.executionKind))reasons.push("execution:"+task.executionKind);
 const visit=(inputs:readonly unknown[]|undefined):void=>{
  for(const input of inputs??["default"]){
   if(--budget<0){reasons.push("expansion-bound");return;}
   if(typeof input==="string"){
    const name=input.replace(/^\^/u,"");
    if(Object.hasOwn(named,name)){
     if(name.toLowerCase().includes(policy.platformNamedInputPattern))reasons.push("named:"+name);
     if(!seen.has(name)){seen.add(name);visit(named[name]);}
    }else if(name==="default"||name==="production"||name==="sharedGlobals"){reasons.push("unresolved:"+name);}
    continue;
   }
   if(input===null||typeof input!=="object"||Array.isArray(input)){reasons.push("uninterpretable");continue;}
   const row=input as Record<string,unknown>;
   if(typeof row.runtime==="string"){if(!(policy.neutralRuntimeCommands as readonly string[]).includes(row.runtime))reasons.push("runtime:"+row.runtime.slice(0,80));}
   else if(typeof row.env==="string"){if(!(policy.neutralEnvironment as readonly string[]).includes(row.env))reasons.push("env:"+row.env);}
   else if(typeof row.input==="string"){const name=row.input;if(name.toLowerCase().includes(policy.platformNamedInputPattern))reasons.push("named:"+name);if(!seen.has(name)&&Object.hasOwn(named,name)){seen.add(name);visit(named[name]);}else if(!Object.hasOwn(named,name))reasons.push("unresolved:"+name);}
   else if(typeof row.fileset!=="string"&&!Array.isArray(row.externalDependencies)&&!("dependentTasksOutputFiles" in row))reasons.push("uninterpretable");
  }
 };
 visit(task.inputs);
 return {platform:reasons.length>0,reasons:[...new Set(reasons)].slice(0,8)};
}
