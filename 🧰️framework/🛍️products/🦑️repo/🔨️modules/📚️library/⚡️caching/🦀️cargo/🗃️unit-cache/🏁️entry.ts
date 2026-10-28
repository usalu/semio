import {runUnitCacheMainV1} from "./📜️script.ts";

/** 🏁️ The entry compiled into the native compiler wrapper (`bun build --compile`): the Windows stand-in Cargo can execute directly, because a batch file cannot take the quoted arguments Cargo passes to the compiler. */
process.exitCode=await runUnitCacheMainV1(process.argv.slice(2),process.env,process.cwd());
