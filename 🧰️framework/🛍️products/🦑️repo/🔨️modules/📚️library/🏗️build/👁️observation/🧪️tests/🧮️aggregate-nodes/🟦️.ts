/** 🧮️ Contract: the aggregate node budget of one record read admits every record the row and byte budgets admit, because a JSON node needs at least one byte and one row costs several nodes. */
import{expect,test}from"bun:test";
import{observationBlockBudgetAdmitsV1}from"../../🟨️.mjs";
import schema from"../../🧬️schema/🔣️.json";

const policy=(schema as any).$defs.observationBlock.const;
const NODES_PER_ROW=16;

test("the aggregate node budget covers the largest admitted record of rows",()=>{
 expect(observationBlockBudgetAdmitsV1(0,policy.maximumAggregateRows*NODES_PER_ROW,policy.maximumAggregateNodes)).toBe(true);
});

test("the aggregate node budget never undercuts the aggregate output bytes",()=>{
 expect(policy.maximumAggregateNodes).toBeGreaterThanOrEqual(policy.maximumAggregateOutputBytes);
});

test("a record of 226000 observations of eleven nodes each fits one lookup",()=>{
 expect(observationBlockBudgetAdmitsV1(0,226000*11*2,policy.maximumAggregateNodes)).toBe(true);
});
