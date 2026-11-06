import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import law from "./🧫️fixtures/🛑️start.json";
test("diff close starts the selected owner only after its flag grant",()=>{const db=new Database(":memory:");try{const q=db.query("SELECT CASE WHEN ?=0 OR ?=0 THEN CASE WHEN ?>=? AND ?>0 THEN 1 ELSE 0 END ELSE 0 END AS work");for(const row of law.cases){const result=q.get(Number(row.parent),Number(row.child),row.grant,law.flagBytes,row.maximumWork) as {work:number};expect(result.work).toBe(row.work);expect(result.work*law.flagBytes).toBe(row.copy);expect(row.parent||(!row.parent&&result.work===1)).toBe(row.nextParent);expect(row.child||(row.parent&&!row.child&&result.work===1)).toBe(row.nextChild)}}finally{db.close()}});
