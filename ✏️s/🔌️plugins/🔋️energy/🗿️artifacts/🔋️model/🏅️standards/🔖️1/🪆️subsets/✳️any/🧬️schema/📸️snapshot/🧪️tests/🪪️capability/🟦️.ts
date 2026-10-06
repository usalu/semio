import {test,expect} from "bun:test";
import * as owner from "../../../../../../../../🟦️.ts";
import corpus from "../../../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json";
test("Energy owning source facade exposes its explicit relational capability",()=>{
  for(const name of corpus.capabilityExports)expect(Object.hasOwn(owner,name)).toBe(true);
});

import {readFileSync} from "node:fs";
import {parse as parseJsonc} from "jsonc-parser";
test("Energy exact owning commands are present in both developer launch surfaces",()=>{
 const expected=["@semio-tech/energy-model-rs:test-snapshot-sqlite ","@semio-tech/energy-model-rs:test-snapshot-sqlite-native ","@semio-tech/energy-model-rs:test-snapshot-sqlite-source ","@semio-tech/energy-model-rs:build ","@semio-tech/energy-model-rs:check ","@semio-tech/energy-model-rs:test ","@semio-tech/energy-model:build ","@semio-tech/energy-model:check ","@semio-tech/energy-model:test "];
 for(const path of[".vscode/🧩️launch.seed.jsonc",".vscode/launch.json"]){const launch=parseJsonc(readFileSync(path,"utf8"))as{configurations:{command?:string;env?:{SEMIO_TEST_LEVEL?:string};presentation?:{order?:number}}[]};for(let n=0;n<expected.length;n++){const entries=launch.configurations.filter(entry=>entry.command?.includes(expected[n]!));expect(entries.length).toBe(1);expect(entries[0]!.presentation?.order).toBe(Number((408.623+n/1000).toFixed(3)));if(n!==3&&n!==4)expect(entries[0]!.env?.SEMIO_TEST_LEVEL).toBe("quick")}expect(launch.configurations.find(entry=>entry.command?.includes(expected[5]!))!.command).toContain("-- quick --no-fail-fast")}
});
