import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import schema from "../🔣️.json";
import corpus from "./cases.json";
test("Source byte encoding agrees with independent work and ownership ledger",()=>{
 expect(new Ajv({strict:false}).compile(schema)(corpus)).toBe(true);
 expect({capacity:corpus.unwind.bytes.length,copies:corpus.unwind.bytes.length}).toEqual({capacity:corpus.unwind.expectedCapacity,copies:corpus.unwind.expectedCopies});expect(corpus.depth.levels<=Math.min(corpus.depth.source,corpus.depth.consumer)).toBe(corpus.depth.admitted);
 for(const row of corpus.cases){
  const db=new Database(":memory:");db.run("CREATE TABLE spans(bytes INTEGER,accepted INTEGER)");for(const chunk of row.chunks)db.run("INSERT INTO spans VALUES(?,0)",[chunk.length]);const {total}=db.query("SELECT SUM(bytes) total FROM spans").get() as {total:number};
  const capacity=total<=row.maximumCapacity?total:0;let copied=0,work=total;const output:number[]=[];
  if(capacity)for(const chunk of row.chunks){if(copied+chunk.length>row.maximumCopy)break;work+=chunk.length;copied+=chunk.length;output.push(...chunk);}
  expect({bytes:output,capacity,work,valid:capacity===total&&copied===total}).toEqual({bytes:row.expectedBytes,capacity:row.expectedCapacity,work:row.expectedWork,valid:row.valid});db.close();
 }
});
