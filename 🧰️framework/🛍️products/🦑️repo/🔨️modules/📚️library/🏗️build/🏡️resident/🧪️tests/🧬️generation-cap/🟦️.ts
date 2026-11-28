/** 🧬️ Contract: the live service generations of one checkout are read from the descriptors of the runtime directory (live process, same identity, not stopping), newest first; other checkouts, dead processes, stopping services and damaged descriptors are not counted. The client side of the cap is in the settle rule.
 * @see ../../🏛️service/🌐️host/🟨️.mjs
 * @see ../../../🧭️invocation/🕰️settled/🧪️tests/🟦️.ts */
import {expect,test} from "bun:test";
import {liveGenerationsV1} from "../../🏛️service/🌐️host/🟨️.mjs";

const identity={root:"/r",state:"/s"},descriptor=(tag:string,over:Record<string,unknown>={})=>({name:`host-${tag.repeat(12)}.json`,body:{pid:Number(tag.charCodeAt(0)),startMs:1,startedAt:Number(tag.charCodeAt(0)),implementation:tag.repeat(64),identity,busy:0,...over}});
const run=(rows:{name:string;body:unknown}[],dead:number[]=[])=>liveGenerationsV1({directory:"/d",identity,alive:pid=>!dead.includes(pid),list:()=>[...rows.map(row=>row.name),"key","host-aaaaaaaaaaaa.log"],read:name=>{const row=rows.find(item=>item.name===name);if(!row)throw Error("x");return row.body as never;}});

test("live generations of this checkout come newest first with their busy count",()=>{
 const found=run([descriptor("a",{busy:2}),descriptor("c"),descriptor("b")]);
 expect(found.map(row=>row.implementation[0])).toEqual(["c","b","a"]);expect(found.find(row=>row.implementation[0]==="a")?.busy).toBe(2);
});

test("a dead process, a stopping service, another checkout and a damaged descriptor are not generations",()=>{
 const rows=[descriptor("a"),descriptor("b",{stopping:true}),descriptor("c",{identity:{root:"/other",state:"/s"}}),descriptor("d"),{name:"host-eeeeeeeeeeee.json",body:null},descriptor("f",{implementation:7})];
 expect(run(rows,["d".charCodeAt(0)]).map(row=>row.implementation[0])).toEqual(["a"]);
 expect(liveGenerationsV1({directory:"/d",identity,list:()=>{throw Error("gone");}})).toEqual([]);
});
