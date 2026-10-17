/// 🌳️ Tree units and chrome bands: the persisted tree units, the Display branch leaves and the navbar and footer band fixtures.

//#region 🌳️TreeUnits
/// 🌳️ Every persisted LEAF carries its single `<leafId>.tree` unit and every BRANCH carries none —
/// React's `panelTabNodeToSkeleton`, so a skeleton this renderer writes round-trips through React's
/// own unit resolution instead of dropping every leaf's units.
#[test]
fn a_persisted_skeleton_carries_each_leafs_tree_unit() {
    let shell = fixture_dock_shell();
    let skeleton = dock_skeleton_of(&SessionCtx::of(&mut shell).default_dock());
    fn walk(entries: &[DockTabSkeleton]) {
        for entry in entries {
            match entry.children.as_deref() {
                Some(children) => {
                    assert_eq!(entry.trees, None, "🌳️ a branch carries no tree units: {}", entry.id);
                    walk(children);
                }
                None => assert_eq!(entry.trees.as_deref(), Some([dock_leaf_tree_unit_id(&entry.id)].as_slice()), "🌳️ leaf {} carries its own single unit", entry.id),
            }
        }
    }
    for anchor in PanelAnchor::ALL {
        walk(skeleton.anchors.get(anchor.as_str()).map(Vec::as_slice).unwrap_or_default());
    }
}
//#endregion 🌳️TreeUnits

//#region 🧭️ChromeBandFixture
/// 🖥️ The Display branch is FRAMEWORK chrome: its children are the framework's own two display
/// leaves (React's `createFrameworkDisplayPanelTabs`), and an app's display-group tabs are its
/// SIBLINGS. Before this, wgpu built the branch out of the app's tabs and dropped the whole "Display"
/// chip for an app that declares none — which is exactly what the first real wgpu boot showed.
#[test]
fn default_dock_display_branch_carries_the_framework_display_leaves() {
    let fixture = default_dock_fixture();
    let shell = fixture_dock_shell();
    let dock = SessionCtx::of(&mut shell).default_dock();
    let bottom_left = dock.tabs(PanelAnchor::BottomLeft);
    let branch = bottom_left.first().expect("bottom-left opens with the framework Display branch");
    assert_eq!(branch.id, fixture["displayBranch"]["id"].as_str().expect("fixture display branch id"));
    assert!(branch.is_branch(), "the Display branch nests the framework display leaves");
    let expected: Vec<&str> = fixture["displayBranch"]["children"].as_array().expect("fixture display children").iter().map(|child| child.as_str().expect("child id")).collect();
    assert_eq!(branch.children.iter().map(|child| child.id.as_str()).collect::<Vec<_>>(), expected);
    assert!(bottom_left.iter().any(|tab| tab.id == "fixture.display"), "🧭️ an app's display-group tab is a SIBLING of the branch, not one of its children");
    assert!(!branch.children.iter().any(|child| child.id == "fixture.display"));
    let empty_app_shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    assert!(SessionCtx::of(&mut empty_app_shell).default_dock().tabs(PanelAnchor::BottomLeft).is_empty(), "🧭️ no session means no dock at all");
}

/// 🧭️ A dock whose chrome bands are exactly what the shared fixture declares.
fn banded_dock_shell() -> ShellState {
    let mut shell = fixture_dock_shell();
    shell.dock.dock_override = None;
    SessionCtx::of(&mut shell).sync_dock_tabs();
    shell
}

