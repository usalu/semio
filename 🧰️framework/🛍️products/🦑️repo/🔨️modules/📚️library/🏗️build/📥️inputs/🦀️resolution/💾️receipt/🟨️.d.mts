import type{CargoLeafV1,CargoReceiptStoreV1}from"../🟦️.ts";
/** 📇️ The leaf name of one crate. */
export function cargoReceiptNameV1(crate:string):string;
/** 📇️ Seals a receipt with its digest and compresses it. */
export function encodeCargoReceiptV1(leaf:CargoLeafV1):Buffer;
/** 📇️ Opens a receipt; anything undecodable, foreign or unsealed is undefined. */
export function decodeCargoReceiptV1(bytes:Uint8Array,crate:string):CargoLeafV1|undefined;
/** 💾️ A reader over the receipts of a graph data directory. */
export function createCargoReceiptReaderV1(directory:string):CargoReceiptStoreV1;
/** 💾️ A reader and writer publishing through the graph's optional-cache publication. */
export function createCargoReceiptWriterV1(directory:string,publish:(path:string,plan:{peakBytes:number;retainedDeltaBytes:number},write:()=>void)=>boolean):CargoReceiptStoreV1&{write:(leaf:CargoLeafV1)=>void};
