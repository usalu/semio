/** 🎫️ Contract: only the outermost command of a run offers the graph queue yield ticket: the resident graph service is a long-lived command whose workers have no front door to wait for the next generation, so it never carries a ticket (a worker that offered one left the queue with the restart code every 15 s for ever), a nested command never does, and an outermost build command does.
 * @see ../../🔄️restart/🟦️.ts */
import {expect,test} from "bun:test";
import {graphQueueYieldPolicyV1} from "../../../🕸️graph/🕰️generations/🛌️yield/🟨️.mjs";
import {runBuildWithRestartV1} from "../../🔄️restart/🟦️.ts";

const ticket=graphQueueYieldPolicyV1().environment.ticket;
const seen=async(argv:string[],environment:Record<string,string|undefined>)=>{let during:string|undefined;await runBuildWithRestartV1(async()=>{during=environment[ticket];},argv,environment as NodeJS.ProcessEnv);return during;};

test("the service command gets no ticket even when it inherited one, a nested command none, an outermost build command one",async()=>{
 expect(await seen(["repo","service","host","run"],{})).toBeUndefined();
 expect(await seen(["repo","service","host","run"],{[ticket]:"/tmp/inherited.json"})).toBeUndefined();
 expect(await seen(["repo","show","projects"],{SEMIO_SCRIPT_PROCESS_INVOCATION:"{}",[ticket]:"/tmp/parent.json"})).toBeUndefined();
 expect(await seen(["repo","show","projects"],{})).toMatch(/semio-graph-yield-\d+\.json$/u);
});
