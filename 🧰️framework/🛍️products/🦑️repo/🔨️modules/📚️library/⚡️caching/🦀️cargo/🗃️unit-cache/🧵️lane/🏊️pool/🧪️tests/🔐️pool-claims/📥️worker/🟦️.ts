/** 📥️ One process of the pool claim test: `work <root> <rounds> <quota> <crates…>` takes a lane, proves it is alone in it, holds it briefly and releases it; `die <root> <crates…>` takes a lane, announces it and waits to be killed; `trim <root> <rounds> <maximumLanes> <quota>` trims the pool repeatedly. */
import {appendFileSync,existsSync,mkdirSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {acquirePooledLaneV1,trimPoolV1} from "../../../🟦️.ts";

const [mode,root,...rest]=process.argv.slice(2) as [string,string,...string[]];
const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const log=(row:object)=>appendFileSync(join(root,"events.jsonl"),JSON.stringify({pid:process.pid,...row})+"\n");
mkdirSync(root,{recursive:true});

if(mode==="work"){
 const rounds=Number(rest[0]),quotaBytes=Number(rest[1]),crates=rest.slice(2);
 for(let round=0;round<rounds;round++){
  const lane=await acquirePooledLaneV1(join(root,"u"),{crates,quotaBytes});
  const mark=join(lane.paths.base,"owner-"+process.pid);
  try{
   const others=readdirSync(lane.paths.base).filter(name=>name.startsWith("owner-"));
   writeFileSync(mark,String(process.pid));
   await sleep(2+Math.floor(Math.random()*6));
   const after=readdirSync(lane.paths.base).filter(name=>name.startsWith("owner-"));
   log({kind:"held",lane:lane.paths.id,choice:lane.choice,alone:others.length===0&&after.length===1,present:existsSync(lane.paths.target)});
  }finally{rmSync(mark,{force:true});lane.release();}
 }
}else if(mode==="die"){
 const lane=await acquirePooledLaneV1(join(root,"u"),{crates:rest,quotaBytes:Number.MAX_SAFE_INTEGER});
 log({kind:"ready",lane:lane.paths.id});
 console.log("ready");
 await sleep(60000);
}else if(mode==="trim"){
 const rounds=Number(rest[0]),maximumLanes=Number(rest[1]),quotaBytes=Number(rest[2]);
 for(let round=0;round<rounds;round++){trimPoolV1(join(root,"u"),{maximumLanes,maximumLiveLanes:32,quotaBytes},Date.now());await sleep(3);}
}
