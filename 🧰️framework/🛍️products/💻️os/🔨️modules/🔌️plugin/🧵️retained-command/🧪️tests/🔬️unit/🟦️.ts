import {expect,test} from "bun:test";
import {pluginTestRunnerSelfTests} from "../../../📦️packages/🦀️rust/📜️script";

test("plugin runner forwards exact library and integration targets against Node's independent argument parser",async()=>{
 expect(await pluginTestRunnerSelfTests()).toBe(10);
});

