import{compareCodePointsV1}from"../🧮️fact-codec/🟨️.mjs";

/**
 * 🫧️ The private result of a narrow read that refreshed its stale owners beside a whole read: the global fragment is stored (content addressed, like every fragment), the manifest is only kept in memory on top of the sealed one and never written, no pointer moves, no index is patched and no journal is touched, so a whole read that publishes meanwhile loses nothing.
 * @see ../🔌️load/🟨️.mjs
 */
export async function overlayResultV1({current,usable,identity,implementation,found,computed,carried,marks,computePhases,envDigest,computedParts,live,owners,returned,globalPayload,store,P,cold,repair}){
 if(cold||repair||!usable)throw Object.assign(Error("An overlay refreshes the owners of a sealed generation"),{code:"GENERATION_OVERLAY_COLD"});
 const global=(await store(P.fragments,{version:1,name:"@global",value:globalPayload??null},".json.br",P.maximumFragmentBytes)).digest,{owners:_table,...envParts}=computedParts??{};
 const next={version:1,generation:usable.generation+1,parent:current.pointer.manifest,identity,implementation,env:envDigest,membership:live.digest,frontier:usable.frontier,factIndex:usable.factIndex,global,...(computedParts?{envParts}:{}),owners,built:{kind:"delta",base:usable.generation,dirty:[...returned].sort(compareCodePointsV1)}};
 return{current:{pointer:{...current.pointer,generation:next.generation},manifest:next},built:true,published:false,ephemeral:true,plan:found,computed,phases:{...marks,...Object.fromEntries(Object.entries(computePhases??{}).map(([name,ms])=>["compute."+name,ms]))},carried:carried?.length??0};
}
