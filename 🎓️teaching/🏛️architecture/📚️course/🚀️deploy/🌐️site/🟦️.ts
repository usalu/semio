import { withCourseAssignmentPdfsV1 } from "../../🏗️builder/🌐️vite/📦️assignments/🟦️.ts";
/** 🌐️ The `publish` verb: the site built with the proctor origin baked in, the CDN artifact verified (entry points, sealed document, hashed assets, no development leftovers, size budget) and staged with `_headers`.
 * @see ../🟦️.ts — the constants, tools and drift check shared by the operator verbs */

import { scriptCompilerAdmissionControlV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { courseOriginalBuildModeV1 } from "../../🏗️builder/🎚️mode/🟦️.ts";
import { courseBudgetMs } from "../../🏗️builder/⏱️budget/🟦️.ts";
import { buildViteArtifact } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts";
import { buildModeEnvironment, type BuildMode } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { createHash } from "node:crypto";
import { copyFileSync, lstatSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { connect } from "node:tls";
import { gzipSync } from "node:zlib";
import { withViteArtifactV1 } from "@semio-tech/repo-lib/dev";
import { type ScriptInvocation } from "@semio-tech/process/routing";
import course from "../../🔣️.json" with { type: "json" };
import { COURSE_BUNDLE_DIRECTORY, COURSE_PAGES_DIRECTORY, COURSE_PROCTOR_ORIGIN, COURSE_SITE_BUDGET, COURSE_SITE_HEADERS, COURSE_SITE_HOST, COURSE_SITE_ORIGIN, courseSiteTopology, say } from "../🟦️.ts";

//#region 🌐️Site
/** 🧭️ The proctor origin a release build bakes in: `PROCTOR_URL`, else the proctor origin of `🔣️.json`. */
export function releaseProctorOrigin(env: NodeJS.ProcessEnv = process.env): string {
  return env.PROCTOR_URL ?? COURSE_PROCTOR_ORIGIN;
}

/** 📦️ The directories of the package's `dist` that hold what a verb staged (`publish`, `docker-stack-bundle`), never the
 * build itself. */
const STAGED_DIRECTORIES: ReadonlySet<string> = new Set([COURSE_PAGES_DIRECTORY.split("/")[0]!, COURSE_BUNDLE_DIRECTORY]);

/** 📂️ Every file below `root`, as `/`-separated relative paths in name order whatever the file system lists, skipping the directories named in `skip`. */
function siteFiles(root: string, skip: ReadonlySet<string> = new Set(), prefix = ""): string[] {
  return readdirSync(join(root, prefix), { withFileTypes: true }).sort((left, right) => (left.name < right.name ? -1 : left.name > right.name ? 1 : 0)).flatMap((entry) => {
    const path = prefix ? `${prefix}/${entry.name}` : entry.name;
    if (path === ".semio-artifact.json") return [];
    if (entry.isDirectory()) return skip.has(path) ? [] : siteFiles(root, skip, path);
    return entry.isFile() ? [path] : [];
  });
}

/** 🛡️ Every reason the release document `html` is not sealed for `proctorOrigin`: no Content-Security-Policy ahead of
 * the first script, a script policy that admits anything inline or evaluated beyond the document's own boot scripts (each
 * must be listed by its SHA-256), connections admitted anywhere but the proctor (HTTPS and its WebSocket twin), plugins or
 * a `base` allowed, no description, or a default language (the app sets `lang` from the learner's locale). */
export function siteDocumentProblems(html: string, proctorOrigin: string): string[] {
  const problems: string[] = [];
  const meta = /<meta http-equiv="Content-Security-Policy" content="([^"]*)"/u.exec(html);
  if (!meta) problems.push("the document declares no Content-Security-Policy");
  else {
    const directives = new Map(meta[1]!.split(";").map((directive) => directive.trim().split(/\s+/u)).map(([name, ...sources]) => [name!, sources]));
    const sources = (name: string): readonly string[] => directives.get(name) ?? directives.get("default-src") ?? [];
    if (meta.index > html.indexOf("<script")) problems.push("the Content-Security-Policy comes after the first script");
    for (const name of ["default-src", "script-src", "style-src"]) for (const source of directives.get(name) ?? ["(missing)"]) if (!/^'(?:self|none|sha256-[A-Za-z0-9+/]{43}=)'$/u.test(source)) problems.push(`${name} admits ${source}`);
    for (const block of html.matchAll(/<script(?![^>]*\ssrc=)[^>]*>([\s\S]*?)<\/script>/gu)) {
      const hash = `'sha256-${createHash("sha256").update(block[1]!).digest("base64")}'`;
      if (!sources("script-src").includes(hash)) problems.push(`script-src does not admit the inline script ${hash}`);
    }
    for (const block of html.matchAll(/<style[^>]*>([\s\S]*?)<\/style>/gu)) {
      const hash = `'sha256-${createHash("sha256").update(block[1]!).digest("base64")}'`;
      if (!sources("style-src").includes(hash)) problems.push(`style-src does not admit the inline style ${hash}`);
    }
    const connections = [proctorOrigin, proctorOrigin.replace(/^http/u, "ws")];
    if ([...sources("connect-src")].sort().join(" ") !== [...connections].sort().join(" ")) problems.push(`connect-src admits ${sources("connect-src").join(" ") || "nothing"}, not exactly ${connections.join(" ")}`);
    for (const name of ["object-src", "base-uri"]) if (directives.get(name)?.join(" ") !== "'none'") problems.push(`${name} is not 'none'`);
    if (!directives.has("form-action")) problems.push("form-action is not restricted");
  }
  if (!/<meta name="description"/u.test(html)) problems.push("the document has no description");
  if (/<html[^>]*\slang=/u.test(html)) problems.push("the document declares a default language");
  return problems;
}

