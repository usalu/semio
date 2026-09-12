#!/usr/bin/env bun
/** 🌊️ Dispatches the actor-import runtime contract from its canonical test case. */
import assert from "node:assert/strict";
import { existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { type ActorImportFactoryPort, testCanonicalActorAsyncImport } from "../../🧪️tests/🌊️actor-import/🟦️.ts";

function repositoryRoot(start: string): string {
  let cursor = resolve(start);
  while (!existsSync(join(cursor, "nx.json"))) {
    const parent = dirname(cursor);
    if (parent === cursor) throw new Error("actor import fixture: repository root unavailable");
    cursor = parent;
  }
  return cursor;
}

if (import.meta.main) {
  const root = repositoryRoot(import.meta.dir);
  const bundle = await import(pathToFileURL(join(import.meta.dir, "..", "..", "📜️script.ts")).href) as { closedBrowserActorBundle: ActorImportFactoryPort };
  const command = process.argv[2] ?? "runtime-check";
  assert(["runtime-check", "pending-host-close-check"].includes(command), `actor import fixture: unknown command ${command}`);
  const evidenceRoot = join(import.meta.dir, "dist", command);
  await testCanonicalActorAsyncImport(root, evidenceRoot, undefined, bundle.closedBrowserActorBundle, true);
}
