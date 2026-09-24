@capability-repo.providers.github-management
@oracle-gh-transcript-reader-typescript
@comparison-ordered-json-v1
Feature: The GitHub management provider issues one fixed argv sequence and parses what gh returns
  Every call the provider makes to the GitHub CLI is a single ProcessRunner invocation, so a recorded
  transcript replaces the machine entirely: no gh binary, no network, no repository. What a scenario
  projects is therefore both halves of the contract at once — the exact argv the provider issued, and
  the record it parsed back out of gh's output. A transcript that does not carry an argv answers exit
  127, so an argv the provider changed can never be mistaken for a parse that still works.

  @id-issue-view-is-parsed
  @level-fundamental
  @mode-differential
  Scenario: An issue view is parsed into url, state, title, body, milestone and labels
    Given the recorded gh transcript shared://🐙️github-management-transcripts/🎞️gh-transcripts.json for this scenario
    When the provider is asked for the details of issue 412
    Then the parsed issue and the issued argv are projected

  @id-milestone-list-is-scanned-for-a-title
  @level-fundamental
  @mode-differential
  Scenario: The paginated milestone stream is scanned line by line for one title
    Given the recorded gh transcript shared://🐙️github-management-transcripts/🎞️gh-transcripts.json for this scenario
    When the provider is asked to find the milestone titled 26/09
    Then the matched milestone and the issued argv are projected

  @id-label-catalog-is-listed
  @level-fundamental
  @mode-differential
  Scenario: The repository label catalogue is parsed out of the label list
    Given the recorded gh transcript shared://🐙️github-management-transcripts/🎞️gh-transcripts.json for this scenario
    When the provider is asked to list the repository labels
    Then the parsed labels and the issued argv are projected

  @id-create-issue-resolves-the-milestone-title-first
  @level-fundamental
  @mode-differential
  Scenario: Creating an issue resolves the milestone title, extracts the url and follows up
    Given the recorded gh transcript shared://🐙️github-management-transcripts/🎞️gh-transcripts.json for this scenario
    When the provider creates a ticket issue against milestone 17
    Then the created url and the whole issued argv sequence are projected

  @id-a-failing-call-carries-the-trimmed-stderr
  @level-fundamental
  @mode-error
  Scenario: A non-zero gh exit is reported as a failure carrying the trimmed stderr
    Given the recorded gh transcript shared://🐙️github-management-transcripts/🎞️gh-transcripts.json for this scenario
    When the provider is asked for the details of an issue that does not exist
    Then a failure carrying the trimmed stderr is projected instead of an empty record
