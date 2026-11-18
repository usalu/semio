Feature: Learning Clips of the Architecture Course
  Scenario: Every rendered video is a clip with a poster
    Given the videos the authoring manifest renders
    When the course lists its learning clips
    Then there is one clip per video, in the manifest's order
    And each clip's poster is the still of the video's first chapter, a 640 by 360 PNG built into the site's hashed assets
  Scenario: Hosting is added later
    Given the owner has not provided any links yet
    Then no clip has an address and each shows its poster as coming soon
    And an address, once added, must be an HTTPS address
  Scenario: Pages may show the assignment PDFs
    Given the release document is sealed
    Then frames are admitted from the site's own origin only and plugins stay refused
