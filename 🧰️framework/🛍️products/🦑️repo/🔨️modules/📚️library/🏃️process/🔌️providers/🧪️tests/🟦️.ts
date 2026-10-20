import { expect, test } from "bun:test";
import Ajv from "ajv";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { SCRIPT_PROVIDERS_ENV, loadScriptProviderV1, readScriptProviderMapV1, type ScriptProviderTokenV1 } from "@semio-tech/process/packages/typescript/providers";
import { CARGO_ADMISSION_PROVIDER_V1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { WASM_BUILD_PROVIDER_V1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts";
import { TYPESCRIPT_CHECK_PROVIDER_V1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts";
import { ARTIFACT_STAGING_PROVIDER_V1, EXTENSION_PACKAGE_PROVIDER_V1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts";
import { REPOSITORY_TOOLING_PROVIDER_V1, getRepoMetaDir, repoCacheDirectory, runCmd, cargoTargetDirectory, devToolingEnv } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { ARTIFACT_PACKAGE_PROVIDER_V1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { CARGO_WORKSPACE_PROVIDER_V1, observeCargoPreparationInputV1, completeCargoPreparationObservationV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import { cargoTargetDirectory as repositoryCargoTargetDirectory } from "../../../⚡️caching/🦀️cargo/🗂️directories/🟦️.ts";
import { getRepoMetaDir as repositoryMetaDirectory } from "../../../🗂️workspaces/🧬️metadata/🟦️.ts";
import { STYLING_PYTHON_PROVIDER_V1 } from "../../../../../../../🔨️modules/🖱️ui/🎨️styling/🏗️builder/🐍️python/📜️script.ts";
import { validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { issueRepositoryScriptProvidersV1, repositoryScriptProvidersV1 } from "../🟦️.ts";
import declaration from "../🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

const tokens: readonly ScriptProviderTokenV1<any>[] = [CARGO_ADMISSION_PROVIDER_V1, WASM_BUILD_PROVIDER_V1, TYPESCRIPT_CHECK_PROVIDER_V1, ARTIFACT_STAGING_PROVIDER_V1, EXTENSION_PACKAGE_PROVIDER_V1, STYLING_PYTHON_PROVIDER_V1, REPOSITORY_TOOLING_PROVIDER_V1, ARTIFACT_PACKAGE_PROVIDER_V1, CARGO_WORKSPACE_PROVIDER_V1];

test("the declaration corpus agrees between the framework validator and Ajv and the repository declaration satisfies it", () => {
  const validate = new Ajv({ strict: false }).compile(schema);
  for (const row of fixture.declarations) {
    expect(validate(row.value), row.id).toBe(row.valid);
    expect(validateJsonSchemaSubset(schema, row.value).length === 0, row.id).toBe(row.valid);
  }
  expect(validate(declaration)).toBe(true);
  expect(validateJsonSchemaSubset(schema, declaration)).toEqual([]);
});

test("the repository designates exactly the framework ports and every designated module exists", () => {
  const designated = repositoryScriptProvidersV1();
  expect(Object.keys(designated).sort()).toEqual(tokens.map(token => token.name).sort());
  expect(Object.keys(declaration.providers).sort()).toEqual(Object.keys(designated).sort());
  for (const path of Object.values(designated)) expect(existsSync(path), path).toBe(true);
});

test("the issued child environment designates modules that implement every declared framework function", async () => {
  const environment: NodeJS.ProcessEnv = {};
  const prior = process.env[SCRIPT_PROVIDERS_ENV];
  try {
    issueRepositoryScriptProvidersV1(environment);
    expect(readScriptProviderMapV1(environment).providers).toEqual(repositoryScriptProvidersV1() as Record<string, string>);
    for (const token of tokens) {
      const provider = await loadScriptProviderV1(token, environment) as unknown as Record<string, unknown>;
      for (const method of token.methods) expect(typeof provider[method], token.name + "." + method).toBe("function");
    }
  } finally {
    if (prior === undefined) delete process.env[SCRIPT_PROVIDERS_ENV]; else process.env[SCRIPT_PROVIDERS_ENV] = prior;
  }
}, 60_000);

test("the synchronous framework wrappers reach the repository implementation and observations stay inert without a designation", () => {
  const environment: NodeJS.ProcessEnv = {};
  const prior = process.env[SCRIPT_PROVIDERS_ENV];
  try {
    delete process.env[SCRIPT_PROVIDERS_ENV];
    expect(() => getRepoMetaDir("/repository")).toThrow("Script provider designation required");
    observeCargoPreparationInputV1("/absent/input");
    completeCargoPreparationObservationV1();
    issueRepositoryScriptProvidersV1(environment);
    expect(getRepoMetaDir("/repository")).toBe(repositoryMetaDirectory("/repository"));
    expect(repoCacheDirectory("/repository", "go", "b")).toBe(join(repositoryMetaDirectory("/repository"), "⚡️cache", "go", "b"));
    expect(cargoTargetDirectory(process.cwd(), { CARGO_TARGET_DIR: "/cargo-target" })).toBe(repositoryCargoTargetDirectory(process.cwd(), { CARGO_TARGET_DIR: "/cargo-target" }));
    expect(devToolingEnv({ SEMIO_PROBE: "1" }).SEMIO_PROBE).toBe("1");
    expect(() => runCmd(process.execPath, ["-e", "process.exit(3)"])).toThrow("exited with status 3");
    runCmd(process.execPath, ["-e", "process.exit(0)"]);
  } finally {
    if (prior === undefined) delete process.env[SCRIPT_PROVIDERS_ENV]; else process.env[SCRIPT_PROVIDERS_ENV] = prior;
  }
}, 60_000);
