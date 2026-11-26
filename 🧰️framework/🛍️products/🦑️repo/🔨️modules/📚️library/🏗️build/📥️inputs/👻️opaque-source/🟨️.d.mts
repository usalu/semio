/** 🫥️ The reserved name prefixes and digest width that mark preservation payloads and retirement journals as opaque to authored Source. */
export const SOURCE_OPACITY_PROTOCOL_V1:Readonly<{prefixes:readonly string[];digestHex:number}>;
/** 🫥️ Keeps schema-reserved preservation payloads and retirement journals outside authored Source. */
export function isOpaqueSourcePathV1(path:string):boolean;
/** 🚫️ Emits equivalent glob exclusions for reserved files and opaque directory payloads. */
export function opaqueSourceInputExclusionsV1(root:string):string[];
