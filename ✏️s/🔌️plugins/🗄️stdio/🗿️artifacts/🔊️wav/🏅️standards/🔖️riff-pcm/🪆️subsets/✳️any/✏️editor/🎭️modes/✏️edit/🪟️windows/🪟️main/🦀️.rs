//! 🔊️ Natural, windowed WAV frame and channel editor.

use crate::editor::wav::edit_audio;
use crate::standards::riff_pcm::subsets::any::schema::snapshot::{WavData, WavSnapshot};
use semio_framework_plugin::app::{editable_table_window_row, row_action, row_target, TableWindowKit, WindowKit, WindowedEditableTableCell};
use semio_framework_plugin::ActionId;
use semio_framework_plugin::Buildable;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::HasBase;
use semio_framework_plugin::HasChildren;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::RowActionPlacement;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::Trigger;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_ui_contract::{self as ui, Label as UiLabel};

pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
const CONTROLLER_ID: &str = "s.stdio.wav@riff-pcm/*#editor";
const MATRIX_CHANNEL_LIMIT: usize = 8;
const SAMPLE_TABLE_ID: &str = "wav-samples";
const FORMAT_TABLE_ID: &str = "wav-format";
const CHANNEL_TABLE_ID: &str = "wav-channels";
const FRAME_TABLE_ID: &str = "wav-frames";

pub fn definition() -> WindowKindDefinition {
    let mut definition = semio_s_artifact_stdio_contract::structural_table_window_kind();
    definition.label = LocalizedLabel::native("Audio samples", "Audiosamples");
    definition.icon_id = "volume-brush".into();
    definition.actions.retain(|action| action.id != semio_s_artifact_stdio_contract::SET_TABLE_HEADER_ACTION_ID);
    definition.actions.extend(edit_audio::extra_actions());
    for action in &mut definition.actions {
        action.in_palette = false;
        match action.id.as_str() {
            edit_audio::SET_SAMPLE_ACTION_ID => action.label = LocalizedLabel::native("Set sample", "Sample setzen"),
            semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID => {
                action.label = LocalizedLabel::native("Append frame", "Frame anhängen");
            }
            semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID => {
                action.label = LocalizedLabel::native("Remove frame", "Frame entfernen");
            }
            semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID => {
                action.label = LocalizedLabel::native("Append channel", "Kanal anhängen");
            }
            semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID => {
                action.label = LocalizedLabel::native("Remove channel", "Kanal entfernen");
            }
            _ => {}
        }
    }
    definition
}

fn sample_len(data: &WavData) -> usize {
    match data {
        WavData::Pcm8(values) | WavData::Raw(values) => values.len(),
        WavData::Pcm16(values) => values.len(),
        WavData::Float32(values) => values.len(),
    }
}

fn sample_text(data: &WavData, index: usize) -> Option<String> {
    match data {
        WavData::Pcm8(values) => values.get(index).map(ToString::to_string),
        WavData::Pcm16(values) => values.get(index).map(ToString::to_string),
        WavData::Float32(values) => values.get(index).map(ToString::to_string),
        WavData::Raw(_) => None,
    }
}

fn action(name: &str) -> semio_framework_plugin::UiAssemblyResult<ActionId> {
    ActionId::try_v1(CONTROLLER_ID, name).ok_or_else(|| PluginAssemblyError::new("stdio.wav.action", format!("invalid action {name}")))
}

fn toolbar_button(id: &str, label: &str, action_id: &str, arguments: UiValue) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    ui::button(UiLabel::try_from(label).map_err(|_| PluginAssemblyError::new("stdio.wav.toolbar-label", "toolbar label exceeds UI bound"))?)
        .try_id(id)
        .map_err(|_| PluginAssemblyError::new("stdio.wav.toolbar-id", "toolbar id exceeds UI bound"))?
        .try_on_with(Trigger::Activate, action(action_id)?, arguments)
        .map_err(|_| PluginAssemblyError::new("stdio.wav.toolbar-action", "toolbar action admission"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("stdio.wav.toolbar-build", "toolbar build admission"))
}

