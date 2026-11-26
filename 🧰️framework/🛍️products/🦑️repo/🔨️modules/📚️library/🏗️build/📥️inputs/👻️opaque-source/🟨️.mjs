import descriptorSchema from"../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json"with{type:"json"};
import retirementSchema from"../../../⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🧬️schema/🔣️.json"with{type:"json"};
/** 🫥️ The reserved name prefixes and digest width that mark preservation payloads and retirement journals as opaque to authored Source; the two schema files are their single source. */
export const SOURCE_OPACITY_PROTOCOL_V1=Object.freeze({prefixes:Object.freeze([descriptorSchema.definitions.RelativePreservationProtocol.const.capsulePrefix,retirementSchema.definitions.Policy.const.markerPrefix]),digestHex:descriptorSchema.definitions.RelativePreservationProtocol.const.capsuleDigestHex});
/** 🫥️ Keeps schema-reserved preservation payloads and retirement journals outside authored Source. */
export function isOpaqueSourcePathV1(path){
  return path.split(/[\\/]/u).some(name=>SOURCE_OPACITY_PROTOCOL_V1.prefixes.some(prefix=>name.startsWith(prefix)&&name.length===prefix.length+SOURCE_OPACITY_PROTOCOL_V1.digestHex&&/^[a-f0-9]+$/u.test(name.slice(prefix.length))));
}
/** 🚫️ Emits equivalent glob exclusions for reserved files and opaque directory payloads. */
export function opaqueSourceInputExclusionsV1(root){
  return SOURCE_OPACITY_PROTOCOL_V1.prefixes.flatMap(prefix=>{
    const name=prefix+"[a-f0-9]".repeat(SOURCE_OPACITY_PROTOCOL_V1.digestHex);
    return [`!${root}/**/${name}`,`!${root}/**/${name}/**/*`];
  });
}
