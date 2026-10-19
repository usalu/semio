/** 🎁️ The `docker-stack-bundle` verb: what the proctor host needs without a registry — the stack files out of the image, an empty `certificates` directory, an `.env` that pins the tag, the image as `proctor-image.tar` and the host's steps.
 * @see ../🟦️.ts — the constants, tools and drift check shared by the operator verbs */

import { copyFileSync, mkdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { type ScriptInvocation } from "@semio-tech/process/routing";
import course from "../../🔣️.json" with { type: "json" };
import { imageLabels } from "../🐳️docker-image/🟦️.ts";
import { COURSE_BUNDLE_DIRECTORY, COURSE_BUNDLE_IMAGE, COURSE_IMAGE_REPOSITORY, COURSE_PROCTOR_HOST, COURSE_PROCTOR_ORIGIN, COURSE_SITE_ORIGIN, COURSE_STACK_CERTIFICATES, COURSE_STACK_FILES, docker, dockerOk, dockerServerVersion, flag, interruption, say } from "../🟦️.ts";
import { expectCheckoutStack, extractCourseStack } from "../🩺️docker-image-check/🟦️.ts";

//#region 🎁️Bundle
/** 🧳️ Enumerates the complete authored stack image set, keeping the proxy's exact digest for offline transfer. */
export function stackBundleImages(tag: string, compose: string): readonly [string, string] {
  const images = [...compose.matchAll(/^ {4}image:\s*(\S+)$/gmu)].map(match => match[1]!);
  const caddy = /^ {2}caddy:\r?\n {4}image:\s*(caddy:[\w.-]+@sha256:[0-9a-f]{64})\r?$/mu.exec(compose)?.[1];
  if (images.length !== 2 || images[0] !== `${COURSE_IMAGE_REPOSITORY}:\${PROCTOR_TAG:-latest}` || images[1] !== caddy || !caddy) throw Error("The offline stack requires exactly its proctor and pinned Caddy images");
  return [`${COURSE_IMAGE_REPOSITORY}:${tag}`, caddy];
}

/** 🏷️ Gives the pinned proxy a portable archive name and prevents the staged stack from consulting a registry. */
export function stackBundlePlan(tag: string, compose: string): { readonly images: readonly [string, string, string]; readonly caddy: string; readonly compose: string } {
  const images = stackBundleImages(tag, compose);
  const caddy = `caddy:sha256-${images[1].split("@sha256:")[1]}`;
  const source = compose.replace(/\r\n/gu, "\n");
  const proxy = `  caddy:\n    image: ${images[1]}\n    pull_policy: missing\n`;
  if (!source.includes(proxy)) throw Error("The offline stack requires its authored Caddy pull policy");
  return { images: [...images, caddy], caddy, compose: source.replace(proxy, `  caddy:\n    image: ${caddy}\n    pull_policy: never\n`) };
}

/** 🎚️ The `.env` of the bundle: the tag of the image the host loads, pinned so that compose never looks for another
 * one, and every other default of `compose.yaml` an operator may override, as a comment holding that default. */
export function stackBundleEnvironment(tag: string): string {
  return ["# Overrides of compose.yaml; both services read this file as their environment.", `PROCTOR_TAG=${tag}`, `# PROCTOR_HOST=${COURSE_PROCTOR_HOST}`, `# PROCTOR_ALLOWED_ORIGINS=${COURSE_SITE_ORIGIN}`, "# COURSE_HTTP_PORT=80", "# COURSE_HTTPS_PORT=443"].map((line) => `${line}\n`).join("");
}

/** 📖️ The steps of the proctor host, as they travel with the bundle for the image tag `tag`. */
export function stackBundleReadme(tag: string, caddyImage: string, localCaddy: string): string {
  return [
    `Architecture course proctor: the Docker stack of ${COURSE_PROCTOR_ORIGIN}`,
    "",
    `The backend of the site ${COURSE_SITE_ORIGIN}:`,
    "the proctor (an API over one SQLite file on a Docker volume) behind Caddy, which terminates TLS. This directory is",
    "everything the host needs; nothing is pulled from a registry.",
    "",
    `  ${COURSE_STACK_FILES.join(", ")}   the stack`,
    `  .env                      overrides of compose.yaml; pins the image tag ${tag}`,
    `  ${COURSE_STACK_CERTIFICATES}/             your own certificate, when Caddy cannot obtain one (step 3)`,
    `  ${COURSE_BUNDLE_IMAGE}         the image ${COURSE_IMAGE_REPOSITORY}:${tag}`,
    `                           and ${caddyImage}`,
    "                           loaded locally as",
    `                           ${localCaddy}`,
    "  compose.yaml binds Caddy to that local digest tag with pull_policy: never.",
    "",
    "The host: linux/amd64 with Docker Engine 25 or newer and its compose plugin 2.24 or newer.",
    "",
    "1. Copy this whole directory to the host (scp -r, rsync, a USB stick) and work inside it.",
    `2. docker load --input ${COURSE_BUNDLE_IMAGE}`,
    "3. The certificate, one of:",
    "   a. Let's Encrypt, nothing to do: Caddy obtains and renews it once ports 80/tcp, 443/tcp and 443/udp of",
    `      ${COURSE_PROCTOR_HOST} are open from the internet.`,
    `   b. Your own: one file ${COURSE_STACK_CERTIFICATES}/<any name>.pem with the certificate and its full chain first and the private`,
    "      key after it, owned by root with mode 0600 (Caddy does not start on a file it cannot read). Caddy serves",
    "      it and orders none. After replacing it: docker compose up --detach --force-recreate caddy",
    "   Either way a learner's browser reaches the proctor only where 443/tcp of the host is open to it.",
    "4. docker compose up --detach --wait",
    `5. curl --fail ${COURSE_PROCTOR_ORIGIN}/instance`,
    '   answers JSON that starts {"id":"teaching-proctor"',
    "",
    "Backup while serving (keep copies off the host):",
    "  docker compose exec -T proctor proctor backup - > proctor.sqlite",
    "",
    "Update an existing stack from this bundle:",
    "  Take the backup above before replacing files.",
    `  Copy ${COURSE_BUNDLE_IMAGE}, compose.yaml and Caddyfile into the existing stack directory.`,
    `  Keep its .env and ${COURSE_STACK_CERTIFICATES}/.`,
    "  In the existing .env, set this image tag and keep every other override:",
    `  PROCTOR_TAG=${tag}`,
    `  docker load --input ${COURSE_BUNDLE_IMAGE}`,
    "  docker compose up --detach --wait",
    `  curl --fail ${COURSE_PROCTOR_ORIGIN}/instance`,
    "  Loading the archive alone does not change the tag selected by the existing .env.",
    "",
    "Stop and keep the data:",
    "  docker compose down",
    "NEVER docker compose down --volumes: it deletes the database and the certificates Caddy obtained.",
    "Logs:",
    "  docker compose logs --since 1h proctor caddy",
    "",
  ].join("\n");
}

/** 🚛️ `docker-stack-bundle [--tag <tag>] [--out <directory>]` — stages everything the proctor host needs without a
 * registry in `--out` (default the package's `dist/proctor`): `compose.yaml` and the `Caddyfile` copied out of the image
 * `--tag` (default `latest`, built beforehand with `docker-image-build`; an image whose stack files are not those of this
 * checkout is refused), the `certificates` directory (what it already holds stays), the `.env`
 * ({@link stackBundleEnvironment}), `README.txt` ({@link stackBundleReadme}) and both stack images as
 * `proctor-image.tar` (`docker save`). Nothing is uploaded: the directory is copied to the host by hand. Answers the
 * directory. */
export async function stageCourseStackBundle(repoRoot: string, bundleRoot: string, segments: readonly string[], invocation: ScriptInvocation): Promise<string> {
  const tag = flag(segments, "--tag") ?? "latest";
  const image = `${COURSE_IMAGE_REPOSITORY}:${tag}`;
  const directory = resolve(flag(segments, "--out") ?? join(bundleRoot, "dist", COURSE_BUNDLE_DIRECTORY));
  const { signal, release } = interruption(invocation);
  let scratch = "";
  try {
    say(`docker ${await dockerServerVersion(signal)}; bundling ${image}`);
    if ((await docker(["image", "inspect", image], { signal })).code !== 0) throw new Error(`image ${image} does not exist; build it first with docker-image-build --tag ${tag}`);
    const labels = await imageLabels(image, signal);
    scratch = await extractCourseStack(image, signal);
    expectCheckoutStack(repoRoot, scratch);
    const plan = stackBundlePlan(tag, readFileSync(join(scratch, "compose.yaml"), "utf8"));
    if ((await docker(["image", "inspect", plan.images[1]], { signal })).code !== 0) await dockerOk(["pull", "--platform", "linux/amd64", plan.images[1]], { signal });
    await dockerOk(["tag", plan.images[1], plan.caddy], { signal });
    mkdirSync(join(directory, COURSE_STACK_CERTIFICATES), { recursive: true });
    for (const file of COURSE_STACK_FILES) copyFileSync(join(scratch, file), join(directory, file));
    writeFileSync(join(directory, "compose.yaml"), plan.compose);
    writeFileSync(join(directory, ".env"), stackBundleEnvironment(tag));
    writeFileSync(join(directory, "README.txt"), stackBundleReadme(tag, plan.images[1], plan.caddy));
    say(`saving ${plan.images.join(" and ")} (${labels["org.opencontainers.image.version"]} of ${labels["org.opencontainers.image.revision"]}) as ${COURSE_BUNDLE_IMAGE}`);
    await dockerOk(["save", "--output", join(directory, COURSE_BUNDLE_IMAGE), ...plan.images], { signal });
    say(`staged ${[...COURSE_STACK_FILES, `${COURSE_STACK_CERTIFICATES}/`, ".env", "README.txt"].join(", ")} and ${COURSE_BUNDLE_IMAGE} (${Math.round(statSync(join(directory, COURSE_BUNDLE_IMAGE)).size / 1024 ** 2)} MiB) in ${directory}`);
    return directory;
  } finally {
    release();
    if (scratch) rmSync(scratch, { recursive: true, force: true });
  }
}
//#endregion 🎁️Bundle
