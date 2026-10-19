Feature: Tree Hover Ancestor Emphasis
  Tree labels, chevrons and indentation guides use the semantic emphasized border color.
  Selection retains its foreground and background independently of pointer hover.

  Scenario Outline: Paint a tree path from the shared appearance fixture
    Given the items and expanded folders in the tree-emphasis fixture
    And the selected item and filter in case "<case>"
    When the pointer hovers the case item
    Then only the hovered item and its ancestor labels and chevrons are emphasized
    And guides belonging to ancestor branches are emphasized
    And unrelated branch guides retain their normal border color
    And selected labels and backgrounds retain their selection colors
    And the color planes match independent tiny-skia rendering
    Examples:
      | case                       |
      | leaf-path                  |
      | selected-ancestor          |
      | filtered-path              |
      | filtered-selected-ancestor |
      | collapsed-folder           |
      | expanded-folder            |
      | cleared-path               |

  Scenario: Hover an expanded folder without emphasizing its descendants
    Given the expanded-folder fixture case with Folder open beneath Root
    When the pointer hovers Folder
    Then Root and Folder labels and chevrons are emphasized
    And Root branch guides remain emphasized throughout its descendants
    And Folder branch guides beneath Folder retain their normal border color
    And descendant labels retain their normal foreground
    And selection retains its foreground and background

  Scenario: Keep hover emphasis visible when semantic palette colors are equal
    Given normal foreground and emphasized border roles resolve to the same terminal color
    When the pointer hovers a leaf
    Then nonselected path labels and chevrons carry the ANSI bold attribute
    And unrelated labels and chevrons retain their original attributes
    And selected path labels and chevrons retain their focused or unfocused selection attributes

  Scenario: Rebuild a hovered tree
    Given a hovered descendant in an expanded tree
    When its ancestor collapses or the filter changes
    Then stale row positions do not emphasize a sibling

  Scenario: Clip a deeply nested tree
    Given a hovered leaf with ancestors above the viewport
    When the viewport clips labels or guides
    Then the visible path guides remain emphasized
    And no paint escapes the tree rectangle
    And keyboard parent and child navigation retains its selection behavior
