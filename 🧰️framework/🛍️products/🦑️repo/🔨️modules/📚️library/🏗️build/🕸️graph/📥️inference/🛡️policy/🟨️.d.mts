import type{GeneratedNodesV1,GeneratedProjectV1}from"../../🟨️.mjs";
/** 🛡️ Exposes the actual policy-script DATA contribution without third-party task-runner types. */
declare const policy:Readonly<{name:string;createNodesV2:readonly[string,(files:readonly string[],options:unknown,context:Readonly<{workspaceRoot:string;projects?:Readonly<Record<string,GeneratedProjectV1>>}>)=>GeneratedNodesV1]}>;
export default policy;
