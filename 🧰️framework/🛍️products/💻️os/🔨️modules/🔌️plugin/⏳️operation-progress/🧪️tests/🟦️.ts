/** ⏳️ Cancellation identity validation agrees with the neutral schema without losing integer precision. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import {decodeOperationCancellation,operationProgressText,cancellationResultLane} from "../🟦️.ts";
const validate=new Ajv().compile(schema);
for(const row of fixture.cases)test("Operation cancellation: "+row.name,()=>{
  expect(validate(row.args)).toBe(row.valid);
  const decoded=decodeOperationCancellation(row.args);expect(decoded!==null).toBe(row.valid);
  if(decoded){expect(decoded.operationId.toString(16).padStart(16,"0")).toBe(row.args.operationId);expect(decoded.generation.toString(16).padStart(16,"0")).toBe(row.args.generation);}
});
for(const locale of ["en","de"] as const)for(const count of fixture.counts)test(`Operation progress ${locale} ${count}`,()=>{
  const units=new Intl.NumberFormat(locale,{useGrouping:false}).format(BigInt(count)),labels=fixture.labels[locale];
  expect(operationProgressText(locale,BigInt(count),false)).toBe(`${labels.working} · ${labels.units}: ${units}`);
  expect(operationProgressText(locale,BigInt(count),true)).toBe(`${labels.cancelling} · ${labels.units}: ${units}`);
});

import outcomeSchema from "../🧬️schema/🏁️terminal.json";
const validateOutcome=new Ajv().compile(outcomeSchema);
for(const row of fixture.cancellationOutcomes)test(`Cancellation outcome user=${row.userRequested} fault=${row.workerFault}`,()=>{
  expect(validateOutcome(row)).toBe(true);
  expect(cancellationResultLane(row.userRequested,row.workerFault)).toBe(row.lane);
  expect(validateOutcome({...row,lane:row.lane==="fault"?"terminal":"fault"})).toBe(false);
});
