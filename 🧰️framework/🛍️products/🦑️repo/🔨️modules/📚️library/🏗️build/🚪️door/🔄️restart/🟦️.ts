import {runWithStaleReexecV1} from "../../🧷️format-stamps/🔄️reexec/🟨️.mjs";
import {useProcessWideImplementationPinV1} from "../../📥️inputs/🔏️revision/🟨️.mjs";
import {graphQueueYieldPolicyV1,waitGraphQueueYieldV1} from "../../🕸️graph/🕰️generations/🛌️yield/🟨️.mjs";
import {tmpdir} from "node:os";
import {join} from "node:path";

const invocationEnvironmentV1="SEMIO_SCRIPT_PROCESS_INVOCATION";
/** 🔖️ The environment variable that marks a command started inside another one, spelled as the routing module spells it (a law compares the two) so the front door need not load the routing module to know whether it is the outermost command. */
export const BUILD_DOOR_INVOCATION_ENVIRONMENT_V1:string=invocationEnvironmentV1;

/** ♻️ Runs the build implementation of one command line as the unit a fresh process can run again: the loading of the implementation is part of `run`, so a module a peer is moving or half editing while it loads, a raw filesystem race and a moved format stamp all end in the declared restart (the outermost command starts again with the current implementation, bounded, paused and announced in both languages by the format stamp protocol) instead of an uncaught failure; a nested command only yields its exit code. The outermost command also offers its children a yield ticket: a run that only waits for a peer's graph build ends with the restart code and a ticket, and this front door waits ALONE (no loader, no child) for the next generation before it starts the same command again (see `../../🕸️graph/🕰️generations/🛌️yield`). This process is one invocation unless it is the resident service, which runs invocations side by side and never opts into the process-wide pin.
 * @see ../../🧷️format-stamps/🔄️reexec/🟨️.mjs */
export async function runBuildWithRestartV1(run:()=>Promise<void>,argv:readonly string[],environment:NodeJS.ProcessEnv=process.env):Promise<void>{
 if(!argv.slice(0,3).includes("service"))useProcessWideImplementationPinV1();
 const outermost=environment[invocationEnvironmentV1]===undefined,ticket=graphQueueYieldPolicyV1().environment.ticket;
 if(!outermost)delete environment[ticket];else if(environment[ticket]===undefined)environment[ticket]=join(tmpdir(),"semio-graph-yield-"+process.pid+".json");
 const outcome=await runWithStaleReexecV1(run,{environment,outermost,yielded:next=>waitGraphQueueYieldV1({environment:next})});
 if(outcome.status!=="completed")process.exit(outcome.code);
}
