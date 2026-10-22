import {createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import type {AncestorV1} from "../🧭️ancestry/🟦️.ts";

/** 📜️ The authored policy of agent sessions: identity rules, resource derivation, liveness windows and store layout. */
export const AGENT_SESSION_POLICY_V1=schema.definitions.Policy.const;
export type AgentSessionPolicyV1=typeof AGENT_SESSION_POLICY_V1;
export type AgentClientV1=(typeof schema.definitions.ClientId.enum)[number];
export type AgentOriginV1=(typeof schema.definitions.Origin.enum)[number];
export type AgentSessionStateV1=(typeof schema.definitions.State.enum)[number];
export type AgentSessionScopeV1=AgentSessionPolicyV1["resources"]["scopes"][number];
export type AgentSessionAnchorV1=Readonly<{pid:number;started:string}>;
/** 🪪️ One issued session (`AgentSession` of the schema): who the agent is and every resource name derived for it. */
export type AgentSessionV1=Readonly<{
 version:1;id:string;worktree:string;agent:Readonly<{id:string;client:AgentClientV1;origin:AgentOriginV1}>;anchors:readonly AgentSessionAnchorV1[];openedAtMs:number;
 resources:Readonly<{cargo:Readonly<{lane:string}>;bazel:Readonly<{preference:number}>;nx:Readonly<{namespace:string}>;tests:Readonly<{root:string}>}>;
 lease:Readonly<{owner:string}>;budget:Readonly<{weight:number}>;capability:Readonly<{scopes:readonly AgentSessionScopeV1[];projects:readonly string[];expiresAtMs?:number}>;
}>;
/** 🌱️ What identifies an agent before a session exists. */
export type AgentSeedV1=Readonly<{agent?:string;client?:string;origin?:"service";connection?:Readonly<{client:string;clientProcess:number;process:number}>;environment:Readonly<Record<string,string>>;ancestry:readonly AncestorV1[];ownerPid?:number;weight?:number}>;
/** 🧾️ The session fields a seed determines, without store facts. */
export type DerivedAgentSessionV1=Omit<AgentSessionV1,"worktree"|"openedAtMs"|"anchors">&Readonly<{anchor?:AgentSessionAnchorV1}>;

/** 🛑️ Raised for a refused input; carries the message id and values so any tool renders it in either language. */
export class AgentSessionErrorV1 extends Error{
 readonly code:string;
 readonly values:Readonly<Record<string,string|number>>;
 constructor(code:string,values:Readonly<Record<string,string|number>>={}){super(`${code} ${JSON.stringify(values)}`);this.name="AgentSessionErrorV1";this.code=code;this.values=values;}
}

const policy=AGENT_SESSION_POLICY_V1,identity=policy.identity,agentPattern=new RegExp(identity.agentPattern,"u"),sessionPattern=new RegExp(identity.sessionPattern,"u"),clientPattern=new RegExp(identity.clientProcessPattern,"iu");
const digest=(...parts:readonly (string|number)[]):string=>createHash("sha256").update(identity.domain+"\0"+parts.join("\0")).digest("hex");
const knownClients=policy.identity.clients.map(row=>row.id as string);

/** 🧼️ Keeps `[A-Za-z0-9._-]`, replaces every other character with an underscore and bounds the length; the result is empty only for empty input. */
export function sanitizeAgentIdV1(raw:string):string{return raw.replace(/[^A-Za-z0-9._-]/gu,"_").slice(0,64);}
/** 🔎️ Tells whether text is a valid agent identifier. */
export const isAgentIdV1=(value:string):boolean=>agentPattern.test(value);
/** 🔎️ Tells whether text is a valid session identifier. */
export const isAgentSessionIdV1=(value:string):boolean=>sessionPattern.test(value);

/** 🏷️ Names the agent client a process command line starts: the pattern is matched against the executable only (never against arguments such as shell snapshots or paths), or against the first script argument of an interpreter such as node or bun. */
export function agentClientProcessV1(command:string):AgentClientV1|undefined{
 const text=command.trim(),quoted=text.startsWith('"')?text.indexOf('"',1):-1,tokens=quoted>0?[text.slice(1,quoted),...text.slice(quoted+1).trim().split(/\s+/u)]:text.split(/\s+/u);
 const base=(path:string)=>path.split(/[\\/]/u).at(-1)!.toLowerCase().replace(/\.exe$/u,""),executable=tokens[0]??"";
 const subject=identity.interpreters.includes(base(executable))?tokens.slice(1).find(token=>token!==""&&!token.startsWith("-")):executable;
 if(subject===undefined)return undefined;
 const match=clientPattern.exec(subject===executable?base(executable):subject);
 return match===null?undefined:agentClientOfV1(match[1]!,"human");
}

/** 🏷️ Names the client of a free-form name or command: the first known client it contains, else the fallback. */
export function agentClientOfV1(text:string,fallback:AgentClientV1):AgentClientV1{
 const lower=text.toLowerCase();return (knownClients.find(client=>lower.includes(client))??fallback) as AgentClientV1;
}

/** 🔑️ The session identifier of an agent: a letter and ten base-36 characters of a domain-separated digest; the default agent keeps the fixed default session. */
export function agentSessionIdV1(agent:string):string{
 if(agent===identity.defaultAgent)return identity.defaultSession;
 return "a"+BigInt("0x"+digest("session",agent).slice(0,12)).toString(36).slice(0,10);
}

/** 🧬️ Derives who the agent is: an explicit name, then the client's own session variable, then the connection, then the nearest agent-client ancestor, else the default agent. */
export function deriveAgentIdentityV1(seed:AgentSeedV1):Readonly<{agent:AgentSessionV1["agent"];anchor?:AgentSessionAnchorV1}>{
 if(seed.agent!==undefined){
  const id=sanitizeAgentIdV1(seed.agent);if(id==="")throw new AgentSessionErrorV1("invalidAgent",{value:seed.agent});
  const client=seed.origin==="service"?"service":agentClientOfV1(seed.client??"",seed.client===undefined?"human":"mcp");
  return {agent:{id,client,origin:seed.origin==="service"?"service":"explicit"}};
 }
 for(const row of identity.clients){
  const name=(row as {environment?:string}).environment,value=name===undefined?undefined:seed.environment[name];
  if(value!==undefined&&sanitizeAgentIdV1(value)!=="")return {agent:{id:sanitizeAgentIdV1(value),client:row.id as AgentClientV1,origin:"environment"}};
 }
 if(seed.connection!==undefined){
  const name=sanitizeAgentIdV1(seed.connection.client.toLowerCase()).slice(0,identity.connectionNameCharacters),base=name===""?"agent":name;
  return {agent:{id:`${base}-${digest("connection",seed.connection.client,seed.connection.clientProcess,seed.connection.process).slice(0,identity.digestCharacters)}`,client:agentClientOfV1(seed.connection.client,"mcp"),origin:"connection"}};
 }
 const found=seed.ancestry.find(row=>agentClientProcessV1(row.command)!==undefined);
 if(found!==undefined){
  const client=agentClientProcessV1(found.command)!;
  return {agent:{id:`${client}-${digest("process",found.pid,found.started).slice(0,identity.digestCharacters)}`,client,origin:"process"},anchor:{pid:found.pid,started:found.started}};
 }
 return {agent:{id:identity.defaultAgent,client:"human",origin:"default"}};
}

/** 🧮️ Derives every resource name of a session from its identifier alone. */
export function agentSessionResourcesV1(id:string):AgentSessionV1["resources"]{
 return Object.freeze({
  cargo:Object.freeze({lane:id}),
  bazel:Object.freeze({preference:Number(BigInt("0x"+digest("bazel",id).slice(0,12))%BigInt(policy.resources.bazelPreferenceModulus))}),
  nx:Object.freeze({namespace:digest("nx",id).slice(0,policy.resources.nxNamespaceCharacters)}),
  tests:Object.freeze({root:id.slice(0,policy.resources.testsRootCharacters)}),
 });
}

/** ⚖️ Validates a budget weight. */
export function agentSessionWeightV1(value:number|undefined):number{
 const rules=policy.resources.weight;if(value===undefined)return rules.default;
 if(!Number.isSafeInteger(value)||value<rules.minimum||value>rules.maximum)throw new AgentSessionErrorV1("invalidWeight",{value:String(value),minimum:rules.minimum,maximum:rules.maximum});
 return value;
}

/** 🪪️ Derives a session from a seed: identity, resources, lease owner tag, budget weight and the default capability. */
export function deriveAgentSessionV1(seed:AgentSeedV1):DerivedAgentSessionV1{
 const found=deriveAgentIdentityV1(seed),id=agentSessionIdV1(found.agent.id),anchor=found.anchor??(seed.ownerPid===undefined?undefined:{pid:seed.ownerPid,started:""});
 return Object.freeze({
  version:1,id,agent:Object.freeze(found.agent),resources:agentSessionResourcesV1(id),
  lease:Object.freeze({owner:`${found.agent.id.slice(0,policy.resources.leaseOwnerAgentCharacters)}/${id}`}),
  budget:Object.freeze({weight:agentSessionWeightV1(seed.weight)}),
  capability:Object.freeze({scopes:Object.freeze([...policy.resources.defaultScopes] as AgentSessionScopeV1[]),projects:Object.freeze([] as string[])}),
  ...(anchor===undefined?{}:{anchor}),
 });
}

/** ⏳️ Classifies a session: live while it is the default session or was seen within the idle window; idle while its owner process is alive within a day or it was seen within the collection window; gone after that. */
export function agentSessionStateV1(facts:Readonly<{ageSeconds:number;anchorAlive:boolean;isDefault:boolean}>):AgentSessionStateV1{
 const rules=policy.liveness;
 if(facts.isDefault||facts.ageSeconds<=rules.idleSeconds)return "live";
 return facts.anchorAlive&&facts.ageSeconds<=rules.anchoredSeconds||facts.ageSeconds<=rules.collectSeconds?"idle":"gone";
}

/** 🥧️ Splits the whole budget between sessions by weight in basis points: largest remainder, ties by identifier, so the shares always add up to the whole. */
export function agentSessionSharesV1(rows:readonly Readonly<{id:string;weight:number}>[]):Readonly<Record<string,number>>{
 const whole=policy.resources.shareBasisPoints,total=rows.reduce((sum,row)=>sum+row.weight,0),result:Record<string,number>={};
 if(total===0)return result;
 const parts=rows.map(row=>({id:row.id,floor:Math.floor(row.weight*whole/total),rest:(row.weight*whole)%total}));
 let left=whole-parts.reduce((sum,row)=>sum+row.floor,0);
 for(const row of [...parts].sort((a,b)=>b.rest-a.rest||(a.id<b.id?-1:a.id>b.id?1:0))){if(left>0&&row.rest>0){row.floor++;left--;}}
 for(const row of parts)result[row.id]=row.floor;
 return result;
}

/** ✅️ Checks a value against the closed session schema. */
export function requireAgentSessionV1(value:unknown):AgentSessionV1{
 const errors=validateJsonSchemaSubset(schema.definitions.AgentSession,value,schema);
 if(errors.length>0)throw new Error(`Closed agent session required: ${errors[0]}`);
 return value as AgentSessionV1;
}
/** ✅️ Tells whether a value is a closed session. */
export const isAgentSessionV1=(value:unknown):value is AgentSessionV1=>validateJsonSchemaSubset(schema.definitions.AgentSession,value,schema).length===0;
