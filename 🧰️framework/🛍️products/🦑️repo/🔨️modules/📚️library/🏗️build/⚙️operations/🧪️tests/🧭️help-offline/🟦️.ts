import { expect, test } from "bun:test";
import { resolve } from "node:path";
import catalog from "../../🔣️catalog.json" with { type: "json" };

const root = resolve(import.meta.dir, "../../../../../../../../.."), entry = resolve(import.meta.dir, "../../../📜️script.ts");
const environment = Object.fromEntries(Object.entries(process.env).filter(([key]) => !key.startsWith("SEMIO_SCRIPT") && !key.startsWith("SEMIO_BUILD_INV") && key !== "SEMIO_LANGUAGE"));

const concurrency = 6;

const help = async (operation: string, language: string): Promise<string> => {
  const child = Bun.spawn([process.execPath, entry, "repo", operation, "--help", `--lang=${language}`], { cwd: root, env: environment, stdout: "pipe", stderr: "pipe" }), timer = setTimeout(() => child.kill(), 120_000);
  const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  clearTimeout(timer);
  expect(status, `${operation} ${language}: ${stderr}`).toBe(0);
  return stdout;
};

const pooled = async <T>(count: number, run: (index: number) => Promise<T>): Promise<T[]> => {
  const rows: T[] = new Array(count);
  let next = 0;
  await Promise.all(Array.from({ length: Math.min(concurrency, count) }, async () => { for (let index = next++; index < count; index = next++) rows[index] = await run(index); }));
  return rows;
};

test("every operation of the catalogue prints its own help in every catalogue language without running anything", async () => {
  const jobs = catalog.operations.flatMap((row) => catalog.languages.map((language) => ({ row, language })));
  const texts = await pooled(jobs.length, async (index) => {
    const { row, language } = jobs[index]!, text = await help(row.name, language);
    expect(text.split("\n")[0], `${row.name} ${language}`).toBe(`repo ${row.name} - ${(row.summary as Record<string, string>)[language]}`);
    return text;
  });
  for (const [index, row] of catalog.operations.entries()) {
    const own = texts.slice(index * catalog.languages.length, (index + 1) * catalog.languages.length);
    expect(new Set(own).size, row.name).toBe(own.length);
  }
}, 240_000);
