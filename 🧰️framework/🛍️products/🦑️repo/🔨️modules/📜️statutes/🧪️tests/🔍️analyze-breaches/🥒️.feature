@capability-repo-statutes-analyze
@no-oracle-repo-statutes-owned-law
@comparison-ordered-json-v1
Feature: The golden trees breach exactly the statutes they are written to breach
  The committed trees under `🧫️fixtures/📁️some/📁️folder` are the analyzer's evidence: `🧪️file`
  and `🧪️file-invalid` carry deliberate header, section and definition breaches, `🧪️file-fixed`
  and `🧪️file-fixable-expected` are clean, and `🧪️file-fixable` carries exactly the one breach the
  autofix repairs. Analysis runs the header policy, the section policy and the requirements policy in
  that order over the whole tree, so the breach list is ordered and every breach carries the same
  identifier, statute, scope, line and summary in both implementations.

  @id-the-golden-tree-breaches
  @level-fundamental
  @mode-differential
  @seed-1
  Scenario: Every source of the golden tree yields the same breach list
    Given the golden source tree
      | shared://📁️some/📁️folder/🟦️.tsx                        |
      | shared://📁️some/📁️folder/🧪️file/🐍️.py                  |
      | shared://📁️some/📁️folder/🧪️file/🔷️.cs                  |
      | shared://📁️some/📁️folder/🧪️file/🟦️.tsx                 |
      | shared://📁️some/📁️folder/🧪️file-fixable/🟦️.tsx         |
      | shared://📁️some/📁️folder/🧪️file-fixable-expected/🟦️.tsx |
      | shared://📁️some/📁️folder/🧪️file-fixed/🐍️.py            |
      | shared://📁️some/📁️folder/🧪️file-fixed/🐹️.go            |
      | shared://📁️some/📁️folder/🧪️file-fixed/🔷️.cs            |
      | shared://📁️some/📁️folder/🧪️file-fixed/🟦️.tsx           |
      | shared://📁️some/📁️folder/🧪️file-invalid/🐍️.py          |
      | shared://📁️some/📁️folder/🧪️file-invalid/🐹️.go          |
      | shared://📁️some/📁️folder/🧪️file-invalid/🔷️.cs          |
      | shared://📁️some/📁️folder/🧪️file-invalid/🟦️.tsx         |
    When the host analyzes the tree and renders every breach with its identifier, statute, scope, line and summary
    Then every implementation projects the reviewed golden breach list local://🔣️breaches.json

  @id-the-clean-sources-are-clean
  @level-fundamental
  @mode-conformance
  @seed-1
  Scenario: The repaired and expected trees raise nothing
    Given the golden source tree
      | shared://📁️some/📁️folder/🧪️file-fixable-expected/🟦️.tsx |
      | shared://📁️some/📁️folder/🧪️file-fixed/🐍️.py            |
      | shared://📁️some/📁️folder/🧪️file-fixed/🐹️.go            |
      | shared://📁️some/📁️folder/🧪️file-fixed/🔷️.cs            |
      | shared://📁️some/📁️folder/🧪️file-fixed/🟦️.tsx           |
    When the host analyzes only the repaired sources
    Then every implementation projects an empty breach list
