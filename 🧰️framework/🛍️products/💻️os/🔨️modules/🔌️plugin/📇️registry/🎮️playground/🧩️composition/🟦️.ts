/** 🧭️ Admits one bounded canonical workspace path selected by its playground owner. */
export function playgroundCompositionPathV1(value:unknown):string{
 if(typeof value!=="string"||value.length>8192||!value||[...value].length>4096||value!==value.normalize("NFC")||/[\\:\x00-\x1f]/u.test(value)||value.split("/").some(part=>!part||part==="."||part===".."))throw Error("Invalid playground composition path");
 return value;
}

/** 📜️ Reads one canonical string declaration from its owning playground metadata block. */
export function declaredPlaygroundCompositionPathV1(block:string):string|undefined{
 const rows=block.split(/\r?\n/u).map(line=>line.trim()).filter(line=>/^compositionConfigPath\s*=/u.test(line));
 if(!rows.length)return undefined;if(rows.length!==1)throw Error("Repeated playground composition declaration");
 const literal=rows[0]!.slice(rows[0]!.indexOf("=")+1).trim().match(/^("(?:[^"\\]|\\.)*")\s*(?:#.*)?$/u)?.[1];
 if(literal===undefined||literal.length>32768)throw Error("Invalid playground composition declaration");
 return playgroundCompositionPathV1(JSON.parse(literal));
}
