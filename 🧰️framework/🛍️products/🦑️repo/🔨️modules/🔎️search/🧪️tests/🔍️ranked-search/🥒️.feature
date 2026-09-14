@capability-repo-search-rank
@capability-repo-search-append-replay
@oracle-semio-search-reference-ts
@no-oracle-repo-search-owned-ranking
@comparison-ordered-json-v1
Feature: Ranked search is deterministic, conjunctive and bounded
  A query is a conjunction: a document that misses one term is not a hit at all. A term scores
  `fuzziness - distance + 1`, where the distance is zero when the term appears anywhere in the whole
  text, and otherwise the smallest bounded edit distance to any word. Words are the runs of ASCII
  letters, ASCII digits and non-ASCII characters. Ties are broken by identifier, ascending. The
  reported total counts every hit BEFORE the size limit truncates the page. No third-party engine
  implements this combination, so the reference is a second implementation written here from the
  same schema — see `repo-search-owned-ranking`.

  @id-vectors-rank-the-same-way
  @level-fundamental
  @mode-differential
  @seed-1
  Scenario: Every vector produces the same total and the same ordered hits
    Given the shared vector set shared://📡️ranking-vectors.json
    When the host indexes each vector's documents, applies its deletions and re-indexings, and searches
    Then every implementation projects the same total and the same ordered hits per vector
