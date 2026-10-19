//! 🔬️ Introspection queries: the window and surface queries, the retained input draft, the presented tree sources and the scene lookups. See [`super`].

use super::*;

// #region engine
//#region 🔬️Introspection
/// 🔬️ Read-only accessors for the wgpu↔React parity structural-dump harness (see
/// `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY` and `framework/renderer/wgpu`'s own
/// `🔬️Introspection` region, which is the actual JSON-building caller): exposes just enough of
/// `Ui`'s private `windows`/`theme` state for a caller to walk a window's retained `UiTree` (via
/// `UiTree::node`/`UiTree::children`, both already public) and know which theme it last painted
/// with. Purely additive and read-only — no new engine behavior, nothing here is called from
/// `apply_tree`/`frame`/`dispatch_event`'s own pipeline.
impl Ui {
    /// 🪟️ Every window id this façade currently tracks retained state for (`HashMap` iteration
    /// order — not insertion order; a caller needing a deterministic pick must sort/filter itself).
    pub fn window_ids(&self) -> impl Iterator<Item = &str> {
        self.windows.ids().map(AsRef::as_ref)
    }

    /// 🪟️ Resolves one stable registry slot for a caller-owned `0..UI_LAYOUT_SURFACE_SLOTS` cursor.
    pub fn clock_surface_at(&self, slot: usize) -> Option<(&str, UiSurfaceToken)> {
        let token = self.windows.token_at(slot)?;
        self.windows.get_token(token).filter(|window| window.closing.is_none())?;
        self.windows.id(token).map(|id| (id.as_ref(), token))
    }

    /// 📐️ `window_id`'s last `set_viewport`/`frame` viewport, if that window has any retained state.
    pub fn viewport(&self, window_id: &str) -> Option<(f32, f32)> {
        self.windows.get(window_id).map(|window| window.viewport)
    }

    /// 🪟️ Every windowed Tree container `window_id` presents, measured against the scroll viewport it paints in — the host half
    /// of the tree window observer (`mounted_layout::tree_window_measures`, the twin of React's `treeWindowContainersUnder`):
    /// `(viewport height, containers)`, or `None` when the surface presents no windowed container.
    pub fn tree_window_measures(&self, window_id: &str) -> Option<(f64, Vec<semio_framework_ui_wgpu_light::tree_window::TreeWindowContainerMeasure>)> {
        let window = self.windows.get(window_id).filter(|window| window.closing.is_none())?;
        let tree = if window.presented_ready { &window.presented_tree } else { &window.tree };
        semio_framework_ui_wgpu_mounted_layout::tree_window_measures(tree, &self.theme, window.viewport, window.router.flow().block == ui_contract::FlowBlock::Up)
    }

    /// 🌲️ Read-only access to `window_id`'s retained tree (root + `Node` arena) for a caller to walk.
    pub fn tree(&self, window_id: &str) -> Option<&UiTree> {
        self.windows.get(window_id).map(|window| if window.presented_ready { &window.presented_tree } else { &window.tree })
    }

    /// 🧾️ Captures only an addressed input commit's current focused edit buffer.
    pub fn retained_input_draft(&self, window_id: &str, intent: &semio_framework_ui_action::UiIntentCommand) -> Option<RetainedInputDraft> {
        if intent.trigger != ui_contract::Trigger::Commit { return None; }
        let surface = self.windows.token(window_id)?;
        let tree = self.tree(window_id)?;
        let id = tree.document_node(UiNodeId(intent.address.node))?;
        let node = tree.node(id)?;
        let UiNode::Input(input) = &node.spec.0 else { return None; };
        if node.intent.as_ref()?.address != intent.address { return None; }
        Some(RetainedInputDraft { surface, node: id, key: node.key.clone(), text: node.state.edit.as_ref()?.text.clone(), action: input.on_change.clone() })
    }

