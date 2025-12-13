use std::path::PathBuf;

use gpui::{Action, App, SharedString};
use gpui_component::{ActiveTheme, Theme, ThemeMode, ThemeRegistry};
use mesh_core::MeshConfig;

use crate::MeshState;

pub fn init(cx: &mut App) {
    let config = &cx.global::<MeshState>().config;
    let theme_name = SharedString::from(config.current_theme());

    if let Err(err) = ThemeRegistry::watch_dir(
        PathBuf::from(MeshConfig::themes_dir_path()),
        cx,
        move |cx| {
            if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
                Theme::global_mut(cx).apply_config(&theme);
            }
        },
    ) {
        log::error!("Failed to watch themes directory: {}", err);
    }

    cx.refresh_windows();

    cx.observe_global::<Theme>(|cx| {
        let theme = cx.theme().theme_name().clone();
        let config = &cx.global::<MeshState>().config;

        config.change_theme(theme.into());
        config.save();
    })
    .detach();

    cx.on_action(|switch: &SwitchTheme, cx| {
        let theme_name = switch.0.clone();
        if let Some(theme_config) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
            Theme::global_mut(cx).apply_config(&theme_config);
        }
        cx.refresh_windows();
    });
    cx.on_action(|switch: &SwitchThemeMode, cx| {
        let mode = switch.0;
        Theme::change(mode, None, cx);
        cx.refresh_windows();
    });
}

pub fn switch_theme_mode(cx: &mut App) {
    if Theme::global_mut(cx).is_dark() {
        Theme::change(ThemeMode::Light, None, cx);
    } else {
        Theme::change(ThemeMode::Dark, None, cx);
    }
    cx.refresh_windows();
}

#[derive(Action, Clone, PartialEq)]
#[action(namespace = themes, no_json)]
pub(crate) struct SwitchTheme(pub(crate) SharedString);

#[derive(Action, Clone, PartialEq)]
#[action(namespace = themes, no_json)]
pub(crate) struct SwitchThemeMode(pub(crate) ThemeMode);
