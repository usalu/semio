@capability-repo.dashboard.command-tree
@no-oracle-repo-dashboard-command-tree
@comparison-ordered-json-v1
Feature: One workspace always projects the same dashboard command tree
  The dashboard discovers what it can run by walking a repository: every `📋️project.json` target
  becomes a registry command and a wizard path whose first step is the verb its name begins with (`task` for any other
  name), whose middle steps are the taxonomy segments of the manifest's directory with the noise
  segments dropped and whose last step is the exact target name, and the repo
  domain contributes its own branches — one branch per ticket found under
  `.🧬semio/🦑️repo/🎫️tickets` with its show, files, close and reopen actions, a goals branch, an
  analyze branch per scope, a tree branch per projection and the statute catalog. A repo leaf
  carries both the action key the Rust crates answer in process and the `semio-repo` argv the Go
  implementation answers the same operation with, so `SEMIO_REPO_IMPLEMENTATION` changes which
  implementation runs and never what the wizard offers. The projection is the JSON document
  `🧬️schema/🔣️.json` describes. See `repo-dashboard-command-tree`.

  @id-the-fixture-workspace-projects-its-command-tree
  @level-fundamental
  @mode-conformance
  Scenario: A frozen workspace projects one command tree
    Given the workspace vector shared://🌳️command-tree-projection/🏗️workspace.json
    When each implementation materialises the workspace and discovers its command tree
    Then every implementation projects the same tree document, with the nx targets first, each leaf naming its registry command id, and the repo-domain branches carrying their action keys and their Go argv

  @id-the-projection-does-not-depend-on-directory-order
  @level-quick
  @mode-property
  Scenario: The projection does not depend on the order the workspace files were written in
    Given the workspace vector shared://🌳️command-tree-projection/🏗️workspace.json
    When each implementation materialises the workspace twice, once in the given order and once in the reversed order, and discovers the command tree of both
    Then every implementation projects the same tree document for both orders, because the walk sorts by verb order at the root and alphabetically below it
