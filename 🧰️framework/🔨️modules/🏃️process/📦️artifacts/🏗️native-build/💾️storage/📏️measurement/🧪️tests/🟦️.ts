import {expect,test} from "bun:test";
import Ajv from "ajv";
import glob from "fast-glob";
import {mkdtempSync,mkdirSync,writeFileSync,rmSync,lstatSync,symlinkSync} from "node:fs";
import {join,dirname} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {measureOwnedArtifactTreeV1} from "../🟦️.ts";
test("bounded artifact measurement agrees with an independent file inventory/stat oracle",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original measurement output authority required");const root=mkdtempSync(join(artifacts,"am-")),control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>{},yieldContinuation:async()=>{}};
 try{for(const row of fixture.files){const file=join(root,row.path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,Buffer.alloc(row.bytes));}const result=await measureOwnedArtifactTreeV1({root,maximumBytes:1048576,receiptBytes:fixture.receiptBytes,copies:fixture.copies as 1},control),files=await glob("**/*",{cwd:root,dot:true,onlyFiles:true,followSymbolicLinks:false}),stats=files.map(path=>lstatSync(join(root,path)));expect(result.logicalBytes).toBe(stats.reduce((sum,row)=>sum+row.size,0));expect(result.allocatedBytes).toBe(stats.reduce((sum,row)=>sum+row.blocks*512,0)+lstatSync(root).blocks*512+lstatSync(join(root,"nested")).blocks*512);expect(result.files).toBe(fixture.filesCount);expect(result.directories).toBe(fixture.directories);expect(result.reservationBytes).toBe(fixture.reservationBytes);expect(new Ajv({strict:true}).compile(schema)(result)).toBe(true);await expect(measureOwnedArtifactTreeV1({root,maximumBytes:result.reservationBytes-1,receiptBytes:fixture.receiptBytes,copies:1},control)).rejects.toThrow("capacity");symlinkSync(join(root,"one"),join(root,"alias"));await expect(measureOwnedArtifactTreeV1({root,maximumBytes:1048576,receiptBytes:0,copies:1},control)).rejects.toThrow("physical");console.log("[DEBUG] Artifact measurement fast-glob/stat oracle exact; cap-plus-one and symlink refused before copy");}finally{rmSync(root,{recursive:true,force:true});}
});
