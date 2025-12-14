use std::rc::Rc;

use gpui::{
    AnyElement, App, Context, Entity, InteractiveElement as _, IntoElement, MouseButton,
    ParentElement as _, Render, SharedString, Styled as _, Subscription, Window, div,
};
use gpui_component::{
    IconName, Sizable as _, Theme, TitleBar,
    button::{Button, ButtonVariants as _},
    menu::AppMenuBar,
};

use crate::{app_menus, themes::switch_theme_mode};

pub struct MeshTitleBar {
    app_menu_bar: Entity<AppMenuBar>,
    // font_size_selector: Entity<FontSizeSelector>,
    child: Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>,
    _subscriptions: Vec<Subscription>,
}

impl MeshTitleBar {
    pub fn new(
        title: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        app_menus::init(title, cx);

        // let font_size_selector = cx.new(|cx| FontSizeSelector::new(window, cx));
        let app_menu_bar = AppMenuBar::new(window, cx);

        Self {
            app_menu_bar,
            // font_size_selector,
            child: Rc::new(|_, _| div().into_any_element()),
            _subscriptions: vec![],
        }
    }

    pub fn child<F, E>(mut self, f: F) -> Self
    where
        E: IntoElement,
        F: Fn(&mut Window, &mut App) -> E + 'static,
    {
        self.child = Rc::new(move |window, cx| f(window, cx).into_any_element());
        self
    }
}

impl Render for MeshTitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        TitleBar::new()
            // left side
            .child(div().flex().items_center().child(self.app_menu_bar.clone()))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .px_2()
                    .gap_2()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .child((self.child.clone())(window, cx))
                    // .child(self.font_size_selector.clone())
                    .child(
                        Button::new("switch-theme-mode")
                            .icon(if Theme::global(cx).is_dark() {
                                IconName::Moon
                            } else {
                                IconName::Sun
                            })
                            .small()
                            .ghost()
                            .on_click(|_, _, cx| switch_theme_mode(cx)),
                    ), // .child(
                       //     div().relative().child(
                       //         Badge::new().count(notifications_count).max(99).child(
                       //             Button::new("bell")
                       //                 .small()
                       //                 .ghost()
                       //                 .compact()
                       //                 .icon(IconName::Bell),
                       //         ),
                       //     ),
                       // ),
            )
    }
}