/// 📑️ **The navbar band pin.** Top-left's folded tabs read at the navbar's LEADING edge in declared
/// order, and the trailing row is enumerated right-to-left so top-middle/top-right read left-to-right
/// with the last tab nearest `Fullscreen` — React's `navbarItems` order. The pre-parity renderer
/// packed all three navbar anchors onto the trailing side in reverse, which is why the boot showed
/// `Catalogue · Artifact · Chat · Tool runs · Inspection` all on the right.
#[test]
fn navbar_bands_match_the_react_chrome_band_fixture() {
    let fixture = default_dock_fixture();
    let shell = banded_dock_shell();
    let bands = &fixture["chromeBands"]["navbar"];
    let leading: Vec<&str> = bands["leading"].as_array().expect("navbar leading band").iter().map(|anchor| anchor.as_str().expect("anchor id")).collect();
    let trailing: Vec<&str> = bands["trailing"].as_array().expect("navbar trailing band").iter().map(|anchor| anchor.as_str().expect("anchor id")).collect();
    let expected_leading: Vec<(String, String)> = leading
        .iter()
        .flat_map(|anchor| {
            let anchor = PanelAnchor::from_str(anchor).expect("fixture anchor");
            shell.dock.dock_tabs.tabs(anchor).iter().map(move |node| (anchor.as_str().to_string(), node.id.clone())).collect::<Vec<_>>()
        })
        .collect();
    for (index, (anchor, id)) in expected_leading.iter().enumerate() {
        let (actual_anchor, actual) = ChromeViewCtx::of(&mut shell).navbar_leading_tab_row_item(index).expect("the leading band carries every top-left tab");
        assert_eq!((actual_anchor.as_str(), actual.id.as_str()), (anchor.as_str(), id.as_str()), "📑️ leading navbar slot {index}");
    }
    assert!(ChromeViewCtx::of(&mut shell).navbar_leading_tab_row_item(expected_leading.len()).is_none(), "📑️ the leading band ends with top-left's last tab");
    let mut expected_trailing: Vec<(String, String)> = trailing
        .iter()
        .flat_map(|anchor| {
            let anchor = PanelAnchor::from_str(anchor).expect("fixture anchor");
            shell.dock.dock_tabs.tabs(anchor).iter().map(move |node| (anchor.as_str().to_string(), node.id.clone())).collect::<Vec<_>>()
        })
        .collect();
    expected_trailing.reverse();
    for (index, (anchor, id)) in expected_trailing.iter().enumerate() {
        let (actual_anchor, actual) = ChromeViewCtx::of(&mut shell).navbar_trailing_tab_row_item(index).expect("the trailing band carries every top-middle/top-right tab");
        assert_eq!((actual_anchor.as_str(), actual.id.as_str()), (anchor.as_str(), id.as_str()), "📑️ trailing navbar paint slot {index}");
    }
    assert!(ChromeViewCtx::of(&mut shell).navbar_trailing_tab_row_item(expected_trailing.len()).is_none());
    assert!(!expected_leading.is_empty() && !expected_trailing.is_empty(), "🧭️ the fixture app populates both navbar bands");
}

/// 📑️ Footer bands follow React's measured placement: edge bands hug their edge and the centered
/// group clamps to the free span left by panel tabs, presence, and the Hub indicator.
#[test]
fn footer_bands_match_the_react_chrome_band_fixture() {
    let fixture = default_dock_fixture();
    let shell = banded_dock_shell();
    let theme = Theme::light();
    let mut atlas = FontAtlas::builtin();
    let (width, btn_h) = (1440.0_f32, theme.control_height);
    let lead_x = theme.padding_standard;
    let mut rows = Vec::new();
    for index in 0..64 {
        let Some((anchor, node, rect)) = PanelsViewCtx::of(&mut shell).footer_tab_row_rect(&mut atlas, &theme, index, width, 0.0, btn_h) else { break };
        rows.push((anchor, node.id.clone(), rect));
    }
    assert!(!rows.is_empty(), "the fixture dock populates the footer");
    let band = |name: &str| -> Vec<(PanelAnchor, String, Rect)> {
        let anchor = PanelAnchor::from_str(name).expect("fixture anchor");
        rows.iter().filter(|(candidate, _, _)| *candidate == anchor).cloned().collect()
    };
    for name in fixture["chromeBands"]["footer"]["leading"].as_array().expect("footer leading band").iter().map(|value| value.as_str().expect("anchor id")) {
        let rows = band(name);
        assert_eq!(rows.first().map(|(_, _, rect)| rect.x), Some(lead_x), "📑️ {name} opens at the footer's leading edge");
    }
    for name in fixture["chromeBands"]["footer"]["centre"].as_array().expect("footer centre band").iter().map(|value| value.as_str().expect("anchor id")) {
        let rows = band(name);
        let (Some(first), Some(last)) = (rows.first(), rows.last()) else { continue };
        let span = (last.2.x + last.2.w) - first.2.x;
        let layout = PanelsViewCtx::of(&mut shell).footer_chrome_layout(&mut atlas, &theme, width, 0.0, btn_h);
        let free = [ShellChromeBandSpan { left: 0.0, right: layout.center.free.left }, ShellChromeBandSpan { left: layout.center.free.right, right: width }];
        let expected = shell_chrome_centered_band(width, &free, span).centered.left;
        assert!((first.2.x - expected).abs() < 0.01, "📑️ {name} sits where the shared centred-band rule (`navbarCenteredLeftV1`, whole pixels inside the band) places it: first={first:?}, last={last:?}, span={span}, expected={expected}");
        assert!(first.2.x >= layout.center.free.left && last.2.x + last.2.w <= layout.center.free.right + 0.01);
    }
    for name in fixture["chromeBands"]["footer"]["trailing"].as_array().expect("footer trailing band").iter().map(|value| value.as_str().expect("anchor id")) {
        let rows = band(name);
        let Some(last) = rows.last() else { continue };
        assert!(((last.2.x + last.2.w) - (width - theme.padding_standard)).abs() < 0.5, "📑️ {name} ends at the footer's trailing edge");
    }
    for window in rows.windows(2) {
        let (left, right) = (&window[0], &window[1]);
        assert!(left.2.x + left.2.w <= right.2.x + 0.01, "📑️ footer chips never overlap: {} then {}", left.1, right.1);
    }
}
//#endregion 🧭️ChromeBandFixture
