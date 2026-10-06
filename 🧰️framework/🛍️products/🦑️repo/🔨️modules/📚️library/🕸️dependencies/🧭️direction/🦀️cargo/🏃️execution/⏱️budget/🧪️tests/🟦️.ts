import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";

import { CargoMetadataCaptureBudget } from "../🟦️.ts";
test("queued preparation leaves the complete aggregate native capture budget intact", () => {
  
  
  const oracle = new Database(":memory:");
  try {
    oracle.exec("CREATE TABLE duration(phase TEXT NOT NULL, milliseconds REAL NOT NULL)");
    for (const row of fixture.cases) {
      oracle.exec("DELETE FROM duration");
      for (const step of row.steps) oracle.query("INSERT INTO duration VALUES (?, ?)").run(step.phase,step.milliseconds);
      const sum = (oracle.query("SELECT COALESCE(SUM(milliseconds),0) AS elapsed FROM duration WHERE phase='capture'").get() as {elapsed:number}).elapsed;
      const independent = {remainingMs:Math.max(0,row.budgetMs-sum),refused:sum>=row.budgetMs};
      expect(independent,row.id).toEqual({remainingMs:row.remainingMs,refused:row.refused});
      const budget = new CargoMetadataCaptureBudget(row.budgetMs);let refused=false;
      for (const step of row.steps) if(step.phase==="capture") {
        try { budget.charge(step.milliseconds); } catch { refused=true;break; }
      }
      expect({remainingMs:budget.remainingMs,refused},row.id).toEqual(independent);
      console.log("[DEBUG] Cargo capture budget "+JSON.stringify({id:row.id,remainingMs:budget.remainingMs,refused}));
    }
    for (const value of [...fixture.invalidBudgets,NaN,Infinity]) expect(()=>new CargoMetadataCaptureBudget(value)).toThrow();
    for (const value of [...fixture.invalidCaptures,NaN,Infinity]) expect(()=>new CargoMetadataCaptureBudget(30000).charge(value)).toThrow();
  } finally { oracle.close(); }
});
