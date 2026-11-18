import {mkdirSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {sliceLabHomeReportV1,sliceLabWarmBuildV1} from "./🏠️home/🟦️.ts";
import {SLICE_LAB_POLICY_V1 as P,readSliceLabOptionsV1,runSliceLabV1,sliceLabReportV1,sliceLabTextV1} from "./🟦️.ts";

/** 🔬️ `slice-lab run [--machines=1|2] [--memory=16384|8192] [--cores=8] [--slice=a,b,...] [--target=build] [--timeout=1800] [--execute] [--stages=workspace|all] [--kinds=go,rust,...] [--frozen] [--stagger=<seconds>|-1] [--out=<dir>] [--work=<dir>] [--preload=<file>] [--lang=en|de] [--keep]` runs the real readiness path of a worker (the stage runner of the sidecar, the project graph read of the host service, the ready step) over a declared slice of the project graph on this machine, with its own empty machine state root and its own temporary fleet store, and prints the timeline and the verdict: ready, or the exact wall. Exit status 1 when the verdict fails. */
async function main(argv:readonly string[]):Promise<number>{
 const option=(name:string):string|undefined=>argv.find(value=>value.startsWith(`--${name}=`))?.slice(name.length+3),language=option("lang")==="de"?"de":"en";
 if(argv[0]==="warm"){const out=option("out");if(out===undefined){process.stderr.write("slice-lab warm --out=<dir>\n");return 2;}const rows=sliceLabWarmBuildV1(process.env.SEMIO_WORKSPACE_ROOT??process.cwd(),out);process.stdout.write(rows.map(row=>`${row.stage} ${row.id} ${row.method} ${row.seconds}s ${row.megabytes} MB`).join("\n")+"\n");return rows.every(row=>row.ok)?0:1;}
 if(argv[0]!=="run"){process.stderr.write(sliceLabTextV1(language,"usage")+"\n");return 2;}
 const number=(name:string,fallback:number)=>option(name)===undefined?fallback:Number(option(name));
 const options=readSliceLabOptionsV1({machines:number("machines",1),memoryMegabytes:number("memory",16384),cores:number("cores",8),slice:option("slice")?.split(",").filter(Boolean)??[...P.slice],target:option("target")??P.target,timeoutSeconds:number("timeout",P.defaultTimeoutSeconds),language,...argv.includes("--execute")?{execute:true}:{},...option("stages")===undefined?{}:{stages:option("stages")},...option("skip")===undefined?{}:{skip:option("skip")!.split(",").filter(Boolean)},...option("warm")===undefined?{}:{warm:option("warm")},...option("kinds")===undefined?{}:{kinds:option("kinds")!.split(",").filter(Boolean)},...argv.includes("--frozen")?{frozen:true}:{},...option("frozen")===undefined?{}:{frozenDirectory:option("frozen")},...option("edit")===undefined?{}:{edit:option("edit")},...option("stagger")===undefined?{}:{stagger:number("stagger",0)},...option("out")===undefined?{}:{out:option("out")},...option("work")===undefined?{}:{work:option("work")},...option("preload")===undefined?{}:{preload:option("preload")}});
 const stop=new AbortController();for(const name of["SIGINT","SIGTERM"] as const)process.on(name,()=>stop.abort());
 const result=await runSliceLabV1(options,{say:line=>process.stderr.write(line+"\n"),keep:argv.includes("--keep"),signal:stop.signal});
 const text=sliceLabReportV1(language,options,result.verdict,result.samples)+"\n## home\n\n"+sliceLabHomeReportV1(language,result.home).join("\n")+"\n";
 if(options.out!==undefined){mkdirSync(options.out,{recursive:true});writeFileSync(join(options.out,"report.md"),text);}
 process.stdout.write(text);
 return result.verdict.pass&&result.home.liveChanges.filter(path=>!path.includes("/.cache")).length===0?0:1;
}
if(import.meta.main)main(process.argv.slice(2)).then(code=>process.exit(code),error=>{process.stderr.write((error instanceof Error?error.message:String(error))+"\n");process.exit(1);});
