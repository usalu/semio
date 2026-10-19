import {test,expect} from "bun:test";
import {readFileSync,readdirSync} from "node:fs";
import {join} from "node:path";
import {receiveScriptProcessInvocation} from "../../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../🏗️build/🎟️capacity/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../🎛️owned-execution/🟦️.ts";
import {checkResourceLeaseControl} from "../../../../../../🔒️leases/🟦️.ts";

/** 📋️ Real compiler callers typecheck the required target owner and native bridges retain one canonical lease. */
test("Cargo compiler target custody is required across every active caller",async()=>{
    await receiveScriptProcessInvocation(process.env,async original=>{
        const workspace=process.cwd(),control=scriptCompilerAdmissionControlV1(original,"test:cargo-target-callers",60000),files:string[]=[];const started=performance.now();let units=0,published=Number.NEGATIVE_INFINITY;
        const visit=async(path:string):Promise<void>=>{for(const row of readdirSync(path,{withFileTypes:true})){if(++units%64===0){checkResourceLeaseControl(control);const now=performance.now();if(now-published>=1000){await control.publish({resource:"test:cargo-target-callers",mode:"shared",elapsedMs:now-started});published=now}await control.yieldContinuation()}if(row.isSymbolicLink())continue;const current=join(path,row.name);if(row.isDirectory()){if(!["node_modules","🗑️generated","dist","target",".cache"].includes(row.name))await visit(current)}else if(["🟦️.ts","📜️script.ts"].includes(row.name)){const source=readFileSync(current,"utf8");if(source.includes("acquireCargoBuildLeaseV1(")||source.includes("runCargoRelayV1(")||source.includes("runCargoCommandV1("))files.push(current)}}};
        for(const owner of ["🧰️framework","🌎️hub","✏️s"])await visit(join(workspace,owner));
        expect(files.length).toBeGreaterThan(20);
        await captureOwnedCommandV1(process.execPath,[Bun.resolveSync("typescript/bin/tsc",import.meta.dir),"--ignoreConfig","--noEmit","--allowJs","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",...files],workspace,{control});
    });
},90000);
