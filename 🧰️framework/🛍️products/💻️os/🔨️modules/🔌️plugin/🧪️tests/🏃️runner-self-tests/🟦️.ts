export function createPluginRunnerTests(dependencies: Record<string, any>, source: { directory: string; url: string }) {
  const { assert, parseArgs, pluginTestInvocation, readFileSync } = dependencies;

  /** 🧪️ Pins exact forwarding against the neutral fixture and Node's independent separator parser. */
  function pluginTestRunnerSelfTests(): number {
    const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🏃️runner/🔣️.json", source.url), "utf8"));
    const level = process.env.SEMIO_TEST_LEVEL,
      coverage = process.env.SEMIO_COVERAGE;
    try {
      for (const row of fixture.cases) {
        const selected = pluginTestInvocation(row.args);
        assert.equal(selected.mode, row.mode);
        assert.deepEqual(selected.args, row.forwarded);
        const options = { "no-run": { type: "boolean" }, lib: { type: "boolean" }, test: { type: "string" }, "all-targets": { type: "boolean" } };
        const parsed = parseArgs({ args: row.args, strict: false, allowPositionals: true, options });
        assert.equal(parsed.values["no-run"] === true ? "inventory" : "budgeted", row.mode);
        const forwarded = parseArgs({ args: selected.args, strict: false, allowPositionals: true, options, tokens: true });
        for (const name of ["lib", "test", "all-targets"]) {
          if (parsed.values[name] === undefined) continue;
          assert.equal(forwarded.values[name], parsed.values[name]);
          assert.equal(forwarded.tokens.filter((token: {kind:string;name?:string}) => token.kind === "option" && token.name === name).length, 1);
        }
        if (parsed.values.test !== undefined || parsed.values["all-targets"] === true) assert.equal(forwarded.values.lib, undefined);
      }
    } finally {
      if (level === undefined) delete process.env.SEMIO_TEST_LEVEL;
      else process.env.SEMIO_TEST_LEVEL = level;
      if (coverage === undefined) delete process.env.SEMIO_COVERAGE;
      else process.env.SEMIO_COVERAGE = coverage;
    }
    return fixture.cases.length;
  }
  return { pluginTestRunnerSelfTests };
}
