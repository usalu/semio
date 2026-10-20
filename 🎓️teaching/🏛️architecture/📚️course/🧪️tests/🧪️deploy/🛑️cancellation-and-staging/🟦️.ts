/** 🛑️ The deployment cancellation, release build and site staging tests. */
import Ajv from "ajv";
import { sha256 } from "@noble/hashes/sha2.js";
import picomatch from "picomatch";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, readdirSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { expect, it } from "vitest";
import { testCoursePreflightAlignmentV1 } from "../../🩺️preflight/🟦️.ts";
import { COURSE_DEPLOY_CLEANUP_MS, deploymentInterruption } from "../../../🚀️deploy/🟦️.ts";
import cancellation from "../../../🧫️fixtures/🛑️cancellation/🔣️.json" with { type: "json" };
import { getEventListeners } from "node:events";
import sitePublication from "../../../🧫️fixtures/🌐️publication/🔣️.json" with { type: "json" };
import { COURSE_PROCTOR_ORIGIN, COURSE_SITE_HEADERS } from "../../../🚀️deploy/🟦️.ts";
import { courseReleaseBuildPlan, siteArtifactProblems, stageCourseSiteArtifact } from "../../../🚀️deploy/🌐️site/🟦️.ts";
import { released, repoRoot, site, tree } from "../🧰️deploy-fixtures/🟦️.ts";

/** 🧹 Registers the preflight alignment, parent cancellation, release build, site staging and staging refusal tests. */
export function registerCancellationAndStagingTests(): void {
  it("aligns generic host admission with CI and the distinct course image runtime contract",testCoursePreflightAlignmentV1);
  it("retains parent cancellation until release and closes its local lifetime like native AbortController", () => {
    expect(COURSE_DEPLOY_CLEANUP_MS).toBe(cancellation.cleanupTimeoutMilliseconds);
    const validate = new Ajv({ strict: true }).compile({ type: "object", additionalProperties: false, required: ["initial", "during", "released"], properties: { initial: { type: "boolean" }, during: { type: "boolean" }, released: { const: true } } });
    for (const row of cancellation.cases) {
      const parent = new AbortController(), oracle = new AbortController(), local = new AbortController();
      const expected = AbortSignal.any([oracle.signal, local.signal]);
      const reason = Error("parent deadline exhausted");
      if (row.phase === "before") { parent.abort(reason); oracle.abort(reason); }
      const before = [process.listenerCount("SIGINT"), process.listenerCount("SIGTERM"), getEventListeners(parent.signal, "abort").length];
      const owned = deploymentInterruption(parent.signal);
      const initial = owned.signal.aborted;
      expect(initial).toBe(expected.aborted);
      if (row.phase === "during") { parent.abort(reason); oracle.abort(reason); }
      const during = owned.signal.aborted;
      expect(during).toBe(expected.aborted);
      if (row.phase !== "release") expect(owned.signal.reason).toBe(reason);
      owned.release();
      local.abort();
      expect(parent.signal.aborted).toBe(row.phase !== "release");
      const result = { initial, during, released: owned.signal.aborted };
      expect(result).toEqual({ initial: row.initial, during: row.during, released: row.released });
      expect(result.released).toBe(expected.aborted);
      expect(validate(result), JSON.stringify(validate.errors)).toBe(true);
      expect([process.listenerCount("SIGINT"), process.listenerCount("SIGTERM"), getEventListeners(parent.signal, "abort").length]).toEqual(before);
    }
  });
  it("builds publication as a release even when the dashboard selected prototype", () => {
    const bundle = join(site, "📦️packages/🟦️typescript");
    const plan = courseReleaseBuildPlan(repoRoot, bundle, { SEMIO_BUILD_MODE: "prototype", PROCTOR_URL: COURSE_PROCTOR_ORIGIN });
    expect({ mode: plan.mode, owner: plan.owner }).toEqual(sitePublication.build);
    expect(plan.environment.SEMIO_BUILD_MODE).toBe(sitePublication.build.mode);
    expect(plan.environment.PROCTOR_URL).toBe(COURSE_PROCTOR_ORIGIN);
    expect(plan.workspace).toBe(repoRoot);
    expect(plan.config).toBe(join(site, "🏗️builder/🌐️vite/🟦️.ts"));
    const project = JSON.parse(readFileSync(join(bundle, "📋️project.json"), "utf8")) as { targets: { build: { cache: boolean; outputs: string[] } } };
    expect(project.targets.build).toMatchObject({ cache: false, outputs: [] });
  });
  for (const row of sitePublication.cases) it(`stages ${row.id} from the current artifact without exporting its internal receipt`, () => {
    const files = { ...released(row.origin), ".semio-artifact.json": JSON.stringify({ version: 1, owner: "course", files: [] }) };
    const source = tree(files);
    const destination = join(tree({ "old.txt": "obsolete" }), "pages");
    mkdirSync(destination);
    writeFileSync(join(destination, "obsolete.txt"), "old");
    const before = Object.fromEntries(Object.keys(files).map(path => [path, createHash("sha256").update(readFileSync(join(source, path))).digest("hex")]));
    stageCourseSiteArtifact(source, destination, row.origin);
    const actual = readdirSync(destination, { recursive: true, withFileTypes: true }).filter(entry => entry.isFile()).map(entry => join(entry.parentPath, entry.name).slice(destination.length + 1).replaceAll("\\", "/")).sort();
    const expected = Object.keys(files).filter(path => !picomatch(sitePublication.internalFiles, { dot: true })(path));
    expect(actual).toEqual([...expected, ...sitePublication.addedFiles].sort());
    for (const path of expected) expect(createHash("sha256").update(readFileSync(join(destination, path))).digest("hex")).toBe(Buffer.from(sha256(readFileSync(join(source, path)))).toString("hex"));
    expect(readFileSync(join(destination, "_headers"), "utf8")).toBe(COURSE_SITE_HEADERS);
    expect(Object.fromEntries(Object.keys(files).map(path => [path, createHash("sha256").update(readFileSync(join(source, path))).digest("hex")]))).toEqual(before);
  });
  it("preserves the immutable source and earlier staging when export is refused or cancelled", () => {
    const source = tree(released(sitePublication.cases[0]!.origin));
    const destination = tree({ "old.txt": "preserved" });
    for (const pages of [source, join(source, "pages"), dirname(source)]) expect(() => stageCourseSiteArtifact(source, pages, COURSE_PROCTOR_ORIGIN)).toThrow("separate");
    expect(() => stageCourseSiteArtifact(source, destination, "https://wrong.example")).toThrow("not publishable");
    const cancellation = new AbortController();
    cancellation.abort(new Error("publication cancelled"));
    expect(() => stageCourseSiteArtifact(source, destination, COURSE_PROCTOR_ORIGIN, cancellation.signal)).toThrow("publication cancelled");
    expect(readFileSync(join(destination, "old.txt"), "utf8")).toBe("preserved");
    expect(siteArtifactProblems(source, COURSE_PROCTOR_ORIGIN)).toEqual([]);
  });
  it("refuses a staging junction or symlink that redirects into the immutable source", () => {
    const source = tree(released(COURSE_PROCTOR_ORIGIN));
    const redirect = join(tree({}), "redirect");
    symlinkSync(source, redirect, process.platform === "win32" ? "junction" : "dir");
    expect(() => stageCourseSiteArtifact(source, join(redirect, "pages"), COURSE_PROCTOR_ORIGIN)).toThrow("links");
  });
}
