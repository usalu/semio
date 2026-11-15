import {nxOperationArgumentsV1} from "../../../../⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🟦️.ts";
import {buildOperationHasNeutralBootstrapV1} from "../../../⚙️operations/🟦️.ts";
import type {BuildBackendLaunchV1} from "../../../🔌️backends/🚀️launch/🟦️.ts";

/** 🚀️ The arguments nx is launched with for one command: the door asks this small module instead of loading the nx adapter.
 * @see ../🔌️backend/🟦️.ts the adapter that executes */
export const launch:BuildBackendLaunchV1=Object.freeze({
 bootstrapArguments(args:readonly string[]){const command=args[0];if(command==="tools")return ["--version"];if(command==="reset")return ["reset"];return command==="watch"||buildOperationHasNeutralBootstrapV1(command??"")?undefined:nxOperationArgumentsV1(args);},
});
