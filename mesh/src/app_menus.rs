use gpui::{App, Menu, MenuItem, SharedString};
use gpui_component::ThemeRegistry;

use crate::{About, Open, Quit, ToggleSearch, themes::SwitchTheme};

pub fn init(title: impl Into<SharedString>, cx: &mut App) {
    cx.set_menus(vec![Menu {
        name: title.into(),
        items: vec![
            MenuItem::action("About", About),
            MenuItem::Separator,
            MenuItem::action("Open...", Open),
            MenuItem::Separator,
            MenuItem::action("Toggle Search", ToggleSearch),
            theme_menu(cx),
            MenuItem::Separator,
            MenuItem::action("Quit", Quit),
        ],
    }]);
}

fn theme_menu(cx: &App) -> MenuItem {
    let themes = ThemeRegistry::global(cx).sorted_themes();
    MenuItem::Submenu(Menu {
        name: "Theme".into(),
        items: themes
            .iter()
            .map(|theme| MenuItem::action(theme.name.clone(), SwitchTheme(theme.name.clone())))
            .collect(),
    })
}
