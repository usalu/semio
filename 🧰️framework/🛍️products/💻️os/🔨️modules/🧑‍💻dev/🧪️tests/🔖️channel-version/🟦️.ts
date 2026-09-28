/**
 * 🔖️ Laws of the one channel version authority (`🔖️channel-version/🟦️.ts`): the repository states the app-engine channel
 * version only through registered consumers of the pin, the census finds a hand-written copy wherever it hides, and the
 * generator rewrites exactly the drifted literals it may while leaving hostile, arbitrary, guest-linked and digest-bearing
 * consumers alone.
 *
 * @vitest-environment node
 */
import { existsSync, mkdtempSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";
import { describe, expect, it } from "vitest";
import { CHANNEL_VERSION_CONSUMERS, CHANNEL_VERSION_PIN_PATH, channelVersionCensus, channelVersionLiterals, writeChannelVersionConsumers } from "../../🔖️channel-version/🟦️.ts";
import { decodePackValue } from "../../../../🟦️.ts";

/** 🧭️ The repository root above this law (the directory holding `.mcp.json`). */
function repositoryRoot(): string {
  let current = dirname(fileURLToPath(import.meta.url));
  while (!existsSync(join(current, ".mcp.json"))) {
    const parent = dirname(current);
    if (parent === current) throw new Error("the channel version laws found no repository root");
    current = parent;
  }
  return current;
}

/** 🔢️ A number's f64 little-endian bytes as the Pack codec stores them, written by Node's own `Buffer` (the oracle). */
function packF64Hex(value: number): string {
  const bytes = Buffer.alloc(8);
  bytes.writeDoubleLE(value);
  return bytes.toString("hex");
}

describe("channel version authority", () => {
  it("the repository states the channel version only through registered consumers that equal the pin", () => {
    const { pin, findings } = channelVersionCensus(repositoryRoot());
    expect(pin).toBeGreaterThan(0);
    expect(findings).toEqual([]);
  });

  it("finds every literal shape once, with the exact span of its number", () => {
    const text = [
      'pub const CHANNEL_VERSION: u32 = 18;',
      'export const APP_CHANNEL_VERSION = 18;',
      '"executionProtocol": { "appChannelVersion": 18 },',
      '"appChannelVersion": {\n  "const": 18\n}',
      'return row.appChannelVersion === 18 ? { appChannelVersion: 18 } : fail();',
      'execution_protocol: ExecutionProtocol { app_channel_version: 14 },',
      '"required": ["appChannelVersion"],',
      '"selection": { "executionProtocol": 17, "closure": ["stdio"] }',
    ].join("\n");
    const literals = channelVersionLiterals(text);
    expect(literals.map((literal) => literal.value)).toEqual([18, 18, 18, 18, 18, 18, 14, 17]);
    for (const literal of literals) expect(text.slice(literal.valueStart, literal.valueStart + String(literal.value).length)).toBe(String(literal.value));
  });

  it("finds a version inside hex-encoded Pack descriptor bytes, as the Pack codec decodes it", () => {
    const fixture = JSON.parse(readFileSync(join(repositoryRoot(), "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🔗️compiled-dependencies/🔣️.json"), "utf8"));
    for (const row of fixture.rawCases) {
      const literals = channelVersionLiterals(JSON.stringify(row));
      const decoded = decodePackValue(Buffer.from(row.hex, "hex")) as { executionProtocol: { appChannelVersion: number } };
      expect(literals).toHaveLength(1);
      expect(literals[0]).toMatchObject({ encoded: true, value: decoded.executionProtocol.appChannelVersion });
    }
    expect(channelVersionLiterals(`"${packF64Hex(17)}6170704368616e6e656c56657273696f6e05${packF64Hex(17)}"`).map((literal) => literal.value)).toEqual([17]);
  });

  it("the generator rewrites only the drifted literals it may and the census names the rest", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-channel-version-"));
    const write = (path: string, text: string): void => {
      mkdirSync(dirname(join(root, path)), { recursive: true });
      writeFileSync(join(root, path), text);
    };
    write(CHANNEL_VERSION_PIN_PATH, JSON.stringify({ channelVersion: 21 }));
    for (const consumer of CHANNEL_VERSION_CONSUMERS) {
      const hostile = consumer.hostileValues?.[0];
      const body = Array.from({ length: consumer.occurrences }, (_, index) => `"appChannelVersion": ${hostile !== undefined && index === 0 ? hostile : 20}`).join(",\n");
      write(consumer.path, `{\n${body}\n}\n`);
    }
    write("unregistered/🟦️.ts", "export const APP_CHANNEL_VERSION_COPY = 20;\n");
    write("unregistered-pack/🔣️.json", `{ "hex": "11${Buffer.from("appChannelVersion").toString("hex")}05${packF64Hex(20)}" }\n`);
    expect(spawnSync("git", ["init", "-q"], { cwd: root }).status).toBe(0);
    const { written, refused } = writeChannelVersionConsumers(root, { guest: false });
    const rewritable = CHANNEL_VERSION_CONSUMERS.filter((consumer) => !consumer.arbitrary && !consumer.derived && !consumer.guest);
    expect([...written].sort()).toEqual(rewritable.map((consumer) => consumer.path).sort());
    expect(refused.length).toBe(CHANNEL_VERSION_CONSUMERS.filter((consumer) => !consumer.arbitrary && (consumer.derived || consumer.guest)).length);
    for (const consumer of CHANNEL_VERSION_CONSUMERS.filter((candidate) => candidate.hostileValues)) {
      expect(readFileSync(join(root, consumer.path), "utf8")).toContain(`"appChannelVersion": ${consumer.hostileValues![0]}`);
    }
    const { findings } = channelVersionCensus(root);
    expect(findings.filter((finding) => finding.problem === "unregistered").map((finding) => finding.path).sort()).toEqual(["unregistered-pack/🔣️.json", "unregistered/🟦️.ts"]);
    expect(findings.filter((finding) => finding.problem === "drift").map((finding) => finding.path).sort()).toEqual(
      CHANNEL_VERSION_CONSUMERS.filter((consumer) => !consumer.arbitrary && (consumer.derived || consumer.guest)).map((consumer) => consumer.path).sort(),
    );
    const withGuest = writeChannelVersionConsumers(root, { guest: true });
    expect(withGuest.written).toEqual(CHANNEL_VERSION_CONSUMERS.filter((consumer) => consumer.guest && !consumer.derived && !consumer.arbitrary).map((consumer) => consumer.path));
  });
});
