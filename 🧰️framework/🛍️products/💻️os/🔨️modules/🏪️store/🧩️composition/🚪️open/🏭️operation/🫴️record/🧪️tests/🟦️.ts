import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import budget from "./🪙️budget.json";
test("a batched original record open settles in turns proportional to its size over the grant",()=>{
 using db=new Database(":memory:");
 const row=db.query("select (?1+?2-1)/?2 as shares from (select 1)").get(budget.batching.titleBytes,Math.min(Math.floor(budget.batching.copyBytes/2),Math.floor(Math.min(budget.batching.scopeWorkCeiling,Math.floor(budget.batching.fuel/2))/budget.batching.actionsPerByteCeiling)))as{shares:number};
 const bound=budget.batching.turnsPerShare*row.shares;
 expect(bound).toBeGreaterThan(0);expect(bound).toBeLessThan(budget.batching.titleBytes);expect(budget.batching.titlePattern.length).toBeGreaterThan(0);
});
test("the floor law selects a turn copy above one receiver",()=>{
 const {numerator,denominator}=budget.floor.turnCopyOverReceiver;expect(numerator/denominator).toBeGreaterThan(1);expect(budget.floor.stockCopyBytes).toBeGreaterThan(0);
});
