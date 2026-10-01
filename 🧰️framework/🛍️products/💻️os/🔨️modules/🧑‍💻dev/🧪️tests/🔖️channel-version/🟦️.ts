/** 🔖️Checks present-owner protocol census and bounded writes through first-party source ports. */
import { describe, expect, it } from "vitest";
import { channelVersionCensus, channelVersionLiterals, channelVersionCensusRoots, writeChannelVersionConsumers } from "../../🔖️channel-version/🔍️census/🟦️.ts";
import { admitChannelVersionContributionsV1 } from "../../🔖️channel-version/📣️contributions/🟦️.ts";
import { proveChannelVersionContributionsV1, proveChannelVersionContributionCensusV1 } from "../../🔖️channel-version/📣️contributions/🧪️tests/🟦️.ts";

describe("channel version authority", () => {
  it("admits closed outward declarations and strictly checks surviving owners", () => {
    expect(proveChannelVersionContributionsV1()).toBeGreaterThan(0);
    expect(proveChannelVersionContributionCensusV1()).toBeGreaterThan(0);
  });

  it("derives current source roots without requiring a concrete owner", () => {
    expect(channelVersionCensusRoots([], [])).toEqual(["🧰️framework"]);
    expect(channelVersionCensusRoots([{ ownerRel: "future/plugin" }], [])).toEqual(["future", "🧰️framework"]);
    expect(channelVersionCensusRoots([], [{ path: "other/schema.json", occurrences: 1 }])).toEqual(["other", "🧰️framework"]);
  });

  it("finds each language's literal shape once with its exact source span", () => {
    const text = ['pub const CHANNEL_VERSION: u32 = 18;', 'export const APP_CHANNEL_VERSION = 18;', '"executionProtocol": { "appChannelVersion": 18 },', '"appChannelVersion": {\n"const": 18\n}', 'return row.appChannelVersion === 18 ? { appChannelVersion: 18 } : fail();', 'execution_protocol: ExecutionProtocol { app_channel_version: 14 },', '"required": ["appChannelVersion"],', '"selection": { "executionProtocol": 17 }'].join("\n");
    const literals = channelVersionLiterals(text);
    expect(literals.map(row => row.value)).toEqual([18, 18, 18, 18, 18, 18, 14, 17]);
    for (const row of literals) expect(text.slice(row.valueStart, row.valueStart + String(row.value).length)).toBe(String(row.value));
    const encoded = Buffer.alloc(8);
    encoded.writeDoubleLE(17);
    expect(channelVersionLiterals(`6170704368616e6e656c56657273696f6e05${encoded.toString("hex")}`).map(row => row.value)).toEqual([17]);
  });

  it("writes through the supplied port while preserving hostile, arbitrary and owner-derived values", () => {
    const consumers = admitChannelVersionContributionsV1([{ ownerRoot: "future", document: { schema: "semio.os.channel-version-consumers/v1", consumers: [{ path: "current.json", occurrences: 1 }, { path: "hostile.json", occurrences: 1, hostileValues: [13] }, { path: "guest.json", occurrences: 1, guest: true }, { path: "derived.json", occurrences: 1, derived: "Owner digest" }, { path: "arbitrary.json", occurrences: 1, arbitrary: true }] } }]);
    const files = new Map(consumers.map(row => [row.path, JSON.stringify({ appChannelVersion: row.hostileValues?.[0] ?? 20 })]));
    const source = { pin: 21, consumers, candidates: [...files.keys()], readText: (path: string) => files.get(path)! };
    const writeText = (path: string, text: string) => { files.set(path, text); };
    const first = writeChannelVersionConsumers(source, { guest: false, writeText });
    expect(first.written).toEqual(["future/current.json"]);
    expect(first.refused).toHaveLength(2);
    expect(JSON.parse(files.get("future/hostile.json")!).appChannelVersion).toBe(13);
    expect(JSON.parse(files.get("future/arbitrary.json")!).appChannelVersion).toBe(20);
    expect(writeChannelVersionConsumers(source, { guest: true, writeText }).written).toEqual(["future/guest.json"]);
    expect(channelVersionCensus(source).findings.map(row => [row.path, row.problem])).toEqual([["future/derived.json", "drift"]]);
  });
});
