/** 🎬️ The learning clips of the course site are held to the videos the energy authoring manifest renders and to the poster
 * images on disk: one clip per video, its poster the still of the video's first chapter, a 640 × 360 PNG the site builds
 * into its hashed assets, and no address until the owner hosts the clip; the release document admits frames of the
 * site's own origin for the assignment PDFs and nothing else.
 * @see ../../🔣️.json — the clips
 * @see ../../📚️course/🖼️posters/🟦️.ts — the poster images as built assets
 * @see ../../../⚡️energy/🎬️clip/🔣️.json — the videos */
import { courseIssues, type Course, type Quiz } from "@semio-tech/course";
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { ARCHITECTURE_CLIP_POSTERS } from "../../📚️course/🖼️posters/🟦️.ts";
import { courseContentSecurityPolicy } from "../../🏗️builder/🌐️vite/🟦️.ts";
import { siteDocumentProblems } from "../../🚀️deploy/🌐️site/🟦️.ts";
import { COURSE_PROCTOR_ORIGIN } from "../../🚀️deploy/🟦️.ts";
import { sealed } from "../🧪️deploy/🧰️deploy-fixtures/🟦️.ts";

const siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const teaching = resolve(siteRoot, "..");
const read = (path: string): unknown => JSON.parse(readFileSync(path, "utf8"));
const course = read(resolve(siteRoot, "🔣️.json")) as Course;
const manifest = read(resolve(teaching, "⚡️energy/🎬️clip/🔣️.json")) as { readonly videos: readonly { readonly id: string; readonly chapters: readonly { readonly scene: string }[] }[] };
const clips = course.clips ?? [];

/** 🗂️ Every poster image below the teaching material by file name. */
function posterFiles(directory: string, found = new Map<string, string>()): Map<string, string> {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory() && entry.name !== "node_modules" && !entry.name.startsWith(".") && entry.name !== "dist" && entry.name !== "📚️course") posterFiles(path, found);
    else if (entry.isFile() && directory.endsWith("🖼️posters") && entry.name.endsWith(".png")) found.set(entry.name, path);
  }
  return found;
}

const posters = posterFiles(teaching);

describe("learning clips of the architecture course", () => {
  it("offers one clip per rendered video, each with the poster of its first chapter, grouped in the order of the course's topics", () => {
    const byId = new Map(clips.map((clip) => [clip.id, clip.poster.file]));
    expect([...byId.keys()].sort()).toEqual(manifest.videos.map((video) => `clip-${video.id.replaceAll("_", "-")}`).sort());
    for (const video of manifest.videos) expect(byId.get(`clip-${video.id.replaceAll("_", "-")}`)).toBe(`${video.chapters[0]!.scene}.png`);
    const order = course.topics!.map((topic) => topic.id);
    const positions = clips.map((clip) => order.indexOf(clip.topic));
    expect(positions).toEqual([...positions].sort((left, right) => left - right));
  });

  it("is valid with every quiz of the course", () => {
    const quizzes = course.quizzes.map((path) => read(resolve(siteRoot, path)) as Quiz);
    expect(courseIssues(course, quizzes)).toEqual([]);
  });

  it("names every clip in both languages and belongs it to a topic of the course", () => {
    const topics = new Set(course.topics?.map((topic) => topic.id));
    for (const clip of clips) {
      expect(topics.has(clip.topic), clip.id).toBe(true);
      for (const key of ["title", "description"] as const) for (const language of ["en", "de"] as const) expect(clip[key][language].trim().length, `${clip.id}/${key}/${language}`).toBeGreaterThan(0);
    }
  });

  it("waits for the owner's links: no clip has an address yet and each address, once given, is HTTPS", () => {
    for (const clip of clips) for (const url of Object.values(clip.urls ?? {})) expect(url, clip.id).toMatch(/^https:\/\//u);
    expect(clips.filter((clip) => clip.urls !== undefined).map((clip) => clip.id)).toEqual([]);
  });

  it("builds exactly the posters its clips name, each a 640 × 360 PNG on disk", () => {
    expect(Object.keys(ARCHITECTURE_CLIP_POSTERS).sort()).toEqual(clips.map((clip) => clip.poster.file).sort());
    expect(new Set(clips.map((clip) => clip.poster.file)).size).toBe(clips.length);
    for (const clip of clips) {
      const path = posters.get(clip.poster.file);
      expect(path !== undefined && existsSync(path), clip.poster.file).toBe(true);
      const bytes = readFileSync(path!);
      expect(bytes.subarray(0, 8).toString("hex"), clip.poster.file).toBe("89504e470d0a1a0a");
      expect([bytes.readUInt32BE(16), bytes.readUInt32BE(20)], clip.poster.file).toEqual([640, 360]);
      expect(statSync(path!).size, clip.poster.file).toBeLessThan(200_000);
      expect(typeof ARCHITECTURE_CLIP_POSTERS[clip.poster.file], clip.poster.file).toBe("string");
    }
  });

  it("seals frames to the site's own origin so that its pages can show the assignment PDFs, as the validator demands", () => {
    const policy = courseContentSecurityPolicy(sealed(COURSE_PROCTOR_ORIGIN), COURSE_PROCTOR_ORIGIN);
    const directives = new Map(policy.split("; ").map((directive) => [directive.split(" ")[0]!, directive.split(" ").slice(1)]));
    expect(directives.get("frame-src")).toEqual(["'self'"]);
    expect(directives.get("object-src")).toEqual(["'none'"]);
    expect(directives.get("default-src")).toEqual(["'self'"]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN, policy), COURSE_PROCTOR_ORIGIN)).toEqual([]);
  });
});
