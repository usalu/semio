/** ⏳️ Cancellation identity validation agrees with the neutral schema without losing integer precision. */
import {expect,test} from "vitest";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";
import {decodeOperationCancellation,operationProgressText,cancellationResultLane,operationCancellationTargetAdmitted} from "../🟦️.ts";
import targetSchema from "../🧬️schema/🎯️target.json";
/** 🪪️ The cancellation identity `🧬️schema/🔣️.json` admits. */
type OperationCancellationArgsV1={readonly operationId:string;readonly generation:string};
/** 🏁️ The terminal outcome `🧬️schema/🏁️terminal.json` admits. */
type CancellationOutcomeV1={readonly userRequested:boolean;readonly workerFault:boolean;readonly lane:"terminal"|"fault"};
for(const row of fixture.cases)test("Operation cancellation: "+row.name,()=>{
  const decoded=decodeOperationCancellation(row.args);expect(decoded!==null).toBe(row.valid);
});
for(const locale of ["en","de"] as const)for(const count of fixture.counts)test(`Operation progress ${locale} ${count}`,()=>{
  const units=new Intl.NumberFormat(locale,{useGrouping:false}).format(BigInt(count)),labels=fixture.labels[locale];
  expect(operationProgressText(locale,BigInt(count),false)).toBe(`${labels.working} · ${labels.units}: ${units}`);
  expect(operationProgressText(locale,BigInt(count),true)).toBe(`${labels.cancelling} · ${labels.units}: ${units}`);
});

import outcomeSchema from "../🧬️schema/🏁️terminal.json";
const validateOutcome=new Ajv().compile<CancellationOutcomeV1>(outcomeSchema);
for(const row of fixture.cancellationOutcomes)test(`Cancellation outcome user=${row.userRequested} fault=${row.workerFault}`,()=>{
  if(!validateOutcome(row))throw new Error(JSON.stringify(validateOutcome.errors));
  expect(cancellationResultLane(row.userRequested,row.workerFault)).toBe(row.lane);
  expect(validateOutcome({...row,lane:row.lane==="fault"?"terminal":"fault"})).toBe(false);
});
const validateTarget=new Ajv().compile(targetSchema);
for(const row of fixture.cancellationTargets)test(`Cancellation target ${row.name}`,()=>{
  if(!validateTarget(row))throw new Error(JSON.stringify(validateTarget.errors));
  expect(operationCancellationTargetAdmitted(row.present,row.terminal,row.cancellable)).toBe(row.admitted);
});
