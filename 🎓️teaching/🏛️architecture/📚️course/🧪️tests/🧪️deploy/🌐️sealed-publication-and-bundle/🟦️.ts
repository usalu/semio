/** 🌐️ The sealed document, CDN artifact, stack bundle, readiness gate and response header tests. */
import { randomBytes } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { parse } from "yaml";
import { expect, it } from "vitest";
import deployment from "../../../🚀️deploy/🔣️.json" with { type: "json" };
import bundleCases from "../../../🧫️fixtures/🚛️bundle/🔣️.json" with { type: "json" };
import { stackBundleImages, stackBundlePlan } from "../../../🚀️deploy/🎁️stack-bundle/🟦️.ts";
import readiness from "../../../🧫️fixtures/🚀️readiness/🔣️.json" with { type: "json" };
import { COURSE_BUNDLE_DIRECTORY, COURSE_BUNDLE_IMAGE, COURSE_PAGES_DIRECTORY, COURSE_PROCTOR_ORIGIN, COURSE_SITE_BUDGET, COURSE_SITE_HEADERS, COURSE_SITE_ORIGIN, COURSE_STACK_CERTIFICATES, COURSE_STACK_FILES, hostZone } from "../../../🚀️deploy/🟦️.ts";
import { siteArtifactProblems, siteDocumentProblems } from "../../../🚀️deploy/🌐️site/🟦️.ts";
import { stackBundleEnvironment, stackBundleReadme } from "../../../🚀️deploy/🎁️stack-bundle/🟦️.ts";
import { deployCheckSteps } from "../../../🚀️deploy/⚖️readiness/🟦️.ts";
import { BOOT, STYLE, compose, digest, read, released, sealed, site, tree } from "../🧰️deploy-fixtures/🟦️.ts";
import type { Service } from "../🧰️deploy-fixtures/🟦️.ts";

