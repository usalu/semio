import {runTestFileCliV1} from "../../../⌨️command/🟦️.ts";

process.exit(await runTestFileCliV1(process.argv[2]!, process.argv.slice(3), {...process.env, SEMIO_LANGUAGE: "en"}));
