/** 📏️ Source-gate budget of one compilation unit. */
export type CrateBudget = Readonly<{ files: number; bytes: number; lines: number }>;

/** 🎯️ The size every authored crate aims for; the hard gate leaves headroom for growth. */
export const CRATE_TARGET_BUDGET: CrateBudget = { files: 28, bytes: 230000, lines: 3500 };

/** 🚧️ The compiler-ingress source boundary. */
export const CRATE_HARD_BUDGET: CrateBudget = { files: 32, bytes: 262144, lines: 4000 };

/** 🎚️ Parses the `files/bytes/lines` text of a `--target` flag into a crate budget; every part is a positive integer. */
export function parseCrateBudget(text: string): CrateBudget {
  const parts = text.split("/");
  const numbers = parts.map((part) => (/^[1-9][0-9]*$/u.test(part) ? Number(part) : Number.NaN));
  if (parts.length !== 3 || numbers.some((value) => !Number.isSafeInteger(value))) throw new Error(`a crate budget is written <files>/<bytes>/<lines> with positive integers, got ${JSON.stringify(text)}`);
  return { files: numbers[0]!, bytes: numbers[1]!, lines: numbers[2]! };
}
