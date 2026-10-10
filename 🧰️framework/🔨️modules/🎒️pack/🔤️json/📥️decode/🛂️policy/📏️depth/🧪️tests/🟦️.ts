import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {existsSync} from "node:fs";
import {resolve} from "node:path";
import {createRequire} from "node:module";
import fixture from "../🧫️fixtures/🔣️.json";
import contract from "../../../../../../🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json";
import ordered from "../../../../../../🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json";
import originalLimits from "../../../../🧫️fixtures/🫳️read-limits.json";

test("complete source container limits match an independent relational JSON tree",()=>{
  expect(existsSync(resolve(import.meta.dir,"../🧬️schema/🔣️.json"))).toBe(false);
  const require=createRequire(import.meta.url),ajv=new(require("ajv"))({strict:false,allErrors:true});
  ajv.addSchema(ordered);ajv.addSchema(contract);
  const validate=ajv.compile({$ref:contract.$id+"#/$defs/Grant"});
  for(const authority of [fixture.normalAuthority,fixture.retirementAuthority,fixture.turn,fixture.closeTurn])expect(validate(authority)).toBe(true);
  const database=new Database(":memory:");
  try{
    for(const row of fixture.cases){
      expect(JSON.parse(row.source)).toBeDefined();
      const independent=database.query("WITH RECURSIVE tree AS (SELECT id,parent,type FROM json_tree(?)), levels AS (SELECT id,type,CASE WHEN type IN ('array','object') THEN 1 ELSE 0 END AS depth FROM tree WHERE parent IS NULL UNION ALL SELECT tree.id,tree.type,levels.depth+CASE WHEN tree.type IN ('array','object') THEN 1 ELSE 0 END FROM tree JOIN levels ON tree.parent=levels.id) SELECT max(depth) AS depth FROM levels").get(row.source) as {depth:number};
      expect(independent.depth).toBe(row.containerDepth);
      expect(independent.depth<=row.maximumDepth?"admitted":"depthLimit").toBe(row.expected);
      if(row.refusedPosition!==null)expect(["[","{"].includes(row.source[row.refusedPosition]!)).toBe(true);
    }
    for(const row of originalLimits.cases){
      const independent=database.query("WITH RECURSIVE tree AS (SELECT id,parent,type FROM json_tree(?)), levels AS (SELECT id,type,CASE WHEN type IN ('array','object') THEN 1 ELSE 0 END AS depth FROM tree WHERE parent IS NULL UNION ALL SELECT tree.id,tree.type,levels.depth+CASE WHEN tree.type IN ('array','object') THEN 1 ELSE 0 END FROM tree JOIN levels ON tree.parent=levels.id) SELECT max(depth) AS depth FROM levels").get(row.source) as {depth:number};
      if(independent.depth>row.maximumDepth){expect(row.accept).toBe(false);expect("kind" in row?row.kind:undefined).toBe("DepthLimit");}
    }
  }finally{database.close();}
  console.log("[DEBUG] JSON seven empty/scalar/mixed container depth boundaries match canonical per-grant admission and independent SQLite recursive JSON tree");
});
