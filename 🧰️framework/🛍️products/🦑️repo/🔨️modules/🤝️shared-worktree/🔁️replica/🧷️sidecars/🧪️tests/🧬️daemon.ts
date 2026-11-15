import {writeFileSync} from "node:fs";
import {join} from "node:path";
import {createSidecarsV1} from "../🟦️.ts";

const worktree=process.argv[2]!,block=process.argv[3]==="block",lease=Number(process.env.SIDECAR_TEST_LEASE_MS);
const supervisor=createSidecarsV1({settings:{enabled:true,startAfterSeconds:0,restart:{initialMilliseconds:60,maximumMilliseconds:240,stableMilliseconds:60_000},graceSeconds:1,children:[{name:"role",script:process.argv[4]!,arguments:["role"],roles:["member","server"],requires:"none",enabled:true}]},facts:{worktree,storeRoot:join(worktree,"store"),originId:"g1-m03",groupId:"g1",platform:"macos"},peers:async()=>3,environment:process.env,...lease>0?{protocol:{leaseMilliseconds:lease}}:{}});
const timer=setInterval(()=>{supervisor.tick(true);},20);
const report=setInterval(()=>{writeFileSync(join(worktree,`report-${process.pid}.json`),JSON.stringify(supervisor.report()));},100);
setTimeout(()=>{
 writeFileSync(join(worktree,"daemon-state"),block?"blocked":"running");
 if(block){clearInterval(timer);clearInterval(report);const end=Date.now()+120_000;while(Date.now()<end){}}
},1500);
setInterval(()=>{},1<<30);
