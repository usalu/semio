import {test,expect} from "bun:test";
import {StoredZipStream} from "../🟦️.ts";
import {decodeEntries} from "./🔮️oracle/🟦️.ts";
import fixtures from "../🧪️fixtures/🔣️.json";
test("stored descriptors preserve Unicode and empty members through independent ZIP",()=>{
    for(const fixture of fixtures.cases){const stream=new StoredZipStream(),chunks:Uint8Array[]=[];const entriesToWrite=[...fixture.known.map(entry=>({...entry,known:true,chunkBytes:4096})),...fixture.streamed.map(entry=>({...entry,known:false}))];for(const entry of entriesToWrite){const bytes=new TextEncoder().encode(entry.text);if(entry.known)chunks.push(stream.knownEntry(entry.name,bytes));else{chunks.push(stream.beginEntry(entry.name));for(let index=0;index<bytes.length;index+=entry.chunkBytes??4096)chunks.push(stream.writeChunk(bytes.subarray(index,index+(entry.chunkBytes??4096))));chunks.push(stream.endEntry());}}for(let chunk=stream.finishOne();chunk;chunk=stream.finishOne())chunks.push(chunk);expect(chunks.every(chunk=>chunk.length<=StoredZipStream.maximumChunkBytes)).toBe(true);const bytes=new Uint8Array(chunks.reduce((total,chunk)=>total+chunk.length,0));let offset=0;for(const chunk of chunks){bytes.set(chunk,offset);offset+=chunk.length;}const entries=decodeEntries(bytes);for(const entry of entriesToWrite)expect(new TextDecoder().decode(entries[entry.name])).toBe(entry.text);stream.close();}
});
