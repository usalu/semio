import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {readFileSync} from "node:fs";

test("refused completion neutral ownership lifecycle matches SQLite",()=>{
  const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/♻️completion-rejection.json",import.meta.url),"utf8"));
  const schema=JSON.parse(readFileSync(new URL("../../🧬️schema/♻️completion-rejection.json",import.meta.url),"utf8"));
  expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
  const oracle=new Database(":memory:");
  try{
    oracle.exec("CREATE TABLE owners(lane TEXT, ordinal INTEGER)");
    for(const row of fixture.cases)for(const mode of fixture.modes){
      expect(["refused","cancelled"]).toContain(mode);
      oracle.exec("DELETE FROM owners");
      for(const [lane,count] of Object.entries({artifact:row.artifactMutations,presence:row.presenceMutations,transient:row.transientMutations}))for(let n=0;n<Number(count);n++)oracle.query("INSERT INTO owners VALUES(?,?)").run(lane,n);
      const count=()=>Number((oracle.query("SELECT COUNT(*) AS n FROM owners").get()as{n:number}).n);
      expect(count()).toBe(row.returnedMutations);
      oracle.query("DELETE FROM owners WHERE rowid IN (SELECT rowid FROM owners LIMIT ?)").run(0);
      expect(count()===row.returnedMutations).toBe(row.zeroGrantPreservesOwner);
      while(count())oracle.query("DELETE FROM owners WHERE rowid IN (SELECT rowid FROM owners LIMIT 1)").run();
      expect(count()===0).toBe(row.terminal);
    }
  }finally{oracle.close();}
  console.log("[DEBUG] six neutral refused/cancelled completion ownership lifecycles match SQLite");
});
