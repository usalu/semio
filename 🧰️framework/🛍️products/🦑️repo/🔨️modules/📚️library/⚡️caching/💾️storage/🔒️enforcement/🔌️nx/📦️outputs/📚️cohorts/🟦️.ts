import{posix,win32}from"node:path";
import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

type Request={version:1;productsRoot:string;mode:"prototype"|"release";platform:"posix"|"windows"};
export interface RepositoryArtifactCohortV1{readonly version:1;readonly mode:"prototype"|"release";readonly root:string;readonly resource:string;readonly members:readonly Readonly<{name:string;path:string}>[]}
const protocol=schema.definitions.Protocol.const;
function request(value:unknown):Request{if(validateJsonSchemaSubset(schema.definitions.Request,value,schema).length)throw Error("Closed repository artifact cohort required");const row=value as Request,path=row.platform==="windows"?win32:posix;if(!path.isAbsolute(row.productsRoot)||path.resolve(row.productsRoot)!==row.productsRoot||/[\u0000-\u001f]/u.test(row.productsRoot))throw Error("Canonical absolute product root required");return{version:1,productsRoot:row.productsRoot,mode:row.mode,platform:row.platform};}

/** 🧩️ Shares immutable Pages location data between its actual domain publisher and Nx output custody. */
export function playPagesArtifactCohortV1(value:unknown):RepositoryArtifactCohortV1{
 const row=request(value),path=row.platform==="windows"?win32:posix,root=path.join(row.productsRoot,protocol.prefix,row.mode,protocol.directory),members=protocol.members.map(name=>Object.freeze({name,path:path.join(root,name)}));if(members.some(member=>member.path.length>protocol.maximumPathUnits))throw Error("Finite cohort member path required");return Object.freeze({version:1,mode:row.mode,root,resource:protocol.resource+row.mode,members:Object.freeze(members)});
}

/** 🗂️ Matches aggregate, member and descendant declarations without opening storage or granting a lease. */
function cohortsForOutputs(value:unknown,outputs:readonly string[]):readonly RepositoryArtifactCohortV1[]{
 const row=request(value),path=row.platform==="windows"?win32:posix,cohorts=(["prototype","release"]as const).map(mode=>playPagesArtifactCohortV1({...row,mode})),selected=new Map<string,RepositoryArtifactCohortV1>();
 for(const output of outputs){if(typeof output!=="string"||!path.isAbsolute(output)||path.resolve(output)!==output||output.length>protocol.maximumPathUnits||/[\u0000-\u001f]/u.test(output))throw Error("Canonical finite Nx product output required");const child=path.relative(row.productsRoot,output);if(!child||child===".."||child.startsWith(".."+path.sep)||path.isAbsolute(child))throw Error("Nx cohort output must be inside actual products root");for(const cohort of cohorts){const inside=path.relative(cohort.root,output);if(inside===""||inside!==".."&&!inside.startsWith(".."+path.sep)&&!path.isAbsolute(inside)){if(cohort.mode!==row.mode)throw Error("Nx cohort must match actual Original mode");selected.set(cohort.resource,cohort);}}}
 return Object.freeze([...selected.values()].sort((left,right)=>left.resource<right.resource?-1:left.resource>right.resource?1:0));
}

/** 🧮️ Captures the complete aggregate and member lease roster for every matching declared output. */
export function nxArtifactOutputCohortPlanV1(value:unknown,outputs:readonly string[]):Readonly<{cohorts:readonly RepositoryArtifactCohortV1[];paths:readonly string[]}>{
 const cohorts=cohortsForOutputs(value,outputs),paths=[...new Set([...outputs,...cohorts.flatMap(cohort=>[cohort.root,...cohort.members.map(member=>member.path)])])].sort((left,right)=>left<right?-1:left>right?1:0);return Object.freeze({cohorts,paths:Object.freeze(paths)});
}
