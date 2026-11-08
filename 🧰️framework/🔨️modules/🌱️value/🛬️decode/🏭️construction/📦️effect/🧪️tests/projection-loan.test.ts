import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "./projection-loan.schema.json";
import fixture from "./projection-loan.json";
test("projection child Birth spends the same original decoder loan",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE budget(fuel INTEGER,copy INTEGER)");db.run("INSERT INTO budget VALUES(?,?)",[fixture.stock.fuel,fixture.stock.copyBytes]);const row=db.query<{fuel:number;copy:number},[]>("SELECT fuel-? AS fuel,copy-? AS copy FROM budget").get(fixture.birth.work,fixture.birth.copyBytes)!;expect(row.fuel).toBe(fixture.stock.fuel-fixture.birth.work);expect(row.copy).toBe(fixture.stock.copyBytes-fixture.birth.copyBytes);expect(fixture.birth.usesOriginalLoan).toBe(true);expect(fixture.control.usesOriginalLoan).toBe(true);expect(fixture.control.work).toBe(1);expect(fixture.control.interrupt).toBe(true);expect(fixture.control.recoveries).toBe(0)}finally{db.close()}});
