import{createClosureSettlerV1,CLOSURE_SETTLE_V1}from"../../../../🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";
import{createNodeStoreV1}from"../../../../🧮️derived-nodes/💾️store/🟨️.mjs";
import{derivedOwnerNameV1,suspectRowsV1}from"../../🟨️.mjs";
import{createFactIndexV1}from"../../../🟨️.mjs";

/** 🪓️ Settles `count` closures in chunks of `chunk` over the store at `root` and kills its own process at the derivation `killAt`, the way the memory guard of a worker does: nothing after the kill runs, so only what the chunks flushed is durable. */
const[root,count,chunk,killAt]=process.argv.slice(2),rowOf=name=>({kind:"derived",path:name,fact:"old"});
const names=Array.from({length:Number(count)},(_,at)=>"q"+String(at).padStart(4,"0"));
const index=createFactIndexV1(new Map([["holder",names.map(rowOf)],...names.map(name=>[derivedOwnerNameV1(rowOf(name)),[rowOf(name),{kind:"file",path:"old-"+name,fact:"h"}]])]));
let derived=0;
const settler=createClosureSettlerV1({store:createNodeStoreV1({root}),limits:{...CLOSURE_SETTLE_V1,settleChunkClosures:Number(chunk),deriveConcurrency:1}});
await settler.settle({pseudoRows:suspectRowsV1(index,new Set(names.map(name=>derivedOwnerNameV1(rowOf(name))))),derive:async row=>{if(++derived===Number(killAt))process.kill(process.pid,"SIGKILL");return{fact:"old",parts:[[{kind:"file",path:"own-"+row.path,fact:"h"}]]};},hold:async()=>true});
