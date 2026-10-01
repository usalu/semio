import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import emojiRegex from "emoji-regex";
import TOML from "@iarna/toml";
import schema from "../../../🧬️schema/🚀️launch/🔣️.json";
import corpus from "../../../🧫️fixtures/🚀️launch/🏷️name-prefix/🔣️.json";
import { parsePlaygroundBlock } from "../../../🎮️playground/🔎️discovery/🟦️.ts";
import { playgroundLaunchNamePrefix } from "../🟦️.ts";

const ajv = new Ajv({ strict: true }).addSchema(schema);
const validate = ajv.getSchema(`${schema.$id}#/$defs/LaunchNamePrefixV1`)!;
expect(ajv.getSchema(`${schema.$id}#/$defs/LaunchNameCasesV1`)!(corpus)).toBe(true);

describe("owner-authored launch names", () => {
  for (const row of corpus.declarations) it(row.id, () => {
    const block = `variant = "future-owner"\nports = { react = 6001, wgpu = 6002 }\n${row.block}`;
    if (!row.accepted) {
      expect(() => TOML.parse(block)).toThrow();
      expect(() => parsePlaygroundBlock(block, "future-owner", "future/owner")).toThrow();
      return;
    }
    expect(TOML.parse(block)["launch-name-prefix"]).toBe(row.prefix);
    const owner = parsePlaygroundBlock(block, "future-owner", "future/owner")!;
    expect(playgroundLaunchNamePrefix(owner, "/unused-deleted-specific-tree", [owner])).toBe(row.prefix);
  });
  for (const row of corpus.cases) it(row.id, () => {
    expect(validate(row.input), row.id).toBe(row.accepted);
    const block = `variant = "future-owner"\nports = { react = 6001, wgpu = 6002 }\nlaunch-name-prefix = ${JSON.stringify(row.input)}\n`;
    if (!row.accepted) { expect(() => parsePlaygroundBlock(block, "future-owner", "future/owner"), row.id).toThrow(); return; }
    expect([...row.input!.matchAll(emojiRegex())][0]?.index).toBe(0);
    const owner = parsePlaygroundBlock(block, "future-owner", "future/owner")!;
    expect(playgroundLaunchNamePrefix(owner, "/unused-deleted-specific-tree", [owner])).toBe(row.input);
  });
});
