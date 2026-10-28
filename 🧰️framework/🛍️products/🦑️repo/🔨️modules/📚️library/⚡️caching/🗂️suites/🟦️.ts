import{resolve}from"node:path";
import{discoverProcessTestFilesV1,processTestCoverageV1,readProcessTestTableV1,type ProcessTestTableV1}from"../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🗂️suites/🟦️.ts";
import table from"./🔣️.json"with{type:"json"};

/** 🗂️ The module root every file of the caching route table is relative to. */
export const cachingTestRootV1:string=resolve(import.meta.dirname,"..");
/** 📖️ Reads the closed caching route table under the process module's route schema: every test file of the caching module belongs to exactly one suite, every suite is run by the project target `test-<id>`, and each test file runs in its own process. */
export function readCachingTestTableV1(value:unknown=table):ProcessTestTableV1{return readProcessTestTableV1(value);}
/** ⚖️ Compares the caching route table with the test files of the caching module: unrouted, duplicated, name-filtered, missing and stale-helper entries. */
export function cachingTestCoverageV1(declared:ProcessTestTableV1=readCachingTestTableV1()):ReturnType<typeof processTestCoverageV1>{return processTestCoverageV1(cachingTestRootV1,declared,discoverProcessTestFilesV1(cachingTestRootV1));}
