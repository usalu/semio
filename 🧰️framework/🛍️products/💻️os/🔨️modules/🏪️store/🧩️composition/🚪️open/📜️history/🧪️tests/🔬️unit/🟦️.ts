import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import history from "../../🧫️fixtures/🔣️.json" with { type: "json" };
import factory from "../../🏭️factory/🧫️fixtures/🔣️.json" with { type: "json" };
import dictionary from "../../🗂️dictionary/🧫️fixtures/🔣️.json" with { type: "json" };

test("retained member logical ownership census includes the explicit opened actor once", () => {
  const utf8 = (value: string) => new TextEncoder().encode(value).length;
  for (const fixture of [history, factory, dictionary]) {
    expect(Buffer.byteLength(fixture.openedActor, "utf8")).toBe(utf8(fixture.openedActor));
    expect(utf8(fixture.openedActor)).toBe(28);
  }
  const identity = Object.values(history.identity).reduce((bytes, value) => bytes + utf8(value), 0);
  expect(Buffer.from(history.historyHex, "hex").length).toBe(255);
  for (const row of history.inputs) expect(row.retiredBytes).toBe(row.historyBytes + 2 + identity + utf8(history.openedActor));
  for (const row of history.lifecycle) expect(row.retiredBytes).toBe(255 + 2 + identity + utf8(history.openedActor));
  const factoryIdentity = factory.requestIdentity.reduce((bytes, value) => bytes + utf8(value), 0);
  expect(factory.retiredBytes).toBe(factory.inputBytes + factoryIdentity + utf8(factory.openedActor));
  expect(factory.caseRetirement["selected-flow"]).toBe(factory.retiredBytes);
  expect(factory.caseRetirement["selected-text"]).toBe(factory.retiredBytes);
  console.log(`[DEBUG] retained logical input census: wire=255 frame=2 identity=${identity} opened-actor=${utf8(history.openedActor)} exact-logical-total=${255 + 2 + identity + utf8(history.openedActor)}; physical allocation release is independently observed in native tests`);
});
