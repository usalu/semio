import { expect, test } from "bun:test";
import Ajv from "ajv";
import { existsSync, mkdirSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";
import { testCacheRoot, testCacheDir, markOutputDir, readOutputMarker, planExecution, type DiscoveredCase } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { materializeRustHost } from "../../🖥️host/🏗️materialization/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧾️artifact-root-source/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🧾️artifact-root-source/🔣️.json"), "utf8"));
const callerRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!callerRoot) throw Error("Artifact-root laws require caller-owned SEMIO_TEST_ARTIFACT_DIR");
const outputs = resolve(callerRoot, "artifact-root-source");
mkdirSync(outputs, {recursive:true});

function environment<T>(values: Record<string, string | undefined>, action: () => T): T {
    const previous = Object.fromEntries(Object.keys(values).map(key => [key,process.env[key]]));
    try {
        for (const [key,value] of Object.entries(values)) { if(value === undefined) delete process.env[key]; else process.env[key]=value; }
        return action();
    } finally {
        for (const [key,value] of Object.entries(previous)) { if(value === undefined) delete process.env[key]; else process.env[key]=value; }
    }
}

test("validates closed neutral output roles with independent Ajv", () => {
    const validate = new Ajv({strict:true,allErrors:true}).compile(schema);
    expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
    expect(validate({...fixture,opaqueRoot:"elsewhere"})).toBe(false);
    expect(fixture.children).toEqual(["work","hosts","results","reports"]);
});

test("selects caller-owned scoped artifacts while preserving canonical default", () => {
    environment({SEMIO_TEST_ARTIFACT_DIR:undefined,SEMIO_TEST_OUTPUT_SCOPE:undefined},()=>expect(testCacheRoot(root)).toBe(join(root,".🧬semio","🦑️repo","⚡️cache","tests")));
    environment({SEMIO_TEST_ARTIFACT_DIR:join(outputs,"paths"),SEMIO_TEST_OUTPUT_SCOPE:fixture.scope},()=>{
        expect(testCacheRoot(root)).toBe(join(outputs,"paths"));
        for(const child of fixture.children) expect(testCacheDir(root,child)).toBe(resolve(outputs,"paths","tasks","lane","scenario",child));
        expect(()=>testCacheDir(root,"unowned")).toThrow();
        for(const scope of fixture.invalidScopes) environment({SEMIO_TEST_OUTPUT_SCOPE:scope},()=>expect(()=>testCacheDir(root,"work"),JSON.stringify(scope)).toThrow());
    });
});

