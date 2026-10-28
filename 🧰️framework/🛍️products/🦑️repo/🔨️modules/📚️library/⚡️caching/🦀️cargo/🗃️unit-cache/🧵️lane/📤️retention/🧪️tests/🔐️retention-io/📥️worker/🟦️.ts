/** 📥️ One process of the retention test: `lane <root> <shared> <index> <count> <trim>` takes a pooled lane, writes `count` receipts of its own, one receipt every worker writes with identical bytes and one every worker writes with its own bytes under one name, releases through the retention rule and prints what the pass did. `trim <root> <rounds>` trims the pool repeatedly beside the workers. */
import {mkdirSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {acquirePooledLaneV1,trimPoolV1} from "../../../../🏊️pool/🟦️.ts";
import {retainLaneOutputsV1,withLaneRetentionV1} from "../../../🟦️.ts";

const [mode,root,...rest]=process.argv.slice(2) as [string,string,...string[]];
const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));

if(mode==="lane"){
 const [shared,index,count]=[rest[0]!,Number(rest[1]),Number(rest[2])],lane=await acquirePooledLaneV1(join(root,"u"),{crates:["w"+index],quotaBytes:Number.MAX_SAFE_INTEGER});
 const ledger=join(lane.paths.build,"semio-cargo-provenance");mkdirSync(ledger,{recursive:true});
 let summary:Awaited<ReturnType<typeof retainLaneOutputsV1>>|undefined;
 await withLaneRetentionV1({lane,shared,language:"en"},async()=>{
  for(let item=0;item<count;item++)writeFileSync(join(ledger,`cargo-unit-provenance-w${index}-${item}.json`),JSON.stringify({worker:index,item,pad:"x".repeat(2000)}));
  writeFileSync(join(ledger,"cargo-unit-provenance-common.json"),JSON.stringify({common:true}));
  writeFileSync(join(ledger,"cargo-unit-provenance-race.json"),JSON.stringify({winner:index}));
  await sleep(5+Math.floor(Math.random()*15));
 });
 console.log(JSON.stringify({index,lane:lane.paths.id}));
}else if(mode==="trim"){
 const rounds=Number(rest[0]);
 for(let round=0;round<rounds;round++){trimPoolV1(join(root,"u"),{maximumLanes:0,maximumLiveLanes:32,quotaBytes:0},Date.now());await sleep(3);}
}
