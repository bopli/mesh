use std::path::PathBuf;

use gpui::{App, SharedString};
use gpui_component::{Theme, ThemeMode, ThemeRegistry};
use serde::{Deserialize, Serialize};

const STATE_FILE: &str = "mesh_state.json";
const THEME_DARK: &str = "Ayu Dark";
const THEME_LIGHT: &str = "Ayu Light";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct State {
    theme: SharedString,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: THEME_LIGHT.into(),
        }
    }
}

pub fn init(cx: &mut App) {
    // Load last theme state
    let json = std::fs::read_to_string(STATE_FILE).unwrap_or(String::default());
    log::info!("Load themes...");
    let state = serde_json::from_str::<State>(&json).unwrap_or_default();
    if let Err(err) = ThemeRegistry::watch_dir(PathBuf::from("./themes"), cx, move |cx| {
        let themes = ThemeRegistry::global(cx).themes();

        if let Some(theme) = themes.get(&state.theme).cloned() {
            Theme::global_mut(cx).apply_config(&theme);
        }

        if &state.theme == THEME_LIGHT {
            if let Some(theme) = ThemeRegistry::global(cx)
                .themes()
                .get(&SharedString::from(THEME_DARK))
                .cloned()
            {
                Theme::global_mut(cx).apply_config(&theme);
            }
            if let Some(theme) = ThemeRegistry::global(cx)
                .themes()
                .get(&SharedString::from(THEME_LIGHT))
                .cloned()
            {
                Theme::global_mut(cx).apply_config(&theme);
            }
        } else {
            if let Some(theme) = ThemeRegistry::global(cx)
                .themes()
                .get(&SharedString::from(THEME_LIGHT))
                .cloned()
            {
                Theme::global_mut(cx).apply_config(&theme);
            }
            if let Some(theme) = ThemeRegistry::global(cx)
                .themes()
                .get(&SharedString::from(THEME_DARK))
                .cloned()
            {
                Theme::global_mut(cx).apply_config(&theme);
            }
        }
    }) {
        log::error!("Failed to watch themes directory: {}", err);
    }

    cx.refresh_windows();

    cx.observe_global::<Theme>(|cx| {
        let state = State {
            theme: Theme::global(cx).theme_name().clone(),
        };

        if let Ok(json) = serde_json::to_string_pretty(&state) {
            // Ignore write errors - if STATE_FILE doesn't exist or can't be written, do nothing
            let _ = std::fs::write(STATE_FILE, json);
        }
    })
    .detach();
}

pub fn switch_theme_mode(cx: &mut App) {
    if Theme::global_mut(cx).is_dark() {
        Theme::change(ThemeMode::Light, None, cx);
    } else {
        Theme::change(ThemeMode::Dark, None, cx);
    }
    cx.refresh_windows();
}
