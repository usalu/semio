import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020.js";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🧫️fixtures/🔣️.json";

test("original VCS order and lane selection match SQLite event projection", () => {
  expect(new Ajv2020({ strict: true }).compile(schema)(corpus)).toBe(true);
  for (const row of corpus.cases) {
    const database = new Database(":memory:");
    database.exec("CREATE TABLE edits(rank INTEGER PRIMARY KEY,id TEXT,actor TEXT,lane TEXT,value INTEGER,applied INTEGER,redo INTEGER)");
    row.edits.forEach((edit, rank) => database.query("INSERT INTO edits VALUES(?,?,?,?,?,1,NULL)").run(rank,edit.id,edit.actor,edit.lane,edit.value));
    let applied=row.edits.map((_,rank)=>rank),redo:number[]=[];
    row.commands.forEach((command, turn) => {
      const original=command.kind==="undo"?applied:redo;
      const index=original.findLastIndex(rank=>row.edits[rank]!.actor===row.actor&&row.edits[rank]!.lane===command.lane);
      expect(index).toBeGreaterThanOrEqual(0);
      const rank=original.splice(index,1)[0]!;
      if(command.kind==="undo"){redo.push(rank);database.query("UPDATE edits SET applied=0,redo=? WHERE rank=?").run(turn,rank);}
      else{applied.push(rank);applied.sort((a,b)=>a-b);database.query("UPDATE edits SET applied=1,redo=NULL WHERE rank=?").run(rank);}
      const sql={applied:database.query("SELECT id FROM edits WHERE applied=1 ORDER BY rank").all().map((e:any)=>e.id),redo:database.query("SELECT id FROM edits WHERE applied=0 ORDER BY redo").all().map((e:any)=>e.id),value:(database.query("SELECT COALESCE(SUM(value),0) AS value FROM edits WHERE applied=1").get() as any).value};
      const actual={applied:applied.map(rank=>row.edits[rank]!.id),redo:redo.map(rank=>row.edits[rank]!.id),value:applied.reduce((sum,rank)=>sum+row.edits[rank]!.value,0)};
      expect(actual).toEqual(sql);expect(actual).toEqual(row.expected[turn]);
    });
    database.close();
  }
});
