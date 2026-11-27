import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {DISK_RECOVERY_POLICY_V1,DISK_WRITERS_V1,collectInOrderV1,requireDiskWritersV1,type DiskWriterV1} from "../../🟦️.ts";
import {realDiskWritersV1} from "../../🔌️real/🟦️.ts";

const facts={partition:"/p",totalBytes:100,freeBytes:10};
const stub=(rows=requireDiskWritersV1(DISK_WRITERS_V1),calls:string[]=[]):DiskWriterV1[]=>rows.map(row=>({row,root:"",measure:()=>0,collect:async()=>{calls.push(row.id);return 0;}}));

test("the Bun package cache is registered as the last derivable writer, for bytes and for inodes",()=>{
 const rows=requireDiskWritersV1(DISK_WRITERS_V1),cache=rows.find(row=>row.id==="bun-cache")!;
 expect(cache.class).toBe("derivable");
 expect(cache.priority).toBe(Math.max(...rows.filter(row=>row.collector!=="manual").map(row=>row.priority)));
 expect(cache.inodeRank).toBeUndefined();
 expect(cache.bound.en.length).toBeGreaterThan(0);expect(cache.bound.de.length).toBeGreaterThan(0);
 expect(DISK_RECOVERY_POLICY_V1.environmentBunCache).toBe("SEMIO_DISK_RECOVERY_BUN_CACHE");
});
test("an inode sweep frees the dependency generations first and the Bun package cache last",async()=>{
 const calls:string[]=[];
 await collectInOrderV1({facts:()=>facts,current:facts,targetFreeBytes:1e9,writers:stub(requireDiskWritersV1(DISK_WRITERS_V1),calls),language:"en",sweep:true,scarce:"inodes",steps:[]});
 const ranked=calls.filter(id=>["dependency-generations","cargo-incremental","tool-generations","nx-native-data","closure-layers","bun-cache"].includes(id));
 expect(ranked.slice(0,5)).toEqual(["dependency-generations","cargo-incremental","tool-generations","nx-native-data","closure-layers"]);
 expect(ranked[0]).toBe("dependency-generations");
 expect(ranked.at(-1)).toBe("bun-cache");
 expect(calls.at(-1)).toBe("bun-cache");
});
test("the real writer measures the cache folder of the dependency store and a dry run removes nothing",async()=>{
 const workspace=mkdtempSync(join(tmpdir(),"semio-bun-cache-"));
 try{
  mkdirSync(join(workspace,".cache","b","d","m","bun-cache","pkg"),{recursive:true});writeFileSync(join(workspace,".cache","b","d","m","bun-cache","pkg","f"),"12345");
  const writer=realDiskWritersV1({workspace,home:workspace,tmp:workspace,uid:0,environment:{},dryRun:true}).find(item=>item.row.id==="bun-cache")!;
  expect(writer.root).toBe(join(workspace,".cache","b","d","m","bun-cache"));
  expect(await writer.measure()).toBeGreaterThanOrEqual(5);
  expect(await writer.collect!({needBytes:1})).toBe(0);
  expect(await writer.measure()).toBeGreaterThanOrEqual(5);
 }finally{rmSync(workspace,{recursive:true,force:true});}
});