/** 🔎️ Every reason `dist` is not the CDN artifact of a release build baked for `proctorOrigin`: the host entry points and
 * markers (`404.html` being the same document as `index.html`), `CNAME` naming the site host, a sealed document
 * ({@link siteDocumentProblems}) whose scripts and stylesheets exist, content-hashed file names under `assets/` (they are
 * served as immutable), the proctor origin in the code and no loopback address outside it, no source map and no
 * development leftover, and the size budget ({@link COURSE_SITE_BUDGET}) of the topology the build was baked for: the scripts
 * the document runs and the stylesheets it links are charged, the lazy chunks it only hints at low priority
 * (`fetchpriority="low"`) are not. */
export function siteArtifactProblems(dist: string, proctorOrigin: string): string[] {
  const problems: string[] = [];
  const files = siteFiles(dist, STAGED_DIRECTORIES);
  const text = (path: string): string => readFileSync(join(dist, path), "utf8");
  for (const required of ["index.html", "404.html", ".nojekyll", "CNAME", "robots.txt", "manifest.webmanifest", "favicon.svg"]) if (!files.includes(required)) problems.push(`${required} is missing`);
  for (const assignment of course.assignments) for (const document of Object.values(assignment.documents)) {
    const path = "assignments/" + document.file;
    if (!files.includes(path)) problems.push(path + " is missing");
    else if (readFileSync(join(dist, path)).subarray(0, 5).toString() !== "%PDF-") problems.push(path + " is not a PDF");
  }
  if (files.includes("CNAME") && text("CNAME").trim() !== COURSE_SITE_HOST) problems.push(`CNAME names ${JSON.stringify(text("CNAME").trim())}, not ${COURSE_SITE_HOST}`);
  if (files.includes("index.html")) {
    const html = text("index.html");
    if (files.includes("404.html") && text("404.html") !== html) problems.push("404.html is not the document of index.html");
    problems.push(...siteDocumentProblems(html, proctorOrigin));
    const tags = [...html.matchAll(/<(?:script|link)\b[^>]*\s(?:src|href)="\/(assets\/[^"]+)"[^>]*>/gu)].map((tag) => ({ path: tag[1]!, idle: /\sfetchpriority="low"/u.test(tag[0]) }));
    for (const { path } of tags) if (!files.includes(path)) problems.push(`the document loads ${path}, which the artifact lacks`);
    const linked = tags.filter(({ idle }) => !idle).map(({ path }) => path);
    for (const [kind, extension, budget] of [["script", ".js", COURSE_SITE_BUDGET.scriptGzipBytes[courseSiteTopology(proctorOrigin)]], ["stylesheet", ".css", COURSE_SITE_BUDGET.styleGzipBytes]] as const) {
      const weight = linked.filter((path) => path.endsWith(extension) && files.includes(path)).reduce((sum, path) => sum + gzipSync(readFileSync(join(dist, path))).length, 0);
      if (weight > budget) problems.push(`the document's ${kind}s weigh ${weight} bytes compressed, over the budget of ${budget}`);
    }
  }
  for (const path of files.filter((file) => file.startsWith("assets/"))) if (!/-[A-Za-z0-9_-]{8}\.[a-z0-9]+$/u.test(path)) problems.push(`${path} carries no content hash, yet /assets/* is served as immutable`);
  for (const path of files.filter((file) => file.endsWith(".map"))) problems.push(`${path} is a source map`);
  const code = files.filter((path) => /\.(?:html|js|mjs|css)$/u.test(path)).map(text);
  if (!code.some((source) => source.includes(JSON.stringify(proctorOrigin)))) problems.push(`no script bakes the proctor origin ${proctorOrigin}`);
  const elsewhere = code.map((source) => [proctorOrigin, proctorOrigin.replace(/^http/u, "ws")].reduce((rest, origin) => rest.split(origin).join(""), source));
  for (const loopback of ["localhost", "127.0.0.1", "[::1]"]) if (elsewhere.some((source) => source.includes(loopback))) problems.push(`the site names ${loopback} outside the baked proctor origin`);
  for (const leftover of ["sourceMappingURL=", "/@vite/client", "react-refresh", "[DEBUG]"]) if (code.some((source) => source.includes(leftover))) problems.push(`the site carries the development leftover ${leftover}`);
  const total = files.reduce((sum, path) => sum + statSync(join(dist, path)).size, 0);
  if (total > COURSE_SITE_BUDGET.totalBytes) problems.push(`the artifact weighs ${total} bytes, over the budget of ${COURSE_SITE_BUDGET.totalBytes}`);
  return problems;
}

