import { expect, test } from "bun:test";
import { LocalDocumentOwnerRegistryV1 } from "../../🗣️Interpreter/🧭️local-document-owner/🟦️.ts";
import { LocalSourceRevisionRegistryV1 } from "./registry.ts";
import { beginDocumentSourceWorkV1 } from "./document.ts";

const identity = { pluginId: "ifc", appId: "model", sessionInstanceId: 7, runtimeKey: "space:document", clientInstanceId: "client-a" };

test("background entry resolves the same actual real owner as the focused document", () => {
  const owners = new LocalDocumentOwnerRegistryV1(), sources = new LocalSourceRevisionRegistryV1();
  const plugin = { subscribeOperationCompletions: () => () => {} };
  let refreshes = 0;
  const work = beginDocumentSourceWorkV1({ owners, sources, refresh: (actual, instance) => { expect(actual).toBe(plugin); expect(instance).toBe(7); refreshes++; } }, identity, plugin)!;
  const owner = owners.acquire(identity), source = sources.acquire(owner, plugin)!;
  expect(source.begin(owner, plugin)).toBeNull();
  work.admit(undefined);
  work.finish();
  expect(source.begin(owner, plugin)).not.toBeNull();
  expect(refreshes).toBe(1);
});

test("coincident client identity and retired document cannot release another real owner", () => {
  const owners = new LocalDocumentOwnerRegistryV1(), sources = new LocalSourceRevisionRegistryV1();
  const plugin = { subscribeOperationCompletions: () => () => {} };
  const host = { owners, sources, refresh: () => {} };
  const first = beginDocumentSourceWorkV1(host, identity, plugin)!;
  const nextIdentity = { ...identity, clientInstanceId: "client-b" };
  const second = beginDocumentSourceWorkV1(host, nextIdentity, plugin)!;
  const nextOwner = owners.acquire(nextIdentity), nextSource = sources.acquire(nextOwner, plugin)!;
  owners.retire(identity);
  first.admit(undefined); first.finish();
  expect(nextSource.begin(nextOwner, plugin)).toBeNull();
  second.admit(undefined); second.finish();
  expect(nextSource.begin(nextOwner, plugin)).not.toBeNull();
});