fn toolbar(revision: &str, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let append_frame = toolbar_button(
        "append-frame",
        match locale {
            Locale::De => "Frame anhängen",
            Locale::En => "Append frame",
        },
        semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID,
        semio_s_artifact_stdio_contract::window_kit_revision_arguments(revision)?,
    )?;
    let append_channel = toolbar_button(
        "append-channel",
        match locale {
            Locale::De => "Kanal anhängen",
            Locale::En => "Append channel",
        },
        semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID,
        semio_s_artifact_stdio_contract::window_kit_revision_arguments(revision)?,
    )?;
    ui::row()
        .try_id("wav-audio-actions")
        .map_err(|_| PluginAssemblyError::new("stdio.wav.toolbar", "toolbar id admission"))?
        .try_children([append_frame, append_channel])
        .map_err(|_| PluginAssemblyError::new("stdio.wav.toolbar", "toolbar child admission"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("stdio.wav.toolbar", "toolbar build admission"))
}

fn format_table(document: &WavSnapshot, revision: &str, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let column = match locale {
        Locale::De => "Abtastrate (Hz)",
        Locale::En => "Sample rate (Hz)",
    };
    TableWindowKit::render_indexed_rows_with_id(
        windows,
        FORMAT_TABLE_ID,
        match locale {
            Locale::De => "Audioformat",
            Locale::En => "Audio format",
        },
        &[column],
        None,
        1,
        |_| {
            editable_table_window_row(
                "sample-rate",
                CONTROLLER_ID,
                locale,
                [WindowedEditableTableCell::new(document.fmt.sample_rate.to_string(), column, edit_audio::SET_SAMPLE_RATE_ACTION_ID, semio_s_artifact_stdio_contract::window_kit_revision_arguments(revision)?)],
                Vec::new(),
                None,
            )
        },
    )
}

fn channel_table(channels: usize, revision: &str, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let column = match locale {
        Locale::De => "Kanal",
        Locale::En => "Channel",
    };
    TableWindowKit::render_indexed_rows_with_id(
        windows,
        CHANNEL_TABLE_ID,
        match locale {
            Locale::De => "Kanäle",
            Locale::En => "Channels",
        },
        &[column],
        Some(match locale {
            Locale::De => "Aktionen",
            Locale::En => "Actions",
        }),
        channels,
        |channel| {
            let insert = row_action(
                "plus",
                match locale {
                    Locale::De => "Kanal davor einfügen",
                    Locale::En => "Insert channel before",
                },
                edit_audio::INSERT_CHANNEL_ACTION_ID,
                RowActionPlacement::Row,
            )?;
            let mut actions = vec![insert];
            if channels > 1 {
                actions.push(row_action(
                    "trash-2",
                    match locale {
                        Locale::De => "Kanal entfernen",
                        Locale::En => "Remove channel",
                    },
                    semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID,
                    RowActionPlacement::Row,
                )?);
            }
            let target = row_target(CONTROLLER_ID, Some(semio_s_artifact_stdio_contract::window_kit_indexed_revision_arguments("column", channel, revision)?), None)?;
            editable_table_window_row(&format!("channel-{channel}"), CONTROLLER_ID, locale, [WindowedEditableTableCell::read_only((channel + 1).to_string(), column)], actions, Some(target))
        },
    )
}

fn frame_table(frames: usize, revision: &str, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let column = match locale {
        Locale::De => "Frame",
        Locale::En => "Frame",
    };
    TableWindowKit::render_indexed_rows_with_id(
        windows,
        FRAME_TABLE_ID,
        match locale {
            Locale::De => "Frameaktionen",
            Locale::En => "Frame actions",
        },
        &[column],
        Some(match locale {
            Locale::De => "Aktionen",
            Locale::En => "Actions",
        }),
        frames,
        |frame| {
            let insert = row_action(
                "plus",
                match locale {
                    Locale::De => "Frame davor einfügen",
                    Locale::En => "Insert frame before",
                },
                edit_audio::INSERT_FRAME_ACTION_ID,
                RowActionPlacement::Row,
            )?;
            let remove = row_action(
                "trash-2",
                match locale {
                    Locale::De => "Frame entfernen",
                    Locale::En => "Remove frame",
                },
                semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID,
                RowActionPlacement::Row,
            )?;
            let target = row_target(CONTROLLER_ID, Some(semio_s_artifact_stdio_contract::window_kit_indexed_revision_arguments("row", frame, revision)?), None)?;
            editable_table_window_row(&format!("frame-control-{frame}"), CONTROLLER_ID, locale, [WindowedEditableTableCell::read_only((frame + 1).to_string(), column)], [insert, remove], Some(target))
        },
    )
}

fn matrix_table(document: &WavSnapshot, channels: usize, frames: usize, revision: &str, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let columns = (0..channels)
        .map(|channel| match locale {
            Locale::De => format!("Kanal {}", channel + 1),
            Locale::En => format!("Channel {}", channel + 1),
        })
        .collect::<Vec<_>>();
    let column_refs = columns.iter().map(String::as_str).collect::<Vec<_>>();
    let title = match locale {
        Locale::De => format!("WAV-Frames — {} Hz, {} Bit", document.fmt.sample_rate, document.fmt.bits_per_sample),
        Locale::En => format!("WAV frames — {} Hz, {} bit", document.fmt.sample_rate, document.fmt.bits_per_sample),
    };
    TableWindowKit::render_indexed_rows_with_id(
        windows,
        SAMPLE_TABLE_ID,
        &title,
        &column_refs,
        Some(match locale {
            Locale::De => "Aktionen",
            Locale::En => "Actions",
        }),
        frames,
        |frame| {
            let cells = (0..channels)
                .map(|channel| {
                    let value = sample_text(&document.data, frame * channels + channel).unwrap_or_default();
                    let arguments = semio_s_artifact_stdio_contract::window_kit_revisioned_cell_arguments(frame, channel, revision)?;
                    Ok(WindowedEditableTableCell::new(value, columns[channel].clone(), edit_audio::SET_SAMPLE_ACTION_ID, arguments))
                })
                .collect::<semio_framework_plugin::UiAssemblyResult<Vec<_>>>()?;
            let insert = row_action(
                "plus",
                match locale {
                    Locale::De => "Frame davor einfügen",
                    Locale::En => "Insert frame before",
                },
                edit_audio::INSERT_FRAME_ACTION_ID,
                RowActionPlacement::Row,
            )?;
            let remove = row_action(
                "trash-2",
                match locale {
                    Locale::De => "Frame entfernen",
                    Locale::En => "Remove frame",
                },
                semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID,
                RowActionPlacement::Row,
            )?;
            let target = row_target(CONTROLLER_ID, Some(semio_s_artifact_stdio_contract::window_kit_indexed_revision_arguments("row", frame, revision)?), None)?;
            editable_table_window_row(&format!("frame-{frame}"), CONTROLLER_ID, locale, cells, [insert, remove], Some(target))
        },
    )
}

fn coordinate_table(document: &WavSnapshot, channels: usize, revision: &str, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let columns = match locale {
        Locale::De => ["Frame", "Kanal", "Wert"],
        Locale::En => ["Frame", "Channel", "Value"],
    };
    let total = sample_len(&document.data);
    TableWindowKit::render_indexed_rows_with_id(
        windows,
        SAMPLE_TABLE_ID,
        match locale {
            Locale::De => "WAV-Samples — vollständige Koordinatenansicht",
            Locale::En => "WAV samples — complete coordinate view",
        },
        &columns,
        None,
        total,
        |index| {
            let frame = index / channels;
            let channel = index % channels;
            editable_table_window_row(
                &format!("sample-{index}"),
                CONTROLLER_ID,
                locale,
                [
                    WindowedEditableTableCell::read_only((frame + 1).to_string(), columns[0]),
                    WindowedEditableTableCell::read_only((channel + 1).to_string(), columns[1]),
                    WindowedEditableTableCell::new(
                        sample_text(&document.data, index).unwrap_or_default(),
                        columns[2],
                        edit_audio::SET_SAMPLE_ACTION_ID,
                        semio_s_artifact_stdio_contract::window_kit_revisioned_cell_arguments(frame, channel, revision)?,
                    ),
                ],
                Vec::new(),
                None,
            )
        },
    )
}

fn raw_table(values: &[u8], locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let columns = [match locale {
        Locale::De => "Rohe Bytes",
        Locale::En => "Raw bytes",
    }];
    TableWindowKit::render_indexed_rows_with_id(
        windows,
        SAMPLE_TABLE_ID,
        match locale {
            Locale::De => "Rohe WAV-Nutzdaten — Framebearbeitung ist nicht verfügbar",
            Locale::En => "Raw WAV payload — frame editing is unavailable",
        },
        &columns,
        None,
        values.len(),
        |row| editable_table_window_row(&format!("raw-{row}"), CONTROLLER_ID, locale, [WindowedEditableTableCell::read_only(values[row].to_string(), columns[0])], Vec::new(), None),
    )
}

pub fn render(document: &WavSnapshot, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(document);
    render_revisioned(document, &revision, locale, &TreeWindows::unhosted())
}

pub fn render_revisioned(document: &WavSnapshot, revision: &str, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut children = Vec::new();
    match &document.data {
        WavData::Raw(values) => children.push(raw_table(values, locale, windows)?),
        _ => match edit_audio::coherent_audio(document) {
            Err(_) => children.push(raw_table(&[], locale, windows)?),
            Ok((channels, frames, _)) => {
                children.push(toolbar(revision, locale)?);
                if channels <= MATRIX_CHANNEL_LIMIT {
                    children.push(matrix_table(document, channels, frames, revision, locale, windows)?);
                } else {
                    children.push(coordinate_table(document, channels, revision, locale, windows)?);
                    children.push(frame_table(frames, revision, locale, windows)?);
                }
                children.push(channel_table(channels, revision, locale, windows)?);
                children.push(format_table(document, revision, locale, windows)?);
            }
        },
    }
    ui::column()
        .try_id("wav-audio-editor")
        .map_err(|_| PluginAssemblyError::new("stdio.wav.editor", "editor id admission"))?
        .try_children(children)
        .map_err(|_| PluginAssemblyError::new("stdio.wav.editor", "editor child admission"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("stdio.wav.editor", "editor build admission"))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
