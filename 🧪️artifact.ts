// #region 🧪️Artifact Test Boundary

import {
  describe as runnerDescribe,
  expect as runnerExpect,
  it as runnerIt,
} from "vitest";

export interface ArtifactExpectation {
  toBe(expected: unknown): void;
  toBeGreaterThan(expected: number): void;
  toContain(expected: unknown): void;
}

/** 🪆️ Registers an artifact-test suite through the repository test runner. */
export function describe(name: string, suite: () => void): void {
  runnerDescribe(name, suite);
}

/** 🧫️ Registers one synchronous or asynchronous artifact assertion. */
export function it(name: string, assertion: () => void | Promise<void>): void {
  runnerIt(name, assertion);
}

/** 🔎️ Adapts repository-runner assertions to the stable artifact-test contract. */
export function expect(actual: unknown): ArtifactExpectation {
  const assertion = runnerExpect(actual);
  return {
    toBe: (expected) => assertion.toBe(expected),
    toBeGreaterThan: (expected) => assertion.toBeGreaterThan(expected),
    toContain: (expected) => assertion.toContain(expected),
  };
}

// #endregion 🧪️Artifact Test Boundary
