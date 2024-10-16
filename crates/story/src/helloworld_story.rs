use gpui::{
    div, CursorStyle, InteractiveElement, ParentElement, Render, StatefulInteractiveElement,
    Styled, View, ViewContext, VisualContext as _, WindowContext,
};

use ui::{
    button::{Button, ButtonStyle},
    checkbox::Checkbox,
    h_flex,
    label::Label,
    tooltip::Tooltip,
    v_flex,
};

pub struct HwStory {
    focus_handle: gpui::FocusHandle,
}

impl HwStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(Self::new)
    }

    fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl super::Story for HwStory {
    fn title() -> &'static str {
        "Hw"
    }

    fn new_view(cx: &mut WindowContext) -> View<impl gpui::FocusableView> {
        Self::view(cx)
    }

    fn zoomable() -> bool {
        false
    }
}
impl gpui::FocusableView for HwStory {
    fn focus_handle(&self, _: &gpui::AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl Render for HwStory {
    fn render(&mut self, _: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        v_flex()
            .p_4()
            .gap_5()
            .child(
                div()
                    .cursor(CursorStyle::PointingHand)
                    .child(
                        Button::new("button")
                            .label("Hover over me Hello World")
                            .style(ButtonStyle::Primary),
                    )
                    .id("tooltip-1")
                    .tooltip(|cx| Tooltip::new("This is a Button", cx)),
            )
            .child(
                h_flex()
                    .justify_center()
                    .cursor(CursorStyle::PointingHand)
                    .child(Label::new("Hover near me Hello World"))
                    .id("tooltip-3")
                    .tooltip(|cx| Tooltip::new("This is a Label", cx)),
            )
            .child(
                div()
                    .cursor(CursorStyle::PointingHand)
                    .child(
                        Checkbox::new("check")
                            .label("Remember me or not")
                            .checked(true),
                    )
                    .id("tooltip-4")
                    .tooltip(|cx| Tooltip::new("Roger Houston Check Two!", cx)),
            )
    }
}
