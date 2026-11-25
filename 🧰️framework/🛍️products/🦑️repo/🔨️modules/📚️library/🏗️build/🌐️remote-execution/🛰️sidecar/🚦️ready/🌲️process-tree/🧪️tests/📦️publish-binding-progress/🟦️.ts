import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {READY_TREE_POLICY_V1 as P,readyDeadlineV1,readyProgressKeyV1} from "../../🟦️.ts";

/** 📦️ Contract: the publication counter of the dependency producer is progress, so a Windows stage that publishes bindings for hours is extended up to the progress ceiling and stopped only by a stall, never by the clock alone. */
const lines=(n:number)=>["[script] {\"owner\":\"repository:build\",\"stage\":\"running\",\"activity\":{\"resource\":\"dependencies:publish-binding:"+n+"\",\"mode\":\"shared\",\"elapsedMs\":0}}"];

test("a moved publication counter is another progress key, a repeated one the same",()=>{
 expect(readyProgressKeyV1(lines(152))).toBe("dependencies:publish-binding:152");
 expect(readyProgressKeyV1(lines(160))).not.toBe(readyProgressKeyV1(lines(152)));
 expect(readyProgressKeyV1(lines(160))).toBe(readyProgressKeyV1(lines(160)));
});

test("a stage past its hard cap keeps running while the counter moved and stops at the ceiling or a stall",()=>{
 const soft=3600000,past=soft*5;
 expect(readyDeadlineV1({stage:"dependencies",elapsedMs:past,softMs:soft,sinceProgressMs:100000})).toBe("extended");
 expect(readyDeadlineV1({stage:"dependencies",elapsedMs:past,softMs:soft,sinceProgressMs:P.stallMilliseconds+1})).toBe("timeout");
 expect(readyDeadlineV1({stage:"dependencies",elapsedMs:soft*(P.progressCeilingMultiplier+1),softMs:soft,sinceProgressMs:1})).toBe("timeout");
});

test("an independent Python regular expression agrees on every progress pattern match",()=>{
 const code="import json,re,sys\npatterns=json.loads(sys.argv[1]);lines=json.loads(sys.argv[2])\nprint(json.dumps([[bool(re.search(p,l)) for p in patterns] for l in lines]))";
 const samples=[...lines(8),...lines(1500),"dependencies:publish-binding:0","dependencies:publish-binding:","dependencies:selected-postorder:7"];
 const python=spawnSync("python3",["-I","-c",code,JSON.stringify(P.progressPatterns),JSON.stringify(samples)],{encoding:"utf8"});
 expect(python.status).toBe(0);
 const expected=samples.map(line=>P.progressPatterns.map(pattern=>new RegExp(pattern,"u").test(line)));
 expect(JSON.parse(python.stdout)).toEqual(expected);
 expect(expected.map(row=>row.some(Boolean))).toEqual([true,true,false,false,true]);
});
