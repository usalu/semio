import{availableParallelism}from"node:os";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const freeze=value=>{if(value!==null&&typeof value==="object"){for(const child of Object.values(value))freeze(child);Object.freeze(value);}return value;};

/** 🪶️ The declared numbers of the lean slice plan: the schema file is the single source.
 * @see ./🧬️schema/🔣️.json */
export const LEAN_SLICE_POLICY_V1=freeze(structuredClone(schema.definitions.Policy.const));

/** 🗣️ The bilingual lines of the lean slice plan, read from the schema file. */
export const LEAN_SLICE_MESSAGES_V1=freeze(structuredClone(schema.definitions.Messages.const));

/** 💬️ Renders one lean line, English first and German second, or the one language the environment names. */
export function renderLeanSliceMessageV1(key,fields,environment=process.env){
 const tag=String(environment.SEMIO_LANGUAGE??"").toLowerCase(),languages=tag==="en"||tag==="de"?[tag]:["en","de"];
 return languages.map(language=>{
  const template=LEAN_SLICE_MESSAGES_V1[language][key];
  if(template===undefined)throw Error("Closed lean slice message required: "+key);
  return template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]));
 });
}

/**
 * 🪶️ The lean plan of a slice read: whether it runs lean and how many inference children of which budget the process tree may hold. A machine without a plan limit (`limitMegabytes` Infinity: it serves the fleet) is not lean unless the mode says `on`; the first child is always granted, further children only while the parent and the children stay within the limit, never more than the cores minus `coreReserve`. `fits` tells whether one child fits beside the parent within the ceiling of the host (`ceilingMegabytes`); where it does not, the caller keeps the work in the parent and says so (`tooSmall`).
 * @see ./🧬️schema/🔣️.json
 */
export function leanSlicePlanV1({limitMegabytes,ceilingMegabytes=Infinity,cores=availableParallelism(),residentMegabytes=0,environment=process.env,policy=LEAN_SLICE_POLICY_V1}){
 const mode=String(environment[policy.modeEnvironment]??"auto").toLowerCase();
 if(!policy.modes.includes(mode))throw Error("Lean slice mode of "+policy.modes.join(", ")+" required: "+policy.modeEnvironment);
 const bounded=Number.isFinite(limitMegabytes),lean=mode==="on"||mode==="auto"&&bounded;
 if(!lean)return Object.freeze({lean:false,limitMegabytes:bounded?limitMegabytes:null,parentMegabytes:0,childMegabytes:0,children:0,ownersPerChild:0,loopsPerChild:0,fits:true});
 const parent=Math.max(policy.parentMegabytes,Math.ceil(residentMegabytes)),room=bounded?Math.floor((limitMegabytes-parent)/policy.childMegabytes):Infinity,cap=Math.max(policy.minimumChildren,Math.floor(cores)-policy.coreReserve),children=Math.min(cap,Math.max(policy.minimumChildren,room));
 return Object.freeze({lean:true,limitMegabytes:bounded?limitMegabytes:null,parentMegabytes:parent,childMegabytes:policy.childMegabytes,children,ownersPerChild:policy.ownersPerChild,loopsPerChild:policy.loopsPerChild,fits:parent+policy.childMegabytes<=ceilingMegabytes});
}

/** 🏊️ The farm policy of a lean read: the declared farm policy with the child budget, the owners one child infers at once and the owners after which a child is replaced. */
export function leanFarmPolicyV1(farm,plan){
 return plan.lean?Object.freeze({...farm,workerMegabytes:plan.childMegabytes,loopsPerWorker:plan.loopsPerChild,recycleAfterProjects:plan.ownersPerChild}):farm;
}