    /// 🛟️ Restores an explicitly discarded submission while preserving newer text and replacement generations.
    pub fn discard_retained_input_draft(&mut self, window_id: &str, submitted: &RetainedInputDraft) -> bool {
        if self.windows.token(window_id) != Some(submitted.surface) { return false; }
        let Some(window) = self.windows.get_mut(window_id).filter(|window| window.closing.is_none()) else { return false };
        let tree = if window.presented_ready { &mut window.presented_tree } else { &mut window.tree };
        let Some(node) = tree.node_mut(submitted.node).filter(|node| node.key == submitted.key && matches!(&node.spec.0, UiNode::Input(input) if input.on_change == submitted.action)) else { return false };
        let Some(published) = semio_framework_ui_wgpu_events::editable_value(&node.spec.0) else { return false };
        let Some(edit) = node.state.edit.as_mut().filter(|edit| edit.text == submitted.text && edit.composition.is_none()) else { return false };
        edit.text = published;
        edit.caret = edit.text.len(); edit.anchor = edit.caret; edit.composition = None; edit.scroll_x = 0.0;
        node.state.number_refusal = None;
        tree.mark_dirty(submitted.node, NodeFlags::DIRTY_PAINT);
        if window.presented_ready {
            window.advance_presented_interaction();
            mark_window_paint_dirty(window);
        }
        true
    }

    /// 💬️ The disabled row action whose reason hint `window_id` shows (audit W1E-1) — its row's document id, the action index and
    /// the reason it shows — read from the router that owns input.
    pub fn revealed_row_reason(&self, window_id: &str) -> Option<(UiNodeId, usize, String)> {
        let window = self.windows.get(window_id)?;
        let (tree, router) = if window.presented_ready { (&window.presented_tree, &window.presented_router) } else { (&window.tree, &window.router) };
        let (row, index) = router.revealed_row_reason()?;
        Some((tree.document_id(row)?, index, semio_framework_ui_wgpu_events::disabled_row_action_reason(tree, row, index)?.to_string()))
    }

    /// 💬️ The absolute rect of `row`'s `index`-th action icon in `window_id` — where a pointer fires or hovers it and its reason
    /// hint anchors.
    pub fn row_action_icon_rect(&self, window_id: &str, row: UiNodeId, index: usize) -> Option<semio_framework_ui_theme::geometry::Rect> {
        let window = self.windows.get(window_id)?;
        let (tree, router) = if window.presented_ready { (&window.presented_tree, &window.presented_router) } else { (&window.tree, &window.router) };
        router.row_action_icon_rect(tree, tree.document_node(row)?, index)
    }

    /// 🥞️ Reports accepted floating content that must remain above browser external hosts.
    pub fn presented_has_overlay(&self, window_id: &str, surface: UiSurfaceToken) -> bool {
        if self.windows.id(surface).is_none_or(|id| id.as_ref() != window_id) {
            return true;
        }
        self.windows.get_token(surface).is_none_or(|window| !window.presented_ready || !window.presented_router.open_overlays().is_empty() || window.presented_tooltip.is_some())
    }

