// #region 🧲️Header
// 🖥️ framework/ui/modules/fuzzy-ranking/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔎️FuzzyRanking
export type FuzzySearchField<Item> = {
  readonly read: (item: Item) => string | null | undefined;
  readonly weight: number;
};

export type FuzzySearchResult<Item> = {
  readonly item: Item;
  readonly refIndex: number;
  readonly score: number;
};

export type FuzzySearchOptions<Item> = {
  readonly fields: readonly FuzzySearchField<Item>[];
  readonly threshold: number;
  readonly limit: number;
};

const FUZZY_TEXT_LIMIT = 96;

function normalizeFuzzyText(value: string): string {
  return value.normalize("NFKD").replace(/\p{M}/gu, "").toLowerCase().trim().slice(0, FUZZY_TEXT_LIMIT);
}

function boundedDamerauLevenshtein(left: string, right: string, maximum: number): number | null {
  if (Math.abs(left.length - right.length) > maximum) return null;
  const width = right.length + 1;
  let previousPrevious = new Uint16Array(width);
  let previous = new Uint16Array(width);
  let current = new Uint16Array(width);
  for (let column = 0; column < width; column += 1) previous[column] = column;
  for (let row = 1; row <= left.length; row += 1) {
    current[0] = row;
    let rowMinimum = row;
    const from = Math.max(1, row - maximum);
    const to = Math.min(right.length, row + maximum);
    for (let column = 1; column < from; column += 1) current[column] = maximum + 1;
    for (let column = from; column <= to; column += 1) {
      const substitution = previous[column - 1] + (left[row - 1] === right[column - 1] ? 0 : 1);
      let distance = Math.min(previous[column] + 1, current[column - 1] + 1, substitution);
      if (row > 1 && column > 1 && left[row - 1] === right[column - 2] && left[row - 2] === right[column - 1]) {
        distance = Math.min(distance, previousPrevious[column - 2] + 1);
      }
      current[column] = distance;
      rowMinimum = Math.min(rowMinimum, distance);
    }
    for (let column = to + 1; column < width; column += 1) current[column] = maximum + 1;
    if (rowMinimum > maximum) return null;
    [previousPrevious, previous, current] = [previous, current, previousPrevious];
  }
  return previous[right.length] <= maximum ? previous[right.length] : null;
}

function fuzzyTokenScore(query: string, value: string, threshold: number): number | null {
  if (!value) return null;
  if (value === query) return 0;
  const extraRatio = Math.max(0, value.length - query.length) / Math.max(1, value.length);
  if (value.startsWith(query)) return 0.04 + extraRatio * 0.12;
  const substringIndex = value.indexOf(query);
  if (substringIndex >= 0) return 0.1 + (substringIndex / value.length) * 0.12 + extraRatio * 0.12;

  let queryIndex = 0;
  let firstIndex = -1;
  let lastIndex = -1;
  for (let valueIndex = 0; valueIndex < value.length && queryIndex < query.length; valueIndex += 1) {
    if (value[valueIndex] !== query[queryIndex]) continue;
    if (firstIndex < 0) firstIndex = valueIndex;
    lastIndex = valueIndex;
    queryIndex += 1;
  }
  if (queryIndex === query.length) {
    const span = lastIndex - firstIndex + 1;
    const gaps = Math.max(0, span - query.length);
    const score = 0.2 + (gaps / Math.max(1, span)) * 0.18 + extraRatio * 0.08 + (firstIndex / value.length) * 0.06;
    if (score <= threshold) return score;
  }

  const maximumDistance = Math.max(1, Math.floor(query.length * threshold));
  let bestDistance: number | null = null;
  for (const candidate of value.split(/[^\p{L}\p{N}]+/u)) {
    if (!candidate) continue;
    const distance = boundedDamerauLevenshtein(query, candidate, maximumDistance);
    if (distance !== null && (bestDistance === null || distance < bestDistance)) bestDistance = distance;
  }
  if (bestDistance === null) return null;
  const score = 0.18 + (bestDistance / Math.max(1, query.length)) * 0.45;
  return score <= threshold ? score : null;
}

/** @emoji 🔎️ Ranks items with deterministic prefix, substring, subsequence, and bounded typo matching. */
export function rankFuzzyItems<Item>(items: readonly Item[], query: string, options: FuzzySearchOptions<Item>): FuzzySearchResult<Item>[] {
  const limit = Math.max(0, Math.floor(options.limit));
  if (limit === 0) return [];
  const normalizedQuery = normalizeFuzzyText(query);
  if (!normalizedQuery) return items.slice(0, limit).map((item, refIndex) => ({ item, refIndex, score: 0 }));
  const queryTokens = normalizedQuery.split(/\s+/u).filter(Boolean);
  const maximumWeight = Math.max(1, ...options.fields.map((field) => Math.max(0, field.weight)));
  const results: FuzzySearchResult<Item>[] = [];
  for (let refIndex = 0; refIndex < items.length; refIndex += 1) {
    const item = items[refIndex];
    if (item === undefined) continue;
    const fields = options.fields.map((field) => ({ value: normalizeFuzzyText(field.read(item) ?? ""), weight: Math.max(0, field.weight) }));
    let score = 0;
    let matched = true;
    for (const token of queryTokens) {
      let tokenScore: number | null = null;
      for (const field of fields) {
        const fieldScore = fuzzyTokenScore(token, field.value, options.threshold);
        if (fieldScore === null) continue;
        const weightedScore = fieldScore + ((maximumWeight - field.weight) / maximumWeight) * 0.08;
        if (tokenScore === null || weightedScore < tokenScore) tokenScore = weightedScore;
      }
      if (tokenScore === null || tokenScore > options.threshold) {
        matched = false;
        break;
      }
      score += tokenScore;
    }
    if (matched) results.push({ item, refIndex, score: score / queryTokens.length });
  }
  return results.sort((left, right) => left.score - right.score || left.refIndex - right.refIndex).slice(0, limit);
}
// #endregion 🔎️FuzzyRanking
