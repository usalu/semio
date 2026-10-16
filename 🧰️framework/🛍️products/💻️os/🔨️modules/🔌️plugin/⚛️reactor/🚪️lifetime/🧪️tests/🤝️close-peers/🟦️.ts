import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import vector from "../🧫️fixtures/🤝️close-peers.json" with { type: "json" };

const lifetime = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
const reactor = readFileSync(new URL("../../../🦀️.rs", import.meta.url), "utf8");

const body = (source: string, signature: string, last = false): string => {
  const start = last ? source.lastIndexOf(signature) : source.indexOf(signature);
  expect(start, signature).toBeGreaterThanOrEqual(0);
  let depth = 0, index = source.indexOf("{", start);
  const open = index;
  for (; index < source.length; index++) {
    if (source[index] === "{") depth++;
    else if (source[index] === "}" && --depth === 0) break;
  }
  return source.slice(open, index + 1);
};
const calls = (text: string): string[] => [...text.matchAll(/\bpeers\.([a-z_]+)\(/gu)].map(match => match[1]!);

/** 🤝️ The lifecycle owner reaches the patch tracker and the cold pair ingress only through the port, in the reserve, activate, complete, release order the close protocol has always used. */
test("the close owner drives its peers through the port in the closed order", () => {
  expect(calls(body(lifetime, "fn request_close<P: ClosePeers + ?Sized>"))).toEqual(vector.owner.requestClose);
  expect(calls(body(lifetime, "fn advance_admission<P: ClosePeers + ?Sized>"))).toEqual(vector.owner.advanceAdmission);
  expect(calls(body(lifetime, "fn terminal_is_empty<P: ClosePeers + ?Sized>(&self, peers: &P) -> Result<bool, semio_framework::Fault> {\n        if !self.lease"))).toEqual(vector.owner.terminalIsEmpty);
  expect(calls(body(lifetime, "fn release_terminal<P: ClosePeers + ?Sized>(owner: &mut Option<Self>, maximum_items: usize, maximum_bytes: usize, peers: &P)", true))).toEqual(vector.owner.releaseTerminal);
});

test("the port names exactly the ten close steps and the lifecycle names no higher static", () => {
  const port = body(lifetime, "pub trait ClosePeers");
  expect([...port.matchAll(/fn ([a-z_]+)\(&self/gu)].map(match => match[1])).toEqual(vector.port);
  const code = lifetime.replace(/\/\/[^\n]*/gu, "");
  for (const name of vector.forbiddenInLifecycle) expect(code.includes(name), name).toBe(false);
});

test("the reactor core implements every step of the port", () => {
  expect(reactor).toContain(vector.reactorImplements);
  const implementation = body(reactor, vector.reactorImplements);
  expect([...implementation.matchAll(/fn ([a-z_]+)\(&self/gu)].map(match => match[1])).toEqual(vector.port);
});
