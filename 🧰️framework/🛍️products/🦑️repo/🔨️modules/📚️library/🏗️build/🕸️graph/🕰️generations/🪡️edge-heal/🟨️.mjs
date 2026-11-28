import{compareCodePointsV1}from"../🧮️fact-codec/🟨️.mjs";
export{componentEdgesV1}from"../🧩️environment-components/🟨️.mjs";

/** 🪡️ The owners of a stored owner table whose edges were made under another environment than `environment`, in code point order. Tagged owners (tests, policies) are rediscovered as a group and cannot get their edges again alone, so they are listed apart in `tagged`. Edges depend on the owner table and the package index of the environment and on the owner's own stored node, never on its inference facts, so an owner in `edges` needs no inference. */
export function staleEdgeOwnersV1(owners,environment,pending=[]){
 const edges=[],tagged=[],later=new Set(pending);
 for(const name of Object.keys(owners).sort(compareCodePointsV1))if(owners[name].edgesEnv!==environment&&!later.has(name))(owners[name].tag?tagged:edges).push(name);
 return{edges,tagged};
}

/** 🪡️ What a compute spec says about stale edges: the untagged owners of `owners` whose edges are not of `environment` and that the build does not infer anyway (`inferred`), as `{edgeStale}`, nothing when there are none or no environment is known. The compute gives them their edges again in the same pass, whether the environment moved or not. */
export function edgeStaleSpecV1(owners,environment,inferred){
 if(typeof environment!=="string")return{};
 const skip=new Set(inferred),edgeStale=staleEdgeOwnersV1(owners,environment).edges.filter(name=>!skip.has(name));
 return edgeStale.length?{edgeStale}:{};
}

/** 🩹️ What a build does about owners that are still stale after its compute (owners `pending` for inference in a later wave of the read are never stale: their wave infers them under the then published environment). Every stale untagged owner is first given its edges again without inference (`edges`); an owner that stayed stale under the same environment although it was given its edges, and every stale tagged owner, is inferred (`infer`) once per environment; whatever is left after both has nothing left to try and the caller refuses the publication with the diagnosis. `tried` and `inferred` are the names (arrays or sets) or the maps name to environment of earlier rounds, so a moving environment starts every owner afresh and a round that repairs nothing for an owner is never repeated for it; `limit` only bounds the number of rounds of a read against a runaway. A cold build tries nothing. */
export function edgeHealPlanV1({owners,environment,cold,healed,tried,inferred,pending=[],limit=64}){
 if(cold||healed>=limit)return{edges:[],infer:[]};
 const{edges,tagged}=staleEdgeOwnersV1(owners,environment,pending),seen=(source,name)=>source instanceof Map?source.get(name)===environment:Array.isArray(source)?source.includes(name):source instanceof Set&&source.has(name);
 return{edges:edges.filter(name=>!seen(tried,name)),infer:[...tagged,...edges.filter(name=>seen(tried,name))].filter(name=>!seen(inferred,name)).sort(compareCodePointsV1)};
}

/** 🚧️ Refuses, with `fail`, a publication whose owner table still holds edges of another environment than `environment`, naming the first such owner and every flag that says which path left it behind (`wave` the owners of this wave, `returned`, `dropped` and `carried` the owners the compute answered for, `before` the owner table before the wave, `previous` the environment it was read for, `spec` the compute spec: the diagnosis ends with the stub kind and tag of the owner, whether the spec named it stale, in the wave, or under a discovered path, and the counts of discovered paths and stubs). */
export function requireEdgeEnvironmentV1({owners,environment,fail,wave,returned,dropped,carried,cold,repair,healed,resumed,before,previous,spec,pending=[]}){
 const later=new Set(pending);
 for(const[name,row]of Object.entries(owners))if(row.edgesEnv!==environment&&!later.has(name))throw fail("Every owner needs edges of the published environment: "+name+" ("+JSON.stringify({edges:String(row.edgesEnv??"none").slice(0,12),environment:String(environment).slice(0,12),root:row.root,dirty:wave.includes(name),returned:returned.has(name),dropped:dropped.has(name),carried:(carried??[]).includes(name),diag:[cold,repair,healed,resumed,before[name]?.edgesEnv?.slice(0,8),previous?.slice(0,8),spec?.stubs?.[name]?.kind,spec?.stubs?.[name]?.tag,spec?.edgeStale?.includes(name),spec?.owners?.includes(name),(spec?.discover??[]).some(path=>String(path).startsWith(row.root)),(spec?.discover??[]).length,Object.keys(spec?.stubs??{}).length].join()})+")","GENERATION_INCOMPLETE");
}

/** 🧩️ Gives the edges of every untagged owner whose environment stamp is older than `environment` that still hold the stamp of `environment` without computing them again: `component.holds` says that no component the owner's edges read moved since its edges were made (any number of environment changes ago), so its edges are those of the present environment. Returns the owner table, with the stamp and the digest of the components renewed, and the names restamped. Owners without a recorded digest, tagged owners and owners in `pending` are left to the heal plan. */
export function restampComponentValidEdgesV1({owners,environment,component,pending=[],say}){
 const later=new Set(pending),restamped=[],next={...owners};
 for(const name of Object.keys(owners)){
  const row=owners[name];
  if(row.edgesEnv===environment||row.tag||later.has(name)||!component.holds(name,row))continue;
  next[name]={...row,edgesEnv:environment};restamped.push(name);
 }
 if(restamped.length)say?.("restamped",{count:restamped.length});
 return{owners:next,restamped:restamped.sort(compareCodePointsV1)};
}

/** 🪡️ Stamps the edges a build keeps: the `carried` owners (the compute proved their edges unchanged) get `environment`, and with a component judge every other owner whose edges provably hold (`restampComponentValidEdgesV1`) too; with a judge each stamped row also carries the digest of the components it read. Changes `owners` in place. */
export function carryEdgeStampsV1({owners,carried=[],returned,environment,component,cold,pending,say}){
 for(const name of carried)if(owners[name]&&!returned.has(name))owners[name]={...owners[name],edgesEnv:environment,...component?.stamp(name,owners[name])};
 if(component&&!cold)Object.assign(owners,restampComponentValidEdgesV1({owners,environment,component,pending,say}).owners);
}
