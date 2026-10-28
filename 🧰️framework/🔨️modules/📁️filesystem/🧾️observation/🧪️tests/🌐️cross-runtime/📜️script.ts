/** 🌐️ Runs one fixed capability scenario of the rooted POSIX descriptor and prints plain JSON; the test runs it under every runtime on the same read-only tree and on one fresh writable tree each. */
import { createHash } from "node:crypto";
import { withArtifactRelativeDirectoryV1 } from "../../📁️descriptor/🟦️.ts";

const [, , readOnlyRoot, writableRoot] = process.argv as [string, string, string, string];
const plain = (value: unknown): unknown => JSON.parse(JSON.stringify(value, (_key, item) => typeof item === "bigint" ? `${item}n` : item));

const observed = await withArtifactRelativeDirectoryV1(readOnlyRoot!, () => {}, async directory => {
  const names = [...await directory.names()].sort(), entries: Record<string, unknown> = {};
  for (const name of [...names, "missing"]) entries[name] = await directory.entry(name);
  const digests: Record<string, string> = {};
  for (const name of names) {
    if ((entries[name] as { kind: string }).kind !== "file") continue;
    digests[name] = await directory.withFile(name, async file => { const size = Number(file.stat().size), bytes = Buffer.alloc(size); file.read(bytes, 0, size, 0); return createHash("sha256").update(bytes).digest("hex"); });
  }
  const nested = await directory.withDirectory("nested", async child => ({ names: [...await child.names()].sort(), stat: child.stat() }));
  return { names, entries, digests, nested, stat: directory.stat() };
});

const written = await withArtifactRelativeDirectoryV1(writableRoot!, () => {}, async directory => {
  await directory.withNewDirectory("made", async child => {
    await child.withNewFile("data.bin", async file => { const bytes = Buffer.from("héllo native\n", "utf8"); file.write(bytes, 0, bytes.length, 0); file.sync(); });
    await child.sync();
  });
  await directory.sync();
  const names = [...await directory.names()].sort();
  const inner = await directory.withDirectory("made", async child => ({ names: [...await child.names()].sort(), entry: await child.entry("data.bin") }));
  const entry = inner.entry as { size: bigint; mode: bigint; nlink: bigint; kind: string };
  const removed = await directory.withDirectory("made", async child => { const target = await child.entry("data.bin"); await child.remove("data.bin", target!); return [...await child.names()]; });
  return { names, inner: { names: inner.names, kind: entry.kind, size: entry.size, mode: entry.mode, nlink: entry.nlink }, removed };
});

process.stdout.write(JSON.stringify(plain({ observed, written })));
