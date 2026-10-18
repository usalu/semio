//! 🎥️ Camera and pointer planning of the flow host: the viewport, the camera, the interaction projection, wheel and pointer planning and commit and the screen wheel gestures. See [`super`].

use super::*;

impl FlowHost {
    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.viewport_w = width.max(1);
        self.viewport_h = height.max(1);
        self.viewport_dpr = dpr.max(1.0);
        self.dag.viewport.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
    }

    pub fn world_from_screen(&self, sx: f64, sy: f64) -> (f64, f64) {
        let p = self.screen_to_world_point(sx, sy);
        (p.x, p.y)
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.host_snapshot.camera = CameraJson { x, y, zoom: zoom.clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX) };
        self.dag.graph.set_camera(x, y, self.host_snapshot.camera.zoom);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
    }

    /// 📷️ The ONE camera of a flow surface. `self.host_snapshot.camera` is the authority — it is what
    /// `screen_to_world_point` projects with, what `build_dag_host_snapshot_v1` re-seeds the dag copy from
    /// on every rebuild, and what the renderer publishes as `nodeGraphViewport`. The dag's own
    /// `host_snapshot.camera` is a derived paint copy; reading it instead is how a fit got published stale.
    pub fn camera(&self) -> [f64; 3] {
        [self.host_snapshot.camera.x, self.host_snapshot.camera.y, self.host_snapshot.camera.zoom]
    }

    /// 📷️ Adopts a camera the DAG computed for itself (a fit, a refit, an opening decision) into the
    /// authority, so the published, projected and painted cameras cannot drift apart.
    fn adopt_dag_camera(&mut self) {
        self.host_snapshot.camera = CameraJson { x: self.dag.graph.host_snapshot.camera.x, y: self.dag.graph.host_snapshot.camera.y, zoom: self.dag.graph.host_snapshot.camera.zoom };
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
    }

    /// 🖼️ Frames the whole graph and keeps the authority on the fitted camera.
    ///
    /// @see `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` — `DagHost::fit_camera_to_content`
    pub fn fit_camera_to_content(&mut self) -> bool {
        let fitted = fit_camera_to_content(&mut self.dag.viewport, &mut self.dag.graph);
        if fitted {
            self.adopt_dag_camera();
        }
        fitted
    }

    /// 🖼️ The opening-camera decision, with the authority kept on whichever camera won.
    ///
    /// @see `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` — `DagHost::adopt_camera_or_fit`
    pub fn adopt_camera_or_fit(&mut self, x: f64, y: f64, zoom: f64) -> bool {
        let fitted = adopt_camera_or_fit(&mut self.dag.viewport, &mut self.dag.graph, x, y, zoom);
        self.adopt_dag_camera();
        fitted
    }

    /// 🖼️ Re-fits a graph that changed under a live camera, with the authority kept on the result.
    ///
    /// @see `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` — `DagHost::refit_camera_if_content_left_view`
    pub fn refit_camera_if_content_left_view(&mut self) -> bool {
        let refitted = refit_camera_if_content_left_view(&mut self.dag.viewport, &mut self.dag.graph);
        if refitted {
            self.adopt_dag_camera();
        }
        refitted
    }

    pub(super) fn refresh_interaction_projection(&mut self) {
        self.interaction_projection = self.dag.graph.bounded_interaction_projection(self.interaction_revision).ok();
    }

    /// 🖱️ Re-seeds the bounded pointer-plan projection after a SCREEN-path gesture mutated this host
    /// directly (drawing a wire, dragging the minimap viewport, dragging an inline widget) — without
    /// it `plan_pointer` answers `Unsupported` for the rest of the session, because its stored
    /// projection is a revision behind and nothing else on that path bumps it back into step.
    pub fn resync_interaction_projection(&mut self) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
    }

    pub fn plan_wheel(&self, sx: f64, sy: f64, delta_x: f64, delta_y: f64, zoom_gesture: bool) -> FlowWheelPlan {
        let camera = &self.host_snapshot.camera;
        let expected = [camera.x, camera.y, camera.zoom];
        let next = if zoom_gesture {
            use canvas::camera::{screen_to_world, Camera, Viewport};
            let viewport = Viewport { width: self.viewport_w, height: self.viewport_h, dpr: self.viewport_dpr };
            let before_camera = Camera { x: camera.x, y: camera.y, zoom: camera.zoom };
            let before = screen_to_world(&before_camera, &viewport, canvas::Point::new(sx, sy));
            let zoom = (camera.zoom * if delta_y < 0.0 { ui_styling::metrics::camera::WHEEL_ZOOM_IN_FACTOR } else { ui_styling::metrics::camera::WHEEL_ZOOM_OUT_FACTOR })
                .clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX);
            let after_camera = Camera { x: camera.x, y: camera.y, zoom };
            let after = screen_to_world(&after_camera, &viewport, canvas::Point::new(sx, sy));
            [camera.x + before.x - after.x, camera.y + before.y - after.y, zoom]
        } else {
            [camera.x - delta_x / camera.zoom, camera.y - delta_y / camera.zoom, camera.zoom]
        };
        FlowWheelPlan { revision: self.interaction_revision, expected, next }
    }

    pub fn commit_wheel(&mut self, plan: FlowWheelPlan) -> bool {
        let camera = &self.host_snapshot.camera;
        if self.interaction_revision != plan.revision || [camera.x.to_bits(), camera.y.to_bits(), camera.zoom.to_bits()] != [plan.expected[0].to_bits(), plan.expected[1].to_bits(), plan.expected[2].to_bits()] {
            return false;
        }
        self.host_snapshot.camera = CameraJson { x: plan.next[0], y: plan.next[1], zoom: plan.next[2] };
        self.dag.graph.set_camera(plan.next[0], plan.next[1], plan.next[2]);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
        true
    }

    /// 🫳️ Whether this host is mid-way through a BOUNDED gesture — a node drag, a marquee or a pan
    /// started by `commit_pointer` and not yet released. A gesture belongs to the path that started it.
    pub fn bounded_pointer_gesture_active(&self) -> bool {
        self.interaction_projection.is_some_and(|projection| projection.gesture_active())
    }

    pub fn plan_pointer(&self, intent: dag::DagPointerIntent) -> Result<dag::DagPointerPlan, dag::DagInteractionPlanFault> {
        let projection = self.interaction_projection.ok_or(dag::DagInteractionPlanFault::NodeCredits)?;
        if projection.revision() != self.interaction_revision {
            return Err(dag::DagInteractionPlanFault::Unsupported);
        }
        derive_pointer_plan(&self.dag.viewport, &self.dag.graph, &self.dag.minimap, projection, intent)
    }

    pub fn commit_pointer(&mut self, plan: dag::DagPointerPlan) -> bool {
        if self.interaction_revision != plan.expected_revision() {
            return false;
        }
        if !plan.previous_gesture_active() && plan.gesture_active() {
            self.begin_gesture();
        }
        for index in 0..plan.move_len() {
            let Some((id, x, y)) = self.dag.graph.pointer_plan_move(&plan, index) else {
                continue;
            };
            if self.host_snapshot.layout.contains_key(id) {
                self.host_snapshot.layout.insert(id.to_owned(), WidgetLayout { x, y });
            }
        }
        self.dag.graph.apply_pointer_plan(&plan);
        if plan.previous_gesture_active() && !plan.gesture_active() {
            self.commit_gesture_history();
        }
        self.interaction_projection = Some(*plan.projection());
        self.interaction_revision = plan.projection().revision();
        true
    }

    pub fn pointer_projection_snapshot(&self, plan: &dag::DagPointerPlan) -> Result<dag::DagPointerSnapshot, dag::DagInteractionPlanFault> {
        let projection = plan.projection();
        let mut bytes = 0usize;
        for id in self.dag.graph.projection_selected_id_refs(projection).chain(self.dag.graph.projection_hovered_id_ref(projection)) {
            bytes = bytes.checked_add(id.len()).ok_or(dag::DagInteractionPlanFault::StringCredits)?;
            if bytes > 16 * 1024 {
                return Err(dag::DagInteractionPlanFault::StringCredits);
            }
        }
        Ok(dag::DagPointerSnapshot { node_ids: self.dag.graph.projection_selected_id_refs(projection).map(str::to_owned).collect(), hovered_id: self.dag.graph.projection_hovered_id_ref(projection).map(str::to_owned), camera: projection.camera() })
    }

    pub fn wheel_zoom_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        let before = self.screen_to_world_point(sx, sy);
        let factor = if delta_y < 0.0 { ui_styling::metrics::camera::WHEEL_ZOOM_IN_FACTOR } else { ui_styling::metrics::camera::WHEEL_ZOOM_OUT_FACTOR };
        let zoom = (self.host_snapshot.camera.zoom * factor).clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX);
        self.host_snapshot.camera.zoom = zoom;
        self.dag.graph.set_camera(self.host_snapshot.camera.x, self.host_snapshot.camera.y, zoom);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        let after = self.screen_to_world_point(sx, sy);
        self.host_snapshot.camera.x += before.x - after.x;
        self.host_snapshot.camera.y += before.y - after.y;
        self.dag.graph.set_camera(self.host_snapshot.camera.x, self.host_snapshot.camera.y, zoom);
    }

    pub fn wheel_pan_screen(&mut self, delta_x: f64, delta_y: f64) {
        let zoom = self.host_snapshot.camera.zoom;
        let x = self.host_snapshot.camera.x - delta_x / zoom;
        let y = self.host_snapshot.camera.y - delta_y / zoom;
        self.set_camera(x, y, zoom);
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_x: f64, delta_y: f64, zoom_gesture: bool) {
        if zoom_gesture {
            self.wheel_zoom_screen(sx, sy, delta_y);
        } else {
            self.wheel_pan_screen(delta_x, delta_y);
        }
    }
}
