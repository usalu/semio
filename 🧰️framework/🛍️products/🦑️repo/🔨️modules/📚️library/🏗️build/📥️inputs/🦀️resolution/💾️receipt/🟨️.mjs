import{createHash}from"node:crypto";
import{mkdirSync,readFileSync,renameSync,writeFileSync,lstatSync}from"node:fs";
import{join}from"node:path";
import{brotliCompressSync,brotliDecompressSync,constants as zlib}from"node:zlib";
import{canonicalJsonV1,readCargoLeafV1}from"../🟦️.ts";
/** 📇️ The leaf name of one crate: `rr/` and the first 24 hex digits of the digest of its directory. */
export const cargoReceiptNameV1=crate=>`rr/${createHash("sha256").update(crate).digest("hex").slice(0,24)}.json`;
/** 📇️ Seals a receipt with the digest of its canonical form and compresses it. */
export function encodeCargoReceiptV1(leaf){const{check:_,...bare}=leaf,sealed={...bare,check:createHash("sha256").update(canonicalJsonV1(bare)).digest("hex")};return brotliCompressSync(Buffer.from(JSON.stringify(sealed),"utf8"),{params:{[zlib.BROTLI_PARAM_QUALITY]:5,[zlib.BROTLI_PARAM_SIZE_HINT]:2048}});}
/** 📇️ Opens a receipt: undecodable, foreign, unsealed or schema-violating bytes are no receipt. */
export function decodeCargoReceiptV1(bytes,crate){try{const value=JSON.parse(brotliDecompressSync(bytes).toString("utf8")),leaf=readCargoLeafV1(value),{check:_,...bare}=leaf;return leaf.crate===crate&&leaf.check===createHash("sha256").update(canonicalJsonV1(bare)).digest("hex")?leaf:undefined;}catch{return undefined;}}
/** 💾️ A reader over the receipts of a graph data directory: a missing, corrupt or foreign leaf answers undefined. */
export function createCargoReceiptReaderV1(directory){return{read:crate=>{try{return decodeCargoReceiptV1(readFileSync(join(directory,cargoReceiptNameV1(crate))),crate);}catch{return undefined;}}};}
/** 💾️ A reader and writer: every receipt is published through the graph's optional-cache publication (`publish`), which refuses a write outside the graph writer's custody and an exhausted quota. */
export function createCargoReceiptWriterV1(directory,publish){const reader=createCargoReceiptReaderV1(directory);return{...reader,write:leaf=>{const target=join(directory,cargoReceiptNameV1(leaf.crate)),bytes=encodeCargoReceiptV1(leaf);let previous=0;try{previous=lstatSync(target).size;}catch{}publish(target,{peakBytes:bytes.byteLength,retainedDeltaBytes:Math.min(bytes.byteLength,bytes.byteLength-previous)},()=>{const temporary=`${target}.${process.pid}.${Date.now()}.tmp`;mkdirSync(join(directory,"rr"),{recursive:true});writeFileSync(temporary,bytes,{flag:"wx"});renameSync(temporary,target);});}};}