/** 🏷️ The stable owner of the site product: `build` writes it and `publish` reads it, under the same mode. */
export const COURSE_SITE_OWNER = "architecture-course:site";

/** 🏗️ Plans the site product: its fixed owner, the original compiler mode and the only environment its bytes depend on, the mode and the proctor origin (never the source revision, a clock or a cache directory of the shell). */
export function courseSiteBuildPlan(repoRoot: string, bundleRoot: string, mode: BuildMode, environment: Readonly<Record<string, string | undefined>> = process.env): { workspace: string; root: string; config: string; mode: BuildMode; owner: string; environment: Record<string, string | undefined> } {
  return { workspace: repoRoot, root: bundleRoot, config: resolve(bundleRoot, "../../🏗️builder/🌐️vite/🟦️.ts"), mode, owner: COURSE_SITE_OWNER, environment: buildModeEnvironment(mode, { PROCTOR_URL: releaseProctorOrigin(environment) }) };
}

/** 🔨️ Builds the site for the original caller's mode under the plan's owner. The product lives in the products store behind a store receipt, so a warm `build` is a cache hit and never reaches here. */
export async function buildCourseSite(repoRoot: string, bundleRoot: string, invocation: ScriptInvocation): Promise<void> {
  const mode = courseOriginalBuildModeV1(invocation, "dev");
  await withCourseAssignmentPdfsV1(repoRoot, invocation, runtimeChannels => buildViteArtifact({ ...courseSiteBuildPlan(repoRoot, bundleRoot, mode), runtimeChannels, invocation, control: scriptCompilerAdmissionControlV1(invocation, "vite:course-site", 0), signal: invocation.control.signal, timeoutMs: courseBudgetMs(invocation) }));
}

/** 🚚️ Reads the release site `build` published, under immutable custody, and exports a separately validated CDN artifact. It builds nothing: the cached `build` task is its prerequisite. */
export async function publishCourseSite(repoRoot: string, bundleRoot: string, invocation: ScriptInvocation): Promise<void> {
  courseOriginalBuildModeV1(invocation, "release");
  const proctorOrigin = releaseProctorOrigin();
  say(`staging the site built for ${COURSE_SITE_ORIGIN} against ${proctorOrigin}`);
  await withViteArtifactV1(repoRoot, "release", COURSE_SITE_OWNER, invocation, scriptCompilerAdmissionControlV1(invocation, "architecture-course:site-reader", 0), async artifact => {
    await artifact.recheck();
    stageCourseSiteArtifact(artifact.path, join(bundleRoot, "dist", "release", COURSE_PAGES_DIRECTORY), proctorOrigin, invocation.control.signal);
  });
  if (proctorOrigin !== COURSE_PROCTOR_ORIGIN) say(`REHEARSAL artifact for ${proctorOrigin}; do not upload it to the production host`);
}

/** 📤️ Stages only validated public files outside their retained immutable producer artifact. */
export function stageCourseSiteArtifact(dist: string, pages: string, proctorOrigin: string, signal?: AbortSignal): void {
  const overlaps = (parent: string, child: string): boolean => { const path = relative(resolve(parent), resolve(child)); return !isAbsolute(path) && path !== ".." && !path.startsWith(`..${sep}`); };
  if (overlaps(dist, pages) || overlaps(pages, dist)) throw Error("CDN staging must be separate from its immutable source artifact");
  signal?.throwIfAborted();
  for (let path = resolve(pages); ; path = dirname(path)) { if (lstatSync(path, { throwIfNoEntry: false })?.isSymbolicLink()) throw Error("CDN staging ancestry cannot contain links"); if (dirname(path) === path) break; }
  const problems = siteArtifactProblems(dist, proctorOrigin);
  if (problems.length) throw Error(`The site artifact is not publishable:\n- ${problems.join("\n- ")}`);
  const files = siteFiles(dist, STAGED_DIRECTORIES);
  rmSync(pages, { recursive: true, force: true });
  for (const path of files) { signal?.throwIfAborted(); mkdirSync(dirname(join(pages, path)), { recursive: true }); copyFileSync(join(dist, path), join(pages, path)); }
  writeFileSync(join(pages, "_headers"), COURSE_SITE_HEADERS);
  say(`staged ${files.length + 1} files in ${pages}`);
}
//#endregion 🌐️Site
