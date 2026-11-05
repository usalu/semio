import{test,expect}from"bun:test";
import Ajv from"ajv";
import graphlib from"graphlib";
import{Database}from"bun:sqlite";
import*as api from"../../🟦️.ts";
import{creditedDeadlineRemainingV1,beginAdmissionWaitV1,admissionWaitedMillisecondsV1}from"../../🐕️watchdog/🟦️.ts";
import fixture from"./wait-model.json";
import schema from"./wait-model.schema.json";

test("neutral DAG bounds credit the union of open overlapping and ended intervals exactly once",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const graph=new graphlib.Graph({directed:true,multigraph:true});
 for(const row of fixture.rows){graph.setNode(row.id);if("parent"in row)graph.setEdge(row.id,row.parent,{},"parent");if("follows"in row)graph.setEdge(row.id,row.follows,{},"follows");}
 const reachable=graphlib.alg.preorder(graph,fixture.target);expect(reachable.length).toBe(3);expect(reachable).not.toContain("unreachable");
 const db=new Database(":memory:"),observed=new Map<string,number>();
 try{
  db.exec("CREATE TABLE waits(start REAL,stop REAL);CREATE TABLE bounds(id TEXT,started REAL,maximum REAL,epoch REAL,waited REAL)");
  for(const row of fixture.rows)if(reachable.includes(row.id))db.query("INSERT INTO bounds VALUES(?,?,?,?,?)").run(row.id,row.started,row.maximum,row.epochDeadline,row.waitedAtStart);
  for(const call of fixture.calls){
   db.exec("DELETE FROM waits");fixture.intervals.forEach((row,index)=>db.query("INSERT INTO waits VALUES(?,?)").run(row.start,index===fixture.intervals.length-1?call.lastEnd:row.end));
   const waited=(db.query("WITH intervals AS(SELECT start,min(coalesce(stop,?1),?1) AS finish FROM waits WHERE start<?1),points AS(SELECT start AS point FROM intervals UNION SELECT finish FROM intervals),segments AS(SELECT point AS low,lead(point) OVER(ORDER BY point) AS high FROM points) SELECT coalesce(sum(CASE WHEN EXISTS(SELECT 1 FROM intervals WHERE start<=low AND finish>=high) THEN high-low ELSE 0 END),0) AS waited FROM segments WHERE high IS NOT NULL").get(call.mono)as{waited:number}).waited;
   expect(waited).toBe(call.waited);
   const sql=(db.query("SELECT min(min(epoch-?1,maximum-(?2-started))+max(0,?3-waited)) AS remaining FROM bounds").get(call.epoch,call.mono,waited)as{remaining:number}).remaining;
   const actual=Math.min(...fixture.rows.filter(row=>reachable.includes(row.id)).map(row=>creditedDeadlineRemainingV1(Math.min(row.epochDeadline-call.epoch,row.maximum-(call.mono-row.started)),waited-row.waitedAtStart)));
   expect(sql).toBe(call.expected);expect(actual).toBe(sql);observed.set(call.id,actual);
  }
 }finally{db.close();}
 expect(observed.get("open-wait-before")).toBe(observed.get("open-wait-after"));expect(observed.get("last-wait-ended")!-observed.get("active-after-ended")!).toBe(100);
});

test("public controls retain open overlap credit then spend active time after both intervals end",async()=>api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:"fixture:wait-model-public",maximumElapsedMilliseconds:0},{fixture:"wait-model-public"},Date.now()),async original=>{
 const row=fixture.publicInterval,control=api.createRequestedIssuedResourceControlV1(api.createIssuedResourceControlV1(original,"fixture:wait-model-public",0),row.budget);
 const waitedBefore=admissionWaitedMillisecondsV1(),endFirst=beginAdmissionWaitV1();let endSecond:(()=>void)|undefined;
 try{
  await new Promise(done=>setTimeout(done,row.first));endSecond=beginAdmissionWaitV1();await new Promise(done=>setTimeout(done,row.overlap));endFirst();await new Promise(done=>setTimeout(done,row.last));
  const open=control.remainingMilliseconds()!;expect(open).toBeGreaterThan(0);expect(open).toBeLessThanOrEqual(row.budget);expect(admissionWaitedMillisecondsV1()-waitedBefore).toBeGreaterThan(row.budget);
  const projected=JSON.parse(api.scriptResourceProcessEnvironmentV1(control,{})[api.SCRIPT_PROCESS_INVOCATION_ENV]!);expect(projected.deadlineEpochMilliseconds).toBeGreaterThan(Date.now());
  endSecond();endSecond=undefined;const closed=control.remainingMilliseconds()!;expect(closed).toBeGreaterThan(0);expect(Math.abs(closed-open)).toBeLessThanOrEqual(row.tolerance);
  const ended=admissionWaitedMillisecondsV1();await new Promise(done=>setTimeout(done,row.active));const active=control.remainingMilliseconds()!;
  expect(admissionWaitedMillisecondsV1()).toBe(ended);expect(active).toBeGreaterThan(0);expect(closed-active).toBeGreaterThanOrEqual(row.active-row.tolerance);expect(closed-active).toBeLessThanOrEqual(row.active+row.tolerance);
 }finally{endFirst();endSecond?.();}
}));
