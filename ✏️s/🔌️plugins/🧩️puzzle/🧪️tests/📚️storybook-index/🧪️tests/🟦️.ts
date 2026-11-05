/** 📇️ Story identities follow the actual index, including explicit IDs and ambiguous-entry refusal. */
import { expect, test } from "vitest";
import { filter } from "lodash";
import { storyId, type StoryIndexEntry } from "../🟦️.ts";
import fixture from "../../🧫️fixtures/📚️storybook-index/🔣️.json";
const cases = fixture.cases as { name: string; title: string; story: string; entries: Record<string, StoryIndexEntry>; expected: string | null }[];
for (const row of cases) test(row.name, () => {
  const matches = filter(Object.entries(row.entries), ([, entry]) => entry.type === "story" && entry.title === row.title && entry.name === row.story);
  expect(matches.length === 1 ? matches[0]![0] : null).toBe(row.expected);
  if (row.expected === null) expect(() => storyId(row.entries, row.title, row.story)).toThrow();
  else expect(storyId(row.entries, row.title, row.story)).toBe(row.expected);
});
