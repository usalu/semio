/** ⌨️ The entry of the host warden: `bun 🟦️.ts [--observe] [--directory <host capacity directory>] [--within <pid>] [--handover <pid>]` runs the loop with the real machine until it has no subject for a while, its sources changed or it is terminated (SIGTERM and SIGINT resume everything it suspended, then end it). The mode is `observe` with the argument, else the content of `warden/mode` (`observe` or `enforce`), else the authored `defaultMode` of the policy.
 * @see ../🔁️loop/🟦️.ts */
import buildPolicy from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json" with {type:"json"};
import {readHostCapacityPolicyV1} from "../../📏️decision/🟦️.ts";
import {defaultHostWardenPortsV1,runHostWardenLoopV1,spawnHostWardenSuccessorV1} from "../🔁️loop/🟦️.ts";

const argv=process.argv.slice(2),directoryAt=argv.indexOf("--directory"),handoverAt=argv.indexOf("--handover"),withinAt=argv.indexOf("--within"),controller=new AbortController();
for(const name of["SIGTERM","SIGINT"] as const)process.on(name,()=>controller.abort());
process.on("SIGHUP",()=>undefined);
const hostPolicy=readHostCapacityPolicyV1((buildPolicy as unknown as {hostCapacity:unknown}).hostCapacity);
const ports=defaultHostWardenPortsV1({hostPolicy,observe:argv.includes("--observe"),...directoryAt>=0?{directory:argv[directoryAt+1]!}:{}});
const home=process.env.HOME??process.env.USERPROFILE??"";
const within=withinAt>=0?Number(argv[withinAt+1]):undefined,observe=argv.includes("--observe"),directory=directoryAt>=0?argv[directoryAt+1]!:undefined;
const result=await runHostWardenLoopV1({ports,signal:controller.signal,prefixes:[process.cwd()+"/",...home?[home+"/"]:[]],...handoverAt>=0?{handover:Number(argv[handoverAt+1])}:{},...within===undefined?{}:{within},successor:()=>spawnHostWardenSuccessorV1({observe,...directory===undefined?{}:{directory},...within===undefined?{}:{within}})});
void result;process.exit(0);
