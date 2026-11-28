import {writeSync} from "node:fs";

/** 🏁️ The final line of a plan request that ended without a result or an error (an event loop that ran dry with the request unsettled, an exit while it waited, a signal): both languages, naming how it ended. */
export function planVerdictTextV1(how:string):string{
 return "The graph plan request ended without an answer ("+how+"); no plan was made, see the lines above and run the command again / Die Graph-Plananfrage endete ohne Antwort ("+how+"); es wurde kein Plan erstellt, siehe die Zeilen darüber; den Befehl erneut ausführen\n";
}

/** 🏁️ Guarantees a typed final verdict for the request: until `done()` is called, an exit of the process (also the Bun/Node exit of a drained event loop with an unsettled promise) prints the bilingual verdict and makes the exit code nonzero. No signal handler is installed: in Bun a signal listener keeps the event loop alive, which would turn a silent exit into a hang. `done()` after the result or the error removes the hook, so a finished request prints nothing here.
 * @see ./🧪️tests/🟦️.ts */
export function guardPlanVerdictV1(ports:{write?:(text:string)=>void;process?:Pick<NodeJS.Process,"on"|"off"|"exitCode">}={}):Readonly<{done:()=>void}>{
 const host=ports.process??process,write=ports.write??(text=>{try{writeSync(2,text);}catch{}});
 let settled=false;
 const onExit=(code:number)=>{if(settled)return;settled=true;write(planVerdictTextV1(code===0?"the process ended while the request was open":"exit code "+code));if(code===0)host.exitCode=1;};
 host.on("exit",onExit);
 return Object.freeze({done(){settled=true;host.off("exit",onExit);}});
}
