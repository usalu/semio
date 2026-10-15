//! 🧹️ Board host scene sync and paint, delete planning, link snapping and pointer commit planning.

use super::*;

impl BoardHost {
    // #region board_host
    /// 🔗️ Authoring sync: every edge the descriptor adds is announced as an `edgeCreate` event.
    pub fn sync_descriptor(&mut self, desc: &SceneDescriptor) -> Result<(), NormalPortError> {
        self.sync_descriptor_with(desc, true)
    }

    /// 🛂️ Every refusal a descriptor sync can raise BEFORE it touches the scene: the two fixed
    /// descriptor ceilings and the handle colours it would have to parse. A sync that fails halfway
    /// leaves a half-built board, and a snapshot parse that clears first leaves an EMPTY one (three
    /// blank panes, 2026-09-17), so both preflight through here and mutate only once it passes.
    fn descriptor_admission(desc: &SceneDescriptor) -> Result<(), NormalPortError> {
        let entity_count = desc
            .nodes
            .len()
            .checked_add(desc.handles.len())
            .and_then(|count| count.checked_add(desc.edges.len()))
            .and_then(|count| count.checked_add(desc.wires.len()))
            .and_then(|count| count.checked_add(desc.regions.len()))
            .ok_or(NormalPortError::EventCredits)?;
        if entity_count > BOARD_DESCRIPTOR_ITEM_CAPACITY {
            return Err(NormalPortError::EventCredits);
        }
        let entity_id_bytes = desc
            .nodes
            .iter()
            .map(|node| node.id.len())
            .chain(desc.handles.iter().map(|handle| handle.id.len()))
            .chain(desc.edges.iter().map(|edge| edge.id.len()))
            .chain(desc.wires.iter().map(|wire| wire.id.len()))
            .chain(desc.regions.iter().map(|region| region.id.len()))
            .try_fold(0usize, usize::checked_add)
            .ok_or(NormalPortError::EventCredits)?;
        if entity_id_bytes > BOARD_DESCRIPTOR_BYTE_CAPACITY {
            return Err(NormalPortError::EventCredits);
        }
        for handle in &desc.handles {
            if let Some(color) = handle.color.as_ref().map(|value| value.trim()).filter(|value| !value.is_empty()) {
                Self::parse_css_color(color).ok_or_else(|| NormalPortError::InvalidHandleColor(handle.id.clone(), color.to_string()))?;
            }
        }
        Ok(())
    }

