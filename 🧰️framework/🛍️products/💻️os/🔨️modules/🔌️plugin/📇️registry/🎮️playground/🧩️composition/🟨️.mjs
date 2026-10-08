/** 🧭️ Admits one bounded canonical workspace path selected by its playground owner. */
export function playgroundCompositionPathV1(value){
 if(typeof value!=="string"||value.length>8192||!value||[...value].length>4096||value!==value.normalize("NFC")||/[\\:\x00-\x1f]/u.test(value)||value.split("/").some(part=>!part||part==="."||part===".."))throw Error("Invalid playground composition path");
 return value;
}
