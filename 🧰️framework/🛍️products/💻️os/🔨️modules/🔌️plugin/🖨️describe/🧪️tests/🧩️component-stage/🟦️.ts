/** 🎛️ The `components describe` argument law: the facet defaults to `model`, `--facet model|browser` composes with `--profile` in either order and with `--all`,
 * and an unknown or missing facet value is the usage error. */
import { expect, test } from "bun:test";
import { componentProfileV1 } from "../../🏷️component-receipt/🟦️.ts";
import { componentsDescribeArgumentsV1 } from "../../🧩️component-stage/🟦️.ts";
import { buildModeEnvironment, buildModePolicy, readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";

import {
  createScriptProcessEnvelope, receiveScriptProcessInvocation, requireIssuedResourceControlInvocationV1,
  scriptInvocationEnvironment, withIssuedResourceControlV1, withScriptProcessEnvelope,
  SCRIPT_PROCESS_INVOCATION_ENV,
} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const USAGE = /usage: components describe/u;
process.env.SEMIO_BUILD_MODE ??= "prototype";
const defaultProfile = componentProfileV1(buildModePolicy(readBuildMode(process.env.SEMIO_BUILD_MODE)).cargoProfile);

test("🧩 the default facet is model", () => {
  const parsed = componentsDescribeArgumentsV1(["alpha", "beta-two"]);
  expect(parsed).toEqual({ all: false, pluginIds: ["alpha", "beta-two"], profile: defaultProfile, facet: "model" });
});

test("🌐 --facet browser keeps the plugin ids", () => {
  expect(componentsDescribeArgumentsV1(["alpha", "--facet", "browser", "beta-two"])).toEqual({ all: false, pluginIds: ["alpha", "beta-two"], profile: defaultProfile, facet: "browser" });
  expect(componentsDescribeArgumentsV1(["--facet", "browser", "alpha"]).facet).toBe("browser");
});

test("🎚️ --facet combines with --profile release in either order", () => {
  const expected = { all: false, pluginIds: ["alpha"], profile: "release", facet: "browser" };
  expect(componentsDescribeArgumentsV1(["--profile", "release", "--facet", "browser", "alpha"])).toEqual(expected);
  expect(componentsDescribeArgumentsV1(["--facet", "browser", "--profile", "release", "alpha"])).toEqual(expected);
});

test("🗂️ --all --facet browser selects every component in the browser facet", () => {
  expect(componentsDescribeArgumentsV1(["--all", "--facet", "browser"])).toEqual({ all: true, pluginIds: [], profile: defaultProfile, facet: "browser" });
  expect(componentsDescribeArgumentsV1(["--facet", "model", "--all"])).toEqual({ all: true, pluginIds: [], profile: defaultProfile, facet: "model" });
});

test("🚫 an invalid facet value is the usage error", () => {
  expect(() => componentsDescribeArgumentsV1(["alpha", "--facet", "js"])).toThrow(USAGE);
  expect(() => componentsDescribeArgumentsV1(["alpha", "--facet", "Browser"])).toThrow(USAGE);
});

test("🚫 a --facet without a value is the usage error", () => {
  expect(() => componentsDescribeArgumentsV1(["alpha", "--facet"])).toThrow(USAGE);
  expect(() => componentsDescribeArgumentsV1(["--facet"])).toThrow(USAGE);
  expect(() => componentsDescribeArgumentsV1(["--profile", "release", "alpha", "--facet"])).toThrow(USAGE);
});

test("compiler admission receives the component invocation from a profile-only environment", async () => {
  const profile = buildModeEnvironment("prototype", {});
  expect(profile[SCRIPT_PROCESS_INVOCATION_ENV]).toBeUndefined();
  const envelope = createScriptProcessEnvelope(
    { version: 1, owner: "test:component-compiler", maximumElapsedMilliseconds: 10000 }, {}, Date.now(),
  );
  await withScriptProcessEnvelope(envelope, original =>
    withIssuedResourceControlV1(original, "test:component-compiler", 1000, async phase => {
      const environment = scriptInvocationEnvironment(original, profile);
      await receiveScriptProcessInvocation(environment, async received => {
        expect(received).toBe(original);
        requireIssuedResourceControlInvocationV1(phase, received);
        expect(() => requireIssuedResourceControlInvocationV1({ ...phase }, received)).toThrow("Live Original resource control issuance required");
      });
      expect(profile[SCRIPT_PROCESS_INVOCATION_ENV]).toBeUndefined();
    }));
});
