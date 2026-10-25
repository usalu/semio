import{guardRuntimeV1}from"./⌨️command/🟦️.ts";

try{await guardRuntimeV1();}catch(error){process.stderr.write(`[runtime guard] skipped: ${(error as Error).message}\n`);}
