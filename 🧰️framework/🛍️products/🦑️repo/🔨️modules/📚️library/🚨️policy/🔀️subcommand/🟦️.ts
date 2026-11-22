/**
 * 🧭️ Nested subcommand dispatch inside a `Script.run` implementation, in its own module so a command closure that only dispatches verbs does not load the policy runner.
 * `handlers` keys are the first argv segment; `defaultKey` runs when argv is empty.
 */
export function dispatchSubcommand(segments: string[], handlers: Record<string, (rest: string[]) => void | Promise<void>>, usage: string, defaultKey?: string): void | Promise<void> {
  const key = segments[0] ?? defaultKey;
  const handler = key ? handlers[key] : undefined;
  if (!handler) {
    console.error(`usage: ${usage}`);
    process.exit(1);
  }
  return handler(segments.slice(1));
}
