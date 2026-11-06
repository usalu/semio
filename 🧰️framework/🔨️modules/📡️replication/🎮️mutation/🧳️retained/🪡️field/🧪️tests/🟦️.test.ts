import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import law from "./🧫️fixtures/🛑️start.json";
test("field child start requires its original flag-copy grant",()=>{const db=new Database(":memory:");try{db.exec("CREATE TABLE turns(grant_bytes INTEGER,work INTEGER,copied INTEGER,started INTEGER)");db.query("INSERT INTO turns VALUES(?,?,?,?)").run(0,law.denied.work,law.denied.copy,Number(law.denied.started));db.query("INSERT INTO turns VALUES(?,?,?,?)").run(law.flagBytes,law.admitted.work,law.admitted.copy,Number(law.admitted.started));expect(db.query("SELECT work,copied,started FROM turns ORDER BY grant_bytes").all()).toEqual([{work:0,copied:0,started:0},{work:1,copied:1,started:1}])}finally{db.close()}});
