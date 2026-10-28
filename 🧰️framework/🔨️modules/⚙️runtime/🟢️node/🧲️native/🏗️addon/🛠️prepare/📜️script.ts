import { isMainModuleV1 } from "../../../../🏁️main/🟦️.ts";
import { runNodeNativePrepareV1 } from "./🟦️.ts";

if (isMainModuleV1(import.meta.url)) process.exitCode = runNodeNativePrepareV1(process.argv.slice(2));
