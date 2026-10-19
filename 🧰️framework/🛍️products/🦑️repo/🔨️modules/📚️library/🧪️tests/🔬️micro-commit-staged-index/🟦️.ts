import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import Ajv from "ajv";

type FileSeed = { path: string; bytes: string };
type Case = {
  id: string;
  tracked: FileSeed[];
  staged: FileSeed[];
  unstaged: FileSeed[];
  untracked: FileSeed[];
  bullets: string[];
  expectExit: number;
};
type Fixture = { version: number; cases: Case[] };

const fixturePath = fileURLToPath(new URL(`../\u{1F9EB}\u{FE0F}fixtures/\u{1F52C}\u{FE0F}micro-commit-staged-index/\u{1F523}\u{FE0F}.json`, import.meta.url));
const featurePath = fileURLToPath(new URL(`./\u{1F952}\u{FE0F}.feature`, import.meta.url));
const libraryPath = fileURLToPath(new URL(`../../\u{1F39B}\u{FE0F}commands/\u{1F4DD}\u{FE0F}commit/\u{1FA99}\u{FE0F}micro-commit/\u{1F7E6}\u{FE0F}.ts`, import.meta.url));
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;
const feature = readFileSync(featurePath, "utf8");

const caseSchema = {
  type: "object",
  additionalProperties: false,
  required: ["version", "cases"],
  properties: {
    version: { const: 1 },
    cases: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        additionalProperties: false,
        required: ["id", "tracked", "staged", "unstaged", "untracked", "bullets", "expectExit"],
        properties: {
          id: { type: "string", minLength: 1 },
          tracked: { type: "array", items: { $ref: "#/$defs/file" } },
          staged: { type: "array", items: { $ref: "#/$defs/file" } },
          unstaged: { type: "array", items: { $ref: "#/$defs/file" } },
          untracked: { type: "array", items: { $ref: "#/$defs/file" } },
          bullets: { type: "array", items: { type: "string", minLength: 1 } },
          expectExit: { type: "integer" },
        },
      },
    },
  },
  $defs: {
    file: {
      type: "object",
      additionalProperties: false,
      required: ["path", "bytes"],
      properties: { path: { type: "string", minLength: 1 }, bytes: { type: "string" } },
    },
  },
};

const childSource = `
import { pathToFileURL } from "node:url";
const lib = process.env.SEMIO_MC_LIB;
const root = process.env.SEMIO_MC_ROOT;
const bullets = process.env.SEMIO_MC_BULLETS ?? "";
if (!lib || !root) throw new Error("missing micro-commit fixture env");
const { runMicroCommit } = await import(pathToFileURL(lib).href);
runMicroCommit(root, bullets ? ["prepare", "--", bullets] : ["prepare"]);
`;

function git(root: string, args: string[]): string {
  const result = spawnSync("git", args, { cwd: root, encoding: "utf8" });
  if (result.status !== 0) throw new Error(`${args.join(" ")}\n${result.stderr}`);
  return result.stdout;
}

function lines(text: string): string[] {
  return text.split("\n").map((line) => line.trimEnd()).filter((line) => line.length > 0).sort();
}

function writeSeed(root: string, file: FileSeed): void {
  const path = join(root, file.path);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, file.bytes);
}

function materialize(item: Case): { parent: string; root: string; bullets: string } {
  const parent = mkdtempSync(join(tmpdir(), "semio-micro-commit-index-"));
  const root = join(parent, "repo");
  mkdirSync(root);
  git(root, ["init", "-b", "fixture-\u26F3\uFE0Fwip"]);
  git(root, ["config", "user.email", "fixture@example.com"]);
  git(root, ["config", "user.name", "Fixture"]);
  git(root, ["config", "commit.gpgsign", "false"]);
  git(root, ["config", "core.hooksPath", join(root, ".git", "hooks")]);
  const devs = join(root, ".\u{1F9EC}semio", "\u{1F991}\u{FE0F}repo", "\u{1F9D1}\u{FE0F}\u{200D}\u{1F4BB}\u{FE0F}devs", "fixture");
  mkdirSync(devs, { recursive: true });
  writeFileSync(join(devs, "\u{1F9D1}\u{FE0F}\u{200D}\u{1F4BB}\u{FE0F}contributor.json"), JSON.stringify({ alias: "fixture", emoji: "\u{1F9EA}", name: "Fixture", email: "fixture@example.com" }));
  writeFileSync(join(devs, "micro-commit.json"), JSON.stringify({ level: "prepare-only" }));
  for (const file of item.tracked) writeSeed(root, file);
  git(root, ["add", "--", ...item.tracked.map((file) => file.path)]);
  git(root, ["commit", "-m", "base"]);
  for (const file of item.staged) writeSeed(root, file);
  if (item.staged.length > 0) git(root, ["add", "--", ...item.staged.map((file) => file.path)]);
  for (const file of item.unstaged) writeSeed(root, file);
  for (const file of item.untracked) writeSeed(root, file);
  const bullets = join(parent, "bullets.txt");
  writeFileSync(bullets, item.bullets.length > 0 ? `${item.bullets.join("\n")}\n` : "");
  return { parent, root, bullets: item.bullets.length > 0 ? bullets : "" };
}

describe("micro-commit prepare keeps the existing index", () => {
  test("validates the case vectors and leaves git status unchanged", () => {
    const ajv = new Ajv({ allErrors: true });
    expect(ajv.validate(caseSchema, fixture)).toBe(true);
    for (const item of fixture.cases) expect(feature).toContain(item.id);
    for (const item of fixture.cases) {
      const repo = materialize(item);
      try {
        const beforePorcelain = lines(git(repo.root, ["status", "--porcelain=v1", "-uall"]));
        const beforeCached = lines(git(repo.root, ["diff", "--cached", "--name-only"]));
        const result = spawnSync(process.execPath, ["-e", childSource], {
          cwd: repo.root,
          encoding: "utf8",
          timeout: 170_000,
          env: { ...process.env, SEMIO_MC_LIB: libraryPath, SEMIO_MC_ROOT: repo.root, SEMIO_MC_BULLETS: repo.bullets },
        });
        const afterPorcelain = lines(git(repo.root, ["status", "--porcelain=v1", "-uall"]));
        const afterCached = lines(git(repo.root, ["diff", "--cached", "--name-only"]));
        expect(result.status).toBe(item.expectExit);
        expect(afterPorcelain).toEqual(beforePorcelain);
        expect(afterCached).toEqual(beforeCached);
        expect(afterCached).toEqual(item.staged.map((file) => file.path).sort());
        if (item.expectExit === 0) {
          expect(result.stdout).toContain(item.bullets[0]);
          expect(result.stdout).toContain("Signed-off-by:");
        } else {
          expect(result.stderr).toContain("nothing is staged");
        }
      } finally {
        rmSync(repo.parent, { recursive: true, force: true });
      }
    }
  }, 180_000);
});
