import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import law from "./binding-read.json";

test("an original surface read selects its exact binding and restores it on cancellation",()=>{
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE binding(surface TEXT PRIMARY KEY,window TEXT NOT NULL);CREATE TABLE loan(surface TEXT PRIMARY KEY,window TEXT NOT NULL)");
  for(const row of law.bindings)db.query("INSERT INTO binding VALUES (?,?)").run(row.surface,row.window);
  for(const cut of [...law.cancelCuts,null]){
   db.transaction(()=>{db.query("INSERT INTO loan SELECT * FROM binding WHERE surface=?").run(law.surface);db.query("DELETE FROM binding WHERE surface=?").run(law.surface)})();
   expect(db.query("SELECT window FROM loan WHERE surface=?").get(law.surface)).toEqual({window:law.expectedWindow});
   db.transaction(()=>{db.exec("INSERT INTO binding SELECT * FROM loan;DELETE FROM loan")})();
   expect(db.query("SELECT window FROM binding WHERE surface=?").get(law.surface)).toEqual({window:law.expectedWindow});
   expect(db.query("SELECT COUNT(*) AS count FROM loan").get()).toEqual({count:0});
  }
 }finally{db.close()}
});
