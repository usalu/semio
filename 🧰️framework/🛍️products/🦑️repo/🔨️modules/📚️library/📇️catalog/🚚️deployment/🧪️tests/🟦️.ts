import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import * as toml from "@iarna/toml";
import { componentDeploymentDirectoryV1 } from "../🟨️.mjs";
import { declaredComponentDeploymentDirectoryV1 } from "../🟦️.ts";

const owner = resolve(import.meta.dir, "..");
const fixture = JSON.parse(readFileSync(resolve(owner, "🧫️fixtures/🔣️.json"), "utf8"));
const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(new URL("../../../../../../../🔨️modules/🪪️identity/📁️installation/🧬️schema/🔣️.json", import.meta.url), "utf8")));

test("component compilation does not imply deployment", () => {
  for (const vector of fixture.vectors) {
    const text = "[package.metadata.semio]\nrole = \"test\"\ncomponent-kind = \"plugin\"\n" + (vector.value === null ? "" : "deployment-directory = " + JSON.stringify(vector.value) + "\n");
    if (!vector.error) expect(JSON.parse(JSON.stringify(Bun.TOML.parse(text)))).toEqual(JSON.parse(JSON.stringify(toml.parse(text))));
    const parsed = toml.parse(text) as { package: { metadata: { semio: Record<string, unknown> } } };
    const valid = vector.value === null || validate(vector.value);
    expect(valid, vector.name).toBe(!vector.error);
    if (vector.error) {
      expect(() => componentDeploymentDirectoryV1(parsed.package.metadata.semio), vector.name).toThrow();
      expect(() => declaredComponentDeploymentDirectoryV1(text), vector.name).toThrow();
    } else {
      expect(componentDeploymentDirectoryV1(parsed.package.metadata.semio) ?? null, vector.name).toBe(vector.expected);
      expect(declaredComponentDeploymentDirectoryV1(text) ?? null, vector.name).toBe(vector.expected);
    }
  }
});

test("repeated deployment tables and fields refuse before publication", () => {
  expect(() => declaredComponentDeploymentDirectoryV1('[package.metadata.semio]\ndeployment-directory="one"\ndeployment-directory="two"')).toThrow();
  expect(() => declaredComponentDeploymentDirectoryV1('[package.metadata.semio]\n[package.metadata.semio]\ndeployment-directory="one"')).toThrow();
});
