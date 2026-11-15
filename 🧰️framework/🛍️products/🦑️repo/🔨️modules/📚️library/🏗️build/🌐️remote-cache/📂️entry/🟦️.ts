import {readFile} from "node:fs/promises";

/** 📏️ Values up to this size are read whole before the response head is decided (4 MiB). */
export const REMOTE_CACHE_EAGER_BYTES_V1=4*1048576;

/** 📂️ Reads a stored value before the response head is decided, so a collector that removes the file afterwards cannot tear the answer (the reader gets the whole value) and a value removed before the read is a miss (`undefined`), never an error or a part. Values larger than the limit answer `"lazy"`: the caller streams them from the path and a value removed before the stream opens ends the transfer, which the client sees as a failed read and handles as a miss; nothing partial is ever taken for a hit. */
export async function readRemoteCacheEntryV1(path:string,size:number,limit:number=REMOTE_CACHE_EAGER_BYTES_V1):Promise<Uint8Array<ArrayBuffer>|"lazy"|undefined>{
 if(size===0)return new Uint8Array(0);
 if(size>limit)return "lazy";
 try{return await readFile(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}
}
