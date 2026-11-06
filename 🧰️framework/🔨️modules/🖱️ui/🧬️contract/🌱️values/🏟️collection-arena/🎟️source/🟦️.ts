import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️.json";
test("UI arena publication requires its admitted native Source and complete owned opening",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);const db=new Database(":memory:");
 for(const row of corpus.cases){const result=db.query("select (? and ? and ? and not ?) ready").get(row.admitted,row.opened,row.registered,row.closing)as{ready:number};expect(Boolean(result.ready)).toBe(row.ready);}
 db.close();
});
test("finite arena reserve includes row initialization and all empty descendant scans",()=>{const r=corpus.reserve,db=new Database(":memory:");const actual=db.query("select 2*?+2*?+14 openingItems,?+?+?+14 closureItems").get(r.pages,r.collections,r.pages,r.collections,r.handbackWords);expect(actual).toEqual({openingItems:r.openingItems,closureItems:r.closureItems});db.close()});
