import{test,expect}from"bun:test";
import storage from"../../🧬️schema/🔣️.json";
import sources from"../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/📤️publication/📥️sources/🧬️schema/🔣️.json";

const protocol=storage.definitions.Protocol.const,limits=Object.fromEntries(Object.entries(sources.properties).map(([name,row])=>[name,(row as{const?:unknown}).const])) as Record<string,number>;

/** 📏️ The repository artifact extent covers the largest retained compiler closure a publication carries beside its executable. */
test("the artifact output extent holds the retained compilation, its index and its receipt",()=>{
 expect(protocol.maximumOutputBytes).toBeGreaterThanOrEqual(limits.maximumRetainedCompilationBytes+limits.maximumRetainedIndexBytes);
 expect(protocol.maximumFiles).toBeGreaterThanOrEqual(limits.maximumFiles);
});

test("the artifact manifest extent lists every file name of a full retained compilation",()=>{
 const names=Array.from({length:protocol.maximumFiles},(_,at)=>"compilation/blobs/"+at.toString(16).padStart(64,"0"));
 expect(Buffer.byteLength(JSON.stringify({version:1,owner:"x".repeat(256),files:names.sort()}))).toBeLessThanOrEqual(protocol.maximumManifestBytes);
});
