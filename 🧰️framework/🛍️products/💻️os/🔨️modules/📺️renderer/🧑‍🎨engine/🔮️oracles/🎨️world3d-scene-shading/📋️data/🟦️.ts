/** 🎨️ Closed DATA projections used by the recorded pixel oracle. */
export function oracleRgbaV1(value:unknown):readonly[number,number,number,number]{
 if(!Array.isArray(value)||value.length!==4||!value.every(channel=>typeof channel==="number"&&Number.isInteger(channel)&&channel>=0&&channel<=255))throw Error("Four RGBA8 channels required");
 const [r,g,b,a]=value;
 if(typeof r!=="number"||typeof g!=="number"||typeof b!=="number"||typeof a!=="number")throw Error("Dense RGBA8 channels required");
 return [r,g,b,a];
}
/** 📏️ Computes every channel only after both actual tuples are admitted. */
export function oraclePixelDeltaV1(actual:unknown,expected:unknown):readonly[number,number,number,number]{
 const a=oracleRgbaV1(actual),b=oracleRgbaV1(expected);return [a[0]-b[0],a[1]-b[1],a[2]-b[2],a[3]-b[3]];
}
/** 🧵️ Self-contained because the actual browser prelude carries this function's Source. */
export function oracleSamplerV1(role:unknown,value:unknown):Readonly<{field:"baseColorTexture"|"metallicRoughnessTexture"|"normalTexture"|"occlusionTexture"|"emissiveTexture";wrapS:33071|33648|10497;wrapT:33071|33648|10497;magFilter:9728|9729;minFilter:9728|9729|9984|9985|9986|9987}>{
 const field=role==="baseColor"?"baseColorTexture":role==="metallicRoughness"?"metallicRoughnessTexture":role==="normal"?"normalTexture":role==="occlusion"?"occlusionTexture":role==="emissive"?"emissiveTexture":null;
 if(field===null)throw Error("Known texture role required");
 const table=value===undefined?{}:value;
 if(table===null||typeof table!=="object"||Array.isArray(table))throw Error("Sampler table required");
 const entry=Object.getOwnPropertyDescriptor(table,field);
 if(entry&&!Object.hasOwn(entry,"value"))throw Error("Sampler DATA required");
 const sampler=entry?entry.value:{};
 if(sampler===null||typeof sampler!=="object"||Array.isArray(sampler))throw Error("Sampler DATA required");
 for(const key of Reflect.ownKeys(sampler))if(typeof key!=="string"||!["wrapS","wrapT","magFilter","minFilter"].includes(key)||!Object.hasOwn(Object.getOwnPropertyDescriptor(sampler,key)??{},"value"))throw Error("Closed sampler DATA required");
 const read=(key:string,fallback:number):unknown=>Object.hasOwn(sampler,key)?Object.getOwnPropertyDescriptor(sampler,key)?.value:fallback;
 const wrapS=read("wrapS",33071),wrapT=read("wrapT",33071),magFilter=read("magFilter",9728),minFilter=read("minFilter",9728);
 if(wrapS!==33071&&wrapS!==33648&&wrapS!==10497||wrapT!==33071&&wrapT!==33648&&wrapT!==10497||magFilter!==9728&&magFilter!==9729||minFilter!==9728&&minFilter!==9729&&minFilter!==9984&&minFilter!==9985&&minFilter!==9986&&minFilter!==9987)throw Error("Supported texture sampler required");
 return {field,wrapS,wrapT,magFilter,minFilter};
}
/** 📜️ Carries a self-contained function into the actual generated browser module. */
export function oracleBrowserDataPreludeV1():string{return "const oracleSamplerV1 = ("+oracleSamplerV1.toString()+");\n";}
/** 🖼️ Narrows the already schema-validated surface object's case projection. */
export function oracleSurfaceCasesV1(value:unknown):readonly Record<string,unknown>[]{
 if(value===null||typeof value!=="object"||!("pixelCases" in value)||!Array.isArray(value.pixelCases))throw Error("Surface pixel cases required");
 return value.pixelCases.map(row=>{if(row===null||typeof row!=="object"||Array.isArray(row))throw Error("Surface pixel case record required");return row;});
}
