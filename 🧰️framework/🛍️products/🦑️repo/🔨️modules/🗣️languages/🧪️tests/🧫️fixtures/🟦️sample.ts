// #region 🔖️Alpha
export const answer = 42;

// #region 🔖️Nested
export function twice(n: number): number {
  return n * 2;
}
// #endregion 🔖️Nested

export const shout = (word: string) => word.toUpperCase();
// #endregion 🔖️Alpha

// #region 🔖️Beta
export interface Shape {
  sides: number;
}
// #endregion 🔖️Beta
