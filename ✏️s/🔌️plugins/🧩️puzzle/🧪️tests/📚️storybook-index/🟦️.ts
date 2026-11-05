/** 📖️ The fields needed to address one story in a published Storybook index. */
export type StoryIndexEntry = Readonly<{ type: string; title: string; name: string }>;

/** 🔎️ Resolves an exact story coordinate through its published ID, refusing absent or ambiguous stories. */
export function storyId(entries: Readonly<Record<string, StoryIndexEntry>>, title: string, name: string): string {
  const matches = Object.entries(entries).filter(([, entry]) => entry.type === "story" && entry.title === title && entry.name === name);
  if (matches.length !== 1) throw new Error(`Expected one Storybook story for ${JSON.stringify([title, name])}; found ${matches.length}`);
  return matches[0]![0];
}
