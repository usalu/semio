/** 🔤️ A set of distinct texts kept as UTF-8 bytes in one buffer. */
export interface TextArenaV1{add(text:string):number;id(text:string):number|undefined;count():number;bytes(number:number):Buffer;sorted():Uint32Array;size():number}
/** 🔤️ Creates an arena; `validate` is called once for every new text. */
export function createTextArenaV1(options?:{validate?:(text:string)=>void}):TextArenaV1;
