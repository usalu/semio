import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import fixture from "./🧫️fixtures/🔣️.json";
test("join close-start preserves originals when denied",()=>{const db=new Database(":memory:");try{db.exec("CREATE TABLE turns(admitted INTEGER,work INTEGER)");db.query("INSERT INTO turns VALUES(?,?)").run(0,fixture.closeStart.deniedWork);db.query("INSERT INTO turns VALUES(?,?)").run(1,fixture.closeStart.selectedWork);expect(db.query("SELECT admitted,work FROM turns ORDER BY admitted").all()).toEqual([{admitted:0,work:0},{admitted:1,work:1}])}finally{db.close()}});