    /// 🚚️ One descriptor sync; `announce_new_edges` decides whether edges absent from the current
    /// scene emit `edgeCreate`. A snapshot parse passes `false`: it clears the scene first, so every
    /// edge is "new" — announcing them echoed the WHOLE document back to the plugin as edge
    /// creations after each re-parse (fill placements re-committing themselves) and, undrained,
    /// they exhausted the event credits so the next parse of any edged document was refused
    /// (three blank panes after one drag, 2026-09-17).
    fn sync_descriptor_with(&mut self, desc: &SceneDescriptor, announce_new_edges: bool) -> Result<(), NormalPortError> {
        Self::descriptor_admission(desc)?;
        if matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. }) {
            self.interaction = Interaction::None;
            self.clear_link_gesture_events();
        }
        if announce_new_edges {
            let mut created_edge_count = 0usize;
            let mut created_edge_bytes = 0usize;
            for edge in desc.edges.iter().filter(|edge| !self.edges.contains_key(&edge.id)) {
                created_edge_count = created_edge_count.checked_add(1).ok_or(NormalPortError::EventCredits)?;
                let bytes = board_edge_event_owned_bytes(&edge.id, &edge.source, &edge.target).ok_or(NormalPortError::EventCredits)?;
                created_edge_bytes = created_edge_bytes.checked_add(bytes).ok_or(NormalPortError::EventCredits)?;
            }
            self.events.reserve(created_edge_count, created_edge_bytes).map_err(|_| NormalPortError::EventCredits)?;
        }
        let want_nodes: BTreeSet<_> = desc.nodes.iter().map(|n| n.id.clone()).collect();
        let want_handles: BTreeSet<_> = desc.handles.iter().map(|h| h.id.clone()).collect();
        let want_edges: BTreeSet<_> = desc.edges.iter().map(|e| e.id.clone()).collect();
        let want_wires: BTreeSet<_> = desc.wires.iter().map(|w| w.id.clone()).collect();
        let want_regions: BTreeSet<_> = desc.regions.iter().map(|r| r.id.clone()).collect();
        self.regions.retain(|id, _| want_regions.contains(id));
        self.edges.retain(|id, _| want_edges.contains(id));
        self.wires.retain(|id, _| want_wires.contains(id));
        self.handles.retain(|id, _| want_handles.contains(id));
        self.nodes.retain(|id, _| want_nodes.contains(id));
        for n in &desc.nodes {
            let shape = if n.shape.as_deref() == Some("rectangle") { NodeShape::Rectangle } else { NodeShape::Circle };
            let (radius, width, height) = match shape {
                NodeShape::Circle => (n.radius.unwrap_or(0.0), 0.0, 0.0),
                NodeShape::Rectangle => (0.0, n.width.unwrap_or(0.0), n.height.unwrap_or(0.0)),
            };
            let node_kind = n.node_kind.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default();
            let properties = n.user_data.as_ref().map(|v| property_bag_from_value(v)).unwrap_or_default();
            self.nodes.insert(
                n.id.clone(),
                NodeData {
                    id: n.id.clone(),
                    x: n.x,
                    y: n.y,
                    shape,
                    radius,
                    width,
                    height,
                    scale: n.scale.filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0),
                    draggable: n.draggable.unwrap_or(true),
                    selected: n.selected.unwrap_or(false),
                    visible: n.visible.unwrap_or(true),
                    locked: n.locked.unwrap_or(false),
                    root: n.root.unwrap_or(false),
                    style: n.style.clone(),
                    text: n.text.clone(),
                    icon_kind: n.icon_kind.clone(),
                    node_kind,
                    properties,
                },
            );
        }
        for h in &desc.handles {
            let kind = h.handle_kind.as_deref().unwrap_or("").trim().to_string();
            let color_fill = match h.color.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
                None => None,
                Some(s) => Some(Self::parse_css_color(s).ok_or_else(|| NormalPortError::InvalidHandleColor(h.id.clone(), s.to_string()))?),
            };
            let icon_kind = h.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
            let properties = h.user_data.as_ref().map(|v| property_bag_from_value(v)).unwrap_or_default();
            self.handles.insert(
                h.id.clone(),
                HandleData {
                    id: h.id.clone(),
                    node_id: h.node_id.clone(),
                    angle: h.angle,
                    radius: h.radius.unwrap_or(ui_styling::radii::HANDLE_DEFAULT),
                    scale: h.scale.filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0),
                    selected: h.selected.unwrap_or(false),
                    visible: h.visible.unwrap_or(true),
                    locked: h.locked.unwrap_or(false),
                    style: h.style.clone(),
                    handle_kind: kind,
                    color_fill,
                    icon_kind,
                    properties,
                },
            );
        }
        for e in &desc.edges {
            let existed = self.edges.contains_key(&e.id);
            let edge_kind = e.edge_kind.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default();
            let source_tip = Self::parse_catalog_tip_slot(e.source_tip.as_deref());
            let target_tip = Self::parse_catalog_tip_slot(e.target_tip.as_deref());
            let properties = e.user_data.as_ref().map(|v| property_bag_from_value(v)).unwrap_or_default();
            self.edges.insert(
                e.id.clone(),
                EdgeData {
                    id: e.id.clone(),
                    source: e.source.clone(),
                    target: e.target.clone(),
                    selected: e.selected.unwrap_or(false),
                    visible: e.visible.unwrap_or(true),
                    locked: e.locked.unwrap_or(false),
                    style: e.style.clone(),
                    edge_kind,
                    source_tip,
                    target_tip,
                    properties,
                },
            );
            if announce_new_edges && !existed {
                let event = BoardOwnedEvent::edge(BoardEventKind::EdgeCreate, &e.id, &e.source, &e.target).expect("new descriptor edges were exactly preflighted");
                self.events.push(event).expect("new descriptor edge credits were reserved before mutation");
            }
        }
        for w in &desc.wires {
            let target = w.target.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
            let (end_x, end_y) = match &target {
                Some(_) => (None, None),
                None => {
                    let x = match w.end_x {
                        Some(v) if v.is_finite() => Some(v),
                        _ => None,
                    };
                    let y = match w.end_y {
                        Some(v) if v.is_finite() => Some(v),
                        _ => None,
                    };
                    if x.is_none() || y.is_none() {
                        continue;
                    }
                    (x, y)
                }
            };
            let wire_kind = w
                .wire_kind
                .as_ref()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .or_else(|| self.handles.get(w.source.as_str()).map(|h| self.resolve_default_wire_kind_for_handle(h)))
                .unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string());
            let properties = w.user_data.as_ref().map(|v| property_bag_from_value(v)).unwrap_or_default();
            self.wires.insert(
                w.id.clone(),
                WireData {
                    id: w.id.clone(),
                    source: w.source.clone(),
                    target,
                    end_x,
                    end_y,
                    selected: w.selected.unwrap_or(false),
                    visible: w.visible.unwrap_or(true),
                    locked: w.locked.unwrap_or(false),
                    style: w.style.clone(),
                    wire_kind,
                    properties,
                },
            );
        }
        for r in &desc.regions {
            self.regions.insert(
                r.id.clone(),
                RegionData {
                    id: r.id.clone(),
                    x: r.x,
                    y: r.y,
                    width: r.width,
                    height: r.height,
                    label: r.label.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
                    hidden: r.hidden.unwrap_or(false),
                    locked: r.locked.unwrap_or(false),
                    selected: r.selected.unwrap_or(false),
                },
            );
        }
        if !self.is_preselect_active() {
            let mut new_selection = BTreeSet::new();
            for r in &desc.regions {
                if r.selected == Some(true) {
                    new_selection.insert(r.id.clone());
                }
            }
            for n in &desc.nodes {
                if n.selected == Some(true) {
                    new_selection.insert(n.id.clone());
                }
            }
            for h in &desc.handles {
                if h.selected == Some(true) {
                    new_selection.insert(h.id.clone());
                }
            }
            for e in &desc.edges {
                if e.selected == Some(true) {
                    new_selection.insert(e.id.clone());
                }
            }
            for w in &desc.wires {
                if w.selected == Some(true) {
                    new_selection.insert(w.id.clone());
                }
            }
            let prev_sel = self.selection.clone();
            if prev_sel != new_selection {
                self.selection_exit_highlight.clear();
            }
            self.selection = new_selection;
            if prev_sel != self.selection {
                self.push_select_event();
            }
        }
        self.sync_selection_flags_to_objects();
        self.bump_content_scene_generation();
        Ok(())
    }

    /// 📍️ Applies peer-pane node drags without a full descriptor re-sync.
    pub fn set_node_positions(&mut self, moves: &[(String, f64, f64)]) {
        let mut geometry_changed = false;
        for (id, x, y) in moves {
            if !x.is_finite() || !y.is_finite() {
                continue;
            }
            if let Some(node) = self.nodes.get_mut(id.as_str()) {
                if (node.x - *x).abs() > 1e-9 || (node.y - *y).abs() > 1e-9 {
                    node.x = *x;
                    node.y = *y;
                    geometry_changed = true;
                }
            }
        }
        if geometry_changed {
            self.bump_content_scene_generation();
        }
    }

    /// 📍️ Parses `[{"id","x","y"},…]` and updates existing host nodes in place.
    pub fn set_node_positions_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        #[derive(Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
        struct NodePositionMoveJson {
            id: String,
            x: f64,
            y: f64,
        }
        let rows: Vec<NodePositionMoveJson> = serde_json::from_str(json)?;
        let moves: Vec<(String, f64, f64)> = rows.into_iter().map(|row| (row.id, row.x, row.y)).collect();
        self.set_node_positions(&moves);
        Ok(())
    }

    pub fn clear_scene(&mut self) {
        self.regions.clear();
        self.edges.clear();
        self.wires.clear();
        self.handles.clear();
        self.nodes.clear();
        self.selection.clear();
        self.preselect.clear();
        self.preselect_removed.clear();
        self.selection_exit_highlight.clear();
    }

    /// 🎲️ Replaces the whole board with owned snapshot facts, ALL OR NOTHING: the descriptor is built and admitted
    /// first, and only a snapshot that will really paint clears the live scene. Clearing before the
    /// refusal left the panes blank until the next parse — the refusal and "the board went empty"
    /// were the same event (2026-09-17 battery, `16-inspection/inspector-fresh-on-open`).
    pub fn load_board_snapshot(&mut self, f: BoardSnapshot) -> bool {
        let port_mode = match f.schema.as_str() {
            "board.normal.undirected.v1" => GraphPortMode::Normal,
            "board.ports.directed.v1" => GraphPortMode::Ported,
            _ => return false,
        };
        let has_ports = port_mode.has_ports();
        let camera = f.camera.as_ref().map(|camera| (camera.x, camera.y, camera.zoom));
        let Some(desc) = Self::board_snapshot_scene_descriptor(f, has_ports) else {
            return false;
        };
        if Self::descriptor_admission(&desc).is_err() {
            return false;
        }
        self.port_mode = port_mode;
        if !has_ports {
            self.selection_options.select_handles = false;
        }
        if let Some((x, y, zoom)) = camera {
            self.set_camera(x, y, zoom);
        }
        self.clear_scene();
        self.sync_descriptor_with(&desc, false).is_ok()
    }

    /// 🧾️ The pure half of [`Self::load_board_snapshot_json`]: one snapshot document into one scene
    /// descriptor, or `None` for a document this port refuses. Touches no board state.
    ///
    /// 🎯️ Target regions are optional by construction (the artifact omits an empty collection),
    /// so a malformed row is skipped rather than refusing the whole document: a board that
    /// cannot paint its constraint rectangles must still paint its graph.
    fn board_snapshot_scene_descriptor(f:BoardSnapshot,has_ports:bool)->Option<SceneDescriptor>{
        let text=|value:Option<String>|value.map(|v|v.trim().to_owned()).filter(|v|!v.is_empty());
        let positive=|value:Option<f64>|value.filter(|v|v.is_finite()&&*v>0.0);
        let mut desc=SceneDescriptor::default();
        for node in f.nodes {
            let(x,y)=(node.x?,node.y?);if !x.is_finite()||!y.is_finite(){return None;}
            let visible=board_visible_option(&node.visibility());let locked=board_locked_option(&node.visibility());
            if has_ports {for h in node.handles? {let angle=h.angle?;if !angle.is_finite(){return None;}let hv=board_visible_option(&h.visibility());let hl=board_locked_option(&h.visibility());desc.handles.push(HandleDescriptor{id:h.id,node_id:node.id.clone(),angle,radius:positive(h.radius),scale:positive(h.scale),selected:None,style:None,handle_kind:Some(text(h.handle_kind).unwrap_or_else(||"port".into())),color:text(h.color),icon_kind:text(h.icon_kind),user_data:None,visible:hv,locked:hl});}}
            else if node.handles.is_some(){return None;}
            let (shape,radius,width,height)=if node.shape==Some(infinite::board::ports::directed::schema::snapshot::BoardNodeShape::Rectangle){("rectangle",None,Some(positive(node.width)?),Some(positive(node.height)?))}else{("circle",Some(positive(node.radius)?),None,None)};
            desc.nodes.push(NodeDescriptor{id:node.id,x,y,draggable:None,selected:None,style:None,text:text(node.text),icon_kind:text(node.icon_kind),node_kind:text(node.node_kind),user_data:None,visible,locked,root:node.root,shape:Some(shape.into()),radius,width,height,scale:positive(node.scale)});
        }
        for edge in f.edges {let visible=board_visible_option(&edge.visibility());let locked=board_locked_option(&edge.visibility());let(source,target)=(edge.source?,edge.target?);if !has_ports{let ids:BTreeSet<&str>=desc.nodes.iter().map(|n|n.id.as_str()).collect();if !ids.contains(source.as_str())||!ids.contains(target.as_str()){return None;}}
            desc.edges.push(EdgeDescriptor{id:edge.id,source,target,edge_kind:text(edge.edge_kind),source_tip:text(edge.source_tip),target_tip:text(edge.target_tip),selected:None,style:None,user_data:None,visible,locked});}
        for region in f.target_regions {let Some(id)=text(Some(region.id))else{continue;};let finite=|value:Option<f64>|value.filter(|v|v.is_finite()).unwrap_or(0.0);desc.regions.push(RegionDescriptor{id,x:finite(region.x),y:finite(region.y),width:finite(region.width),height:finite(region.height),label:text(region.label),hidden:region.hidden,locked:region.locked,selected:region.selected});}Some(desc)
    }

    fn drawable_cull_pad_world(&self) -> f64 {
        16.0 / self.camera.zoom.max(1e-9)
    }

    fn handle_world_bounds_cull(&self, h: &HandleData) -> Option<WorldBox> {
        let pos = self.handle_world_pos(h)?;
        let pad = self.drawable_cull_pad_world() + self.effective_handle_radius(h).max(1.0);
        Some(inflate_world_box(WorldBox { min_x: pos.x, min_y: pos.y, max_x: pos.x, max_y: pos.y }, pad))
    }

    fn indirect_handle_world_bounds_cull(&self, h: &HandleData) -> Option<WorldBox> {
        let pos = self.indirect_handle_world_pos(h)?;
        let pad = self.drawable_cull_pad_world() + self.indirect_handle_marker_radius_world(h).max(1.0);
        Some(inflate_world_box(WorldBox { min_x: pos.x, min_y: pos.y, max_x: pos.x, max_y: pos.y }, pad))
    }

    fn edge_world_bounds_for_cull(&self, e: &EdgeData) -> Option<WorldBox> {
        let c = self.edge_curve(e)?;
        let axis = cubic_bezier_axis_bounds(c);
        let half_w_world = self.camera.zoom.max(0.75) / self.camera.zoom.max(1e-9);
        Some(inflate_world_box(axis, half_w_world + self.drawable_cull_pad_world()))
    }

    fn stroke_world_step_grid(&self, scene: &mut Scene, color: Color, stroke_px: f64, world_step: f64, min_step_screen: f64) {
        let step = world_step * self.camera.zoom;
        if step < min_step_screen {
            return;
        }
        let stroke = Stroke::new(stroke_px);
        let w = self.width as f64;
        let h = self.height as f64;
        let origin = self.world_to_screen(Point::new(0.0, 0.0));
        let x_off = ((origin.x % step) + step) % step;
        let y_off = ((origin.y % step) + step) % step;
        let mut p = semio_framework_canvas::BezPath::new();
        let mut x = x_off;
        while x <= w {
            p.move_to(Point::new(x, 0.0));
            p.line_to(Point::new(x, h));
            x += step;
        }
        let mut y = y_off;
        while y <= h {
            p.move_to(Point::new(0.0, y));
            p.line_to(Point::new(w, y));
            y += step;
        }
        scene.stroke(&stroke, Affine::IDENTITY, color, None, &p);
    }

    pub(super) fn draw_space_point(&self, world: Point, world_space: bool) -> Point {
        if world_space {
            world
        } else {
            self.world_to_screen(world)
        }
    }

    pub(super) fn draw_space_len(&self, len_world: f64, world_space: bool) -> f64 {
        if world_space {
            len_world.max(1e-9)
        } else {
            (len_world * self.camera.zoom).max(1.0)
        }
    }

    fn append_handle_marker(
        &self,
        scene: &mut Scene,
        h: &HandleData,
        center: Point,
        radius_world: f64,
        draw_icon: bool,
        style_kind: BoardElementStyleKind,
        paint_override: Option<(Color, Color, f64)>,
        world_space: bool,
        layer: NodeHandlePaintLayer,
        exterior_cap: bool,
    ) {
        let c = self.draw_space_point(center, world_space);
        let r = self.draw_space_len(radius_world, world_space);
        let (fill, stroke_c, stroke_px) =
            if let Some((f, s, sw)) = paint_override { (f, s, sw) } else { (self.resolve_handle_fill_color(h, &self.canvas_theme, style_kind), self.resolve_handle_stroke_color(h, &self.canvas_theme, style_kind), 2.0_f64) };
        let paint_fill = matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill);
        let paint_stroke = matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Stroke);
        let paint_icons = draw_icon && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Icons);
        let outward = if exterior_cap { self.nodes.get(h.node_id.as_str()).and_then(|n| handle_outward_at_node_rim(center, Point::new(n.x, n.y), n.shape, n.radius, n.width, n.height)) } else { None };
        if paint_fill {
            if let Some(out) = outward {
                scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &handle_exterior_cap_fill_path(c, out, r));
            } else {
                scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &Circle::new(c, r));
            }
        }
        if paint_stroke {
            if let Some(out) = outward {
                scene.stroke(&Stroke::new(stroke_px), Affine::IDENTITY, stroke_c, None, &handle_exterior_cap_stroke_path(c, out, r));
            } else {
                scene.stroke(&Stroke::new(stroke_px), Affine::IDENTITY, stroke_c, None, &Circle::new(c, r));
            }
        }
        if paint_icons {
            if let Some(k) = h.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
                let preserve_original_style = self.preserve_original_element_style || style_kind == BoardElementStyleKind::Original;
                let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&self.canvas_theme);
                if let Some(paint) = self.get_or_build_icon_paint(k, icon_fg, icon_bg, preserve_original_style) {
                    let (bx, by, bw, bh) = paint.bounds();
                    let fit_inset = 0.62;
                    let s = self.draw_space_len(radius_world, world_space) * fit_inset;
                    let cx = bx + bw * 0.5;
                    let cy = by + bh * 0.5;
                    let avail = 2.0 * s;
                    let scale = (avail / bw).min(avail / bh);
                    let aff = Affine::IDENTITY.translate((c.x - scale * cx, c.y - scale * cy)) * Affine::IDENTITY.scale(scale);
                    let r_clip = self.draw_space_len(radius_world, world_space) * 0.82;
                    let disc = Circle::new(c, r_clip);
                    scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &disc);
                    match paint.body() {
                        CachedIconBody::Vector(icon_scene) => {
                            scene.append(icon_scene, Some(aff));
                        }
                        CachedIconBody::Raster(img) => {
                            scene.draw_image(img, aff);
                        }
                    }
                    scene.pop_layer();
                }
            }
        }
    }

    fn append_indirect_handle_ring(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, node_id: &str, chrome_pass: StyleChromePass, world_space: bool) {
        for h in self.handles.values() {
            if h.node_id != node_id || !self.handle_effectively_visible(h.id.as_str()) {
                continue;
            }
            if !self.indirect_ring_handle_eligible(h.id.as_str(), node_id) {
                continue;
            }
            if let Some(tb) = tile_filter {
                let Some(hb) = self.indirect_handle_world_bounds_cull(h) else { continue };
                if !world_boxes_overlap(*tb, hb) {
                    continue;
                }
            }
            let Some(wp) = self.indirect_handle_world_pos(h) else { continue };
            let style_kind = self.resolve_handle_style_kind(h, chrome_pass);
            let stroke_px = 2.0_f64;
            let paint_override = if matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral) { Some((self.canvas_theme.indirect_handle_fill, self.canvas_theme.indirect_handle_stroke, stroke_px)) } else { None };
            self.append_handle_marker(scene, h, wp, self.indirect_handle_marker_radius_world(h), false, style_kind, paint_override, world_space, NodeHandlePaintLayer::Full, false);
        }
    }

    /// 📏️ Screen-pixel edge stroke width (world-clip tiles and post-cache overlay).
    fn edge_screen_stroke_width_px(&self, lod: BoardDrawLod) -> f64 {
        match lod {
            BoardDrawLod::Minimap => ui_styling::strokes::EDGE_MINIMAP,
            BoardDrawLod::Overview | BoardDrawLod::Compact => (ui_styling::strokes::EDGE_OVERVIEW).max(ui_styling::strokes::EDGE_BASE * self.camera.zoom),
            _ => 2.0 * self.camera.zoom.max(0.75),
        }
    }

    /// 📏️ Edge stroke in world units so {@link BoardHost.camera_content_affine} yields ~{@link Self::edge_screen_stroke_width_px}.
    fn edge_world_stroke_width(&self, lod: BoardDrawLod) -> f64 {
        let screen_px = self.edge_screen_stroke_width_px(lod);
        let z = self.camera.zoom.max(1e-9);
        (screen_px / z).max(1e-3)
    }

    fn paint_node_geometry(&self, scene: &mut Scene, n: &NodeData, lod: BoardDrawLod, world_space: bool, layer: NodeHandlePaintLayer, chrome_pass: StyleChromePass, link_compat: bool) {
        let draw_node_icons = matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro);
        let resolved_style_kind = self.resolve_node_style_kind(n, chrome_pass);
        let style_kind = if link_compat && matches!(resolved_style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral) { BoardElementStyleKind::Highlighted } else { resolved_style_kind };
        let draw_node_stroke = lod != BoardDrawLod::Minimap || !matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral);
        let stroke_c = Self::node_stroke_for_style(&self.canvas_theme, style_kind);
        let fill = if lod == BoardDrawLod::Minimap { stroke_c } else { self.resolve_node_fill_color(n, &self.canvas_theme, style_kind) };
        let sw = 2.0_f64;
        let paint_fill = if lod == BoardDrawLod::Minimap {
            matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill)
        } else {
            matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill) && !matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral)
        };
        let paint_stroke = draw_node_stroke && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Stroke);
        let paint_icons = draw_node_icons && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Icons);
        match n.shape {
            NodeShape::Circle => {
                let c = self.draw_space_point(Point::new(n.x, n.y), world_space);
                let r = self.draw_space_len(self.scaled_node_radius(n), world_space);
                let circle = Circle::new(c, r);
                if paint_fill {
                    scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &circle);
                }
                if paint_stroke {
                    scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &circle);
                }
                if paint_icons {
                    self.paint_node_icon(scene, n, world_space, style_kind, Some(&circle), None);
                }
            }
            NodeShape::Rectangle => {
                let hw = self.scaled_node_width(n) / 2.0;
                let hh = self.scaled_node_height(n) / 2.0;
                let p0 = self.draw_space_point(Point::new(n.x - hw, n.y - hh), world_space);
                let p1 = self.draw_space_point(Point::new(n.x + hw, n.y + hh), world_space);
                let rect = Rect::from_points(p0, p1);
                if paint_fill {
                    scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &rect);
                }
                if paint_stroke {
                    scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &rect);
                }
                if paint_icons {
                    self.paint_node_icon(scene, n, world_space, style_kind, None, Some(rect));
                }
            }
        }
    }

    fn paint_node_icon(&self, scene: &mut Scene, n: &NodeData, world_space: bool, style_kind: BoardElementStyleKind, circle_clip: Option<&Circle>, rect_clip: Option<Rect>) {
        if let Some(k) = n.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
            let preserve_original_style = self.preserve_original_element_style || style_kind == BoardElementStyleKind::Original;
            let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&self.canvas_theme);
            if let Some(paint) = self.get_or_build_icon_paint(k, icon_fg, icon_bg, preserve_original_style) {
                let (bx, by, bw, bh) = paint.bounds();
                let clip_inset = ui_styling::metrics::icon::CLIP_INSET;
                let fit_inset = ui_styling::metrics::icon::FIT_INSET;
                let (sx_half, sy_half) = match n.shape {
                    NodeShape::Circle => {
                        let s = self.draw_space_len(self.scaled_node_radius(n), world_space) * fit_inset;
                        (s, s)
                    }
                    NodeShape::Rectangle => (self.draw_space_len(self.scaled_node_width(n), world_space) * fit_inset * 0.5, self.draw_space_len(self.scaled_node_height(n), world_space) * fit_inset * 0.5),
                };
                let center = self.draw_space_point(Point::new(n.x, n.y), world_space);
                let cx = bx + bw * 0.5;
                let cy = by + bh * 0.5;
                let avail_w = 2.0 * sx_half;
                let avail_h = 2.0 * sy_half;
                let scale = (avail_w / bw).min(avail_h / bh);
                let aff = Affine::IDENTITY.translate((center.x - scale * cx, center.y - scale * cy)) * Affine::IDENTITY.scale(scale);
                match n.shape {
                    NodeShape::Circle => {
                        let r_clip = self.draw_space_len(self.scaled_node_radius(n), world_space) * clip_inset;
                        let disc = circle_clip.copied().unwrap_or_else(|| Circle::new(center, r_clip));
                        scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &disc);
                        match paint.body() {
                            CachedIconBody::Vector(icon_scene) => {
                                scene.append(icon_scene, Some(aff));
                            }
                            CachedIconBody::Raster(img) => {
                                scene.draw_image(img, aff);
                            }
                        }
                        scene.pop_layer();
                    }
                    NodeShape::Rectangle => {
                        let hw = self.draw_space_len(self.scaled_node_width(n), world_space) * clip_inset * 0.5;
                        let hh = self.draw_space_len(self.scaled_node_height(n), world_space) * clip_inset * 0.5;
                        let clip_r = rect_clip.unwrap_or_else(|| Rect::from_points(Point::new(center.x - hw, center.y - hh), Point::new(center.x + hw, center.y + hh)));
                        scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &clip_r);
                        match paint.body() {
                            CachedIconBody::Vector(icon_scene) => {
                                scene.append(icon_scene, Some(aff));
                            }
                            CachedIconBody::Raster(img) => {
                                scene.draw_image(img, aff);
                            }
                        }
                        scene.pop_layer();
                    }
                }
            }
        }
    }

    fn append_nodes_and_handles_with_overlay_chrome(
        &self,
        scene: &mut Scene,
        tile_filter: Option<&WorldBox>,
        lod: BoardDrawLod,
        world_space: bool,
        only_ids: Option<&BTreeSet<String>>,
        overlay_ids: &BTreeSet<String>,
        layer: NodeHandlePaintLayer,
    ) {
        let pad = self.drawable_cull_pad_world();
        let draw_handles = self.has_ports() && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
        let draw_handle_icons = lod == BoardDrawLod::Micro;
        let link_source = self.active_link_source_handle_id().map(str::to_string);
        let link_compat_nodes: BTreeSet<String> = link_source.as_ref().map(|s| self.link_drag_compatible_target_node_ids(s).into_iter().collect()).unwrap_or_default();
        for h in self.handles.values() {
            if !draw_handles || !self.handle_effectively_visible(h.id.as_str()) {
                continue;
            }
            if let Some(ids) = only_ids {
                if !ids.contains(&h.id) {
                    continue;
                }
            }
            if let Some(tb) = tile_filter {
                let Some(hb) = self.handle_world_bounds_cull(h) else { continue };
                if !world_boxes_overlap(*tb, hb) {
                    continue;
                }
            }
            let Some(wp) = self.handle_world_pos(h) else { continue };
            let style_kind = self.resolve_handle_style_kind(h, self.chrome_pass_for_entity(&h.id, overlay_ids));
            self.append_handle_marker(scene, h, wp, self.effective_handle_radius(h), draw_handle_icons, style_kind, None, world_space, layer, true);
        }
        let paint_node = |scene: &mut Scene, n: &NodeData, chrome_pass: StyleChromePass| {
            self.paint_node_geometry(scene, n, lod, world_space, layer, chrome_pass, link_compat_nodes.contains(&n.id));
        };
        for n in self.nodes.values() {
            if !n.visible {
                continue;
            }
            if let Some(ids) = only_ids {
                if !ids.contains(&n.id) {
                    continue;
                }
            }
            if let Some(tb) = tile_filter {
                if !world_boxes_overlap(*tb, self.node_world_bounds(n, pad)) {
                    continue;
                }
            }
            if overlay_ids.contains(&n.id) {
                continue;
            }
            paint_node(scene, n, StyleChromePass::CachedBase);
        }
        for n in self.nodes.values() {
            if !n.visible {
                continue;
            }
            if let Some(ids) = only_ids {
                if !ids.contains(&n.id) {
                    continue;
                }
            }
            if let Some(tb) = tile_filter {
                if !world_boxes_overlap(*tb, self.node_world_bounds(n, pad)) {
                    continue;
                }
            }
            if !overlay_ids.contains(&n.id) {
                continue;
            }
            paint_node(scene, n, StyleChromePass::InteractionOverlay);
        }
    }

    fn append_nodes_and_handles(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, lod: BoardDrawLod, world_space: bool, only_ids: Option<&BTreeSet<String>>, chrome_pass: StyleChromePass, layer: NodeHandlePaintLayer) {
        let pad = self.drawable_cull_pad_world();
        let draw_handles = self.has_ports() && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
        let draw_handle_icons = lod == BoardDrawLod::Micro;
        let link_source = self.active_link_source_handle_id().map(str::to_string);
        let link_compat_nodes: BTreeSet<String> = link_source.as_ref().map(|s| self.link_drag_compatible_target_node_ids(s).into_iter().collect()).unwrap_or_default();
        for h in self.handles.values() {
            if !draw_handles || !self.handle_effectively_visible(h.id.as_str()) {
                continue;
            }
            if let Some(ids) = only_ids {
                if !ids.contains(&h.id) {
                    continue;
                }
            }
            if let Some(tb) = tile_filter {
                let Some(hb) = self.handle_world_bounds_cull(h) else { continue };
                if !world_boxes_overlap(*tb, hb) {
                    continue;
                }
            }
            let Some(wp) = self.handle_world_pos(h) else { continue };
            let style_kind = self.resolve_handle_style_kind(h, chrome_pass);
            self.append_handle_marker(scene, h, wp, self.effective_handle_radius(h), draw_handle_icons, style_kind, None, world_space, layer, true);
        }
        for n in self.nodes.values() {
            if !n.visible {
                continue;
            }
            if let Some(ids) = only_ids {
                if !ids.contains(&n.id) {
                    continue;
                }
            }
            if let Some(tb) = tile_filter {
                if !world_boxes_overlap(*tb, self.node_world_bounds(n, pad)) {
                    continue;
                }
            }
            self.paint_node_geometry(scene, n, lod, world_space, layer, chrome_pass, link_compat_nodes.contains(&n.id));
        }
    }

    fn append_edges_wires_and_link(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, lod: BoardDrawLod, world_space: bool, only_ids: Option<&BTreeSet<String>>, overlay_ids: Option<&BTreeSet<String>>) {
        let edge_sw = if world_space { self.edge_world_stroke_width(lod) } else { self.edge_screen_stroke_width_px(lod) };
        for e in self.edges.values() {
            if !self.edge_effectively_visible(e) {
                continue;
            }
            if let Some(ids) = only_ids {
                if !ids.contains(&e.id) {
                    continue;
                }
            }
            if let Some(tb) = tile_filter {
                let Some(eb) = self.edge_world_bounds_for_cull(e) else { continue };
                if !world_boxes_overlap(*tb, eb) {
                    continue;
                }
            }
            if let Some(c) = self.edge_curve(e) {
                let p0 = self.draw_space_point(c.p0(), world_space);
                let p1 = self.draw_space_point(c.p1(), world_space);
                let p2 = self.draw_space_point(c.p2(), world_space);
                let p3 = self.draw_space_point(c.p3(), world_space);
                let curve = CubicBez::new(p0, p1, p2, p3);
                let chrome_pass = overlay_ids.map_or(StyleChromePass::CachedBase, |ids| self.chrome_pass_for_entity(&e.id, ids));
                let (stroke_color, edge_stroke, stroke_w) = self.resolve_edge_stroke_paint(e, chrome_pass, lod, edge_sw);
                scene.stroke(&edge_stroke, Affine::IDENTITY, stroke_color, None, &curve);
                let (source_tip, target_tip) = self.resolve_edge_tips(e);
                Self::append_edge_tips_on_curve(scene, &curve, stroke_color, stroke_w, source_tip, target_tip);
            }
        }
        let wire_sw = 2.25_f64;
        let wire_stroke = Stroke::new(wire_sw);
        for w in self.wires.values() {
            if !self.wire_effectively_visible(w) {
                continue;
            }
            if let Some(ids) = only_ids {
                if !ids.contains(&w.id) {
                    continue;
                }
            }
            if let Some(c) = self.wire_curve(w) {
                let p0 = self.draw_space_point(c.p0(), world_space);
                let p1 = self.draw_space_point(c.p1(), world_space);
                let p2 = self.draw_space_point(c.p2(), world_space);
                let p3 = self.draw_space_point(c.p3(), world_space);
                let curve = CubicBez::new(p0, p1, p2, p3);
                let chrome_pass = overlay_ids.map_or(StyleChromePass::CachedBase, |ids| self.chrome_pass_for_entity(&w.id, ids));
                let wc = Self::wire_stroke_for_style(&self.canvas_theme, self.resolve_wire_style_kind(w, chrome_pass));
                scene.stroke(&wire_stroke, Affine::IDENTITY, wc, None, &curve);
            }
        }
        let link_wire_sw = 2.85_f64;
        let link_wire_stroke = Stroke::new(link_wire_sw);
        let link_wire_color = self.canvas_theme.node_stroke;
        if let Some(c) = self.active_link_wire_curve() {
            let p0 = self.draw_space_point(c.p0(), world_space);
            let p1 = self.draw_space_point(c.p1(), world_space);
            let p2 = self.draw_space_point(c.p2(), world_space);
            let p3 = self.draw_space_point(c.p3(), world_space);
            let curve = CubicBez::new(p0, p1, p2, p3);
            scene.stroke(&link_wire_stroke, Affine::IDENTITY, link_wire_color, None, &curve);
        }
    }

    /// 🎨️ Bakes each visible node's icon (and its handle icons) as a contiguous span at the node's current center.
    fn bake_node_icon_spans(&self, scene: &mut Scene, lod: BoardDrawLod) -> Vec<NodeIconSpan> {
        let draw_handles = self.has_ports() && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
        let draw_handle_icons = lod == BoardDrawLod::Micro;
        let link_source = self.active_link_source_handle_id().map(str::to_string);
        let link_compat_nodes: BTreeSet<String> = link_source.as_ref().map(|source| self.link_drag_compatible_target_node_ids(source).into_iter().collect()).unwrap_or_default();
        let mut spans = Vec::new();
        for node in self.nodes.values() {
            if !node.visible {
                continue;
            }
            let start = scene.command_len();
            if draw_handles {
                for handle in self.handles.values() {
                    if handle.node_id != node.id || !self.handle_effectively_visible(handle.id.as_str()) {
                        continue;
                    }
                    let Some(world) = self.handle_world_pos(handle) else { continue };
                    let style_kind = self.resolve_handle_style_kind(handle, StyleChromePass::CachedBase);
                    self.append_handle_marker(scene, handle, world, self.effective_handle_radius(handle), draw_handle_icons, style_kind, None, true, NodeHandlePaintLayer::Icons, true);
                }
            }
            self.paint_node_geometry(scene, node, lod, true, NodeHandlePaintLayer::Icons, StyleChromePass::CachedBase, link_compat_nodes.contains(&node.id));
            let end = scene.command_len();
            if end > start {
                spans.push(NodeIconSpan { id: node.id.clone(), start, end, x: node.x, y: node.y });
            }
        }
        spans
    }

    /// 🗺️ Draws baked icon spans at each node's live center. A pending cache retirement keeps the
    /// previous bake; the translation is the whole correction a drag needs.
    fn append_baked_node_icons(&self, scene: &mut Scene, cache: &WorldContentCache, cam_aff: Affine) {
        if cache.spans.is_empty() {
            scene.append(&cache.scene, Some(cam_aff));
            return;
        }
        for span in &cache.spans {
            let Some(node) = self.nodes.get(&span.id) else { continue };
            if !node.visible {
                continue;
            }
            let dx = node.x - span.x;
            let dy = node.y - span.y;
            let placed = if dx.abs() <= 1e-9 && dy.abs() <= 1e-9 { cam_aff } else { cam_aff * Affine::IDENTITY.translate((dx, dy)) };
            scene.append_range(&cache.scene, span.start, span.end, Some(placed));
        }
    }

    /// 🎯️ Target regions are the backdrop: painted before the first entity layer, always in
    /// world space, so nothing they overlap is ever hidden behind them.
    fn append_cached_world_content(&self, scene: &mut Scene, lod: BoardDrawLod) {
        self.advance_opaque_scene_retirement_step();
        let generation = self.content_scene_generation;
        let cam_aff = self.camera_content_affine();
        let overlay_ids = self.interaction_overlay_entity_ids();
        if !self.regions.is_empty() || self.region_paint.is_some() {
            let mut region_layer = Scene::new();
            self.append_target_region_paint(&mut region_layer, true);
            scene.append(&region_layer, Some(cam_aff));
        }
        let mut fill_layer = Scene::new();
        self.append_nodes_and_handles_with_overlay_chrome(&mut fill_layer, None, lod, true, None, &overlay_ids, NodeHandlePaintLayer::Fill);
        scene.append(&fill_layer, Some(cam_aff));
        let mut cache = self.world_content_cache.borrow_mut();
        let needs_rebuild = cache.as_ref().is_none_or(|c| c.generation != generation || c.lod != lod);
        if needs_rebuild && self.opaque_scene_retirement.get().is_none() {
            if cache.is_some() {
                let Some(token) = semio_framework_canvas::reserve_opaque_scene_retirement() else {
                    self.opaque_scene_fault.set(true);
                    return;
                };
                let WorldContentCache { scene: stale, .. } = cache.take().expect("stale world content cache was witnessed occupied");
                self.publish_opaque_scene_retirement(token, stale);
            }
            let mut content = Scene::new();
            let spans = self.bake_node_icon_spans(&mut content, lod);
            *cache = Some(WorldContentCache { generation, lod, scene: content, spans });
        }
        if let Some(cached) = cache.as_ref() {
            self.append_baked_node_icons(scene, cached, cam_aff);
        }
        let edges_in_world_space = matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Minimap);
        if edges_in_world_space {
            let mut edge_layer = Scene::new();
            self.append_edges_wires_and_link(&mut edge_layer, None, lod, true, None, Some(&overlay_ids));
            scene.append(&edge_layer, Some(cam_aff));
        } else {
            self.append_edges_wires_and_link(scene, None, lod, false, None, Some(&overlay_ids));
        }
        let mut stroke_layer = Scene::new();
        self.append_nodes_and_handles_with_overlay_chrome(&mut stroke_layer, None, lod, false, None, &overlay_ids, NodeHandlePaintLayer::Stroke);
        scene.append(&stroke_layer, None);
        if let Some(c) = self.active_link_wire_curve() {
            let link_wire_stroke = Stroke::new(ui_styling::strokes::WIRE_HIGHLIGHT);
            let link_wire_color = self.canvas_theme.node_stroke;
            let p0 = self.draw_space_point(c.p0(), false);
            let p1 = self.draw_space_point(c.p1(), false);
            let p2 = self.draw_space_point(c.p2(), false);
            let p3 = self.draw_space_point(c.p3(), false);
            let curve = CubicBez::new(p0, p1, p2, p3);
            scene.stroke(&link_wire_stroke, Affine::IDENTITY, link_wire_color, None, &curve);
        }
        if self.has_ports() {
            if let Some(node_id) = self.indirect_ring_node_id(lod) {
                self.append_indirect_handle_ring(scene, None, &node_id, StyleChromePass::CachedBase, false);
            }
        }
        let previews_in_world_space = matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Minimap);
        if previews_in_world_space {
            let mut preview_layer = Scene::new();
            if self.drop_preview.is_some() {
                self.append_drop_preview_paint(&mut preview_layer, lod, true);
            }
            if self.active_utility == ActiveUtility::Brush || self.brush_preview.is_some() {
                self.append_brush_preview_paint(&mut preview_layer, lod, true);
            }
            self.append_transform_gumball_paint(&mut preview_layer, true);
            scene.append(&preview_layer, Some(cam_aff));
        } else {
            if self.drop_preview.is_some() {
                self.append_drop_preview_paint(scene, lod, false);
            }
            if self.active_utility == ActiveUtility::Brush || self.brush_preview.is_some() {
                self.append_brush_preview_paint(scene, lod, false);
            }
            self.append_transform_gumball_paint(scene, false);
        }
    }

    pub fn set_wheel_zoom_active(&mut self, active: bool) {
        if active && !self.wheel_zoom_active {
            self.wheel_zoom_render_lod = Some(self.current_draw_lod());
        }
        if !active {
            self.wheel_zoom_render_lod = None;
        }
        self.wheel_zoom_active = active;
    }

    pub fn build_vector_scene(&self) -> Scene {
        let mut inner = Scene::new();
        let lod = self.draw_lod_for_frame();
        if self.grid_visible && !self.wheel_zoom_active {
            let grid_color = self.canvas_theme.grid_minor_stroke;
            if lod != BoardDrawLod::Minimap {
                self.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_LARGE, self.grid_step_large_world(), 0.0);
                match lod {
                    BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro => {
                        self.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_MEDIUM, self.grid_step_medium_world(), 0.0);
                    }
                    BoardDrawLod::Minimap | BoardDrawLod::Overview | BoardDrawLod::Compact => {}
                }
                if matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro) {
                    self.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_SMALL, self.grid_step_small_world(), 0.0);
                }
                if lod == BoardDrawLod::Micro {
                    self.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_MICRO, self.grid_step_micro_world(), 0.0);
                }
            }
        }
        if let Some(ref pts) = self.selection_screen_preview {
            if pts.len() >= 2 {
                let mut path = semio_framework_canvas::BezPath::new();
                path.move_to(pts[0]);
                for p in pts.iter().skip(1) {
                    path.line_to(*p);
                }
                path.close_path();
                inner.fill(FillRule::NonZero, Affine::IDENTITY, self.canvas_theme.selection_preview_fill, None, &path);
                let mut preview_stroke = Stroke::new(ui_styling::strokes::SELECTION_PREVIEW);
                if self.selection_preview_crossing {
                    preview_stroke.set_dash_pattern(vec![5.0, 4.0]);
                }
                inner.stroke(&preview_stroke, Affine::IDENTITY, self.canvas_theme.selection_preview_stroke, None, &path);
            }
        }
        self.append_cached_world_content(&mut inner, lod);
        let scale = self.dpr.max(1.0);
        if (scale - 1.0).abs() < f64::EPSILON {
            inner
        } else {
            let mut scene = Scene::new();
            scene.append(&inner, Some(Affine::IDENTITY.scale(scale)));
            scene
        }
    }

    pub fn encoded_scene_hint(&self) -> usize {
        let s = self.build_vector_scene();
        s.path_count()
    }

    pub fn update_hover_from_world(&mut self, world: Point) {
        let next = self.resolve_hover_world(world);
        self.set_hovered_id(next);
    }

    pub fn set_hovered_id(&mut self, id: Option<String>) {
        let event_kind = id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
        if self.hovered_id == id && self.hovered_kind.is_none() {
            return;
        }
        let Some(reservation) = self.reserve_owned_event(BoardOwnedEvent::hover(id.as_deref(), event_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str())))) else {
            return;
        };
        self.hovered_id = id;
        self.hovered_kind = None;
        self.publish_event_reservation(reservation);
    }

    /// 🖱️ Sets transitive kind hover from a catalog row (clears direct `hovered_id`).
    pub fn set_hovered_kind(&mut self, domain: Option<String>, kind_id: Option<String>) {
        let next_kind = domain.zip(kind_id);
        if self.hovered_id.is_none() && self.hovered_kind == next_kind {
            return;
        }
        let Some(reservation) = self.reserve_owned_event(BoardOwnedEvent::hover(None, next_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str())))) else {
            return;
        };
        self.hovered_id = None;
        self.hovered_kind = next_kind;
        self.publish_event_reservation(reservation);
    }

    /// 🔇️ Updates hover chrome without emitting `hover` (controlled React sync).
    pub fn set_hovered_id_silent(&mut self, id: Option<String>) {
        if self.hovered_id == id && self.hovered_kind.is_none() {
            return;
        }
        self.hovered_id = id;
        self.hovered_kind = None;
    }

    /// 💠️ Externally driven highlight set (cross-panel binding); does not emit events.
    pub fn set_highlighted_ids(&mut self, ids: Vec<String>) {
        let next: BTreeSet<String> = ids.into_iter().collect();
        if self.highlighted_ids == next {
            return;
        }
        self.highlighted_ids = next;
    }

    /// 💠️ Current externally driven highlight ids as JSON array.
    pub fn highlighted_ids_json(&self) -> Result<String, NormalPortError> {
        Ok(serde_json::to_string(&self.highlighted_ids.iter().cloned().collect::<Vec<_>>())?)
    }

    /// 🔇️ Mirrors controlled kind hover without emitting `hover`.
    pub fn set_hovered_kind_silent(&mut self, domain: Option<String>, kind_id: Option<String>) {
        let next_kind = domain.zip(kind_id);
        if self.hovered_id.is_none() && self.hovered_kind == next_kind {
            return;
        }
        self.hovered_id = None;
        self.hovered_kind = next_kind;
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        let plan = self.plan_wheel(sx, sy, delta_y);
        let _ = self.commit_wheel(&plan);
    }

    pub fn plan_wheel(&self, sx: f64, sy: f64, delta_y: f64) -> BoardWheelPlan {
        let mut next = self.camera.clone();
        semio_framework_canvas::camera::wheel_screen(&mut next, &self.viewport(), sx, sy, delta_y);
        BoardWheelPlan { revision: self.interaction_revision, expected: self.camera.clone(), next }
    }

    pub fn commit_wheel(&mut self, plan: &BoardWheelPlan) -> bool {
        if self.interaction_revision != plan.revision || [self.camera.x.to_bits(), self.camera.y.to_bits(), self.camera.zoom.to_bits()] != [plan.expected.x.to_bits(), plan.expected.y.to_bits(), plan.expected.zoom.to_bits()] {
            return false;
        }
        self.set_camera_silent(plan.next.x, plan.next.y, plan.next.zoom);
        self.interaction_revision = plan.revision.wrapping_add(1);
        true
    }

    fn begin_delete_property_audit(&mut self, planning: &mut BoardDeletePlanningOperation, kind: BoardDeleteKind, id: &str) {
        if planning.plan.contains(kind, id) {
            return;
        }
        if id.len() > BOARD_POINTER_BYTE_CAPACITY.saturating_sub(usize::from(planning.plan.byte_len)) {
            planning.fault = Some(BoardEventFault::ByteCredits);
            return;
        }
        let properties = match kind {
            BoardDeleteKind::Edge => self.edges.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
            BoardDeleteKind::Node => self.nodes.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
            BoardDeleteKind::Handle => self.handles.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
            BoardDeleteKind::Wire => self.wires.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
        };
        let Some(properties) = properties else {
            planning.fault = Some(BoardEventFault::Schema);
            return;
        };
        planning.property_audit = Some(BoardPropertyAudit::new(kind, id.to_owned(), properties));
    }

    fn restore_delete_properties(&mut self, kind: BoardDeleteKind, id: &str, properties: semio_framework_graph_manifest::PropertyBag) -> bool {
        match kind {
            BoardDeleteKind::Edge => self.edges.get_mut(id).map(|entity| entity.properties = properties),
            BoardDeleteKind::Node => self.nodes.get_mut(id).map(|entity| entity.properties = properties),
            BoardDeleteKind::Handle => self.handles.get_mut(id).map(|entity| entity.properties = properties),
            BoardDeleteKind::Wire => self.wires.get_mut(id).map(|entity| entity.properties = properties),
        }
        .is_some()
    }

    pub(super) fn step_delete_planning(&mut self) -> BoardAuthorityStep {
        let Some(mut planning) = self.pending_delete_planning.take() else {
            return BoardAuthorityStep::Complete;
        };
        if let Some(audit) = planning.property_audit.as_mut() {
            if !audit.step() {
                self.pending_delete_planning = Some(planning);
                return BoardAuthorityStep::Pending;
            }
            let Some((kind, id, properties, nodes, bytes, fault)) = audit.take_result() else {
                planning.fault = Some(BoardEventFault::Schema);
                self.pending_delete_planning = Some(planning);
                return BoardAuthorityStep::Pending;
            };
            planning.property_audit = None;
            if !self.restore_delete_properties(kind, &id, properties) {
                planning.fault = Some(BoardEventFault::Schema);
            } else if let Some(fault) = fault {
                planning.fault = Some(fault);
            } else if !planning.cancelling {
                if let Err(fault) = planning.plan.push_admitted_entity(kind, &id, nodes, bytes) {
                    planning.fault = Some(fault);
                }
            }
            self.pending_delete_planning = Some(planning);
            return BoardAuthorityStep::Pending;
        }
        if planning.cancelling {
            return BoardAuthorityStep::Cancelled;
        }
        if planning.fault.is_some() || self.interaction_revision != planning.plan.revision {
            self.event_schema_fault = true;
            return BoardAuthorityStep::Fault;
        }
        match planning.phase {
            BoardDeletePlanningPhase::SelectedEdges => {
                let next = match planning.scan_after.as_ref() {
                    Some(after) => self.selection.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                    None => self.selection.first(),
                };
                let next = next.map(|id| admitted_board_pointer_id(id).map(|owned| (self.edges.contains_key(id), owned)));
                if let Some(next) = next {
                    match next {
                        Ok((is_edge, id)) => {
                            planning.scan_after = Some(id.clone());
                            if is_edge {
                                self.begin_delete_property_audit(&mut planning, BoardDeleteKind::Edge, &id);
                            }
                        }
                        Err(fault) => planning.fault = Some(fault),
                    }
                } else {
                    planning.scan_after = None;
                    planning.phase = BoardDeletePlanningPhase::Nodes;
                }
            }
            BoardDeletePlanningPhase::Wires => {
                let handle_id = planning.handle_id.as_deref().expect("wire planning owns a handle");
                let next = match planning.relation_after.as_ref() {
                    Some(after) => self.wires.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                    None => self.wires.first_key_value(),
                }
                .map(|(id, wire)| admitted_board_pointer_id(id).map(|owned| (wire.source == handle_id || wire.target.as_deref() == Some(handle_id), owned)));
                if let Some(next) = next {
                    match next {
                        Ok((remove, id)) => {
                            planning.relation_after = Some(id.clone());
                            if remove {
                                self.begin_delete_property_audit(&mut planning, BoardDeleteKind::Wire, &id);
                            }
                        }
                        Err(fault) => planning.fault = Some(fault),
                    }
                } else {
                    planning.relation_after = None;
                    planning.phase = BoardDeletePlanningPhase::Edges;
                }
            }
            BoardDeletePlanningPhase::Handles => {
                let node_id = planning.node_id.as_deref().expect("handle planning owns a node");
                let next = match planning.handle_after.as_ref() {
                    Some(after) => self.handles.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                    None => self.handles.first_key_value(),
                }
                .map(|(id, handle)| admitted_board_pointer_id(id).map(|owned| (handle.node_id == node_id, owned)));
                if let Some(next) = next {
                    match next {
                        Ok((belongs, id)) => {
                            planning.handle_after = Some(id.clone());
                            if belongs {
                                planning.handle_id = Some(id);
                                planning.relation_after = None;
                                planning.phase = BoardDeletePlanningPhase::Wires;
                            }
                        }
                        Err(fault) => planning.fault = Some(fault),
                    }
                } else {
                    planning.handle_after = None;
                    planning.phase = BoardDeletePlanningPhase::Node;
                }
            }
            BoardDeletePlanningPhase::Edges => {
                let endpoint_id = planning.handle_id.as_deref().or(planning.node_id.as_deref()).expect("edge planning owns an endpoint");
                let next = match planning.relation_after.as_ref() {
                    Some(after) => self.edges.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                    None => self.edges.first_key_value(),
                }
                .map(|(id, edge)| admitted_board_pointer_id(id).map(|owned| (edge.source == endpoint_id || edge.target == endpoint_id, owned)));
                if let Some(next) = next {
                    match next {
                        Ok((remove, id)) => {
                            planning.relation_after = Some(id.clone());
                            if remove {
                                self.begin_delete_property_audit(&mut planning, BoardDeleteKind::Edge, &id);
                            }
                        }
                        Err(fault) => planning.fault = Some(fault),
                    }
                } else {
                    planning.relation_after = None;
                    planning.phase = if planning.handle_id.is_some() { BoardDeletePlanningPhase::Handle } else { BoardDeletePlanningPhase::Node };
                }
            }
            BoardDeletePlanningPhase::Nodes => {
                let next = match planning.node_after.as_ref() {
                    Some(after) => self.nodes.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                    None => self.nodes.first_key_value(),
                }
                .map(|(id, _)| admitted_board_pointer_id(id).map(|owned| (self.selection.contains(id), owned)));
                if let Some(next) = next {
                    match next {
                        Ok((selected, id)) => {
                            planning.node_after = Some(id.clone());
                            planning.node_id = Some(id);
                            planning.node_relevant = selected;
                            planning.handle_after = None;
                            planning.phase = BoardDeletePlanningPhase::DiscoverNode;
                        }
                        Err(fault) => planning.fault = Some(fault),
                    }
                } else {
                    planning.node_after = None;
                    planning.phase = BoardDeletePlanningPhase::SelectionEvent;
                }
            }
            BoardDeletePlanningPhase::DiscoverNode => {
                if !self.has_ports() {
                    if planning.node_relevant {
                        planning.relation_after = None;
                        planning.phase = BoardDeletePlanningPhase::Edges;
                    } else {
                        planning.node_id = None;
                        planning.phase = BoardDeletePlanningPhase::Nodes;
                    }
                } else {
                    let node_id = planning.node_id.as_deref().expect("node discovery owns a node");
                    let next = match planning.handle_after.as_ref() {
                        Some(after) => self.handles.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                        None => self.handles.first_key_value(),
                    }
                    .map(|(id, handle)| admitted_board_pointer_id(id).map(|owned| (handle.node_id == node_id && self.selection.contains(id), owned)));
                    if let Some(next) = next {
                        match next {
                            Ok((selected, id)) => {
                                planning.handle_after = Some(id);
                                if selected {
                                    planning.node_relevant = true;
                                    planning.handle_after = None;
                                    planning.phase = BoardDeletePlanningPhase::Handles;
                                }
                            }
                            Err(fault) => planning.fault = Some(fault),
                        }
                    } else if planning.node_relevant {
                        planning.handle_after = None;
                        planning.phase = BoardDeletePlanningPhase::Handles;
                    } else {
                        planning.handle_after = None;
                        planning.node_id = None;
                        planning.phase = BoardDeletePlanningPhase::Nodes;
                    }
                }
            }
            BoardDeletePlanningPhase::Handle => {
                let id = planning.handle_id.take().expect("handle deletion owns an id");
                planning.phase = BoardDeletePlanningPhase::Handles;
                self.begin_delete_property_audit(&mut planning, BoardDeleteKind::Handle, &id);
            }
            BoardDeletePlanningPhase::Node => {
                let id = planning.node_id.take().expect("node deletion owns an id");
                planning.phase = BoardDeletePlanningPhase::Nodes;
                self.begin_delete_property_audit(&mut planning, BoardDeleteKind::Node, &id);
            }
            BoardDeletePlanningPhase::SelectionEvent => {
                let next = match planning.scan_after.as_ref() {
                    Some(after) => self.selection.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                    None => self.selection.first(),
                }
                .map(|id| admitted_board_pointer_id(id));
                match next {
                    Some(Ok(id)) => {
                        planning.scan_after = Some(id.clone());
                        if !planning.plan.removes_selection_id(&id) {
                            let write = (|| {
                                if !planning.select_first {
                                    planning.select_builder.raw(",")?;
                                }
                                planning.select_builder.string(&id)?;
                                Ok::<(), BoardEventFault>(())
                            })();
                            if let Err(fault) = write {
                                planning.fault = Some(fault);
                            }
                            planning.select_first = false;
                        }
                    }
                    Some(Err(fault)) => planning.fault = Some(fault),
                    None => planning.phase = BoardDeletePlanningPhase::Finish,
                }
            }
            BoardDeletePlanningPhase::Finish => {
                if let Err(fault) = planning.select_builder.raw("],\"exitHighlightIds\":[]}") {
                    planning.fault = Some(fault);
                }
                if planning.fault.is_some() {
                    self.event_schema_fault = true;
                    return BoardAuthorityStep::Fault;
                }
                let select_event = std::mem::replace(&mut planning.select_builder, BoardPayloadBuilder::new()).finish_owned(BoardEventKind::Select);
                self.pending_delete_operation = Some(BoardDeleteOperation {
                    plan: planning.plan,
                    select_event: Some(select_event),
                    mutation_cursor: 0,
                    publication_cursor: 0,
                    remaining_claimed_items: 0,
                    remaining_claimed_bytes: 0,
                    claimed: false,
                    retiring_entity: None,
                    cancelling: false,
                });
                return BoardAuthorityStep::Pending;
            }
        }
        self.pending_delete_planning = Some(planning);
        BoardAuthorityStep::Pending
    }

    #[cfg(test)]
    pub(super) fn plan_delete_selection(&self) -> Result<BoardDeletePlan, BoardEventFault> {
        let mut plan = BoardDeletePlan::new(self.interaction_revision);
        for id in self.selection.iter().filter(|id| self.edges.contains_key(*id)) {
            let edge = self.edges.get(id).expect("selected edge exists");
            plan.push_entity(BoardDeleteKind::Edge, id, &edge.properties)?;
        }
        if self.has_ports() {
            for (node_id, node) in self.nodes.iter().filter(|(node_id, _)| self.selection.contains(*node_id) || self.handles.values().any(|handle| handle.node_id == **node_id && self.selection.contains(&handle.id))) {
                for (handle_id, handle) in self.handles.iter().filter(|(_, handle)| handle.node_id == *node_id) {
                    for (wire_id, wire) in self.wires.iter().filter(|(_, wire)| wire.source == *handle_id || wire.target.as_ref() == Some(handle_id)) {
                        plan.push_entity(BoardDeleteKind::Wire, wire_id, &wire.properties)?;
                    }
                    for (edge_id, edge) in self.edges.iter().filter(|(_, edge)| edge.source == *handle_id || edge.target == *handle_id) {
                        plan.push_entity(BoardDeleteKind::Edge, edge_id, &edge.properties)?;
                    }
                    plan.push_entity(BoardDeleteKind::Handle, handle_id, &handle.properties)?;
                }
                plan.push_entity(BoardDeleteKind::Node, node_id, &node.properties)?;
            }
        } else {
            for (node_id, node) in self.nodes.iter().filter(|(node_id, _)| self.selection.contains(*node_id)) {
                for (edge_id, edge) in self.edges.iter().filter(|(_, edge)| edge.source == *node_id || edge.target == *node_id) {
                    plan.push_entity(BoardDeleteKind::Edge, edge_id, &edge.properties)?;
                }
                plan.push_entity(BoardDeleteKind::Node, node_id, &node.properties)?;
            }
        }
        Ok(plan)
    }

    pub(super) fn step_pending_delete_operation(&mut self) {
        let Some(mut operation) = self.pending_delete_operation.take() else {
            return;
        };
        if let Some(retiring) = operation.retiring_entity.as_mut() {
            match retiring.step() {
                Ok(true) => {
                    debug_assert!(retiring.terminal_is_empty());
                    operation.retiring_entity = None;
                }
                Ok(false) => {}
                Err(()) => self.event_schema_fault = true,
            }
            self.pending_delete_operation = Some(operation);
            return;
        }
        if !operation.claimed {
            if self.interaction_revision != operation.plan.revision {
                self.event_schema_fault = true;
                return;
            }
            let event_count = operation.plan.emitted_event_count() + 1;
            let Some(event_bytes) = operation.plan.emitted_event_bytes().and_then(|bytes| bytes.checked_add(operation.select_event.as_ref()?.owned_bytes())) else {
                self.event_schema_fault = true;
                return;
            };
            if self.events.claim(event_count, event_bytes).is_err() {
                self.pending_delete_operation = Some(operation);
                return;
            }
            operation.claimed = true;
            operation.remaining_claimed_items = event_count as u16;
            operation.remaining_claimed_bytes = event_bytes;
            self.pending_delete_operation = Some(operation);
            return;
        }
        if operation.mutation_cursor < operation.plan.len {
            let entry = operation.plan.entries[usize::from(operation.mutation_cursor)].expect("delete plan entry");
            let id = operation.plan.id(entry.id);
            let retiring = match entry.kind {
                BoardDeleteKind::Edge => {
                    let removed = self.edges.remove(id).map(BoardRemovedEntity::Edge);
                    self.selection.remove(id);
                    removed
                }
                BoardDeleteKind::Node => {
                    let removed = self.nodes.remove(id).map(BoardRemovedEntity::Node);
                    self.selection.remove(id);
                    removed
                }
                BoardDeleteKind::Handle => {
                    let removed = self.handles.remove(id).map(BoardRemovedEntity::Handle);
                    self.selection.remove(id);
                    removed
                }
                BoardDeleteKind::Wire => {
                    let removed = self.wires.remove(id).map(BoardRemovedEntity::Wire);
                    self.selection.remove(id);
                    removed
                }
            };
            operation.mutation_cursor += 1;
            operation.retiring_entity = retiring.map(BoardEntityRetirement::new);
            self.pending_delete_operation = Some(operation);
            return;
        }
        if operation.publication_cursor < operation.plan.len {
            let entry = operation.plan.entries[usize::from(operation.publication_cursor)].expect("delete plan entry");
            operation.publication_cursor += 1;
            let id = operation.plan.id(entry.id);
            let event = match entry.kind {
                BoardDeleteKind::Edge => Some(BoardOwnedEvent::id(BoardEventKind::EdgeDelete, id).expect("delete edge payload was exactly preflighted")),
                BoardDeleteKind::Node => Some(BoardOwnedEvent::id(BoardEventKind::NodeDelete, id).expect("delete node payload was exactly preflighted")),
                BoardDeleteKind::Handle | BoardDeleteKind::Wire => None,
            };
            if let Some(event) = event {
                operation.remaining_claimed_items -= 1;
                operation.remaining_claimed_bytes -= event.owned_bytes();
                self.events.push_claimed(event).expect("delete event owns claimed queue credits");
            }
            self.pending_delete_operation = Some(operation);
            return;
        }
        if let Some(event) = operation.select_event.take() {
            operation.remaining_claimed_items -= 1;
            operation.remaining_claimed_bytes -= event.owned_bytes();
            self.events.push_claimed(event).expect("delete selection owns claimed queue credits");
            self.pending_delete_operation = Some(operation);
            return;
        }
        if let Some(id) = self.selection_exit_highlight.pop_first() {
            drop(id);
            self.pending_delete_operation = Some(operation);
            return;
        }
        if operation.remaining_claimed_items != 0 || operation.remaining_claimed_bytes != 0 {
            let _ = self.events.release_claim(usize::from(operation.remaining_claimed_items), operation.remaining_claimed_bytes);
            self.event_schema_fault = true;
        }
        self.bump_content_scene_generation();
        self.interaction_revision = operation.plan.revision.wrapping_add(1);
    }

    pub fn delete_selection(&mut self) {
        if self.pending_delete_planning.is_some() || self.pending_delete_operation.is_some() {
            return;
        }
        let mut select_builder = BoardPayloadBuilder::new();
        if select_builder.raw("{\"ids\":[").is_err() {
            self.event_schema_fault = true;
            return;
        }
        self.pending_delete_planning = Some(BoardDeletePlanningOperation {
            plan: BoardDeletePlan::new(self.interaction_revision),
            phase: BoardDeletePlanningPhase::SelectedEdges,
            scan_after: None,
            node_after: None,
            handle_after: None,
            relation_after: None,
            node_id: None,
            handle_id: None,
            node_relevant: false,
            property_audit: None,
            select_builder,
            select_first: true,
            fault: None,
            cancelling: false,
        });
    }

    fn link_snap_drag_tolerance_screen(&self, h: &HandleData) -> f64 {
        let z = self.camera.zoom.max(1e-9);
        HANDLE_HIT_TOLERANCE_PX + LINK_HANDLE_SNAP_EXTRA_PX + self.effective_handle_radius(h) * z
    }

    pub(super) fn link_snap_commit_proximity_ok(&self, target_handle_id: &str, world: Point) -> bool {
        let Some(h) = self.handles.get(target_handle_id) else {
            return false;
        };
        if !self.handle_selectable(target_handle_id) {
            return false;
        }
        let Some(pw) = self.handle_world_pos(h) else {
            return false;
        };
        let z = self.camera.zoom.max(1e-9);
        let d_screen = distance_between(self.world_to_screen(world), self.world_to_screen(pw));
        let tol_commit = HANDLE_HIT_TOLERANCE_PX + LINK_COMMIT_SNAP_TIGHT_PX + self.effective_handle_radius(h) * z;
        d_screen <= tol_commit
    }

    /// 🔗️ True when any edge uses this handle as `source` or `target` (handle already participates in a link).
    pub(super) fn handle_has_incident_edge(&self, handle_id: &str) -> bool {
        self.edges.values().any(|e| e.source == handle_id || e.target == handle_id)
    }

    fn node_has_any_incident_edge(&self, node_id: &str) -> bool {
        self.handles.values().filter(|h| h.node_id == node_id).any(|h| self.handle_has_incident_edge(h.id.as_str()))
    }

    fn lod_allows_node_proximity_connect(&self) -> bool {
        matches!(self.current_draw_lod(), BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro)
    }

    /// 🧲️ While dragging a node with no incident edges, overlapping bounds pick the nearest compatible free handle pair.
    pub(super) fn node_drag_proximity_handle_pair(&self, moving_node_id: &str) -> Option<(String, String)> {
        if !self.lod_allows_node_proximity_connect() {
            return None;
        }
        if !self.node_selectable(moving_node_id) {
            return None;
        }
        if self.node_has_any_incident_edge(moving_node_id) {
            return None;
        }
        let moving = self.nodes.get(moving_node_id)?;
        let moving_bounds = self.node_world_bounds(moving, 0.0);
        let mut best: Option<(f64, String, String)> = None;
        for (target_id, target) in &self.nodes {
            if target_id == moving_node_id || !self.node_selectable(target_id.as_str()) {
                continue;
            }
            let target_bounds = self.node_world_bounds(target, 0.0);
            if !world_boxes_overlap(moving_bounds, target_bounds) {
                continue;
            }
            let moving_handles: Vec<_> = self.handles.iter().filter(|(id, h)| h.node_id == moving_node_id && self.handle_selectable(id.as_str()) && !self.handle_has_incident_edge(id.as_str())).collect();
            let target_handles: Vec<_> = self.handles.iter().filter(|(id, h)| h.node_id == target_id.as_str() && self.handle_selectable(id.as_str()) && !self.handle_has_incident_edge(id.as_str())).collect();
            for (src_id, src_h) in &moving_handles {
                let Some(src_pos) = self.handle_world_pos(src_h) else {
                    continue;
                };
                for (tgt_id, tgt_h) in &target_handles {
                    let Some(tgt_pos) = self.handle_world_pos(tgt_h) else {
                        continue;
                    };
                    let d = distance_between(src_pos, tgt_pos);
                    let pair = if self.handles_link_compatible_for_drag(src_h, tgt_h) {
                        Some(((*src_id).clone(), (*tgt_id).clone()))
                    } else if self.handles_link_compatible_for_drag(tgt_h, src_h) {
                        Some(((*tgt_id).clone(), (*src_id).clone()))
                    } else {
                        None
                    };
                    if let Some((s, t)) = pair {
                        if best.as_ref().is_none_or(|(bd, _, _)| d < *bd) {
                            best = Some((d, s.to_string(), t.to_string()));
                        }
                    }
                }
            }
        }
        best.map(|(_, s, t)| (s, t))
    }

    fn node_effectively_visible(&self, node_id: &str) -> bool {
        self.nodes.get(node_id).is_some_and(|n| n.visible)
    }

    pub(super) fn node_selectable(&self, node_id: &str) -> bool {
        self.nodes.get(node_id).is_some_and(|n| n.visible && !n.locked)
    }

    pub(super) fn handle_selectable(&self, handle_id: &str) -> bool {
        self.handles.get(handle_id).is_some_and(|h| h.visible && !h.locked && self.node_selectable(h.node_id.as_str()))
    }

    pub(super) fn edge_selectable(&self, edge: &EdgeData) -> bool {
        if !edge.visible || edge.locked {
            return false;
        }
        if !self.has_ports() {
            return self.node_selectable(edge.source.as_str()) && self.node_selectable(edge.target.as_str());
        }
        self.handle_selectable(edge.source.as_str()) && self.handle_selectable(edge.target.as_str())
    }

    pub(super) fn wire_selectable(&self, wire: &WireData) -> bool {
        if !wire.visible || wire.locked {
            return false;
        }
        if !self.handle_selectable(wire.source.as_str()) {
            return false;
        }
        wire.target.as_ref().is_none_or(|id| self.handle_selectable(id.as_str()))
    }

    pub(super) fn handle_effectively_visible(&self, handle_id: &str) -> bool {
        self.handles.get(handle_id).is_some_and(|h| h.visible && self.node_effectively_visible(h.node_id.as_str()))
    }

    fn edge_effectively_visible(&self, edge: &EdgeData) -> bool {
        if !self.has_ports() {
            return edge.visible && self.node_effectively_visible(edge.source.as_str()) && self.node_effectively_visible(edge.target.as_str());
        }
        edge.visible && self.handle_effectively_visible(edge.source.as_str()) && self.handle_effectively_visible(edge.target.as_str())
    }

    fn wire_effectively_visible(&self, wire: &WireData) -> bool {
        wire.visible && self.handle_effectively_visible(wire.source.as_str()) && wire.target.as_ref().is_none_or(|id| self.handle_effectively_visible(id.as_str()))
    }

    /// 💫️ True when the handle may be drawn or hit-tested on the indirect-connect ghost ring (`overview`/`normal` LOD).
    pub(super) fn handle_eligible_indirect_connect_ring(&self, handle_id: &str) -> bool {
        self.handle_selectable(handle_id) && !self.handle_has_incident_edge(handle_id)
    }

    /// 📍️ Drag-phase link snap tests **screen px** to the handle anchor so detail/micro zoom keeps a stable hit halo; pointer-up re-checks with `link_snap_commit_proximity_ok` before `proximityConnect`.
    pub(super) fn nearest_link_snap_handle_world(&self, source_handle_id: &str, world: Point) -> Option<String> {
        if matches!(self.current_draw_lod(), BoardDrawLod::Minimap) {
            return None;
        }
        let source_handle = self.handles.get(source_handle_id)?;
        if !self.handle_selectable(source_handle_id) {
            return None;
        }
        let source_node_id = source_handle.node_id.as_str();
        let p_scr = self.world_to_screen(world);
        let mut best: Option<(f64, String)> = None;
        for (id, h) in &self.handles {
            if id == source_handle_id || !self.handle_selectable(id.as_str()) {
                continue;
            }
            if self.handle_has_incident_edge(id.as_str()) {
                continue;
            }
            if h.node_id == source_node_id {
                continue;
            }
            if !self.handles_link_compatible_for_drag(source_handle, h) {
                continue;
            }
            let pw = self.handle_world_pos(h)?;
            let h_scr = self.world_to_screen(pw);
            let d_screen = distance_between(p_scr, h_scr);
            let tol_screen = self.link_snap_drag_tolerance_screen(h);
            if d_screen <= tol_screen && best.as_ref().is_none_or(|(bd, _)| d_screen < *bd) {
                best = Some((d_screen, id.clone()));
            }
        }
        best.map(|(_, id)| id)
    }

    pub(super) fn try_commit_link_edge(&mut self, source_handle_id: &str, target_handle_id: &str, also_emit: Option<BoardEventKind>) -> bool {
        if source_handle_id == target_handle_id {
            return false;
        }
        if !self.handle_selectable(source_handle_id) || !self.handle_selectable(target_handle_id) {
            return false;
        }
        let Some(source_row) = self.handles.get(source_handle_id) else {
            return false;
        };
        let Some(target_row) = self.handles.get(target_handle_id) else {
            return false;
        };
        if source_row.node_id == target_row.node_id {
            return false;
        }
        if !self.handles_link_compatible_for_drag(source_row, target_row) {
            return false;
        }
        if self.handle_has_incident_edge(source_handle_id) || self.handle_has_incident_edge(target_handle_id) {
            return false;
        }
        for e in self.edges.values() {
            if e.source == source_handle_id && e.target == target_handle_id {
                return false;
            }
        }
        let mut n = self.edges.len().saturating_add(1);
        let id = loop {
            let candidate = format!("edge-link-{n}");
            if !self.edges.contains_key(&candidate) {
                break candidate;
            }
            n = n.saturating_add(1);
        };
        let edge_kind = self.default_edge_kind_for_created_link(source_row, target_row);
        let Some(reservation) = self.reserve_owned_batch(BoardOwnedEvent::edge(BoardEventKind::EdgeCreate, &id, source_handle_id, target_handle_id), also_emit.map(|kind| BoardOwnedEvent::edge(kind, &id, source_handle_id, target_handle_id)))
        else {
            return false;
        };
        self.edges.insert(
            id.clone(),
            EdgeData {
                id,
                source: source_handle_id.to_string(),
                target: target_handle_id.to_string(),
                selected: false,
                visible: true,
                locked: false,
                style: None,
                edge_kind,
                source_tip: None,
                target_tip: None,
                properties: semio_framework_graph_manifest::PropertyBag::new(),
            },
        );
        self.publish_event_batch(reservation);
        true
    }

    fn build_selection_pointer_plan(&self, kind: BoardPointerPlanKind, next: &BTreeSet<String>, anchor: &BTreeSet<String>, gesture: Option<&str>, points: &[Point], screen_points: &[Point]) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        if next.len() > BOARD_POINTER_ITEM_CAPACITY || anchor.len() > BOARD_POINTER_ITEM_CAPACITY || points.len() != screen_points.len() || points.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardPointerPlanFault::ItemCredits);
        }
        if anchor.iter().map(String::len).try_fold(0usize, usize::checked_add).is_none_or(|bytes| bytes > BOARD_POINTER_BYTE_CAPACITY) {
            return Err(BoardPointerPlanFault::ByteCredits);
        }
        let mut plan = BoardPointerPlan::empty(self.interaction_revision, kind);
        for id in next {
            plan.push_delta(id, 0.0, 0.0)?;
        }
        for (world, screen) in points.iter().copied().zip(screen_points.iter().copied()) {
            plan.push_point(world, screen)?;
        }
        let event = match kind {
            BoardPointerPlanKind::SelectionPreview { .. } => {
                let removed: BTreeSet<String> = anchor.difference(next).cloned().collect();
                BoardOwnedEvent::preselect_sets(next, &removed, gesture)
            }
            BoardPointerPlanKind::SelectionCommit => BoardOwnedEvent::select_set(next, Some(anchor), gesture),
            _ => return Err(BoardPointerPlanFault::Unsupported),
        }
        .map_err(|fault| match fault {
            BoardEventFault::ItemCredits => BoardPointerPlanFault::ItemCredits,
            BoardEventFault::ByteCredits | BoardEventFault::KeyCredits | BoardEventFault::Schema => BoardPointerPlanFault::ByteCredits,
        })?;
        plan.seal_owned_event(&event)?;
        Ok(plan)
    }

    fn plan_selection_pending_pointer(&self, intent: BoardPointerIntent, initial_ids: &BTreeSet<String>, start: Point, start_screen: Point, screen: Point, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        match intent.phase {
            BoardPointerPhase::Move if distance_between(start_screen, screen) < SELECTION_CLICK_MAX_DISTANCE_PX => Ok(BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle)),
            BoardPointerPhase::Move => {
                let points = [start, world];
                let screen_points = [start_screen, screen];
                let merge_mode = pick_merge_mode_for_modifiers(intent.ctrl_or_meta, intent.shift, self.selection_options.mode.as_str());
                let next = self.resolve_area_selection_with_initial(initial_ids, start, &points, merge_mode.as_str());
                let gesture = (intent.ctrl_or_meta || intent.shift).then_some(merge_mode.as_str());
                self.build_selection_pointer_plan(BoardPointerPlanKind::SelectionPreview { start, start_screen }, &next, initial_ids, gesture, &points, &screen_points)
            }
            BoardPointerPhase::Up => {
                let merge_mode = pick_merge_mode_for_modifiers(intent.ctrl_or_meta, intent.shift, self.selection_options.mode.as_str());
                let next = if intent.ctrl_or_meta || intent.shift { self.resolve_area_selection_with_initial(initial_ids, start, &[start], merge_mode.as_str()) } else { BTreeSet::new() };
                let gesture = (intent.ctrl_or_meta || intent.shift).then_some(merge_mode.as_str());
                self.build_selection_pointer_plan(BoardPointerPlanKind::SelectionCommit, &next, initial_ids, gesture, &[], &[])
            }
            BoardPointerPhase::Leave => Ok(BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle)),
        }
    }

    fn plan_selection_pointer(
        &self,
        intent: BoardPointerIntent,
        initial_ids: &BTreeSet<String>,
        points: &[Point],
        screen_points: &[Point],
        start: Point,
        start_screen: Point,
        screen: Point,
        world: Point,
    ) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        if intent.phase == BoardPointerPhase::Leave {
            return Ok(BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle));
        }
        if points.len() != screen_points.len() || points.len() >= BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardPointerPlanFault::ItemCredits);
        }
        let mut next_points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]> = Box::new([Point::ZERO; BOARD_POINTER_ITEM_CAPACITY]);
        let mut next_screen_points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]> = Box::new([Point::ZERO; BOARD_POINTER_ITEM_CAPACITY]);
        next_points[..points.len()].copy_from_slice(points);
        next_screen_points[..screen_points.len()].copy_from_slice(screen_points);
        let mut len = points.len();
        let last_screen = screen_points.last().copied().unwrap_or(start_screen);
        let add_point = intent.phase == BoardPointerPhase::Up || self.selection_options.method == "lasso" || distance_between(screen, last_screen) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
        if add_point {
            next_points[len] = world;
            next_screen_points[len] = screen;
            len += 1;
        } else if len > 0 {
            next_points[len - 1] = world;
            next_screen_points[len - 1] = screen;
        }
        let merge_mode = pick_merge_mode_for_modifiers(intent.ctrl_or_meta, intent.shift, self.selection_options.mode.as_str());
        let end_screen = next_screen_points[len.saturating_sub(1)];
        let click_only = intent.phase == BoardPointerPhase::Up && distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
        let next = if click_only { BTreeSet::new() } else { self.resolve_area_selection_with_initial(initial_ids, start, &next_points[..len], merge_mode.as_str()) };
        let gesture = (intent.ctrl_or_meta || intent.shift).then_some(merge_mode.as_str());
        let kind = if intent.phase == BoardPointerPhase::Up { BoardPointerPlanKind::SelectionCommit } else { BoardPointerPlanKind::SelectionPreview { start, start_screen } };
        self.build_selection_pointer_plan(kind, &next, initial_ids, gesture, &next_points[..len], &next_screen_points[..len])
    }

    fn plan_link_move_pointer(&self, source_id: &str, start_screen: Option<Point>, screen: Point, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        let activated = start_screen.is_none_or(|start_screen| distance_between(screen, start_screen) >= LINK_DRAG_MIN_DISTANCE_PX);
        let target_id = activated.then(|| self.nearest_link_snap_handle_world(source_id, world)).flatten();
        let hover_id = target_id.clone().or_else(|| self.resolve_hover_world(world));
        let node_ids = self.link_drag_compatible_target_node_ids(source_id);
        let ring_node_id = activated.then(|| self.link_drag_ring_target_node_id(source_id, world)).flatten().filter(|node_id| self.link_compatible_handle_count_on_node(source_id, node_id) > 1);
        let ring_handle_ids = ring_node_id.as_deref().map(|node_id| self.link_compatible_handle_ids_on_node(source_id, node_id)).unwrap_or_default();
        if node_ids.len().saturating_add(ring_handle_ids.len()) > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardPointerPlanFault::ItemCredits);
        }
        let compat_key = format!("{}|{}", source_id, node_ids.join(","));
        let ring_key = format!("{}|{}|{}", source_id, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
        let hover_kind = hover_id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
        let events = [
            (self.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str())).then(|| BoardOwnedEvent::link_compatible(source_id, &node_ids)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
            (self.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str())).then(|| BoardOwnedEvent::link_ring(source_id, ring_node_id.as_deref(), &ring_handle_ids)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
            (self.hovered_id != hover_id || self.hovered_kind.is_some())
                .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
                .transpose()
                .map_err(|_| BoardPointerPlanFault::ByteCredits)?,
        ];
        let mut plan = BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle);
        let source = plan.push_id(source_id)?;
        let target = target_id.as_deref().map(|target_id| plan.push_id(target_id)).transpose()?;
        let hover = hover_id.as_deref().map(|hover_id| plan.push_id(hover_id)).transpose()?;
        let compat_key = plan.push_id(&compat_key)?;
        let ring_key = plan.push_id(&ring_key)?;
        plan.kind = BoardPointerPlanKind::LinkMove { source, target, hover, compat_key, ring_key, end_world: world, activated, start_screen: start_screen.unwrap_or(screen) };
        plan.seal_optional_events(&events)?;
        Ok(plan)
    }

    fn planned_link_edge_id(&self, source_handle_id: &str, target_handle_id: &str) -> Option<String> {
        if source_handle_id == target_handle_id || !self.handle_selectable(source_handle_id) || !self.handle_selectable(target_handle_id) {
            return None;
        }
        let source = self.handles.get(source_handle_id)?;
        let target = self.handles.get(target_handle_id)?;
        if source.node_id == target.node_id || !self.handles_link_compatible_for_drag(source, target) || self.handle_has_incident_edge(source_handle_id) || self.handle_has_incident_edge(target_handle_id) {
            return None;
        }
        if self.edges.values().any(|edge| edge.source == source_handle_id && edge.target == target_handle_id) {
            return None;
        }
        let mut serial = self.edges.len().saturating_add(1);
        loop {
            let candidate = format!("edge-link-{serial}");
            if !self.edges.contains_key(&candidate) {
                return Some(candidate);
            }
            serial = serial.saturating_add(1);
        }
    }

    fn plan_link_finish_pointer(&self, source_id: &str, target_id: Option<&str>, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        let direct_target = target_id.filter(|target_id| self.link_snap_commit_proximity_ok(target_id, world));
        let mut target_node_id = None;
        let mut edge_target = direct_target.map(str::to_string);
        let mut also_emit = direct_target.map(|_| BoardEventKind::ProximityConnect);
        if edge_target.is_none() {
            if let Some(node_id) = self.resolve_node_hit_world(world).filter(|node_id| self.handles.get(source_id).is_some_and(|source| source.node_id.as_str() != node_id.as_str())) {
                if let Some(sole_target) = self.node_sole_free_link_compatible_handle(source_id, &node_id) {
                    edge_target = Some(sole_target);
                    also_emit = Some(BoardEventKind::IndirectConnect);
                } else if self.node_has_any_free_link_compatible_handle(source_id, &node_id) {
                    target_node_id = Some(node_id);
                }
            }
        }
        let edge_id = edge_target.as_deref().and_then(|target| self.planned_link_edge_id(source_id, target));
        if edge_id.is_none() {
            edge_target = None;
            also_emit = None;
        }
        let hover_id = target_node_id.clone().or_else(|| self.resolve_hover_world(world));
        let hover_kind = hover_id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
        let mut events: [Option<BoardOwnedEvent>; 5] = std::array::from_fn(|_| None);
        let mut compat_key_owned = None;
        let mut ring_key_owned = None;
        if let (Some(edge_id), Some(edge_target)) = (edge_id.as_deref(), edge_target.as_deref()) {
            events[0] = Some(BoardOwnedEvent::edge(BoardEventKind::EdgeCreate, edge_id, source_id, edge_target).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
            if let Some(also_emit) = also_emit {
                events[1] = Some(BoardOwnedEvent::edge(also_emit, edge_id, source_id, edge_target).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
            }
        }
        if let Some(target_node_id) = target_node_id.as_deref() {
            let node_ids = self.link_drag_compatible_target_node_ids(source_id);
            let ring_handle_ids = self.link_compatible_handle_ids_on_node(source_id, target_node_id);
            compat_key_owned = Some(format!("{}|{}", source_id, node_ids.join(",")));
            ring_key_owned = Some(format!("{}|{}|{}", source_id, target_node_id, ring_handle_ids.join(",")));
            events[2] = Some(BoardOwnedEvent::link_compatible(source_id, &node_ids).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
            events[3] = Some(BoardOwnedEvent::link_ring(source_id, Some(target_node_id), &ring_handle_ids).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
        } else {
            let empty: &[String] = &[];
            events[2] = self.link_compat_nodes_emit_key.is_some().then(|| BoardOwnedEvent::link_compatible("", empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?;
            events[3] = self.link_target_ring_emit_key.is_some().then(|| BoardOwnedEvent::link_ring("", None, empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?;
        }
        if self.hovered_id != hover_id || self.hovered_kind.is_some() {
            events[4] = Some(BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
        }
        let mut plan = BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle);
        let source = plan.push_id(source_id)?;
        let target = edge_target.as_deref().map(|target| plan.push_id(target)).transpose()?;
        let edge = edge_id.as_deref().map(|edge| plan.push_id(edge)).transpose()?;
        let target_node = target_node_id.as_deref().map(|target_node| plan.push_id(target_node)).transpose()?;
        let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
        let compat_key = compat_key_owned.as_deref().map(|key| plan.push_id(key)).transpose()?;
        let ring_key = ring_key_owned.as_deref().map(|key| plan.push_id(key)).transpose()?;
        plan.kind = BoardPointerPlanKind::LinkFinish { source, target, edge, target_node, hover, compat_key, ring_key };
        plan.seal_optional_events(&events)?;
        Ok(plan)
    }

    fn plan_link_retain_pointer(&self, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        let hover_id = self.resolve_hover_world(world);
        let hover_kind = hover_id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
        let event = (self.hovered_id != hover_id || self.hovered_kind.is_some())
            .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
            .transpose()
            .map_err(|_| BoardPointerPlanFault::ByteCredits)?;
        let mut plan = BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle);
        let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
        plan.kind = BoardPointerPlanKind::LinkRetain { hover };
        plan.seal_optional_events(&[event])?;
        Ok(plan)
    }

    fn plan_link_clear_pointer(&self, source_id: &str, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        let hover_id = self.resolve_hover_world(world);
        let hover_kind = hover_id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
        let empty: &[String] = &[];
        let events = [
            self.link_compat_nodes_emit_key.is_some().then(|| BoardOwnedEvent::link_compatible("", empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
            self.link_target_ring_emit_key.is_some().then(|| BoardOwnedEvent::link_ring("", None, empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
            (self.hovered_id != hover_id || self.hovered_kind.is_some())
                .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
                .transpose()
                .map_err(|_| BoardPointerPlanFault::ByteCredits)?,
        ];
        let mut plan = BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle);
        let source = plan.push_id(source_id)?;
        let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
        plan.kind = BoardPointerPlanKind::LinkFinish { source, target: None, edge: None, target_node: None, hover, compat_key: None, ring_key: None };
        plan.seal_optional_events(&events)?;
        Ok(plan)
    }

    fn plan_hover_pointer(&self, hover_id: Option<&str>) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        let hover_kind = hover_id.and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
        let event = (self.hovered_id.as_deref() != hover_id || self.hovered_kind.is_some())
            .then(|| BoardOwnedEvent::hover(hover_id, hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
            .transpose()
            .map_err(|_| BoardPointerPlanFault::ByteCredits)?;
        let mut plan = BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle);
        let hover = hover_id.map(|hover| plan.push_id(hover)).transpose()?;
        plan.kind = BoardPointerPlanKind::Hover { hover };
        plan.seal_optional_events(&[event])?;
        Ok(plan)
    }

    fn plan_brush_pointer(&self, intent: BoardPointerIntent, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        let next_source = if intent.phase == BoardPointerPhase::Move { self.brush_nearest_slot_source(world) } else { None };
        let offset = if intent.alt || self.brush_slot_suggestions_active { self.suggestion_offset } else { 0.0 };
        let next_candidates = if let Some(source_id) = next_source.as_deref() {
            if self.brush_slot_source_id.as_deref() == Some(source_id) {
                self.brush_candidates.clone()
            } else {
                let source = self.handles.get(source_id).ok_or(BoardPointerPlanFault::Unsupported)?;
                self.brush_compatible_candidates(source).ok_or(BoardPointerPlanFault::ItemCredits)?
            }
        } else {
            BrushCandidatePage::default()
        };
        if next_candidates.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardPointerPlanFault::ItemCredits);
        }
        let next_index = if self.brush_slot_source_id == next_source { self.brush_candidate_index.min(next_candidates.len().saturating_sub(1)) } else { 0 };
        let next_preview = next_source.as_deref().and_then(|source_id| next_candidates.get(next_index).and_then(|candidate| self.brush_build_preview_with_offset(source_id, candidate, offset)));
        let leaving_old = self.brush_slot_source_id.is_some() && self.brush_slot_source_id != next_source;
        let commit_old = intent.alt && leaving_old && self.brush_preview.is_some();
        let old_preview = self.brush_slot_source_id.as_deref().and_then(|source_id| self.brush_candidates.get(self.brush_candidate_index).and_then(|candidate| self.brush_build_preview_with_offset(source_id, candidate, offset)));
        let hover_id = next_source.clone();
        let hover_kind = hover_id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
        let serial = self.brush_placement_serial.wrapping_add(1);
        let node_id = format!("puzzle2d.brush.{serial}");
        let edge_id = format!("puzzle2d.brush.edge.{serial}");
        let events = [
            commit_old.then(|| old_preview.as_ref().ok_or(BoardEventFault::Schema).and_then(|preview| BoardOwnedEvent::brush_place(preview, &node_id, &edge_id))).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
            Some(BoardOwnedEvent::brush_preview(next_preview.as_ref()).map_err(|_| BoardPointerPlanFault::ByteCredits)?),
            Some(BoardOwnedEvent::brush_candidates(next_source.as_deref().unwrap_or(""), &next_candidates, next_index, false).map_err(|_| BoardPointerPlanFault::ByteCredits)?),
            (self.hovered_id != hover_id || self.hovered_kind.is_some())
                .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
                .transpose()
                .map_err(|_| BoardPointerPlanFault::ByteCredits)?,
        ];
        let mut plan = BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle);
        let source = next_source.as_deref().map(|source| plan.push_id(source)).transpose()?;
        let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
        for candidate in next_candidates.iter() {
            plan.push_delta(candidate.node_kind_id, candidate.target_handle_index as f64, 0.0)?;
        }
        plan.kind = BoardPointerPlanKind::Brush { source, hover, alt: intent.alt, commit_old };
        plan.seal_optional_events(&events)?;
        Ok(plan)
    }

    pub fn plan_pointer(&self, intent: BoardPointerIntent) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        if self.interaction_revision == u64::MAX || (matches!(intent.phase, BoardPointerPhase::Move | BoardPointerPhase::Up) && self.pointer_lane_is_direct()) {
            return Err(BoardPointerPlanFault::Unsupported);
        }
        let screen = Point::new(intent.x, intent.y);
        let world = self.screen_to_world(screen);
        if self.active_utility == ActiveUtility::Brush && !matches!(self.interaction, Interaction::Pan { .. }) {
            return self.plan_brush_pointer(intent, world);
        }
        match (intent.phase, &self.interaction) {
            (BoardPointerPhase::Move, Interaction::Pan { origin, start_screen }) => {
                let delta = screen - *start_screen;
                let camera = [origin.x - delta.x / origin.zoom, origin.y - delta.y / origin.zoom, origin.zoom];
                Ok(BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Pan { camera }))
            }
            (BoardPointerPhase::Move, Interaction::DragNodes { primary_id, offset, start_positions, .. }) => self.plan_drag_pointer(world, (primary_id, *offset, start_positions), None),
            (_, Interaction::SelectionPending { initial_ids, start, start_screen }) => self.plan_selection_pending_pointer(intent, initial_ids, *start, *start_screen, screen, world),
            (_, Interaction::Selection { initial_ids, points, screen_points, start, start_screen }) => self.plan_selection_pointer(intent, initial_ids, points, screen_points, *start, *start_screen, screen, world),
            (BoardPointerPhase::Move, Interaction::LinkAtSourceHandle { source_id, start_screen }) => self.plan_link_move_pointer(source_id, Some(*start_screen), screen, world),
            (BoardPointerPhase::Move, Interaction::LinkDragSnap { source_id, .. }) => self.plan_link_move_pointer(source_id, None, screen, world),
            (BoardPointerPhase::Up, Interaction::LinkDragSnap { source_id, target_id, .. }) => self.plan_link_finish_pointer(source_id, target_id.as_deref(), world),
            (BoardPointerPhase::Up, Interaction::LinkAtSourceHandle { source_id, .. }) => self.plan_link_finish_pointer(source_id, None, world),
            (BoardPointerPhase::Move, Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. }) => self.plan_link_retain_pointer(world),
            (BoardPointerPhase::Up, Interaction::LinkTargetNode { source_id, .. }) => self.plan_link_clear_pointer(source_id, world),
            (BoardPointerPhase::Up, Interaction::ExternalLinkPreview { .. }) => self.plan_link_retain_pointer(world),
            (BoardPointerPhase::Move, Interaction::None) => self.plan_hover_pointer(self.resolve_hover_world(world).as_deref()),
            (BoardPointerPhase::Up, Interaction::Pan { origin, start_screen }) => {
                let delta = screen - *start_screen;
                let camera = [origin.x - delta.x / origin.zoom, origin.y - delta.y / origin.zoom, origin.zoom];
                Ok(BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::FinishPan { camera }))
            }
            (BoardPointerPhase::Up, Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair, gesture, .. }) => self.plan_drag_pointer(world, (primary_id, *offset, start_positions), Some((gesture, proximity_pair.as_ref()))),
            (BoardPointerPhase::Up, Interaction::None) => Ok(BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle)),
            (BoardPointerPhase::Leave, Interaction::None) => self.plan_hover_pointer(None),
            (BoardPointerPhase::Leave, _) => Ok(BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle)),
        }
    }

    /// 🧲️ A drag frame (`DragMove`) or its release (`FinishDrag`) as a retained plan: every member's resolved
    /// position, and for the release the staged `select` plus the ONE gesture record, sealed as one page.
    fn plan_drag_pointer(&self, world: Point, drag: (&str, Vec2, &BTreeMap<String, (f64, f64)>), release: Option<(&GestureStage, Option<&(String, String)>)>) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        let (primary_id, offset, start_positions) = drag;
        if start_positions.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardPointerPlanFault::ItemCredits);
        }
        let delta = self.drag_delta(world, primary_id, offset, start_positions).ok_or(BoardPointerPlanFault::Unsupported)?;
        let mut plan = BoardPointerPlan::empty(self.interaction_revision, if release.is_some() { BoardPointerPlanKind::FinishDrag } else { BoardPointerPlanKind::DragMove });
        for (id, (x, y)) in start_positions {
            plan.push_delta(id, x + delta.x, y + delta.y)?;
        }
        if let Some((gesture, proximity_pair)) = release {
            let [select, record] = Self::gesture_release_events(gesture, Self::drag_gesture_record(gesture, start_positions, delta, proximity_pair));
            plan.seal_optional_events(&[select.transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?, record.transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?])?;
        }
        Ok(plan)
    }

    pub fn begin_pointer_commit(&mut self, plan: BoardPointerPlan) -> Result<(), BoardPointerPlan> {
        if self.pointer_publication.is_some() || self.interaction_revision != plan.revision || !plan.requires_retained_commit() {
            return Err(plan);
        }
        if let Some(active) = self.pending_pointer_commit.as_ref() {
            if active.plan.revision != plan.revision || !matches!(active.plan.kind, BoardPointerPlanKind::DragMove) || !matches!(plan.kind, BoardPointerPlanKind::DragMove | BoardPointerPlanKind::FinishDrag) {
                return Err(plan);
            }
            match self.queued_pointer_commit.as_ref().map(|queued| queued.kind) {
                None => {
                    self.queued_pointer_commit = Some(plan);
                    return Ok(());
                }
                Some(BoardPointerPlanKind::DragMove) => {
                    self.queued_pointer_commit = Some(plan);
                    return Ok(());
                }
                Some(_) => return Err(plan),
            }
        }
        self.pending_pointer_commit = Some(BoardPointerCommitOperation::new(plan));
        Ok(())
    }

    pub fn step_pointer_commit(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> BoardAuthorityStep {
        let Some(mut operation) = self.pending_pointer_commit.take() else {
            return BoardAuthorityStep::Complete;
        };
        if context.should_yield() {
            self.pending_pointer_commit = Some(operation);
            return BoardAuthorityStep::Pending;
        }
        if context.is_cancelled() && operation.phase == 0 {
            operation.cancelling = true;
        }
        if self.interaction_revision != operation.plan.revision && operation.phase == 0 {
            operation.cancelling = true;
            operation.faulted = true;
        }
        if operation.cancelling {
            if operation.cursor < operation.plan.delta_len {
                operation.plan.deltas[usize::from(operation.cursor)] = None;
                operation.cursor += 1;
                context.consume_fuel(1);
                self.pending_pointer_commit = Some(operation);
                return BoardAuthorityStep::Pending;
            }
            if operation.retire_one() || operation.points.pop().is_some() || operation.screen_points.pop().is_some() || operation.overlay_points.pop().is_some() {
                context.consume_fuel(1);
                self.pending_pointer_commit = Some(operation);
                return BoardAuthorityStep::Pending;
            }
            if operation.scan_after.take().is_some() {
                context.consume_fuel(1);
                self.pending_pointer_commit = Some(operation);
                return BoardAuthorityStep::Pending;
            }
            return if operation.faulted { BoardAuthorityStep::Fault } else { BoardAuthorityStep::Cancelled };
        }
        let complete = self.step_pointer_commit_operation(&mut operation);
        context.consume_fuel(1);
        if !complete {
            self.pending_pointer_commit = Some(operation);
            return BoardAuthorityStep::Pending;
        }
        if operation.faulted {
            return BoardAuthorityStep::Fault;
        }
        let completed_revision = operation.plan.revision;
        if self.interaction_revision != completed_revision {
            return BoardAuthorityStep::Fault;
        }
        self.interaction_revision = completed_revision.wrapping_add(1);
        if operation.plan.output_len > 2 {
            self.pointer_publication = Some(BoardPointerPublication { bytes: operation.plan.output, len: operation.plan.output_len });
        }
        if let Some(mut queued) = self.queued_pointer_commit.take() {
            if queued.revision != completed_revision {
                return BoardAuthorityStep::Fault;
            }
            queued.revision = self.interaction_revision;
            self.pending_pointer_commit = Some(BoardPointerCommitOperation::new(queued));
            return BoardAuthorityStep::Pending;
        }
        BoardAuthorityStep::Complete
    }

    pub fn take_pointer_publication(&mut self) -> Option<BoardPointerPublication> {
        self.pointer_publication.take()
    }

    pub fn pointer_publication(&self) -> Option<&BoardPointerPublication> {
        self.pointer_publication.as_ref()
    }

    pub fn begin_pointer_cancel(&mut self) {
        let revision = self.interaction_revision.wrapping_add(1);
        self.interaction_revision = revision;
        if let Some(operation) = self.pending_pointer_commit.as_mut() {
            operation.cancelling = true;
            operation.plan.revision = revision;
        }
        if self.cancelled_pointer_plan_retirement.is_none() {
            self.cancelled_pointer_plan_retirement = self.queued_pointer_commit.take().map(BoardPointerPlanRetirement::new);
        }
    }

    pub fn step_pointer_cancel(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if self.pending_pointer_commit.is_some() {
            let _ = self.step_pointer_commit(context);
            return false;
        }
        let Some(retirement) = self.cancelled_pointer_plan_retirement.as_mut() else {
            return true;
        };
        if retirement.close_step() {
            self.cancelled_pointer_plan_retirement = None;
        }
        context.consume_fuel(1);
        false
    }

    pub fn pointer_cancel_pending(&self) -> bool {
        self.cancelled_pointer_plan_retirement.is_some() || self.pending_pointer_commit.as_ref().is_some_and(|operation| operation.cancelling)
    }

    pub fn close_pointer_authority_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if let Some(operation) = self.pending_pointer_commit.as_mut() {
            if operation.cursor == 0 {
                operation.cancelling = true;
            }
            let _ = self.step_pointer_commit(context);
            return false;
        }
        if self.queued_pointer_commit.take().is_some() {
            context.consume_fuel(1);
            return false;
        }
        if let Some(retirement) = self.cancelled_pointer_plan_retirement.as_mut() {
            if retirement.close_step() {
                self.cancelled_pointer_plan_retirement = None;
            }
            context.consume_fuel(1);
            return false;
        }
        if let Some(publication) = self.pointer_publication.as_mut() {
            if publication.close_step() {
                self.pointer_publication = None;
            }
            return false;
        }
        true
    }

    pub fn pointer_authority_terminal_is_empty(&self) -> bool {
        self.pending_pointer_commit.is_none() && self.queued_pointer_commit.is_none() && self.cancelled_pointer_plan_retirement.is_none() && self.pointer_publication.is_none()
    }
    // #endregion board_host
}
