import { expect, test } from "vitest";
import { parseSurfaceAppId } from "@semio-tech/framework";
import { PLAYGROUND_BUILD_TARGETS } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";
import corpus from "../../🧫️fixtures/🚀️renderer-boot-variants/🔣️.json";

test("puzzle owns all four original renderer variant coordinates", () => {
  expect(corpus.rows).toHaveLength(4);
  for (const row of corpus.rows) {
    const target = PLAYGROUND_BUILD_TARGETS.find(target => target.variant === row.variant || target.aliases.includes(row.variant));
    expect(target?.pluginId).toBe("puzzle");
    expect(target?.app).toBe(row.appId);
    expect(parseSurfaceAppId(row.appId).role).toBe("editor");
  }
});
