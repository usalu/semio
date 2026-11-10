/** 🧪️ Planning benchmark conformance: the scenario document and the report follow one JSON schema, plan lines recorded from real runs split into the same phases and counters as an independent Python oracle computes, and the report renders in both languages. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {planningMedianV1,planningObserveV1,planningOomKillsV1,planningRenderV1,planningScenariosV1,planningSchemaV1,type PlanningReportV1} from "../🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),ajv=new Ajv({strict:false,allErrors:true}),schema=planningSchemaV1 as unknown as {definitions:Record<string,unknown>},
 check=(name:string)=>ajv.compile({$ref:`#/definitions/${name}`,definitions:schema.definitions} as never);

test("scenario document matches its schema and covers the eight measurements",()=>{
 const valid=check("Scenarios");
 expect(valid(planningScenariosV1)).toBe(true);
 expect(planningScenariosV1.scenarios.map(row=>row.kind).sort()).toEqual(["cold","concurrent","edit","edit","edit","host","store","warm"]);
 for(const row of planningScenariosV1.scenarios)if(row.kind!=="host"&&row.kind!=="store")expect(row.dryRun,row.id).toBe(row.id!=="task-keys-edit");
 for(const row of planningScenariosV1.markers.concat(planningScenariosV1.counters as never))expect(()=>new RegExp(row.pattern,"u")).not.toThrow();
 expect(valid({...planningScenariosV1,scenarios:[{...planningScenariosV1.scenarios[0],runs:0}]})).toBe(false);
});

test("every path in the scenarios stays under 256 characters",()=>{
 for(const row of planningScenariosV1.scenarios)if(row.edit)expect(row.edit.file.length).toBeLessThan(256);
});

test("recorded plan lines split into phases and counters like the Python oracle",()=>{
 const oracle=spawnSync("/home/deck/semio/.venv/bin/python",["-I",join(here,"🐍️oracle","🐍️.py"),join(here,"..","🎯️scenarios","🔣️.json"),join(here,"🧫️fixtures","🔣️.json")],{encoding:"utf8"});
 expect(oracle.status).toBe(0);
 const other=JSON.parse(oracle.stdout) as Record<string,unknown>;
 for(const row of fixture.cases){
  const mine=planningObserveV1(row.lines.map(([atMs,text])=>({atMs:atMs as number,text:text as string})),row.wallMs);
  expect(mine.phaseMs).toEqual(row.expected.phaseMs as never);
  expect(mine.counters).toEqual(row.expected.counters as never);
  expect(other[row.id]).toEqual({phaseMs:mine.phaseMs,counters:mine.counters});
  expect(Object.values(mine.phaseMs).reduce((a,b)=>a+b,0)).toBe(row.wallMs);
 }
});

test("report format accepts the recorded report and rejects malformed ones",()=>{
 const valid=check("Report");
 expect(valid(fixture.report)).toBe(true);
 for(const bad of fixture.badReports)expect(valid({...fixture.report,...bad.patch})).toBe(false);
 const {results:_,...partial}=fixture.report;
 expect(valid(partial)).toBe(false);
});

test("report renders as a table in English and German with medians",()=>{
 const report=fixture.report as unknown as PlanningReportV1,en=planningRenderV1(report,"en"),de=planningRenderV1(report,"de");
 expect(en.split("\n")[0]).toContain("Wall ms (median)");
 expect(de.split("\n")[0]).toContain("Laufzeit ms (Median)");
 expect(en).toContain("| warm-noop | ran | 2 | 5000 | 1500 | 0 | 0 | 3500 | -/- | 3050 | 7550 | 0 | - |");
 expect(en).toContain("| store-disk | ran | 1 | 0 | 0 | 0 | 0 | 0 | -/- | - | - | - | 52000 |");
});

test("median and OOM counting follow the fixture",()=>{
 expect(planningMedianV1(fixture.median.values)).toBe(fixture.median.expected);
 expect(planningOomKillsV1(fixture.oom.journal)).toBe(fixture.oom.expected);
});
