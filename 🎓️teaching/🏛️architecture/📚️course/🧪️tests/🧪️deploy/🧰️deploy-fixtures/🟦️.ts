/** 🧰️ The shared deployment test fixtures: the course locations, the parsed compose and workflow documents and the sealed document builders. */
import { createHash } from "node:crypto";
import { mkdtempSync, realpathSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "yaml";
import deployment from "../../../🚀️deploy/🔣️.json" with { type: "json" };

export const site = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");

export const repoRoot = resolve(site, "../../..");

export const deploy = join(site, "🚀️deploy");

export const read = (name: string): string => readFileSync(join(deploy, name), "utf8");

export const DRIFT_SOURCES = ["🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Dockerfile", "🎓️teaching/🏛️architecture/📚️course/🚀️deploy/compose.yaml", "🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Caddyfile", ".github/workflows/architecture-course.yml", "🎓️teaching/🏛️architecture/📚️course/README.md", "🎓️teaching/🛂️proctor/README.md", "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/package.json", "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json", "package.json"] as const;

export type Limits = { cpus: string; memory: string; pids: number };

export type Service = { image: string; pull_policy?: string; build?: unknown; expose?: string[]; ports?: string[]; env_file?: { path: string; required: boolean }[]; environment?: Record<string, string>; volumes?: string[]; depends_on?: Record<string, { condition: string }>; healthcheck?: { test: string[] }; read_only?: boolean; cap_drop?: string[]; cap_add?: string[]; security_opt?: string[]; stop_grace_period?: string; restart?: string; logging?: { driver: string; options: Record<string, string> }; deploy?: { resources: { limits: Limits } } };

export type Step = { uses?: string; run?: string; with?: Record<string, unknown>; "continue-on-error"?: boolean };

export type Job = { if?: string; needs?: string; outputs?: Record<string,string>; "runs-on": string; "timeout-minutes"?: number; "continue-on-error"?: boolean; permissions?: Record<string, string>; steps: Step[] };

export const compose = parse(read("compose.yaml")) as { name: string; services: Record<string, Service>; volumes: Record<string, unknown> };

export const workflow = parse(readFileSync(join(repoRoot, ".github/workflows/architecture-course.yml"), "utf8")) as { on: Record<string, unknown>; permissions: Record<string, string>; concurrency: Record<string, unknown>; jobs: Record<string, Job> };

export const scratch: string[] = [];

/** 📂️ A throw-away directory holding `files` (relative path → content). */
export function tree(files: Record<string, string | Uint8Array>): string {
  const root = mkdtempSync(join(realpathSync(tmpdir()), "course-deploy-"));
  scratch.push(root);
  for (const [path, content] of Object.entries(files)) {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), content);
  }
  return root;
}

/** 🪞️ Holds the drift sources in memory with one seeded change and no checkout files copied into ticket output. */
export function drifted(path: (typeof DRIFT_SOURCES)[number], change: (text: string) => string): (source: string) => string {
  const sources = new Map<string, string>(DRIFT_SOURCES.map(source => [source, readFileSync(join(repoRoot, source), "utf8")]));
  sources.set(path, change(sources.get(path)!));
  return source => {
    const text = sources.get(source);
    if (text === undefined) throw Error("Unseeded deployment drift source: " + source);
    return text;
  };
}

export const digest = (text: string): string => `'sha256-${createHash("sha256").update(text).digest("base64")}'`;

export const BOOT = "document.documentElement.dataset.ready='1'";

export const STYLE = "body{margin:0}";

/** 📄️ A release document sealed for `origin` the way the site's builder seals it: `policy` replaces the whole policy. */
export function sealed(origin: string, policy = `default-src 'self'; script-src 'self' ${digest(BOOT)}; style-src 'self' ${digest(STYLE)}; style-src-attr 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src ${origin} ${origin.replace(/^http/u, "ws")}; manifest-src 'self'; object-src 'none'; base-uri 'none'; form-action 'self'`): string {
  return `<!doctype html>\n<html>\n  <head>\n    <meta charset="UTF-8" />\n    <meta http-equiv="Content-Security-Policy" content="${policy}" />\n    <title>Architecture and Technology</title>\n    <meta name="description" lang="en" content="Quizzes" />\n    <script>${BOOT}</script>\n    <style>${STYLE}</style>\n    <script type="module" crossorigin src="/assets/app-AbCd1234.js"></script>\n    <link rel="stylesheet" crossorigin href="/assets/app-EfGh5678.css">\n  </head>\n  <body><div id="root"></div></body>\n</html>\n`;
}

/** 📦️ A whole CDN artifact baked for `origin`. */
export function released(origin: string): Record<string, string | Uint8Array> {
  return { "index.html": sealed(origin), "404.html": sealed(origin), ".nojekyll": "", CNAME: `${deployment.site.host}\n`, "robots.txt": "User-agent: *\nAllow: /\n", "manifest.webmanifest": "{}", "favicon.svg": "<svg xmlns=\"http://www.w3.org/2000/svg\"/>", "assets/app-AbCd1234.js": `const proctor = ${JSON.stringify(origin)};`, "assets/app-EfGh5678.css": STYLE };
}
