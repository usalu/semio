@capability-repo.codebase.technology-detection
@no-oracle-repo-codebase-technology-detection
@comparison-ordered-json-v1
Feature: Technologies and bundles are detected the same way everywhere
  Which root directories are technologies, which of their children are bundles, what kind each
  bundle is and which emoji it renders with are all read off the filesystem: a `README.md` front
  matter block names and classifies a technology, an `AGENTS.md` front matter block carries the
  emoji, a `project.json` or `package.json` declares the bundle kind, source root and tags, and the
  `sites` directory is expanded one level deeper as site bundles. Everything downstream keys on the
  resulting bundle label. This layout convention is this repository's own — no external tool reads
  it — so the recorded no-oracle decision `repo-codebase-technology-detection` rests the case on two
  independent implementations detecting the same tree.

  @id-technologies-and-bundles-agree
  @level-fundamental
  @mode-differential
  Scenario: The detected technologies and their bundles agree
    Given the repository tree shared://📡️repo-tree.json materialised into the work directory
    When each implementation detects the technologies and bundles of that tree
    Then every implementation projects the same names, roots, kinds, emojis, source roots and tags in the same order

  @id-bundle-ids-and-labels-agree
  @level-fundamental
  @mode-differential
  Scenario: The emoji id and the normalised label of every bundle agree
    Given the repository tree shared://📡️repo-tree.json materialised into the work directory
    When each implementation renders the emoji id and the normalised label of every detected bundle
    Then every implementation projects the same id and label per bundle
