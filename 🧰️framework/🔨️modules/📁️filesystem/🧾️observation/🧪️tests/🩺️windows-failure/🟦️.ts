/** 🩺️ A failed Windows relative open names its code, origin, root, entry, role and access; the pure message is checked against hand-authored text and an independent Python oracle. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {artifactWindowsRelativeFailureMessageV1,artifactWindowsRootAccessLadderV1} from "../../📁️descriptor/🟦️.ts";
import fixture from "./🔣️.json";

test("the failure message names code, origin, root, entry, role and access exactly",()=>{
  for(const row of fixture.rows)expect(artifactWindowsRelativeFailureMessageV1(row.operation,row.code,row.origin as "nt"|"win32",row.context)).toBe(row.message);
});

test("an independent Python oracle composes the same messages",()=>{
  const run=spawnSync("python3",[fileURLToPath(new URL("./🐍️oracle.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});
  expect(run.status).toBe(0);expect(JSON.parse(run.stdout)).toEqual(fixture.rows.map(row=>row.message));
});

test("a long root is bounded",()=>{
  expect(artifactWindowsRelativeFailureMessageV1("open",5,"win32",{root:"C:\\"+"x".repeat(5000)}).length).toBeLessThan(300);
});

test("a drive root that refuses write access is retried read-only, a deeper root keeps reading first",()=>{
  expect(artifactWindowsRootAccessLadderV1(0)).toEqual([0xc0000000,0x80000000,0x120081]);
  expect(artifactWindowsRootAccessLadderV1(2)).toEqual([0x80000000,0x120081]);
});
