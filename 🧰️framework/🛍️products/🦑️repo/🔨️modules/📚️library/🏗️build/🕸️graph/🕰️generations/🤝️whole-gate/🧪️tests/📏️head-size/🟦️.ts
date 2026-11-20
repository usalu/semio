/** 📏️ Contract: the head of the generation of the whole workspace fits the action result the fleet cache server accepts, with the document and leaf counts of the real workspace (4785 owners, 6293 closure leaves in the store of this machine on 2026-11-18) and room for three times as many leaves.
 * @see ../../🟦️.ts */
import {createHash} from "node:crypto";
import {expect,test} from "bun:test";
import policy from "../../../../../🔣️policy.json" with {type:"json"};
import exchange from "../../../🌐️remote-exchange/🧬️schema/🔣️.json" with {type:"json"};
import {REAPI_V1,decodeProtobufV1,encodeProtobufV1} from "../../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";

const sha=(text:string)=>createHash("sha256").update(text).digest("hex"),limit=policy.remoteCache.server.maximumActionResultBytes,limits=exchange.definitions.Policy.const;
const head=(owners:number,leaves:number)=>{
 const entries=[{path:`m/${sha("m")}.json`},{path:`x/${sha("x")}.facts.br`},{path:`s/${sha("s").slice(0,16)}.snapshot.br`},...Array.from({length:owners+1},(_,index)=>({path:`f/${sha("f"+index)}.json.br`})),...Array.from({length:leaves},(_,index)=>({path:`c/${index%7===0?"l":"i"}/${sha("c"+index)}.json`}))];
 const rows=entries.map(row=>({path:row.path,digest:{hash:row.path.slice(-80,-16).replace(/[^0-9a-f]/gu,"").padEnd(64,"0").slice(0,64),size_bytes:123456}}));
 return encodeProtobufV1(REAPI_V1.actionResult,{output_files:rows,stdout_raw:Buffer.from(JSON.stringify({version:1,owners,stale:0}))});
};

test("the head of 4785 owners and 6293 leaves is far below the action result limit of the server",()=>{
 const bytes=head(4785,6293);
 expect(bytes.length).toBeLessThan(limit/2);
 expect(decodeProtobufV1(REAPI_V1.actionResult,bytes).output_files.length).toBe(3+4786+6293);
});

test("three times the leaves still fit; the declared document and leaf bounds are looser than the server limit, so a head at both bounds is refused by the server (a typed upload failure that ends in a local read)",()=>{
 expect(head(4785,3*6293).length).toBeLessThan(limit);
 expect(limits.maximumDocuments).toBeGreaterThan(4785*2);expect(limits.maximumClosures).toBeGreaterThan(6293*3);
 expect(head(limits.maximumDocuments,limits.maximumClosures).length).toBeGreaterThan(limit);
});
