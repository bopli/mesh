use gpui::{prelude::*, *};
use gpui_component::{
    ActiveTheme as _, h_flex,
    input::{InputEvent, InputState},
    resizable::{h_resizable, resizable_panel},
    v_flex,
};
use gpui_component_assets::Assets;
// use gpui_component_story::*;

pub struct Gallery {
    // stories: Vec<(&'static str, Vec<Entity<StoryContainer>>)>,
    active_group_index: Option<usize>,
    active_index: Option<usize>,
    // collapsed: bool,
    search_input: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
}

impl Gallery {
    pub fn new(init_story: Option<&str>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder("Search..."));
        let _subscriptions = vec![cx.subscribe(&search_input, |this, _, e, cx| match e {
            InputEvent::Change => {
                this.active_group_index = Some(0);
                this.active_index = Some(0);
                cx.notify()
            }
            _ => {}
        })];

        let mut this = Self {
            search_input,
            // stories,
            active_group_index: Some(0),
            active_index: Some(0),
            // collapsed: false,
            _subscriptions,
        };

        if let Some(init_story) = init_story {
            this.set_active_story(init_story, window, cx);
        }

        this
    }

    fn set_active_story(&mut self, name: &str, window: &mut Window, cx: &mut App) {
        let name = name.to_string();
        self.search_input.update(cx, |this, cx| {
            this.set_value(&name, window, cx);
        })
    }

    fn view(init_story: Option<&str>, window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(init_story, window, cx))
    }
}

impl Render for Gallery {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // let query = self.search_input.read(cx).value().trim().to_lowercase();

        h_resizable("mesh")
            .child(
                resizable_panel()
                    .size(px(255.))
                    .size_range(px(200.)..px(320.)),
            )
            .child(
                v_flex()
                    .flex_1()
                    .h_full()
                    .overflow_x_hidden()
                    .child(
                        h_flex()
                            .id("header")
                            .p_4()
                            .border_b_1()
                            .border_color(cx.theme().border)
                            .justify_between()
                            .items_start(), // .child(
                                            //     v_flex()
                                            //         .gap_1()
                                            //         .child(div().text_xl().child(story_name))
                                            //         .child(
                                            //             div()
                                            //                 .text_color(cx.theme().muted_foreground)
                                            //                 .child(description),
                                            //         ),
                                            // ),
                    )
                    .child(
                        div().id("story").flex_1().overflow_y_scroll(), // .when_some(active_story, |this, active_story| {
                                                                        //     this.child(active_story.clone())
                                                                        // }),
                    )
                    .into_any_element(),
            )
    }
}

fn main() {
    let app = Application::new().with_assets(Assets);

    // Parse `cargo run -- <story_name>`
    let name = std::env::args().nth(1);

    app.run(move |cx| {
        mesh::init(cx);

        cx.activate(true);

        mesh::create_new_window(
            "Mesh",
            None,
            move |window, cx| Gallery::view(name.as_deref(), window, cx),
            cx,
        );
    });
}
