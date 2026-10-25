/** 🔸️ The part2 phase of transformTestClosure. */
import { existsSync } from "node:fs";
import { basename, dirname, resolve } from "node:path";
import { identifierSet, parseRustSourceTree, prunedUse, useLeafNames } from "../../🌳️module-tree/🟦️.ts";
import { relativePosix } from "../../📝️render/🟦️.ts";
import type { useResultState } from "../🔶️result/🟦️.ts";
import { declaredNames, discoverMounts, fixturesToRuntime, hostPrivate, prependAfterInner, snake } from "../🟢️harness/🟦️.ts";
import type { TestMount, TestTransformOptions } from "../🟢️harness/🟦️.ts";

/** 🔹 Runs the effects and registrations of transformTestClosure. */
export function usePart2({ options, rootFile, result, readText, setText, repoRoot, subtreeText, rootNames, artifactSources, hostGlobCarry, widenFor, queueEdit, removalOf, orphanedDocRemoval, retargetPrivateImportPaths, flushEdits, overlay }: { readonly options: TestTransformOptions; readonly rootFile: string; readonly repoRoot: string; readonly rootNames: ReadonlySet<string> } & Pick<ReturnType<typeof useResultState>, "result" | "readText" | "setText" | "subtreeText" | "artifactSources" | "hostGlobCarry" | "widenFor" | "queueEdit" | "removalOf" | "orphanedDocRemoval" | "retargetPrivateImportPaths" | "flushEdits" | "overlay">) {
  if (options.hoist || options.detach) {
    const { mounts } = discoverMounts(rootFile, result.rootSource, readText);
    const seen = new Set<string>();
    const visit = (file: string): void => {
      if (seen.has(file)) return;
      seen.add(file);
      const text = readText(file);
      if (text === null) return;
      const plain = text.replace(/\bcrate((?:::[A-Za-z_][A-Za-z0-9_]*)*?)::component(?=::)/gu, "crate$1");
      if (plain !== text) { setText(file, plain); result.notes.push(`dropped the private component segment from the absolute paths of ${relativePosix(repoRoot, file)}`); }
      for (const node of parseRustSourceTree(plain).modules) if (!node.inline && node.pathAttribute !== null) visit(resolve(dirname(file), node.pathAttribute));
    };
    for (const mount of mounts) visit(mount.testFile);
  }
  if (options.hoist) {
    const { mounts } = discoverMounts(rootFile, result.rootSource, readText);
    const hoisted: string[] = [];
    for (const mount of mounts) {
      const body = readText(mount.testFile);
      if (body === null) continue;
      const bound = [...subtreeText(mount.testFile).matchAll(/\bcrate::([A-Za-z_][A-Za-z0-9_]*)/gu)].map((match) => match[1]!).filter((name) => rootNames.has(name));
      if (!bound.length || mount.hostIsRoot || /\bsuper::/u.test(body)) continue;
      const whole = subtreeText(mount.testFile);
      const privateUse = hostPrivate(readText(mount.hostFile) ?? "", identifierSet(whole), declaredNames(body), whole);
      if (privateUse.relative.length) { result.notes.push(`kept ${relativePosix(repoRoot, mount.testFile)} in place (it uses private items of its host module: ${privateUse.relative.join(", ").slice(0, 120)})`); continue; }
      const name = mount.node.name;
      const hostLast = mount.ownerPath.filter((segment) => segment !== "component").at(-1);
      const named = hostLast === undefined ? null : new RegExp(`\\b${hostLast}::${name}\\b`, "u");
      const referenced = mounts.some((other) => other !== mount && new RegExp(`\\bsuper::(?:super::)*${name}\\b`, "u").test(readText(other.testFile) ?? "")) || (named !== null && artifactSources().some(([file, text]) => file !== mount.testFile && named.test(text)));
      if (referenced) { result.notes.push(`kept ${relativePosix(repoRoot, mount.testFile)} in place (another test module names it through super::${name})`); continue; }
      const rawGlobCarry = hostGlobCarry(mount, body);
      const known = new Set([...privateUse.carry.flatMap((statement) => useLeafNames(statement)), ...declaredNames(body)]);
      const globCarry = rawGlobCarry === null ? null : rawGlobCarry.flatMap((statement) => {
        const leaves = useLeafNames(statement);
        if (!leaves.length) return [`#[allow(unused_imports)]\n${statement}`];
        const fresh = leaves.filter((leaf) => !known.has(leaf));
        return fresh.length ? [`#[allow(unused_imports)]\n${prunedUse(statement, fresh)}`] : [];
      });
      if (globCarry === null) { result.notes.push(`kept ${relativePosix(repoRoot, mount.testFile)} in place (it glob-imports its host, whose private imports are relative)`); continue; }
      const widenedUse = widenFor(mount, privateUse.items, body);
      if (widenedUse === null) { result.notes.push(`kept ${relativePosix(repoRoot, mount.testFile)} in place (it uses private items of its host module that cannot be widened: ${privateUse.items.join(", ").slice(0, 120)})`); continue; }
      queueEdit(mount.hostFile, removalOf(mount.hostFile, mount.node));
      for (const edit of orphanedDocRemoval(mount.hostFile, mount.node)) queueEdit(mount.hostFile, edit);
      if (privateUse.carry.length || widenedUse.length || globCarry.length) setText(mount.testFile, prependAfterInner(retargetPrivateImportPaths(body, mount, privateUse.carry), [...privateUse.carry, ...globCarry, ...widenedUse]));
      const hoistedName = `hoisted_${snake(basename(dirname(dirname(mount.testFile))))}_${hoisted.length}`;
      const formerPath = [...mount.ownerPath.filter((segment) => segment !== "component"), mount.node.name].join("::");
      const former = new RegExp(`\\bcrate::${formerPath}(?![A-Za-z0-9_])`, "gu");
      const retarget = (file: string, seenFiles = new Set<string>()): void => {
        if (seenFiles.has(file)) return;
        seenFiles.add(file);
        const text = readText(file);
        if (text === null) return;
        const rewritten = text.replace(former, `crate::${hoistedName}`);
        if (rewritten !== text) setText(file, rewritten);
        for (const node of parseRustSourceTree(rewritten).modules) if (!node.inline && node.pathAttribute !== null) retarget(resolve(dirname(file), node.pathAttribute), seenFiles);
      };
      retarget(mount.testFile);
      hoisted.push(`#[cfg(test)]\n#[path = "${relativePosix(dirname(rootFile), mount.testFile)}"]\nmod ${hoistedName};`);
      result.notes.push(`hoisted ${relativePosix(repoRoot, mount.testFile)} to the artifact root (references ${[...new Set(bound)].join(", ")}${privateUse.items.length ? `; widened ${privateUse.items.join(", ")} of the host to pub` : ""})`);
    }
    flushEdits();
    if (hoisted.length) setText(rootFile, `${result.rootSource.trimEnd()}\n\n${hoisted.join("\n")}\n`);
  }
  if (options.fixtures) {
    const { mounts } = discoverMounts(rootFile, result.rootSource, readText);
    const converted = new Set<string>();
    const convert = (file: string): void => {
      if (converted.has(file)) return;
      converted.add(file);
      const text = readText(file);
      if (text === null) return;
      const rewritten = fixturesToRuntime(text, file, repoRoot);
      if (rewritten !== null) { setText(file, rewritten); result.notes.push(`runtime fixtures in ${relativePosix(repoRoot, file)}`); }
      for (const node of parseRustSourceTree(readText(file) ?? text).modules) if (!node.inline && node.pathAttribute !== null) convert(resolve(dirname(file), node.pathAttribute));
    };
    for (const mount of mounts) convert(mount.testFile);
    const rootRewrite = fixturesToRuntime(result.rootSource, rootFile, repoRoot);
    if (rootRewrite !== null) setText(rootFile, rootRewrite);
  }
  if (options.aggregate) {
    const { mounts, inline } = discoverMounts(rootFile, result.rootSource, readText);
    const groups = new Map<string, TestMount[]>();
    for (const mount of mounts) {
      if (mount.ownerPath.length < 2) continue;
      const key = mount.ownerPath.slice(0, -1).join("::");
      groups.set(key, [...(groups.get(key) ?? []), mount]);
    }
    for (const [key, members] of groups) {
      const parent = inline.get(key);
      const eligible = members.filter((mount) => {
        const body = readText(mount.testFile);
        if (body === null) return false;
        const uses = body.match(/\buse super::\*;/gu)?.length ?? 0;
        const others = (body.match(/\bsuper::/gu)?.length ?? 0) - uses;
        if (!(uses <= 1 && others === 0 && !/^\s*(?:pub\s+)?mod\s/mu.test(body) && !/^#!\[/mu.test(body))) return false;
        if (mount.hostIsRoot) return true;
        const privateUse = hostPrivate(readText(mount.hostFile) ?? "", identifierSet(body), declaredNames(body), body);
        return privateUse.items.length === 0 && privateUse.relative.length === 0;
      });
      if (!parent || parent.node.children.some((child) => child.name === "component") || eligible.length < (options.minimumAggregateGroup ?? 3)) continue;
      const lca = (() => {
        const parts = eligible.map((mount) => dirname(mount.testFile).split("/"));
        let length = parts[0]!.length;
        for (const other of parts) { let i = 0; while (i < length && other[i] === parts[0]![i]) i += 1; length = i; }
        return parts[0]!.slice(0, length).join("/");
      })();
      let aggregate = resolve(lca, "🧪️tests/🔬️unit/🦀️.rs");
      if (readText(aggregate) !== null && !eligible.some((mount) => mount.testFile === aggregate)) aggregate = resolve(lca, "🧪️tests/🧩️leaves/🦀️.rs");
      const moduleNames = new Map<string, number>();
      const modules = eligible.map((mount) => {
        const owner = mount.ownerPath[mount.ownerPath.length - 1]!;
        const body = readText(mount.testFile)!.replace(/\buse super::\*;/u, `use super::super::${owner}::*;`).trim();
        const used = identifierSet(body);
        const hostText = readText(mount.hostFile) ?? "";
        const carried = mount.hostIsRoot ? [] : parseRustSourceTree(hostText).items.filter((item) => item.kind === "use" && !item.visibility.startsWith("pub")).map((item) => ({ statement: hostText.slice(item.declStart, item.end), leaves: useLeafNames(hostText.slice(item.declStart, item.end)).filter((leaf) => used.has(leaf) && !new RegExp(`\\buse\\s+[^;]*\\b${leaf}\\b`, "u").test(body)) })).filter((entry) => entry.leaves.length && !/\*\s*;\s*$/u.test(entry.statement)).map((entry) => prunedUse(entry.statement, entry.leaves));
        const seen = (moduleNames.get(owner) ?? 0) + 1;
        moduleNames.set(owner, seen);
        return `mod ${seen === 1 ? owner : `${owner}_${seen}`} {\n${prependAfterInner(body, carried)}\n}`;
      });
      for (const mount of eligible) {
        queueEdit(mount.hostFile, removalOf(mount.hostFile, mount.node));
        for (const edit of orphanedDocRemoval(mount.hostFile, mount.node)) queueEdit(mount.hostFile, edit);
        result.deletes.add(mount.testFile);
        result.edits.delete(mount.testFile);
      }
      const aggregateText = `//! 🧪️ Aggregated unit tests of the \`${key.split("::").pop()}\` leaves, one module per leaf.\n\n${modules.join("\n\n")}\n`;
      result.deletes.delete(aggregate);
      if (existsSync(aggregate)) result.edits.set(aggregate, aggregateText);
      else result.creates.set(aggregate, aggregateText);
      overlay.set(aggregate, aggregateText);
      queueEdit(parent.file, { start: parent.node.bodyEnd, end: parent.node.bodyEnd, text: `\n#[cfg(test)]\n#[path = "${relativePosix(parent.base, aggregate)}"]\nmod leaf_tests;\n` });
      result.notes.push(`aggregated ${eligible.length} leaf test files under ${key} into ${relativePosix(repoRoot, aggregate)}`);
    }
    flushEdits();
  }
}
