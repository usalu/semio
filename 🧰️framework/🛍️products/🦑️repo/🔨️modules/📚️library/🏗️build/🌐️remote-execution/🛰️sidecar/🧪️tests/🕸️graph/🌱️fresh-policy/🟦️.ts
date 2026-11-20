/** 🌱️ Law: the whole read the graph server runs for itself is the hours of inference it was started for, so it names the graph policy `fresh` whatever its environment says; a whole question without that policy is answered from a sealed generation or refused (`GENERATION_NONE_SEALED`), so a read without it ends in seconds with the refusal and no machine ever builds its first generation (the read of 2026-11-18 15:02 and 15:12 UTC).
 * @see ../../../🕸️graph/🔭️read/🟦️.ts `startGraphReadV1` */
import{afterAll,expect,test}from"bun:test";
import{chmodSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{wantedServerPolicyV1}from"../../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import{inferGraphReadV1}from"../../../🕸️graph/🔭️read/🟦️.ts";

const made:string[]=[];
afterAll(()=>{for(const path of made)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`graph-read-${label}-`));made.push(path);return path;};
const policy={...wantedServerPolicyV1,inferStallMilliseconds:600000,watchMilliseconds:40,readBeatMilliseconds:50,readPollMilliseconds:40};
const run=async(environment:Record<string,string>,reason:"request"|"refresh")=>{
 const dir=temp("policy"),repo=join(dir,"repo"),seen=join(dir,"seen");
 writeFileSync(repo,`#!/bin/sh\necho "$SEMIO_BUILD_GRAPH_POLICY|$*" > "${seen}"\nexit 0\n`);chmodSync(repo,0o755);
 const result=await inferGraphReadV1({command:repo,cwd:tmpdir(),environment,reason,signal:undefined,language:"en",directory:join(dir,"read"),policy});
 return{result,seen:readFileSync(seen,"utf8").trim()};
};

test("a refresh read names the policy fresh, with an empty environment and against an inherited current",async()=>{
 for(const environment of[{},{SEMIO_BUILD_GRAPH_POLICY:"current"}]){
  const{result,seen}=await run(environment,"refresh");
  expect(result.ok).toBe(true);expect(seen).toBe("fresh|show projects");
 }
},60000);

test("a read that answers a request names it too",async()=>{
 const{result,seen}=await run({},"request");
 expect(result.ok).toBe(true);expect(seen).toBe("fresh|show projects");
},60000);
