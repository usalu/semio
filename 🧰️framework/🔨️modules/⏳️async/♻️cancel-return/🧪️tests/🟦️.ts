import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import jsonPatch from "fast-json-patch";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import {CancelReturnCursor} from "../🟦️.ts";
test("original cancellation node and backing returns obey every denied axis and independent custody oracle",()=>{
 expect(new Ajv({strict:false}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.cases){const cursor=new CancelReturnCursor(row.aliases,row.nodes,row.waiters,row.emptyCapacity,64,24);let released=0;let oracle={aliases:row.aliases,node:true,body:true,waiters:row.waiters};
  while(cursor.state!=="empty"){
   const demand=cursor.demands();if(demand===null){expect(row.outcome).toBe("retained");expect(cursor.advance(1,512,512,1)).toEqual({release:0,retained:true});break;}
   for(const grant of [[0,demand.copy,demand.release,1],[1,Math.max(0,demand.copy-1),demand.release,1],[1,demand.copy,Math.max(0,demand.release-1),1],[1,demand.copy,demand.release,0]]){if(grant[0]&&grant[1]===demand.copy&&grant[2]===demand.release&&grant[3])continue;const state=cursor.state;expect(cursor.advance(grant[0]!,grant[1]!,grant[2]!,grant[3]!)).toEqual({release:0,retained:true});expect(cursor.state).toBe(state);}
   released+=cursor.advance(1,demand.copy,demand.release,1).release;
  }
  if(row.outcome==="retained"){expect(cursor.state).toBe("body");expect(released).toBe(64);}
  else if(row.outcome==="alias-returned"){oracle=jsonPatch.applyPatch(oracle,[{op:"replace",path:"/aliases",value:row.aliases-1}]).newDocument;expect(oracle.node).toBe(true);expect(released).toBe(0);}
  else{oracle=jsonPatch.applyPatch(oracle,[{op:"remove",path:"/node"},{op:"remove",path:"/body"}]).newDocument;expect(released).toBe(row.nodes*64+row.emptyCapacity*24);expect("node" in oracle).toBe(false);}
 }
});
test("cancellation raw Arc remains sealed and actual owner transfers before wake refusal",()=>{
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8"),handle=readFileSync(join(import.meta.dir,"../🔐️handle/🦀️.rs"),"utf8");
 expect(handle).toContain("Arc::into_inner(self.inner)");expect(handle).not.toContain("Weak<");expect(handle).not.toContain("Arc::downgrade");expect(handle).not.toContain("weak_count");expect(handle).not.toContain("strong_count");expect(source).toContain("self.body = source.0.return_original()");expect(source).toContain("original cancellation node retains a registered wake owner");expect(source).not.toContain("waker.wake()");expect(source).not.toContain("waiters.pop()");
});
