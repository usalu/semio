import {expect,test} from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
import law from "./depth.json";
import schema from "./depth.schema.json";
test("metadata controllers preserve payload depth and real element edges consume one",()=>{expect(new Ajv2020({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("SELECT ?+? AS leaf,?+? AS child").get(law.leafDepth,law.metadataExtraDepth,law.leafDepth,law.payloadExtraDepth)).toEqual({leaf:1,child:law.childDepth});expect(db.query("WITH RECURSIVE path(n,depth) AS(SELECT 0,? UNION ALL SELECT n+1,depth+? FROM path WHERE n<?) SELECT max(depth) AS demand FROM path").get(law.leafDepth,law.payloadExtraDepth,law.arrayEdges)).toEqual({demand:law.arrayDemand});expect(law.arrayDemand<=law.sourceDepth).toBe(true);expect(law.deniedWork).toBe(0)}finally{db.close()}});