    /// 🧪️ Moves one existing fixture tree into an empty admitted accepted window; no GPU acceptance is asserted.
    #[cfg(any(test,feature="testkit"))]
    pub fn donate_original_presented_fixture_tree(&mut self,surface:UiSurfaceToken,original:&mut Option<UiTree>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        let window=self.windows.get_token_mut(surface).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original accepted fixture surface is absent"))?;
        if window.presented_source.is_some()||window.presented_ready||window.presented_tree.root.is_some()||original.is_none(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original accepted fixture source or receiver custody refused").into())}
        window.presented_tree=original.take().unwrap();window.presented_revision=window.presented_tree.document().map_or(1,|document|document.revision().0);window.presented_accessibility_generation=1;window.presented_ready=true;
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }
    /// 🧪️ Returns that same original fixture owner only after every actual accepted-source loan was returned.
    #[cfg(any(test,feature="testkit"))]
    pub fn take_original_presented_fixture_tree(&mut self,surface:UiSurfaceToken,target:&mut Option<UiTree>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        let window=self.windows.get_token_mut(surface).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original accepted fixture surface is absent"))?;
        if window.presented_source.is_some()||target.is_some(){return Ok(false)}
        *target=Some(std::mem::replace(&mut window.presented_tree,UiTree::new()));window.presented_ready=false;
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }
    /// 🎟️ Admits one exact accepted-tree alias through the initiating caller's original stock and clock.
    pub fn admit_presented_tree_source(&mut self,surface:UiSurfaceToken,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<UiPresentedTreeLoan>,semio_framework_job::OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        let Some(window)=self.windows.get_token_mut(surface).filter(|window|window.closing.is_none()&&window.presented_ready)else{return Ok(None)};
        if window.presented_source.is_some(){return Ok(None)}
        let identity=UiPresentedSourceIdentity{operation:cx.operation(),generation:cx.generation(),revision:window.presented_revision,accessibility:window.presented_accessibility_generation,interaction:window.presented_interaction_epoch,clock:cx.clock_source()as usize,deadline:cx.deadline_us()};
        window.presented_source=Some(identity);cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(Some(UiPresentedTreeLoan{surface,identity,active:true}))
    }
    /// 👓️ Borrows the original accepted owner without copying a tree, document or params source.
    pub fn original_presented_tree_source(&self,loan:&UiPresentedTreeLoan)->Option<&UiTree>{
        let window=self.windows.get_token(loan.surface)?;if !loan.active||window.presented_source!=Some(loan.identity)||window.presented_revision!=loan.identity.revision||window.presented_accessibility_generation!=loan.identity.accessibility||window.presented_interaction_epoch!=loan.identity.interaction{return None}Some(&window.presented_tree)
    }
    /// 🤝️ Returns the same alias before accepted-tree replacement or retirement resumes.
    pub fn return_presented_tree_source(&mut self,loan:&mut UiPresentedTreeLoan,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if !loan.active{return Ok(true)}let identity=loan.identity;
        if(identity.operation,identity.generation,identity.clock,identity.deadline)!=(cx.operation(),cx.generation(),cx.clock_source()as usize,cx.deadline_us()){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"accepted tree alias requires its original caller").into())}
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        let window=self.windows.get_token_mut(loan.surface).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"accepted tree alias lost its original surface"))?;
        if window.presented_source!=Some(identity){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"accepted tree alias owner changed before return").into())}
        window.presented_source=None;loan.active=false;cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }

    /// 🎬️ Exposes only the tree whose surface identity was acknowledged by GPU presentation.
    pub fn presented_tree(&self, window_id: &str, surface: UiSurfaceToken) -> Option<&UiTree> {
        if self.windows.id(surface).is_none_or(|id| id.as_ref() != window_id) {
            return None;
        }
        self.windows.get_token(surface).filter(|window| window.closing.is_none() && window.presented_ready).map(|window| &window.presented_tree)
    }

    /// 🎨️ Read-only candidate tree for the paint/hit-registration owner before presentation.
    pub fn candidate_tree(&self, window_id: &str) -> Option<&UiTree> {
        self.windows.get(window_id).map(|window| &window.tree)
    }

    pub fn presented_document_id(&self, window_id: &str, node: NodeId) -> Option<UiNodeId> {
        self.windows.get(window_id)?.presented_tree.document_id(node)
    }

    pub fn presented_node_at(&self, window_id: &str, surface: UiSurfaceToken, document_id: UiNodeId) -> Option<NodeId> {
        if self.windows.id(surface).is_none_or(|id| id.as_ref() != window_id) {
            return None;
        }
        let window = self.windows.get_token(surface).filter(|window| window.closing.is_none() && window.presented_ready)?;
        window.presented_tree.document_node(document_id)
    }

    pub fn retained_document_id(&self, window_id: &str, node: NodeId) -> Option<UiNodeId> {
        let window = self.windows.get(window_id)?;
        if window.presented_ready {
            window.presented_tree.document_id(node)
        } else {
            window.tree.document_id(node)
        }
    }

    pub fn candidate_document_id(&self, window_id: &str, node: NodeId) -> Option<UiNodeId> {
        self.windows.get(window_id)?.tree.document_id(node)
    }

    /// 🧬️ Returns the retained tree identity revision used to reject stale interactive intents.
    pub fn tree_revision(&self, window_id: &str) -> Option<u64> {
        self.windows.get(window_id).map(|window| if window.presented_ready { window.presented_revision } else { window.revision })
    }

    pub fn surface_generation(&self, window_id: &str) -> Option<u64> {
        self.windows.get(window_id).map(|window| if window.presented_ready { window.presented_accessibility_generation } else { window.accessibility_generation })
    }

    pub fn candidate_surface_generation(&self, window_id: &str) -> Option<u64> {
        self.windows.get(window_id).map(|window| window.accessibility_generation)
    }

    /// 🖱️ The `ComponentScene` leaf under a WINDOW-LOCAL point, resolved through the same
    /// reverse-paint-order walk a real press uses (`events::hit_test`) and the same ancestor-offset
    /// accumulation `scene_slots` does — but WITHOUT mutating focus, capture, hover or the tree.
    /// This is the read-only query a right-click needs: the host has to know which surface (and with
    /// what geometry) a context menu is about to be built for before any event is dispatched, and
    /// dispatching a `PointerDown` to find out would move focus and arm a press capture.
    ///
    /// A point on chrome inside the scene's own subtree still answers that scene, because the walk
    /// climbs `parent` links until it finds one — the same rule React's `onContextMenu` gets for free
    /// from DOM event bubbling.
    pub fn scene_at(&self, window_id: &str, x: f32, y: f32) -> Option<UiSceneHit> {
        let window = self.windows.get(window_id).filter(|window| window.closing.is_none())?;
        let tree = if window.presented_ready { &window.presented_tree } else { &window.tree };
        let root = tree.root?;
        let mut cursor = Some(semio_framework_ui_wgpu_events::hit_test(tree, root, x, y)?);
        while let Some(id) = cursor {
            if let Some(slot) = scene_slot_for_node_absolute(tree, id) {
                return Some(slot);
            }
            cursor = tree.node(id)?.parent;
        }
        None
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn progressive_layout_preview(&self, window_id: &str) -> Option<MountedLayoutResult> {
        self.windows.get(window_id).and_then(|window| window.layout_preview)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn progressive_glyph_preview(&self, window_id: &str) -> Option<RetainedGlyphPreview> {
        self.windows.get(window_id).and_then(|window| window.glyph_preview)
    }

    /// 🎨️ The theme this façade last painted every window with (`Theme` is `Copy`).
    pub fn theme(&self) -> Theme {
        self.theme
    }

    /// 🎯️ Whether `window_id`'s retained content currently has a focused node — `false` if that
    /// window has no retained state at all. Lets a host (`w2-input-wiring`,
    /// `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w2-input-wiring.md`) decide whether real
    /// keyboard/IME events belong to this window's content (route via `dispatch_event`) or should
    /// fall back to chrome-level shortcuts. Forwards to `EventRouter::is_focused`, itself added this
    /// same pass — both purely additive reads, no change to `dispatch_event`'s own focus logic.
    pub fn window_has_focus(&self, window_id: &str) -> bool {
        self.windows.get(window_id).is_some_and(|window| window.presented_ready && window.presented_router.is_focused())
    }

    /// 🖱️ The window whose retained content currently holds pointer capture, if any — a host routes
    /// every pointer move THERE while a gesture is live, regardless of what its own flat hit
    /// registry answers for the current point.
    ///
    /// 🩸️ Without it a `Slider` (or `Ring`) drag died the instant the pointer left the control's own
    /// rect: the host resolved the point to some other target, stopped feeding the router moves, and
    /// the value froze mid-drag — while a browser keeps the gesture with the element that captured
    /// it until release (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn window_with_pointer_capture(&self, pointer_id: u64) -> Option<&str> {
        self.windows.ids().map(AsRef::as_ref).find(|window_id| self.windows.get(window_id).is_some_and(|window| window.presented_ready && window.presented_router.capture_pointer_id() == Some(pointer_id)))
    }

    pub fn any_pointer_capture(&self) -> Option<(&str, u64)> {
        self.windows.ids().map(AsRef::as_ref).find_map(|window_id| {
            let window = self.windows.get(window_id)?;
            window.presented_ready.then(|| window.presented_router.capture_pointer_id()).flatten().map(|pointer_id| (window_id, pointer_id))
        })
    }
}
//#endregion 🔬️Introspection
// #endregion engine
