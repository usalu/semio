export function createPluginRunnerTests(dependencies: Record<string, any>, source: { directory: string; url: string }) {
  const { Ajv, assert, parseArgs, pluginTestInvocation, readFileSync } = dependencies;

  /** 🧪️ Pins exact forwarding against the neutral fixture and Node's independent separator parser. */
  function pluginTestRunnerSelfTests(): number {
    const fixture = JSON.parse(readFileSync(new URL("../../🧪️tests/🏃️runner/🧪️fixture/🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("../../🧪️tests/🏃️runner/🧬️schema/🔣️.json", source.url), "utf8"));
    const runnerAjv = new Ajv({ strict: true, allErrors: true });
    runnerAjv.addSchema(schema);
    const validate = runnerAjv.getSchema(`${schema.$id}#/$defs/TestRunnerForwardingV1`)!;
    assert(validate(fixture), JSON.stringify(validate.errors));
    const level = process.env.SEMIO_TEST_LEVEL,
      coverage = process.env.SEMIO_COVERAGE;
    try {
      for (const row of fixture.cases) {
        const selected = pluginTestInvocation(row.args);
        assert.equal(selected.mode, row.mode);
        assert.deepEqual(selected.args, row.forwarded);
        const parsed = parseArgs({ args: row.args, strict: false, allowPositionals: true, options: { "no-run": { type: "boolean" } } });
        assert.equal(parsed.values["no-run"] === true ? "inventory" : "budgeted", row.mode);
        assert.equal(validate({ ...fixture, cases: fixture.cases.map((other: object) => (other === row ? { ...row, mode: row.mode === "inventory" ? "budgeted" : "inventory" } : other)) }), false);
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
