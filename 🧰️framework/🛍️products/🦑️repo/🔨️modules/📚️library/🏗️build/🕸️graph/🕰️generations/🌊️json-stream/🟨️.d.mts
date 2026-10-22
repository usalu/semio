/** 🌊️ Writes a JSON value as the exact text of `JSON.stringify(value, null, indent)` in bounded pieces; resolves with the number of pieces written. */
export function writeJsonStreamV1(value:unknown,write:(text:string)=>void|Promise<void>,options?:{indent?:number;chunkCharacters?:number;signal?:AbortSignal}):Promise<number>;
