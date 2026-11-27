import{compareCodePointsV1}from"../🧮️fact-codec/🟨️.mjs";

/** 🪡️ The owners of a stored owner table whose edges were made under another environment than `environment`, in code point order. Tagged owners (tests, policies) are rediscovered as a group and cannot get their edges again alone, so they are listed apart in `tagged`. Edges depend on the owner table and the package index of the environment and on the owner's own stored node, never on its inference facts, so an owner in `edges` needs no inference. */
export function staleEdgeOwnersV1(owners,environment){
 const edges=[],tagged=[];
 for(const name of Object.keys(owners).sort(compareCodePointsV1))if(owners[name].edgesEnv!==environment)(owners[name].tag?tagged:edges).push(name);
 return{edges,tagged};
}

/** 🪡️ What a compute spec says about stale edges: the untagged owners of `owners` whose edges are not of `environment` and that the build does not infer anyway (`inferred`), as `{edgeStale}`, nothing when there are none or no environment is known. The compute gives them their edges again in the same pass, whether the environment moved or not. */
export function edgeStaleSpecV1(owners,environment,inferred){
 if(typeof environment!=="string")return{};
 const skip=new Set(inferred),edgeStale=staleEdgeOwnersV1(owners,environment).edges.filter(name=>!skip.has(name));
 return edgeStale.length?{edgeStale}:{};
}

/** 🩹️ What a build does about owners that are still stale after its compute: the first round gives the untagged ones their edges again without inference (`edges`) and infers the tagged ones (`infer`); the second round infers whatever the first could not repair; after `limit` rounds, or on a cold build, nothing is left to try and the caller refuses the publication. */
export function edgeHealPlanV1({owners,environment,cold,healed,limit=2}){
 if(cold||healed>=limit)return{edges:[],infer:[]};
 const{edges,tagged}=staleEdgeOwnersV1(owners,environment);
 return healed===0?{edges,infer:tagged}:{edges:[],infer:[...edges,...tagged].sort(compareCodePointsV1)};
}

/** 🚧️ Refuses, with `fail`, a publication whose owner table still holds edges of another environment than `environment`, naming the first such owner and every flag that says which path left it behind (`wave` the owners of this wave, `returned`, `dropped` and `carried` the owners the compute answered for, `before` the owner table before the wave, `previous` the environment it was read for). */
export function requireEdgeEnvironmentV1({owners,environment,fail,wave,returned,dropped,carried,cold,repair,healed,resumed,before,previous}){
 for(const[name,row]of Object.entries(owners))if(row.edgesEnv!==environment)throw fail("Every owner needs edges of the published environment: "+name+" ("+JSON.stringify({edges:String(row.edgesEnv??"none").slice(0,12),environment:String(environment).slice(0,12),root:row.root,dirty:wave.includes(name),returned:returned.has(name),dropped:dropped.has(name),carried:(carried??[]).includes(name),diag:[cold,repair,healed,resumed,before[name]?.edgesEnv?.slice(0,8),previous?.slice(0,8)].join()})+")","GENERATION_INCOMPLETE");
}
