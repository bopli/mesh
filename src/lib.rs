use gpui::{
    AnyView, App, AppContext, Bounds, Context, Entity, Global, IntoElement, KeyBinding,
    ParentElement, Pixels, Render, SharedString, Size, Styled, Window, WindowBounds, WindowKind,
    WindowOptions, actions, div, px, size,
};
use gpui_component::{Root, TitleBar, v_flex};

mod app_menus;
mod themes;
mod title_bar;

pub use crate::title_bar::MeshTitleBar;

actions!(mesh, [About, Open, Quit, CloseWindow, ToggleSearch,]);

pub struct AppState {
    // pub invisible_panels: Entity<Vec<SharedString>>,
}
impl AppState {
    fn init(cx: &mut App) {
        let state = Self {};
        cx.set_global::<AppState>(state);
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<Self>()
    }
}
impl Global for AppState {}

pub fn create_new_window<F, E>(
    title: &str,
    window_size: Option<Size<Pixels>>,
    crate_view_fn: F,
    cx: &mut App,
) where
    E: Into<AnyView>,
    F: FnOnce(&mut Window, &mut App) -> E + Send + 'static,
{
    let mut window_size = window_size.unwrap_or(size(px(1600.0), px(1200.0)));
    if let Some(display) = cx.primary_display() {
        let display_size = display.bounds().size;
        window_size.width = window_size.width.min(display_size.width * 0.85);
        window_size.height = window_size.height.min(display_size.height * 0.85);
    }
    let window_bounds = Bounds::centered(None, window_size, cx);
    let title = SharedString::from(title.to_string());

    cx.spawn(async move |cx| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            titlebar: Some(TitleBar::title_bar_options()),
            window_min_size: Some(gpui::Size {
                width: px(480.),
                height: px(320.),
            }),
            kind: WindowKind::Normal,
            ..Default::default()
        };

        let window = cx
            .open_window(options, |window, cx| {
                let view = crate_view_fn(window, cx);
                let root = cx.new(|cx| StoryRoot::new(title.clone(), view, window, cx));

                cx.new(|cx| Root::new(root, window, cx))
            })
            .expect("failed to open window");

        window
            .update(cx, |_, window, _| {
                window.activate_window();
                window.set_window_title(&title);
            })
            .expect("failed to update window");

        Ok::<_, anyhow::Error>(())
    })
    .detach();
}

struct StoryRoot {
    title_bar: Entity<MeshTitleBar>,
    view: AnyView,
}

impl StoryRoot {
    pub fn new(
        title: impl Into<SharedString>,
        view: impl Into<AnyView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let title_bar = cx.new(|cx| MeshTitleBar::new(title, window, cx));
        Self {
            title_bar,
            view: view.into(),
        }
    }
}

impl Render for StoryRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        div()
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(self.title_bar.clone())
                    .child(div().flex_1().overflow_hidden().child(self.view.clone())),
            )
            .children(sheet_layer)
            .children(dialog_layer)
            .children(notification_layer)
    }
}

pub fn init(cx: &mut App) {
    env_logger::init_from_env(env_logger::Env::new().filter("MESH_LOG"));

    gpui_component::init(cx);
    AppState::init(cx);
    themes::init(cx);
    // stories::init(cx);

    // let http_client = std::sync::Arc::new(
    //     reqwest_client::ReqwestClient::user_agent("gpui-component/story").unwrap(),
    // );
    // cx.set_http_client(http_client);

    cx.bind_keys([
        KeyBinding::new("/", ToggleSearch, None),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-o", Open, None),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("alt-f4", Quit, None),
    ]);

    cx.on_action(|_: &Quit, cx: &mut App| {
        cx.quit();
    });

    // register_panel(cx, PANEL_NAME, |_, _, info, window, cx| {
    //     let story_state = match info {
    //         PanelInfo::Panel(value) => StoryState::from_value(value.clone()),
    //         _ => {
    //             unreachable!("Invalid PanelInfo: {:?}", info)
    //         }
    //     };

    //     let view = cx.new(|cx| {
    //         let (title, description, closable, zoomable, story, on_active) =
    //             story_state.to_story(window, cx);
    //         let mut container = StoryContainer::new(window, cx)
    //             .story(story, story_state.story_klass)
    //             .on_active(on_active);

    //         cx.on_focus_in(
    //             &container.focus_handle,
    //             window,
    //             |this: &mut StoryContainer, _, _| {
    //                 println!("StoryContainer focus in: {}", this.name);
    //             },
    //         )
    //         .detach();

    //         container.name = title.into();
    //         container.description = description.into();
    //         container.closable = closable;
    //         container.zoomable = zoomable;
    //         container
    //     });
    //     Box::new(view)
    // });

    cx.activate(true);
}

// #[derive(IntoElement)]
// struct StorySection {
//     base: Div,
//     title: SharedString,
//     sub_title: Vec<AnyElement>,
//     children: Vec<AnyElement>,
// }

// impl StorySection {
//     // pub fn sub_title(mut self, sub_title: impl IntoElement) -> Self {
//     //     self.sub_title.push(sub_title.into_any_element());
//     //     self
//     // }

//     #[allow(unused)]
//     fn max_w_md(mut self) -> Self {
//         self.base = self.base.max_w(rems(48.));
//         self
//     }

//     #[allow(unused)]
//     fn max_w_lg(mut self) -> Self {
//         self.base = self.base.max_w(rems(64.));
//         self
//     }

//     #[allow(unused)]
//     fn max_w_xl(mut self) -> Self {
//         self.base = self.base.max_w(rems(80.));
//         self
//     }

//     #[allow(unused)]
//     fn max_w_2xl(mut self) -> Self {
//         self.base = self.base.max_w(rems(96.));
//         self
//     }
// }

// impl ParentElement for StorySection {
//     fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
//         self.children.extend(elements);
//     }
// }

// impl Styled for StorySection {
//     fn style(&mut self) -> &mut gpui::StyleRefinement {
//         self.base.style()
//     }
// }

// impl RenderOnce for StorySection {
//     fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
//         GroupBox::new()
//             .id(self.title.clone())
//             .outline()
//             .title(
//                 h_flex()
//                     .justify_between()
//                     .w_full()
//                     .gap_4()
//                     .child(self.title)
//                     .children(self.sub_title),
//             )
//             .content_style(
//                 StyleRefinement::default()
//                     .rounded(cx.theme().radius_lg)
//                     .overflow_x_hidden()
//                     .items_center()
//                     .justify_center(),
//             )
//             .child(self.base.children(self.children))
//     }
// }
