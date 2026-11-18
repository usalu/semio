import {mkdirSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {auditIsolationV1,readRehearsalOptionsV1,rehearsalLimitsV1,runRehearsalV1,type RehearsalVerdictV1} from "./🟦️.ts";

/** 🚪️ `rehearse [--machines=3] [--memory=16384|8192] [--cores=8] [--owners=470] [--owner-seconds=3] [--owner-megabytes=128] [--scenario=steady|landing] [--landing-after=30] [--landing-keeps-revision] [--timeout=900] [--out=<dir>]` runs a rehearsal of the fleet on this machine and prints the timeline and the verdict; `audit` prints which state of the checkout a simulated machine shares with the real one. Exits 1 when the verdict fails. */
const table=(rows:readonly (readonly string[])[]):string=>rows.map(row=>"| "+row.join(" | ")+" |").join("\n");
export function reportV1(verdict:RehearsalVerdictV1,timeline:readonly {atSeconds:number;machine:string;ledger:Record<string,number>;pool:{state:string;reason:string|null;running:number;open:number};footprintMegabytes:number;cpu:number}[],machines:readonly string[]):string{
 const times=[...new Set(timeline.map(row=>row.atSeconds))].sort((a,b)=>a-b);
 const lines=[table([["t (s)","machine","total","present","fetched","here","wanted","pool","reason","running","footprint MB","cpu %"],["---","---","---","---","---","---","---","---","---","---","---","---"],...times.flatMap(at=>machines.map(machine=>{const row=timeline.find(sample=>sample.atSeconds===at&&sample.machine===machine);return row===undefined?[]:[String(at),machine,...["total","present","fetched","here","wanted"].map(name=>String(row.ledger[name])),row.pool.state,row.pool.reason??"-",String(row.pool.running),String(row.footprintMegabytes),String(row.cpu)];}).filter(row=>row.length>0))])];
 const verdictLines=[`- verdict: ${verdict.pass?"PASS":"FAIL"}`,`- every machine ready: ${verdict.allReady} (${Object.entries(verdict.readyAtSeconds).map(([machine,at])=>`${machine} ${at} s`).join(", ")||"none"})`,`- owners inferred per machine: ${JSON.stringify(verdict.inferredByMachine)}, fetched: ${JSON.stringify(verdict.fetchedByMachine)}, inferred in total: ${verdict.inferredTotal}`,`- owners stored more than once: ${verdict.duplicateOwners.length}`,`- peak footprint (limit ${verdict.limitMegabytes} MB): ${Object.entries(verdict.peakFootprintMegabytes).map(([machine,peak])=>`${machine} ${peak} MB ${verdict.withinLimit[machine]?"within":"OVER"}`).join(", ")}`,`- owners per minute: ${verdict.ownersPerMinute}, time to the end: ${verdict.elapsedSeconds} s`,`- orphan processes after the end: ${verdict.orphans}, work directory removed: ${verdict.workRemoved}`,...verdict.landing===undefined?[]:[`- landing at ${verdict.landing.atSeconds} s, revision moved: ${verdict.landing.movedRevision}: stored results lost ${verdict.landing.storedLost} of ${verdict.landing.storedBefore}, pool idle with owners open ${verdict.landing.poolIdleSeconds} s (sum over machines)`]];
 return `${verdictLines.join("\n")}\n\n${lines.join("\n")}\n`;
}

async function main(argv:readonly string[]):Promise<number>{
 const option=(name:string):string|undefined=>argv.find(value=>value.startsWith(`--${name}=`))?.slice(name.length+3),flag=(name:string)=>argv.includes(`--${name}`);
 if(argv[0]==="audit"){const audit=auditIsolationV1(option("root")??process.env.SEMIO_WORKSPACE_ROOT??process.cwd());process.stdout.write(JSON.stringify(audit,null,1)+"\n");return audit.shared.length===0?0:1;}
 if(argv[0]!=="rehearse")throw Error("usage: rehearse|audit");
 const number=(name:string,fallback:number)=>option(name)===undefined?fallback:Number(option(name));
 const options=readRehearsalOptionsV1({machines:number("machines",3),memoryMegabytes:number("memory",16384),cores:number("cores",8),owners:number("owners",470),ownerSeconds:number("owner-seconds",3),ownerMegabytes:number("owner-megabytes",128),scenario:option("scenario")??"steady",...option("landing-after")===undefined?{}:{landingAfterSeconds:number("landing-after",30)},...flag("landing-keeps-revision")?{landingMovesRevision:false}:{},timeoutSeconds:number("timeout",900),...option("out")===undefined?{}:{out:option("out")}});
 const stop=new AbortController();for(const name of["SIGINT","SIGTERM"] as const)process.on(name,()=>stop.abort());
 const result=await runRehearsalV1(options,{say:line=>process.stderr.write(line+"\n"),keep:flag("keep"),signal:stop.signal}),machines=[...new Set(result.timeline.map(row=>row.machine))];
 const text=reportV1(result.verdict,result.timeline as never,machines);
 if(options.out!==undefined){mkdirSync(options.out,{recursive:true});writeFileSync(join(options.out,"verdict.json"),JSON.stringify({options,limits:rehearsalLimitsV1(options.memoryMegabytes),verdict:result.verdict},null,1));writeFileSync(join(options.out,"report.md"),text);}
 process.stdout.write(text);
 return result.verdict.pass?0:1;
}
if(import.meta.main)main(process.argv.slice(2)).then(code=>process.exit(code),error=>{process.stderr.write((error instanceof Error?error.message:String(error))+"\n");process.exit(1);});
