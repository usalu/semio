import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import fixture from "./🧫️fixtures/🚫️one-shot.json";
test("one original request refuses while later requests remain usable",()=>{for(const row of fixture.cases){const db=new Database(":memory:");try{db.exec("CREATE TABLE requests(position INTEGER PRIMARY KEY,bytes INTEGER,accepted INTEGER)");db.query("INSERT INTO requests VALUES(?,?,?)").run(1,fixture.bytes,0);db.query("INSERT INTO requests VALUES(?,?,?)").run(2,fixture.nextBytes,1);const actual=db.query("SELECT SUM(bytes) AS requested,SUM(1-accepted) AS refused,SUM(bytes*accepted) AS released FROM requests").get();expect(actual).toEqual({requested:row.requested,refused:row.refused,released:row.released})}finally{db.close()}}});
