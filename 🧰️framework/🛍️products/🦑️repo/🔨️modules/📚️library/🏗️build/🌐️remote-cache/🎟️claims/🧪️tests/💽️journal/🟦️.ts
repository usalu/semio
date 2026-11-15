import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createClaimTableV1} from "../../🟦️.ts";
import {openClaimJournalV1} from "../../💽️journal/🟦️.ts";

const scratch:string[]=[];
const directory=():string=>{const path=mkdtempSync(join(tmpdir(),"claims-journal-"));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const python=async(program:string,input:string):Promise<string>=>{const child=Bun.spawn(["python3","-I","-c",program,input],{stdout:"pipe",stderr:"pipe"}),[out,code]=await Promise.all([new Response(child.stdout).text(),child.exited]);expect(code).toBe(0);return out.trim();};
const oracle="import sys,json\nlive={}\nlast=0\nfor l in open(sys.argv[1]):\n  l=l.strip()\n  if not l: continue\n  try: r=json.loads(l)\n  except Exception: continue\n  last=max(last,r.get('at',0))\n  if r['t']=='set': live[r['address']]=r\n  elif r['t']=='drop': live.pop(r['address'],None)\nprint(json.dumps(sorted(a for a,r in live.items() if r['untilMs']>=last)))";

test("a table writes its changes to the journal and a second table restores the live claims with a grace; python replays the same live set",async()=>{
 const file=join(directory(),"claims.jsonl");let clock=1_000_000;
 const first=createClaimTableV1({now:()=>clock,sweepMilliseconds:0,journal:openClaimJournalV1(file,{now:()=>clock})});
 expect(first.claim("main","ac","a".repeat(64),{owner:"m1:1:aa",ttlMilliseconds:30000},false).state).toBe("granted");
 expect(first.claim("main","ac","b".repeat(64),{owner:"m2:2:bb",ttlMilliseconds:30000},false).state).toBe("granted");
 expect(first.claim("main","nx","gone",{owner:"m3:3:cc",ttlMilliseconds:30000},false).state).toBe("granted");
 clock+=5000;first.release("main","nx","gone","m3:3:cc");first.stored("main","ac","b".repeat(64));
 clock+=5000;expect(first.renew("main","ac","a".repeat(64),{owner:"m1:1:aa",ttlMilliseconds:30000}).state).toBe("granted");
 const live=JSON.parse(await python(oracle,file)) as string[];expect(live.length).toBe(1);
 first.close();clock+=3000;
 const second=createClaimTableV1({now:()=>clock,sweepMilliseconds:0,journal:openClaimJournalV1(file,{now:()=>clock,graceMilliseconds:20000})});
 expect(second.summary().active).toBe(1);
 const other=second.claim("main","ac","a".repeat(64),{owner:"m9:9:zz",ttlMilliseconds:30000},false);
 expect(other.state,"another machine must not compute the unit the server forgot nothing about").toBe("held");
 expect(other.state==="held"&&other.by).toBe("m1:1:aa");
 expect(second.renew("main","ac","a".repeat(64),{owner:"m1:1:aa",ttlMilliseconds:30000}).state).toBe("granted");
 expect(second.claim("main","ac","b".repeat(64),{owner:"m9:9:zz",ttlMilliseconds:30000},false).state,"a stored claim is not restored").toBe("granted");
 second.close();
});

test("a claim that lapsed before the server stopped, a torn last line and a finished cool-off are not restored; a restored claim whose holder never returns lapses after the grace",async()=>{
 const file=join(directory(),"claims.jsonl");let clock=5_000_000;
 const first=createClaimTableV1({now:()=>clock,sweepMilliseconds:0,journal:openClaimJournalV1(file,{now:()=>clock})});
 first.claim("main","ac","c".repeat(64),{owner:"m1:1:aa",ttlMilliseconds:1000},false);
 clock+=10_000;first.claim("main","ac","d".repeat(64),{owner:"m2:2:bb",ttlMilliseconds:1000},false);
 writeFileSync(file,readFileSync(file,"utf8")+'{"at":1,"t":"set","address":"torn');
 first.close();clock+=1000;
 const second=createClaimTableV1({now:()=>clock,sweepMilliseconds:0,journal:openClaimJournalV1(file,{now:()=>clock,graceMilliseconds:15000})});
 expect(second.summary().active,"only the claim that was alive at the last journal write").toBe(1);
 clock+=16_000;second.sweep();expect(second.summary().active,"its holder never came back: it lapses").toBe(0);
 expect(second.claim("main","ac","d".repeat(64),{owner:"m9:9:zz",ttlMilliseconds:30000},false).state).toBe("granted");
 second.close();
});

test("the journal is compacted after many changes and stays equal to the live table; an unwritable journal never breaks the table",async()=>{
 const file=join(directory(),"claims.jsonl");let clock=9_000_000;
 const table=createClaimTableV1({now:()=>clock,sweepMilliseconds:0,journal:openClaimJournalV1(file,{now:()=>clock,compactEvents:50})});
 for(let index=0;index<200;index++){const key=index.toString(16).padStart(64,"0");table.claim("main","ac",key,{owner:"m1:1:aa",ttlMilliseconds:30000},false);if(index%2===0)table.release("main","ac",key,"m1:1:aa");clock+=10;}
 expect(readFileSync(file,"utf8").split("\n").filter(Boolean).length).toBeLessThan(120);
 expect(JSON.parse(await python(oracle,file)).length).toBe(table.summary().active);
 const blocked=join(directory(),"not-a-directory");writeFileSync(blocked,"x");
 const survivor=createClaimTableV1({now:()=>clock,sweepMilliseconds:0,journal:openClaimJournalV1(join(blocked,"claims.jsonl"),{now:()=>clock})});
 expect(survivor.claim("main","ac","e".repeat(64),{owner:"m1:1:aa",ttlMilliseconds:30000},false).state).toBe("granted");
});
