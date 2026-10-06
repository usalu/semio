Feature: Authored scientific surface windows
  Scenario Outline: Surface bodies consume independently authored axes
    Given the neutral rainfall and sampled seismic records
    And explicit language and appearance choices
    When a canonical mutation selects the <window> coordinate window
    And its diff is replayed, inverted, inferred and compiled with pinned Tectonic
    Then all rainfall rectangle corners match independent D3 scales
    And every seismic polyline vertex matches independent D3 scales and waveform samples
    And omitted windows retain reversed native defaults
    And input snapshots remain unchanged

    Examples:
      | window   |
      | omitted  |
      | native-omitted |
      | domain   |
      | range    |
      | both     |
      | reversed |
