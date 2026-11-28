import{P,fail,isObject,requireKeys,requireHex,requireCount,requireText}from"../🧱️guards/🟨️.mjs";
import{readVerifiedStampV1}from"../🔭️coverage/🟨️.mjs";
import{readEdgeReadsV1,readEnvironmentPartsV1}from"../🧵️edge-reads/🟨️.mjs";

/** 📌️ Admits the closed pointer to the newest generation: a manifest digest, or null after an explicit invalidation. */
export function readGenerationPointerV1(bytes){
 const value=JSON.parse(Buffer.from(bytes).toString("utf8"));
 requireKeys(value,["version","generation","manifest","parent"],["reason","verified"],"generation pointer");
 if(value.version!==1)throw fail("Generation pointer version 1 required");
 requireCount(value.generation,"generation");
 if(value.manifest!==null)requireHex(value.manifest,"manifest");
 if(value.parent!==null)requireHex(value.parent,"parent");
 if(value.reason!==undefined)requireText(value.reason,"reason",P.reasonMaximumCharacters);
 if(value.verified!==undefined)readVerifiedStampV1(value.verified);
 return Object.freeze(value);
}

/** 📜️ Admits one closed immutable generation manifest. */
export function readGenerationManifestV1(bytes){
 const value=JSON.parse(Buffer.from(bytes).toString("utf8"));
 requireKeys(value,["version","generation","parent","identity","implementation","env","membership","frontier","factIndex","global","owners","built"],["envParts"],"generation manifest");
 if(value.envParts!==undefined)readEnvironmentPartsV1(value.envParts);
 if(value.version!==1)throw fail("Generation manifest version 1 required");
 requireCount(value.generation,"generation");
 if(value.parent!==null)requireHex(value.parent,"parent");
 for(const key of["identity","implementation","env","membership","factIndex","global"])requireHex(value[key],key);
 if(value.frontier!==null)requireHex(value.frontier,"frontier");
 if(!isObject(value.owners)||Object.keys(value.owners).length>P.maximumOwners)throw fail("Bounded owner table required");
 for(const[name,row]of Object.entries(value.owners)){
  requireText(name,"owner name");requireKeys(row,["root","kind","fragment","bytes","edgesEnv","targets"],["config","nativeRoot","tag","reads","edgesReads"],"generation owner");if(row.edgesReads!==undefined)requireHex(row.edgesReads,"edgesReads");if(row.reads!==undefined)readEdgeReadsV1(row.reads);
  requireText(row.root,"owner root");if(!["project","unavailable"].includes(row.kind))throw fail("Closed owner kind required");
  requireHex(row.fragment,"fragment");requireHex(row.edgesEnv,"edgesEnv");requireCount(row.bytes,"fragment bytes",P.maximumFragmentBytes);
  if(!Array.isArray(row.targets)||row.targets.some(target=>typeof target!=="string"))throw fail("Target name list required");
  if(row.config!==undefined)requireText(row.config,"owner config");if(row.nativeRoot!==undefined)requireText(row.nativeRoot,"owner native root");if(row.tag!==undefined)requireText(row.tag,"owner tag",64);
 }
 requireKeys(value.built,["kind","base","dirty"],[],"generation provenance");
 if(!["cold","delta","refresh"].includes(value.built.kind))throw fail("Closed provenance kind required");
 if(value.built.base!==null)requireCount(value.built.base,"base generation");
 if(!Array.isArray(value.built.dirty)||value.built.dirty.some(name=>typeof name!=="string"))throw fail("Dirty owner list required");
 return Object.freeze(value);
}

/** 🧩️ Admits one closed owner fragment: the neutral node, its outgoing edges and the environment those edges were computed under. */
export function readGenerationFragmentV1(value){
 requireKeys(value,["version","name","root","kind","node","edges","edgesEnv","targets"],["config","nativeRoot"],"generation fragment");
 if(value.version!==1)throw fail("Generation fragment version 1 required");
 requireText(value.name,"fragment name");requireText(value.root,"fragment root");requireHex(value.edgesEnv,"edgesEnv");
 if(!["project","unavailable"].includes(value.kind)||!Array.isArray(value.edges))throw fail("Closed fragment required");
 return value;
}
