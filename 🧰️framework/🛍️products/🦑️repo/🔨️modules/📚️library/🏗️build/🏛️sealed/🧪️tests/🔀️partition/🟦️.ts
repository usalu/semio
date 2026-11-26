import {test,expect} from "bun:test";
import fixture from "../🧫️fixtures/🔀️partition.json" with {type:"json"};
import {sealedTreesCoherentV1} from "../../🔀️partition/🟦️.ts";
test("authenticated partitions agree on every shared source",()=>{for(const row of fixture.cases){const tree=(files:Readonly<Record<string,string|undefined>>)=>({files:Object.entries(files).flatMap(([path,digest])=>digest===undefined?[]:[[path,digest,"stamp"] as const]),directories:[],listings:[],absent:[]});expect(sealedTreesCoherentV1(tree(row.door),tree(row.kernel))).toBe(row.coherent);}});
