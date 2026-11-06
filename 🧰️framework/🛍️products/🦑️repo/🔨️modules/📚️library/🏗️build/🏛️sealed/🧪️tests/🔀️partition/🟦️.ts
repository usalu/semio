import {test,expect} from "bun:test";
import fixture from "../🧫️fixtures/🔀️partition.json" with {type:"json"};
import {sealedTreesCoherentV1} from "../../🔀️partition/🟦️.ts";
test("authenticated partitions agree on every shared source",()=>{for(const row of fixture.cases){const tree=(files:Record<string,string>)=>({files:Object.entries(files).map(([path,digest])=>[path,digest,"stamp"] as const),directories:[],listings:[],absent:[]});expect(sealedTreesCoherentV1(tree(row.door),tree(row.kernel))).toBe(row.coherent);}});
