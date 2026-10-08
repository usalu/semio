//! 🧩️ Physical text presentation for owned generation values.
use crate::{DslValue,ToValue,PlaybookBlock,PlaybookSpec,PlaybookValues,default_value_for_block,is_block_visible};
use crate::generation_forms::generation_action;
use semio_framework_value::{NativeEncodeControl,ValueError};
use semio_framework_ui_locale::Label;
use ui_wgpu::wgpu::{ui_stack_vertical,ui_text,build_text_editor_scene,UiControlNode,UiFieldNode,UiInputNode,UiNode,UiPresence,UiSelectItem,UiSelectNode,UiSliderNode,UiToggleNode,TextEditorScene};
#[path="🌱️value-text/🦀️.rs"]
pub mod value_text;

    fn render_question_field(question: &PlaybookBlock, values: &PlaybookValues, controller_id: &str, patch_action: &str, generation_id: &str, control: &mut NativeEncodeControl<'_>) -> Result<Option<UiNode>,ValueError> {
        if !is_block_visible(question, values) {
            return Ok(None);
        }
        control.checkpoint()?;
        let value=match values.get(&question.id) {Some(value)=>value.to_value_controlled(control)?,None=>match question.default.as_ref() {Some(value)=>value.to_value_controlled(control)?,None=>default_value_for_block(question)}};
        let field_id = format!("generate.form.{}", question.id);
        let on_change = || generation_action(controller_id, patch_action, Some(DslValue::object([("generationId".to_string(), DslValue::String(generation_id.to_string())), ("questionId".to_string(), DslValue::String(question.id.clone()))])));
        let child = match question.kind.as_str() {
            "text" | "longText" => UiControlNode::Input(UiInputNode {
                id: format!("{field_id}.input"),
                input_kind: if question.kind == "longText" { "textarea".into() } else { "text".into() },
                value: value.as_str().unwrap_or_default().to_string(),
                placeholder: question.placeholder.clone().map(Label::data),
                accessibility_label: None,
                commit: None,
                on_change: on_change(),
                min: None,
                max: None,
                step: None,
                accept: None,
                precision: None,
                snaps: Vec::new(),
                on_submit: None, on_abort: None, on_repeat_last: None, presence: UiPresence::default(),
                menu: None,
                ..Default::default()
            }),
            "number" => UiControlNode::Input(UiInputNode {
                id: format!("{field_id}.input"),
                input_kind: "number".into(),
                value: value.as_f64().map(|number| number.to_string()).unwrap_or_default(),
                placeholder: question.placeholder.clone().map(Label::data),
                accessibility_label: None,
                commit: None,
                on_change: on_change(),
                min: None,
                max: None,
                step: None,
                accept: None,
                precision: None,
                snaps: Vec::new(),
                on_submit: None, on_abort: None, on_repeat_last: None, presence: UiPresence::default(),
                menu: None,
                ..Default::default()
            }),
            "slider" => UiControlNode::Slider(UiSliderNode {
                id: format!("{field_id}.slider"),
                value: value.as_f64().unwrap_or_else(|| question.min.unwrap_or(0.0)),
                min: question.min.unwrap_or(0.0),
                max: question.max.unwrap_or(100.0),
                step: question.step.unwrap_or(1.0),
                on_change: on_change(),
                unit: None,
                snaps: Vec::new(),
                presence: UiPresence::default(),
                menu: None,
                ..Default::default()
            }),
            "boolean" => UiControlNode::Toggle(UiToggleNode {
                appearance: Default::default(),
                id: format!("{field_id}.toggle"),
                icon_id: "toggle-left".into(),
                text: Some(Label::data(question.label.clone())),
                on_change: on_change(),
                presence: UiPresence::selected(value.as_bool().unwrap_or(false)),
                menu: None,
            }),
            "single" => {
                let items = question.options.as_ref().map(|options| options.iter().map(|option| UiSelectItem { value: option.value.clone(), label: Label::data(option.label.clone()) }).collect()).unwrap_or_default();
                UiControlNode::Select(UiSelectNode {
                    id: format!("{field_id}.select"),
                    value: value.as_str().unwrap_or_default().to_string(),
                    items,
                    placeholder: question.placeholder.clone().map(Label::data),
                    on_change: on_change(),
                    presence: UiPresence::default(),
                    menu: None,
                })
            }
            "vector" => {
                let numbers: Vec<DslValue> = value.as_array().map_or_else(|| question.fields.as_ref().map(|fields| fields.iter().map(|field| DslValue::float(field.value.unwrap_or(0.0))).collect()).unwrap_or_default(), |slice| slice.to_vec());
                let labels: Vec<String> = question
                    .fields
                    .as_ref()
                    .map_or_else(|| numbers.iter().enumerate().map(|(index, _)| format!("Field {}", index + 1)).collect(), |fields| fields.iter().map(|field| field.label.clone().unwrap_or_else(|| field.key.clone())).collect());
                let children: Vec<UiNode> = numbers
                    .iter()
                    .enumerate()
                    .map(|(index, number)| {
                        let label = labels.get(index).cloned().unwrap_or_else(|| format!("Field {}", index + 1));
                        UiNode::Field(UiFieldNode {
                            id: format!("{field_id}.vector.{index}"),
                            label: Label::data(label),
                            child: Box::new(UiNode::Input(UiInputNode {
                                id: format!("{field_id}.vector.{index}.input"),
                                input_kind: "number".into(),
                                value: number.as_f64().map(|entry| entry.to_string()).unwrap_or_default(),
                                placeholder: None,
                                accessibility_label: None,
                                commit: None,
                                on_change: generation_action(
                                    controller_id,
                                    patch_action,
                                    Some(DslValue::object([
                                        ("generationId".to_string(), DslValue::String(generation_id.to_string())),
                                        ("questionId".to_string(), DslValue::String(question.id.clone())),
                                        ("fieldIndex".to_string(), DslValue::uint(index as u64)),
                                    ])),
                                ),
                                min: None,
                                max: None,
                                step: None,
                                accept: None,
                                precision: None,
                                snaps: Vec::new(),
                                on_submit: None, on_abort: None, on_repeat_last: None, presence: UiPresence::default(),
                                menu: None,
                                ..Default::default()
                            })),
                            description: None,
                            required: None,
                            error: None,
                            presence: UiPresence::default(),
                            menu: None,
                        })
                    })
                    .collect();
                return Ok(Some(ui_stack_vertical(children)));
            }
            "note" => return Ok(Some(ui_text(Label::data(question.text.clone().unwrap_or_default())))),
            "image" => return Ok(Some(ui_text(Label::data(question.src.clone().unwrap_or_else(|| "(no image)".into()))))),
            _ => UiControlNode::Input(UiInputNode {
                id: format!("{field_id}.input"),
                input_kind: "text".into(),
                value: value_text::generation_value_input_text_v1(&value,control)?,
                placeholder: question.placeholder.clone().map(Label::data),
                accessibility_label: None,
                commit: None,
                on_change: on_change(),
                min: None,
                max: None,
                step: None,
                accept: None,
                precision: None,
                snaps: Vec::new(),
                on_submit: None, on_abort: None, on_repeat_last: None, presence: UiPresence::default(),
                menu: None,
                ..Default::default()
            }),
        };
        Ok(Some(UiNode::Field(UiFieldNode {
            id: field_id,
            label: Label::data(question.label.clone()),
            child: Box::new(ui_wgpu::wgpu::ui_control_to_node(child)),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
            menu: None,
        })))
    }

    /// ⌨️ Presents generation fields through caller-controlled physical text output.
    pub fn render_generation_form_body(form_spec: &PlaybookSpec, values: &PlaybookValues, controller_id: &str, patch_action: &str, generation_id: &str, control: &mut NativeEncodeControl<'_>) -> Result<UiNode,ValueError> {
        let mut children = Vec::new();
        for step in &form_spec.steps {
            if !step.blocks.is_empty() {
                children.push(ui_text(Label::data(step.title.clone())));
            }
            for question in &step.blocks {
                if let Some(field) = render_question_field(question, values, controller_id, patch_action, generation_id,control)? {
                    children.push(field);
                }
            }
        }
        if children.is_empty() {
            return Ok(ui_text(Label::data("No input widgets to generate from.")));
        }
        Ok(ui_stack_vertical(children))
    }

    /// 📜️ Presents already emitted generation text through its editor surface.
    pub fn render_generation_preview_text(surface: &str, controller_id: &str, text: &str) -> UiNode {
        build_text_editor_scene(surface, controller_id, TextEditorScene::base(text.to_string(), Some("json".into()), None))
    }
