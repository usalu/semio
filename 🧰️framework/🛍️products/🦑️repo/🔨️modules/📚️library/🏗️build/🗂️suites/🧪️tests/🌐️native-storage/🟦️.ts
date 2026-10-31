import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import{readBuildTestTableV1}from"../../🟦️.ts";
import{routeProcessTestFilesV1,processTestSuiteRunsOnV1}from"../../../../../../../../🔨️modules/🏃️process/🧪️testing/🗂️suites/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};
declareLibraryBuildTestEnvelopeV1({suite:"build-route-table"});
test("Native integration routes retain exactly one declared owner on every supported platform, agreeing with SQLite",()=>{
 expect(Object.keys(fixture).sort()).toEqual(["version","suite","files","platforms","scratch","budgetMs"].sort());expect(fixture.version).toBe(1);expect(fixture.files.length).toBe(5);expect(fixture.platforms).toEqual(["darwin","linux","win32"]);
 const table=readBuildTestTableV1(),routes=routeProcessTestFilesV1(fixture.files,table),db=new Database(":memory:");
 try{for(const file of fixture.files){const owners=routes.get(file)!;expect(owners.map(row=>row.id),file).toEqual([fixture.suite]);const owner=owners[0]!;expect(owner.scratch===fixture.scratch,file+" scratch").toBe(true);expect(owner.budgetMs).toBe(fixture.budgetMs);for(const platform of fixture.platforms){const oracle=(db.query("select exists(select 1 from json_each(?) where value=?) as allowed").get(JSON.stringify(owner.platforms),platform)as{allowed:number}).allowed===1;expect(oracle,file+":"+platform).toBe(true);expect(processTestSuiteRunsOnV1(owner,platform),file+":"+platform).toBe(oracle);}}}finally{db.close();}
});
