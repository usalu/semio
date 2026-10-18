//! 🛂️ Palette overlay value codec of the board schema: the value encoder and decoder of the palette overlay. See [`super`].

use super::*;

impl ToValue for BoardPaletteOverlay{fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<DslValue,ValueError>{let count=[self.raster_clear,self.grid_minor_stroke,self.edge_stroke,self.edge_stroke_hovered,self.edge_stroke_selected,self.edge_stroke_selection_exit,self.edge_stroke_disabled,self.node_fill,self.node_stroke,self.node_fill_hovered,self.node_stroke_hovered,self.node_fill_selected,self.node_stroke_selected,self.node_fill_selection_exit,self.node_stroke_selection_exit,self.node_fill_disabled,self.node_stroke_disabled,self.node_stroke_computing,self.node_stroke_stale,self.node_stroke_error,self.node_stroke_blocked,self.indirect_handle_fill,self.indirect_handle_stroke,self.handle_fill,self.handle_stroke,self.handle_fill_hovered,self.handle_stroke_hovered,self.handle_fill_selected,self.handle_stroke_selected,self.handle_fill_selection_exit,self.handle_stroke_selection_exit,self.handle_fill_disabled,self.handle_stroke_disabled,self.wire_stroke,self.wire_stroke_hovered,self.wire_stroke_selected,self.wire_stroke_highlighted,self.wire_stroke_disabled,self.selection_preview_fill,self.selection_preview_stroke,self.label_fill,self.label_fill_hovered,self.label_halo,self.minimap_widget_panel_fill,self.minimap_widget_panel_stroke,self.minimap_widget_viewport_fill,self.minimap_widget_viewport_stroke,self.minimap_widget_viewport_stroke_hovered].into_iter().filter(Option::is_some).count();let mut fields=<Vec<(String,DslValue)> as FromValue>::guard_decoded(control.allocate_vec(count)?);
if let Some(value)=self.raster_clear{let key=control.copy_text("rasterClear")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.grid_minor_stroke{let key=control.copy_text("gridMinorStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.edge_stroke{let key=control.copy_text("edgeStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.edge_stroke_hovered{let key=control.copy_text("edgeStrokeHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.edge_stroke_selected{let key=control.copy_text("edgeStrokeSelected")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.edge_stroke_selection_exit{let key=control.copy_text("edgeStrokeSelectionExit")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.edge_stroke_disabled{let key=control.copy_text("edgeStrokeDisabled")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_fill{let key=control.copy_text("nodeFill")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke{let key=control.copy_text("nodeStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_fill_hovered{let key=control.copy_text("nodeFillHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_hovered{let key=control.copy_text("nodeStrokeHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_fill_selected{let key=control.copy_text("nodeFillSelected")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_selected{let key=control.copy_text("nodeStrokeSelected")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_fill_selection_exit{let key=control.copy_text("nodeFillSelectionExit")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_selection_exit{let key=control.copy_text("nodeStrokeSelectionExit")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_fill_disabled{let key=control.copy_text("nodeFillDisabled")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_disabled{let key=control.copy_text("nodeStrokeDisabled")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_computing{let key=control.copy_text("nodeStrokeComputing")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_stale{let key=control.copy_text("nodeStrokeStale")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_error{let key=control.copy_text("nodeStrokeError")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_blocked{let key=control.copy_text("nodeStrokeBlocked")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.indirect_handle_fill{let key=control.copy_text("indirectHandleFill")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.indirect_handle_stroke{let key=control.copy_text("indirectHandleStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_fill{let key=control.copy_text("handleFill")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_stroke{let key=control.copy_text("handleStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_fill_hovered{let key=control.copy_text("handleFillHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_stroke_hovered{let key=control.copy_text("handleStrokeHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_fill_selected{let key=control.copy_text("handleFillSelected")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_stroke_selected{let key=control.copy_text("handleStrokeSelected")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_fill_selection_exit{let key=control.copy_text("handleFillSelectionExit")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_stroke_selection_exit{let key=control.copy_text("handleStrokeSelectionExit")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_fill_disabled{let key=control.copy_text("handleFillDisabled")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_stroke_disabled{let key=control.copy_text("handleStrokeDisabled")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.wire_stroke{let key=control.copy_text("wireStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.wire_stroke_hovered{let key=control.copy_text("wireStrokeHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.wire_stroke_selected{let key=control.copy_text("wireStrokeSelected")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.wire_stroke_highlighted{let key=control.copy_text("wireStrokeHighlighted")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.wire_stroke_disabled{let key=control.copy_text("wireStrokeDisabled")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.selection_preview_fill{let key=control.copy_text("selectionPreviewFill")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.selection_preview_stroke{let key=control.copy_text("selectionPreviewStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.label_fill{let key=control.copy_text("labelFill")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.label_fill_hovered{let key=control.copy_text("labelFillHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.label_halo{let key=control.copy_text("labelHalo")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.minimap_widget_panel_fill{let key=control.copy_text("minimapWidgetPanelFill")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.minimap_widget_panel_stroke{let key=control.copy_text("minimapWidgetPanelStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.minimap_widget_viewport_fill{let key=control.copy_text("minimapWidgetViewportFill")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.minimap_widget_viewport_stroke{let key=control.copy_text("minimapWidgetViewportStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.minimap_widget_viewport_stroke_hovered{let key=control.copy_text("minimapWidgetViewportStrokeHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
Ok(DslValue::Object(fields.take()))}
fn to_value(&self)->DslValue{let mut fields=Vec::new();
if let Some(value)=self.raster_clear{fields.push(("rasterClear".into(),value.to_value()));}
if let Some(value)=self.grid_minor_stroke{fields.push(("gridMinorStroke".into(),value.to_value()));}
if let Some(value)=self.edge_stroke{fields.push(("edgeStroke".into(),value.to_value()));}
if let Some(value)=self.edge_stroke_hovered{fields.push(("edgeStrokeHovered".into(),value.to_value()));}
if let Some(value)=self.edge_stroke_selected{fields.push(("edgeStrokeSelected".into(),value.to_value()));}
if let Some(value)=self.edge_stroke_selection_exit{fields.push(("edgeStrokeSelectionExit".into(),value.to_value()));}
if let Some(value)=self.edge_stroke_disabled{fields.push(("edgeStrokeDisabled".into(),value.to_value()));}
if let Some(value)=self.node_fill{fields.push(("nodeFill".into(),value.to_value()));}
if let Some(value)=self.node_stroke{fields.push(("nodeStroke".into(),value.to_value()));}
if let Some(value)=self.node_fill_hovered{fields.push(("nodeFillHovered".into(),value.to_value()));}
if let Some(value)=self.node_stroke_hovered{fields.push(("nodeStrokeHovered".into(),value.to_value()));}
if let Some(value)=self.node_fill_selected{fields.push(("nodeFillSelected".into(),value.to_value()));}
if let Some(value)=self.node_stroke_selected{fields.push(("nodeStrokeSelected".into(),value.to_value()));}
if let Some(value)=self.node_fill_selection_exit{fields.push(("nodeFillSelectionExit".into(),value.to_value()));}
if let Some(value)=self.node_stroke_selection_exit{fields.push(("nodeStrokeSelectionExit".into(),value.to_value()));}
if let Some(value)=self.node_fill_disabled{fields.push(("nodeFillDisabled".into(),value.to_value()));}
if let Some(value)=self.node_stroke_disabled{fields.push(("nodeStrokeDisabled".into(),value.to_value()));}
if let Some(value)=self.node_stroke_computing{fields.push(("nodeStrokeComputing".into(),value.to_value()));}
if let Some(value)=self.node_stroke_stale{fields.push(("nodeStrokeStale".into(),value.to_value()));}
if let Some(value)=self.node_stroke_error{fields.push(("nodeStrokeError".into(),value.to_value()));}
if let Some(value)=self.node_stroke_blocked{fields.push(("nodeStrokeBlocked".into(),value.to_value()));}
if let Some(value)=self.indirect_handle_fill{fields.push(("indirectHandleFill".into(),value.to_value()));}
if let Some(value)=self.indirect_handle_stroke{fields.push(("indirectHandleStroke".into(),value.to_value()));}
if let Some(value)=self.handle_fill{fields.push(("handleFill".into(),value.to_value()));}
if let Some(value)=self.handle_stroke{fields.push(("handleStroke".into(),value.to_value()));}
if let Some(value)=self.handle_fill_hovered{fields.push(("handleFillHovered".into(),value.to_value()));}
if let Some(value)=self.handle_stroke_hovered{fields.push(("handleStrokeHovered".into(),value.to_value()));}
if let Some(value)=self.handle_fill_selected{fields.push(("handleFillSelected".into(),value.to_value()));}
if let Some(value)=self.handle_stroke_selected{fields.push(("handleStrokeSelected".into(),value.to_value()));}
if let Some(value)=self.handle_fill_selection_exit{fields.push(("handleFillSelectionExit".into(),value.to_value()));}
if let Some(value)=self.handle_stroke_selection_exit{fields.push(("handleStrokeSelectionExit".into(),value.to_value()));}
if let Some(value)=self.handle_fill_disabled{fields.push(("handleFillDisabled".into(),value.to_value()));}
if let Some(value)=self.handle_stroke_disabled{fields.push(("handleStrokeDisabled".into(),value.to_value()));}
if let Some(value)=self.wire_stroke{fields.push(("wireStroke".into(),value.to_value()));}
if let Some(value)=self.wire_stroke_hovered{fields.push(("wireStrokeHovered".into(),value.to_value()));}
if let Some(value)=self.wire_stroke_selected{fields.push(("wireStrokeSelected".into(),value.to_value()));}
if let Some(value)=self.wire_stroke_highlighted{fields.push(("wireStrokeHighlighted".into(),value.to_value()));}
if let Some(value)=self.wire_stroke_disabled{fields.push(("wireStrokeDisabled".into(),value.to_value()));}
if let Some(value)=self.selection_preview_fill{fields.push(("selectionPreviewFill".into(),value.to_value()));}
if let Some(value)=self.selection_preview_stroke{fields.push(("selectionPreviewStroke".into(),value.to_value()));}
if let Some(value)=self.label_fill{fields.push(("labelFill".into(),value.to_value()));}
if let Some(value)=self.label_fill_hovered{fields.push(("labelFillHovered".into(),value.to_value()));}
if let Some(value)=self.label_halo{fields.push(("labelHalo".into(),value.to_value()));}
if let Some(value)=self.minimap_widget_panel_fill{fields.push(("minimapWidgetPanelFill".into(),value.to_value()));}
if let Some(value)=self.minimap_widget_panel_stroke{fields.push(("minimapWidgetPanelStroke".into(),value.to_value()));}
if let Some(value)=self.minimap_widget_viewport_fill{fields.push(("minimapWidgetViewportFill".into(),value.to_value()));}
if let Some(value)=self.minimap_widget_viewport_stroke{fields.push(("minimapWidgetViewportStroke".into(),value.to_value()));}
if let Some(value)=self.minimap_widget_viewport_stroke_hovered{fields.push(("minimapWidgetViewportStrokeHovered".into(),value.to_value()));}
DslValue::Object(fields)}}

impl FromValue for BoardPaletteOverlay{fn from_value(value:DslValue)->Result<Self,ValueError>{let mut accepted=|_|true;let mut control=NativeDecodeControl::new(usize::MAX,&mut accepted);Self::from_value_controlled(&value,&mut control)}
fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let DslValue::Object(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Board palette overlay must be an object"))};let mut output=Self::default();let mut seen=[false;48];for (key,value)in fields{control.step()?;let index=match key.as_str(){
"rasterClear"=>0,
"gridMinorStroke"=>1,
"edgeStroke"=>2,
"edgeStrokeHovered"=>3,
"edgeStrokeSelected"=>4,
"edgeStrokeSelectionExit"=>5,
"edgeStrokeDisabled"=>6,
"nodeFill"=>7,
"nodeStroke"=>8,
"nodeFillHovered"=>9,
"nodeStrokeHovered"=>10,
"nodeFillSelected"=>11,
"nodeStrokeSelected"=>12,
"nodeFillSelectionExit"=>13,
"nodeStrokeSelectionExit"=>14,
"nodeFillDisabled"=>15,
"nodeStrokeDisabled"=>16,
"nodeStrokeComputing"=>17,
"nodeStrokeStale"=>18,
"nodeStrokeError"=>19,
"nodeStrokeBlocked"=>20,
"indirectHandleFill"=>21,
"indirectHandleStroke"=>22,
"handleFill"=>23,
"handleStroke"=>24,
"handleFillHovered"=>25,
"handleStrokeHovered"=>26,
"handleFillSelected"=>27,
"handleStrokeSelected"=>28,
"handleFillSelectionExit"=>29,
"handleStrokeSelectionExit"=>30,
"handleFillDisabled"=>31,
"handleStrokeDisabled"=>32,
"wireStroke"=>33,
"wireStrokeHovered"=>34,
"wireStrokeSelected"=>35,
"wireStrokeHighlighted"=>36,
"wireStrokeDisabled"=>37,
"selectionPreviewFill"=>38,
"selectionPreviewStroke"=>39,
"labelFill"=>40,
"labelFillHovered"=>41,
"labelHalo"=>42,
"minimapWidgetPanelFill"=>43,
"minimapWidgetPanelStroke"=>44,
"minimapWidgetViewportFill"=>45,
"minimapWidgetViewportStroke"=>46,
"minimapWidgetViewportStrokeHovered"=>47,
_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Unknown board palette field"))};if seen[index]{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Duplicate board palette field"))}seen[index]=true;let color=<[u8;4] as FromValue>::from_value_controlled(value,control).map_err(|e|e.under(key))?;match index{
0=>output.raster_clear=Some(color),
1=>output.grid_minor_stroke=Some(color),
2=>output.edge_stroke=Some(color),
3=>output.edge_stroke_hovered=Some(color),
4=>output.edge_stroke_selected=Some(color),
5=>output.edge_stroke_selection_exit=Some(color),
6=>output.edge_stroke_disabled=Some(color),
7=>output.node_fill=Some(color),
8=>output.node_stroke=Some(color),
9=>output.node_fill_hovered=Some(color),
10=>output.node_stroke_hovered=Some(color),
11=>output.node_fill_selected=Some(color),
12=>output.node_stroke_selected=Some(color),
13=>output.node_fill_selection_exit=Some(color),
14=>output.node_stroke_selection_exit=Some(color),
15=>output.node_fill_disabled=Some(color),
16=>output.node_stroke_disabled=Some(color),
17=>output.node_stroke_computing=Some(color),
18=>output.node_stroke_stale=Some(color),
19=>output.node_stroke_error=Some(color),
20=>output.node_stroke_blocked=Some(color),
21=>output.indirect_handle_fill=Some(color),
22=>output.indirect_handle_stroke=Some(color),
23=>output.handle_fill=Some(color),
24=>output.handle_stroke=Some(color),
25=>output.handle_fill_hovered=Some(color),
26=>output.handle_stroke_hovered=Some(color),
27=>output.handle_fill_selected=Some(color),
28=>output.handle_stroke_selected=Some(color),
29=>output.handle_fill_selection_exit=Some(color),
30=>output.handle_stroke_selection_exit=Some(color),
31=>output.handle_fill_disabled=Some(color),
32=>output.handle_stroke_disabled=Some(color),
33=>output.wire_stroke=Some(color),
34=>output.wire_stroke_hovered=Some(color),
35=>output.wire_stroke_selected=Some(color),
36=>output.wire_stroke_highlighted=Some(color),
37=>output.wire_stroke_disabled=Some(color),
38=>output.selection_preview_fill=Some(color),
39=>output.selection_preview_stroke=Some(color),
40=>output.label_fill=Some(color),
41=>output.label_fill_hovered=Some(color),
42=>output.label_halo=Some(color),
43=>output.minimap_widget_panel_fill=Some(color),
44=>output.minimap_widget_panel_stroke=Some(color),
45=>output.minimap_widget_viewport_fill=Some(color),
46=>output.minimap_widget_viewport_stroke=Some(color),
47=>output.minimap_widget_viewport_stroke_hovered=Some(color),
_=>unreachable!()}}Ok(output)}}