test("materializes exact owned host plan and result paths without compiling", () => {
    const artifact=join(outputs,"execution"),inputs=join(outputs,"inputs");
    environment({SEMIO_TEST_ARTIFACT_DIR:artifact,SEMIO_TEST_OUTPUT_SCOPE:fixture.scope,CARGO_TARGET_DIR:join(outputs,"warm"),CARGO_BUILD_BUILD_DIR:join(outputs,"warm")},()=>{
        expect(testCacheRoot(root)).toBe(artifact);
        mkdirSync(inputs,{recursive:true});
        const featurePath=join(inputs,"🥒️.feature"),adapterPath=join(inputs,"🦀️.rs");
        writeFileSync(featurePath,"Feature: explicit owned output\n  @level-quick\n  @id-owned-output\n  @mode-property\n  Scenario: keeps its caller-owned paths\n    Given a declared owner\n    Then an unchanged literal\n");
        writeFileSync(adapterPath,"pub fn adapter() -> semio_repo_test_host::Adapter { semio_repo_test_host::Adapter::new(\"rust\") }\n");
        const discovered:DiscoveredCase={owner:relative(root,inputs).split("\\").join("/"),ownerName:"owned-output",case:"fixture",caseDir:relative(root,inputs).split("\\").join("/"),featurePath:relative(root,featurePath).split("\\").join("/"),adapters:{rust:relative(root,adapterPath).split("\\").join("/")},sharedFixtureDir:null,projectName:"owned-output-fixture"};
        const planned=planExecution(root,discovered,"quick","subject","rust");
        expect(planned.plan.scenarios).toHaveLength(1);
        const work=join(artifact,"tasks","lane","scenario","work","owned-output-fixture-subject-rust");
        const results=join(artifact,"tasks","lane","scenario","results","owned-output-fixture-subject-rust");
        expect(planned.planPath).toBe(join(work,"📋️plan.json"));
        expect(planned.plan.resultsPath).toBe(join(results,"📤️results.jsonl"));
        expect(planned.plan.artifactDir).toBe(join(results,"📦️artifacts"));
        expect(readOutputMarker(root,work)?.testId).toBe(discovered.owner+"::fixture");
        expect(JSON.parse(readFileSync(planned.planPath,"utf8")).resultsPath).toBe(planned.plan.resultsPath);
        const host=materializeRustHost(root,discovered,"subject",planned.planPath,planned.plan.resultsPath);
        expect(host.hostDir).toBe(join(artifact,"tasks","lane","scenario","hosts","owned-output-fixture-subject-rust"));
        expect(host.args).toEqual(["--plan",planned.planPath,"--out",planned.plan.resultsPath]);
        expect(host.env.CARGO_TARGET_DIR).toBe(join(outputs,"warm"));
        expect(host.env.CARGO_BUILD_BUILD_DIR).toBe(join(outputs,"warm"));
        const main=readFileSync(join(host.hostDir!,"src","main.rs"),"utf8");
        expect(main).toContain(JSON.stringify(adapterPath));
        expect(main).toContain("semio_repo_test_host::run_main(adapter::adapter())");
        expect(existsSync(join(host.hostDir!,"Cargo.toml"))).toBe(true);
        expect(existsSync(planned.plan.resultsPath)).toBe(false);
    });
},{timeout:30000});

test("refuses escape and foreign output ownership before marker writes", () => {
    const artifact=join(outputs,"markers"),owned=join(artifact,"work","owned");
    environment({SEMIO_TEST_ARTIFACT_DIR:artifact,SEMIO_TEST_OUTPUT_SCOPE:undefined},()=>{
        expect(testCacheRoot(root)).toBe(artifact);
        expect(()=>markOutputDir(root,join(artifact,"..","foreign"),fixture.marker)).toThrow();
        markOutputDir(root,owned,fixture.marker);
        const before=readOutputMarker(root,owned);
        expect(()=>markOutputDir(root,owned,{testId:"foreign-owner::fixture",cacheKey:"other"})).toThrow();
        expect(readOutputMarker(root,owned)).toEqual(before);
        markOutputDir(root,owned,{...fixture.marker,cacheKey:"source-v2"});
        expect(readOutputMarker(root,owned)?.cacheKey).toBe("source-v2");
        const malformed=join(artifact,"work","malformed");
        mkdirSync(malformed,{recursive:true});
        writeFileSync(join(malformed,"🔣️.json"),"{\"kind\":\"foreign\"}");
        expect(()=>markOutputDir(root,malformed,fixture.marker)).toThrow();
    });
});

test("refuses symlink escapes from the declared artifact owner", () => {
    const artifact=join(outputs,"symlink-owner"),outside=join(outputs,"outside-owner"),link=join(artifact,"escape");
    environment({SEMIO_TEST_ARTIFACT_DIR:artifact,SEMIO_TEST_OUTPUT_SCOPE:undefined},()=>{
        expect(testCacheRoot(root)).toBe(artifact);
        mkdirSync(artifact,{recursive:true});mkdirSync(outside,{recursive:true});
        if(!existsSync(link)) symlinkSync(outside,link,"junction");
        expect(()=>markOutputDir(root,join(link,"unowned"),fixture.marker)).toThrow();
        expect(existsSync(join(outside,"unowned"))).toBe(false);
    });
});
