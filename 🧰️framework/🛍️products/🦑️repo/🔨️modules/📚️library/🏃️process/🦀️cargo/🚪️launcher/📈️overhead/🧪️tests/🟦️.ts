import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {createCargoOverheadSamplerV1} from "../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

test("direct Cargo reserves resident launcher and wrapper memory once",async()=>{
 const database=new Database(":memory:");
 try{
  let lastChildKiB=0,row=fixture.cases[0]!;
  const sampler=createCargoOverheadSamplerV1(fixture.root,async()=>row.rows.map(([pid,ppid,kilobytes])=>({pid:pid!,ppid:ppid!,kilobytes:kilobytes!,name:"controlled"})),()=>row.launcherKiB*1024);
  const oracle=database.query(`WITH RECURSIVE processes(pid,ppid,rss) AS (
   SELECT json_extract(value,'$[0]'),json_extract(value,'$[1]'),json_extract(value,'$[2]') FROM json_each(?1)
  ), owned(pid,ppid,rss) AS (
   SELECT * FROM processes WHERE pid=?2 UNION ALL SELECT processes.* FROM processes JOIN owned ON processes.ppid=owned.pid
  ) SELECT
   coalesce(sum(CASE WHEN pid<>?2 AND ppid NOT IN (SELECT value FROM json_each(?3)) THEN rss ELSE 0 END),?4) AS children,
   CASE WHEN count(*)=0 THEN ?4 ELSE sum(CASE WHEN pid<>?2 AND ppid NOT IN (SELECT value FROM json_each(?3)) THEN rss ELSE 0 END) END +
     max(?5,coalesce(max(CASE WHEN pid=?2 THEN rss END),0)) AS total FROM owned`);
  for(row of fixture.cases){
   const {children,total}=oracle.get(JSON.stringify(row.rows),fixture.root,JSON.stringify(row.schedulers),lastChildKiB,row.launcherKiB) as {children:number;total:number};
   lastChildKiB=children;
   expect(total,row.name).toBe(row.expectedKiB);
   expect(await sampler(row.schedulers),row.name).toBe(total*1024);
  }
 }finally{database.close();}
});
