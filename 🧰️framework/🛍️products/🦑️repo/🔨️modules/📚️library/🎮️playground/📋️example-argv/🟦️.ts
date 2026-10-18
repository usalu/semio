/** 🔒️ Parses optional `example <id>` argv prefix for playground play scripts. */
export function consumePlaygroundExampleArgv(segments: string[], resolveExampleId: (slug: string) => string | undefined): { readonly segments: string[]; readonly exampleEnv: NodeJS.ProcessEnv } {
  if (segments[0] !== "example" || !segments[1]) {
    return { segments, exampleEnv: {} };
  }
  const exampleId = resolveExampleId(segments[1]);
  if (!exampleId) {
    console.error(`[play] unknown example ${JSON.stringify(segments[1])}`);
    process.exit(1);
  }
  return {
    segments: segments.slice(2),
    exampleEnv: { PLAYGROUND_LOCKED_EXAMPLE_ID: exampleId },
  };
}