/** 🚛 Registers the sealed document, CDN artifact, stack bundle, readiness gate and response header tests. */
export function registerSealedPublicationAndBundleTests(): void {
  it("accepts exactly a document sealed for the baked proctor origin", () => {
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN), COURSE_PROCTOR_ORIGIN)).toEqual([]);
    expect(siteDocumentProblems(sealed("https://localhost:18443"), "https://localhost:18443")).toEqual([]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN), "https://staging.example")).toEqual([`connect-src admits ${COURSE_PROCTOR_ORIGIN} wss://${deployment.proctor.host}, not exactly https://staging.example wss://staging.example`]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN).replace(/\s*<meta http-equiv[^>]*>/u, ""), COURSE_PROCTOR_ORIGIN)).toEqual(["the document declares no Content-Security-Policy"]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN).replace(BOOT, `${BOOT};fetch('/')`), COURSE_PROCTOR_ORIGIN)).toEqual([`script-src does not admit the inline script ${digest(`${BOOT};fetch('/')`)}`]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN).replace(STYLE, "body{margin:1px}"), COURSE_PROCTOR_ORIGIN)).toEqual([`style-src does not admit the inline style ${digest("body{margin:1px}")}`]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN).replace("<html>", "<html lang=\"en\">").replace(/\s*<meta name="description"[^>]*>/u, ""), COURSE_PROCTOR_ORIGIN)).toEqual(["the document has no description", "the document declares a default language"]);
    const moved = sealed(COURSE_PROCTOR_ORIGIN);
    const meta = /\s*<meta http-equiv[^>]*>/u.exec(moved)![0];
    expect(siteDocumentProblems(moved.replace(meta, "").replace("<style>", `${meta.trim()}<style>`), COURSE_PROCTOR_ORIGIN)).toEqual(["the Content-Security-Policy comes after the first script"]);
    const loose = `default-src *; script-src 'self' 'unsafe-inline' 'unsafe-eval' https:; style-src 'self' 'unsafe-inline'; connect-src *; object-src 'self'`;
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN, loose), COURSE_PROCTOR_ORIGIN)).toEqual(["default-src admits *", "script-src admits 'unsafe-inline'", "script-src admits 'unsafe-eval'", "script-src admits https:", "style-src admits 'unsafe-inline'", `script-src does not admit the inline script ${digest(BOOT)}`, `style-src does not admit the inline style ${digest(STYLE)}`, `connect-src admits *, not exactly ${COURSE_PROCTOR_ORIGIN} wss://${deployment.proctor.host}`, "object-src is not 'none'", "base-uri is not 'none'", "form-action is not restricted"]);
  });
  it("publishes exactly a CDN artifact that bakes the proctor origin", () => {
    expect(siteArtifactProblems(tree(released(COURSE_PROCTOR_ORIGIN)), COURSE_PROCTOR_ORIGIN)).toEqual([]);
    expect(siteArtifactProblems(tree(released("https://localhost:18443")), "https://localhost:18443")).toEqual([]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), CNAME: "quizze.example\n" }), COURSE_PROCTOR_ORIGIN)).toEqual([`CNAME names "quizze.example", not ${deployment.site.host}`]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "assets/dev-AbCd1234.js": "fetch('http://127.0.0.1:8791')" }), COURSE_PROCTOR_ORIGIN)).toEqual(["the site names 127.0.0.1 outside the baked proctor origin"]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "404.html": "<!doctype html>" }), COURSE_PROCTOR_ORIGIN)).toEqual(["404.html is not the document of index.html"]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "assets/app-AbCd1234.js.map": "{}", "assets/chunk.js": "//# sourceMappingURL=chunk.js.map\nconsole.log('[DEBUG] x')" }), COURSE_PROCTOR_ORIGIN)).toEqual(["assets/app-AbCd1234.js.map carries no content hash, yet /assets/* is served as immutable", "assets/chunk.js carries no content hash, yet /assets/* is served as immutable", "assets/app-AbCd1234.js.map is a source map", "the site carries the development leftover sourceMappingURL=", "the site carries the development leftover [DEBUG]"]);
    const { "assets/app-EfGh5678.css": _stylesheet, "robots.txt": _robots, ...partial } = released(COURSE_PROCTOR_ORIGIN);
    expect(siteArtifactProblems(tree(partial), COURSE_PROCTOR_ORIGIN)).toEqual(["robots.txt is missing", "the document loads assets/app-EfGh5678.css, which the artifact lacks"]);
    expect(siteArtifactProblems(tree({ "index.html": sealed(COURSE_PROCTOR_ORIGIN), CNAME: `${deployment.site.host}\n` }), COURSE_PROCTOR_ORIGIN)).toEqual(["404.html is missing", ".nojekyll is missing", "robots.txt is missing", "manifest.webmanifest is missing", "favicon.svg is missing", "the document loads assets/app-AbCd1234.js, which the artifact lacks", "the document loads assets/app-EfGh5678.css, which the artifact lacks", `no script bakes the proctor origin ${COURSE_PROCTOR_ORIGIN}`]);
    const heavy = `${released(COURSE_PROCTOR_ORIGIN)["assets/app-AbCd1234.js"] as string}\n/* ${randomBytes(COURSE_SITE_BUDGET.scriptGzipBytes * 2).toString("base64")} */`;
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "assets/app-AbCd1234.js": heavy }), COURSE_PROCTOR_ORIGIN)).toEqual([expect.stringMatching(/^the document's scripts weigh \d+ bytes compressed, over the budget of \d+$/u)]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "🖼️assets/font.woff2": randomBytes(COURSE_SITE_BUDGET.totalBytes) }), COURSE_PROCTOR_ORIGIN)).toEqual([expect.stringMatching(/^the artifact weighs \d+ bytes, over the budget of \d+$/u)]);
  });
  it("bundles the proctor stack for a host without a registry: the tag pinned, the defaults of compose.yaml listed, every step told", () => {
    const defaults = Object.fromEntries([...read("compose.yaml").matchAll(/\$\{([A-Z_]+):-([^}]*)\}/gu)].map((substitution) => [substitution[1]!, substitution[2]!]));
    expect(Object.keys(defaults).sort()).toEqual(["COURSE_HTTPS_PORT", "COURSE_HTTP_PORT", "PROCTOR_ALLOWED_ORIGINS", "PROCTOR_HOST", "PROCTOR_TAG"]);
    const settings = (tag: string): string[] => stackBundleEnvironment(tag).split("\n").filter((line) => line.includes("="));
    expect(settings(defaults.PROCTOR_TAG!).map((line) => line.replace(/^# /u, "")).sort()).toEqual(Object.entries(defaults).map(([name, value]) => `${name}=${value}`).sort());
    expect(settings("sha-0123456789ab").filter((line) => !line.startsWith("#"))).toEqual(["PROCTOR_TAG=sha-0123456789ab"]);
    expect(stackBundleEnvironment("latest")).toBe(`# Overrides of compose.yaml; both services read this file as their environment.\nPROCTOR_TAG=latest\n# PROCTOR_HOST=${deployment.proctor.host}\n# PROCTOR_ALLOWED_ORIGINS=${COURSE_SITE_ORIGIN}\n# COURSE_HTTP_PORT=80\n# COURSE_HTTPS_PORT=443\n`);
    const readme = stackBundleReadme("sha-0123456789ab", compose.services.caddy!.image, bundleCases.proxy.image);
    for (const row of bundleCases.cases) {
      const images = stackBundleImages(row.tag, read("compose.yaml"));
      expect(images).toEqual(row.images);
      expect(images).toEqual(Object.values(compose.services).map(service => service.image.replace("${PROCTOR_TAG:-latest}", row.tag)));
      const plan = stackBundlePlan(row.tag, read("compose.yaml"));
      expect(plan.images).toEqual([...row.images, bundleCases.proxy.image]);
      expect(plan.caddy).toBe(bundleCases.proxy.image);
      const staged = parse(plan.compose) as { services: Record<string, Service> };
      expect(staged.services.caddy).toEqual({ ...compose.services.caddy, ...bundleCases.proxy });
      expect(staged.services.proctor).toEqual(compose.services.proctor);
      for (const image of plan.images) expect(stackBundleReadme(row.tag, images[1], plan.caddy)).toContain(image);
    }
    for (const row of bundleCases.updates) {
      const guide = stackBundleReadme(row.tag, compose.services.caddy!.image, bundleCases.proxy.image);
      expect(guide).toContain("Update an existing stack from this bundle:");
      expect(guide).toContain("In the existing .env, set this image tag and keep every other override:");
      expect(guide).toContain(row.setting);
      expect(guide).not.toContain(`PROCTOR_TAG=${row.currentTag}`);
      const selected = /^PROCTOR_TAG=(.+)$/mu.exec(stackBundleEnvironment(row.tag))![1]!;
      expect(parse(stackBundlePlan(row.tag, read("compose.yaml")).compose).services.proctor.image.replace("${PROCTOR_TAG:-latest}", selected)).toBe(row.image);
      const runbook = readFileSync(join(site, "README.md"), "utf8");
      expect(runbook).toContain("set `PROCTOR_TAG` in the retained `.env` to the exact value from the new bundle's `.env`");
      expect(runbook).toContain("Keep the other overrides and `certificates/`");
      const update = runbook.slice(runbook.indexOf("### Update"), runbook.indexOf("### Rollback"));
      expect(update).toContain("set `PROCTOR_TAG` in the retained `.env` to the exact value from the new bundle's `.env`");
      expect(update).toContain("Keep the other overrides and `certificates/`");
    }
    for (const row of bundleCases.registryUpdates) {
      const selected = { ...row.retained, PROCTOR_TAG: row.selectedTag };
      expect({ ...selected, PROCTOR_TAG: row.retained.PROCTOR_TAG }).toEqual(row.retained);
      expect(compose.services.proctor!.image.replace("${PROCTOR_TAG:-latest}", selected.PROCTOR_TAG)).toBe(row.image);
      const runbook = readFileSync(join(site, "README.md"), "utf8");
      const registry = runbook.slice(runbook.indexOf("### Update"), runbook.indexOf("The backup comes first:"));
      expect(registry).toContain("set `PROCTOR_TAG` in the retained `.env` to the intended release tag");
      expect(registry).toContain("Keep every other override");
      expect(registry).toContain('image=$(docker compose config --images proctor)');
      expect(registry).toContain('container=$(docker create "$image")');
      expect(registry.indexOf("proctor backup")).toBeLessThan(registry.indexOf("set `PROCTOR_TAG`"));
      expect(registry).not.toContain(`docker create ${deployment.proctor.image}:latest`);
    }
    for (const row of bundleCases.invalid) {
      const changed = "replace" in row ? read("compose.yaml").replace(row.replace!, row.with!) : read("compose.yaml") + row.append;
      expect(() => stackBundleImages("latest", changed), row.id).toThrow(row.error);
    }
    const steps = ["1. Copy this whole directory to the host", `2. docker load --input ${COURSE_BUNDLE_IMAGE}`, "3. The certificate, one of:", "4. docker compose up --detach --wait", `5. curl --fail ${COURSE_PROCTOR_ORIGIN}/instance`];
    expect(readme.match(/^\d+\. /gmu)).toEqual(steps.map((step) => step.slice(0, 3)));
    const told = steps.map((step) => readme.indexOf(`\n${step}`));
    expect(told.every((at, index) => at > (told[index - 1] ?? 0)), JSON.stringify(told)).toBe(true);
    for (const fact of [COURSE_SITE_ORIGIN, "linux/amd64", "Docker Engine 25 or newer", "compose plugin 2.24 or newer", `${deployment.proctor.image}:sha-0123456789ab`, "pins the image tag sha-0123456789ab", "80/tcp, 443/tcp and 443/udp", `${deployment.proctor.host} are open from the internet`, `${COURSE_STACK_CERTIFICATES}/<any name>.pem`, "full chain first", "key after it", "owned by root with mode 0600", "docker compose up --detach --force-recreate caddy", "docker compose exec -T proctor proctor backup - > proctor.sqlite", "docker compose down\n", "NEVER docker compose down --volumes", "docker compose logs --since 1h proctor caddy", ...COURSE_STACK_FILES, ".env", `${COURSE_STACK_CERTIFICATES}/`, COURSE_BUNDLE_IMAGE]) expect(readme, fact).toContain(fact);
    const named = new Set(readme.match(new RegExp(`(?:[a-z0-9_-]+\\.)*(?:${[deployment.site.host, deployment.proctor.host].map(hostZone).join("|").replace(/\./gu, "\\.")})`, "gu")));
    expect([...named].sort()).toEqual([deployment.site.host, deployment.proctor.host].sort());
    expect(readme.split("\n").every((line) => line.length <= 120)).toBe(true);
    expect(compose.services.caddy!.volumes).toContain(`./${COURSE_STACK_CERTIFICATES}:/${COURSE_STACK_CERTIFICATES}:ro`);
    expect(compose.services.proctor!.pull_policy).toBe("missing");
    expect(compose.services.caddy!.pull_policy).toBe("missing");
    const origin = COURSE_PROCTOR_ORIGIN;
    expect(siteArtifactProblems(tree({ ...released(origin), [`${COURSE_BUNDLE_DIRECTORY}/probe.js`]: "fetch('http://localhost')", [`${COURSE_PAGES_DIRECTORY}/index.html`]: "<!doctype html>" }), origin)).toEqual([]);
    expect(siteArtifactProblems(tree({ ...released(origin), "elsewhere/probe.js": "fetch('http://localhost')" }), origin)).toEqual(["the site names localhost outside the baked proctor origin"]);
  });
  it("checks the sources against the compiler early in the readiness gate, before anything is built, and ends with the end-to-end gate", () => {
    const project = JSON.parse(readFileSync(join(site, "📦️packages/🟦️typescript/📋️project.json"), "utf8")) as { targets: Record<string, { options?: { command?: string }; dependsOn?: string[] }> };
    expect(project.targets.typecheck?.options?.command).toBe("bun ./📜️script.ts typecheck");
    expect(project.targets["deploy-check"]?.dependsOn).toContain(readiness.sourceProducer);
    expect(project.targets.typecheck?.dependsOn).toEqual(["@semio-tech/framework-rs:generate", "@semio-tech/ui-contract-rs:generate", "@semio-tech/ui-rs:generate", "@semio-tech/framework-actor-rs:typegen", "@semio-tech/ui-styling-tokens:generate", "@semio-tech/assets:build"]);
    const steps = deployCheckSteps(project.targets);
    expect(steps).toEqual(["no drift from 🚀️deploy/🔣️.json", "sources type-checked", "course valid in the Rust core", "site built and verified as the CDN artifact", "proctor image built", "proctor image checked", "stack checked", "end-to-end gate"]);
    expect(deployCheckSteps({})).toEqual(steps.filter((step) => step !== "sources type-checked" && step !== "end-to-end gate"));
    const readme = readFileSync(join(site, "README.md"), "utf8");
    const gate = readme.slice(readme.indexOf("### Readiness gate"));
    const told = gate.slice(0, gate.indexOf("\n#", 1)).match(/^\d+\. /gmu) ?? [];
    expect(told).toEqual(steps.map((_, index) => `${index + 1}. `));
    const include = (JSON.parse(readFileSync(join(site, "📦️packages/🟦️typescript/tsconfig.json"), "utf8")) as { include: string[] }).include;
    for (const covered of ["./📜️script.ts", "../../🟦️.ts", "../../🏗️builder/**/*.ts", "../../🧱️stack/**/*.ts", "../../🚀️deploy/**/*.ts", "../../🎭️e2e/**/*.ts", "../../🧪️tests/**/*.ts"]) expect(include).toContain(covered);
  });
  it("hardens every response and caches hashed assets, without two rules setting one header for a path", () => {
    const rules = COURSE_SITE_HEADERS.trimEnd().split("\n").filter((line) => !line.startsWith(" "));
    expect(rules).toEqual(["/*", "/assets/*", "/", "/index.html", "/404.html"]);
    expect(COURSE_SITE_HEADERS).toContain("/assets/*\n  Cache-Control: public, max-age=31536000, immutable\n");
    expect(COURSE_SITE_HEADERS.match(/Cache-Control: no-cache/gu)).toHaveLength(3);
    const everywhere = COURSE_SITE_HEADERS.slice(0, COURSE_SITE_HEADERS.indexOf("/assets/*"));
    expect(everywhere).not.toContain("Cache-Control");
    for (const header of ["X-Content-Type-Options: nosniff", "X-Frame-Options: DENY", "Content-Security-Policy: frame-ancestors 'none'", "Referrer-Policy: no-referrer"]) expect(everywhere).toContain(`  ${header}\n`);
  });
}
