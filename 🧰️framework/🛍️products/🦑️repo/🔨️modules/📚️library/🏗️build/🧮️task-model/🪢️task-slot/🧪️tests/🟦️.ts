import {expect,test} from "bun:test";
import {taskPlatformDependenceV1,TASK_SLOT_POLICY_V1} from "../🟦️.ts";

const named={default:["{projectRoot}/**/*","sharedGlobals"],sharedGlobals:[{runtime:"repo tools exec --light node -- -p \"process.platform.concat(process.arch)\""},{runtime:"repo tools exec --light bun -- --version"},{env:"SEMIO_BUILD_MODE"}],neutral:["{projectRoot}/**/*",{runtime:"repo tools exec --light bun -- --version"},{env:"SEMIO_BUILD_MODE"},{externalDependencies:["x"]}],nativeSources:["{projectRoot}/**/*.rs"],cc:[{env:"CC"}]};

test("a task reading the default group is platform-bound, a task reading only neutral facts is not",()=>{
 expect(taskPlatformDependenceV1({inputs:["default","^default"],namedInputs:named}).platform).toBe(true);
 expect(taskPlatformDependenceV1({inputs:undefined,namedInputs:named}).platform).toBe(true);
 expect(taskPlatformDependenceV1({inputs:["neutral","^neutral",{dependentTasksOutputFiles:"**/*"},{fileset:"{projectRoot}/a"}],namedInputs:named})).toEqual({platform:false,reasons:[]});
});

test("native sources, toolchain environment, unknown runtime commands, native execution and the unresolvable are platform-bound",()=>{
 for(const inputs of[["neutral","nativeSources"],["^nativeSources"],["neutral","cc"],["neutral",{runtime:"rustc -vV"}],["neutral",{env:"HOME"}],["neutral",{input:"nativeSources",projects:"x"}],["missing"],["neutral",7 as unknown],["neutral",{weird:1}]])expect(taskPlatformDependenceV1({inputs:inputs as unknown[],namedInputs:{...named,missing:undefined}}).platform,JSON.stringify(inputs)).toBe(true);
 expect(taskPlatformDependenceV1({inputs:["neutral"],namedInputs:named,executionKind:"native"}).platform).toBe(true);
 expect(taskPlatformDependenceV1({inputs:["default"],namedInputs:{}}).reasons).toContain("unresolved:default");
 expect(TASK_SLOT_POLICY_V1.domain).toBe("nx-task");
});
