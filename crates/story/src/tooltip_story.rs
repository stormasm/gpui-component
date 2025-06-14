use gpui::{
    actions, div, prelude::FluentBuilder, px, App, AppContext, Context, Entity, Focusable, Hsla,
    InteractiveElement, IntoElement, KeyBinding, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Window,
};

use gpui_component::{
    button::{Button, ButtonVariant, ButtonVariants},
    chart::BarChart,
    checkbox::Checkbox,
    divider::Divider,
    dock::PanelControl,
    h_flex,
    radio::Radio,
    switch::Switch,
    tooltip::Tooltip,
    v_flex, ActiveTheme, IconName, StyledExt,
};

use serde::Deserialize;

use crate::{section, Story};

actions!(tooltip, [Info]);

#[derive(Clone, Deserialize)]
struct MonthlyDevice {
    pub month: SharedString,
    pub desktop: f64,
    pub color: Hsla,
}

pub fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("ctrl-shift-delete", Info, Some("Tooltip"))]);
}

pub struct TooltipStory {
    focus_handle: gpui::FocusHandle,
    monthly_devices: Vec<MonthlyDevice>,
}

impl TooltipStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        let monthly_devices = serde_json::from_str::<Vec<MonthlyDevice>>(include_str!(
            "fixtures/monthly-devices.json"
        ))
        .unwrap();

        Self {
            focus_handle: cx.focus_handle(),
            monthly_devices,
        }
    }
}

impl Story for TooltipStory {
    fn title() -> &'static str {
        "Tooltip"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render + Focusable> {
        Self::view(window, cx)
    }

    fn zoomable() -> Option<PanelControl> {
        None
    }
}

impl Focusable for TooltipStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

fn chart_container(
    title: &str,
    chart: impl IntoElement,
    center: bool,
    cx: &mut Context<TooltipStory>,
) -> impl IntoElement {
    v_flex()
        .flex_1()
        .h_full()
        .border_1()
        .border_color(cx.theme().border)
        .rounded_lg()
        .p_4()
        .child(
            div()
                .when(center, |this| this.text_center())
                .font_semibold()
                .child(title.to_string()),
        )
        .child(
            div()
                .when(center, |this| this.text_center())
                .text_color(cx.theme().muted_foreground)
                .text_sm()
                .child("January-June 2025"),
        )
        .child(div().flex_1().py_4().child(chart))
        .child(
            div()
                .when(center, |this| this.text_center())
                .font_semibold()
                .text_sm()
                .child("Trending up by 5.2% this month"),
        )
        .child(
            div()
                .when(center, |this| this.text_center())
                .text_color(cx.theme().muted_foreground)
                .text_sm()
                .child("Showing total visitors for the last 6 months"),
        )
}

impl Render for TooltipStory {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex()
            .p_4()
            .gap_5()
            .child(
                section("Tooltip for Button")
                    .child(
                        Button::new("btn0")
                            .label("Search")
                            .with_variant(ButtonVariant::Primary)
                            .tooltip("This is a search Button."),
                    )
                    .child(Button::new("btn1").label("Info").tooltip_with_action(
                        "This is a tooltip with Action for display keybinding.",
                        &Info,
                        Some("Tooltip"),
                    ))
                    .child(
                        div()
                            .child(Button::new("btn3").label("Hover me"))
                            .id("tooltip-4")
                            .tooltip(|window, cx| {
                                Tooltip::element(|_, cx| {
                                    h_flex()
                                        .gap_x_1()
                                        .child(IconName::Info)
                                        .child(
                                            div()
                                                .child("Muted Foreground")
                                                .text_color(cx.theme().muted_foreground),
                                        )
                                        .child(div().child("Danger").text_color(cx.theme().danger))
                                        .child(IconName::ArrowUp)
                                })
                                .build(window, cx)
                            }),
                    ),
            )
            .child(
                section("Label Tooltip").child(div().child("Hover me").id("tooltip-2").tooltip(
                    |window, cx| {
                        Tooltip::new("This is a Label")
                            .action(&Info, Some("Tooltip"))
                            .build(window, cx)
                    },
                )),
            )
            .child(
                section("Checkbox Tooltip").child(
                    Checkbox::new("check")
                        .label("Remember me")
                        .checked(true)
                        .tooltip(|window, cx| Tooltip::new("This is a checkbox").build(window, cx)),
                ),
            )
            .child(
                section("Radio Tooltip").child(
                    Radio::new("radio")
                        .label("Radio with tooltip")
                        .checked(true)
                        .tooltip(|window, cx| {
                            Tooltip::new("This is a radio button").build(window, cx)
                        }),
                ),
            )
            .child(
                section("Switch Tooltip").child(
                    Switch::new("switch1")
                        .checked(true)
                        .tooltip("This is a switch 1"),
                ),
            )
            .child(
                section("Switch Tooltip").child(
                    Switch::new("switch2")
                        .checked(true)
                        .tooltip("This is a switch 2"),
                ),
            )
            .child(
                section("Switch Tooltip").child(
                    Switch::new("switch3")
                        .checked(true)
                        .tooltip("This is a switch 3"),
                ),
            )
            .child(Divider::horizontal())
            .child(
                h_flex()
                    .gap_x_8()
                    .h(px(400.))
                    .child(chart_container(
                        "Bar Chart",
                        BarChart::new(self.monthly_devices.clone())
                            .x(|d| d.month.clone())
                            .y(|d| d.desktop),
                        false,
                        cx,
                    ))
                    .child(chart_container(
                        "Bar Chart - Mixed",
                        BarChart::new(self.monthly_devices.clone())
                            .x(|d| d.month.clone())
                            .y(|d| d.desktop)
                            .fill(|d| d.color),
                        false,
                        cx,
                    ))
                    .child(chart_container(
                        "Bar Chart - Label",
                        BarChart::new(self.monthly_devices.clone())
                            .x(|d| d.month.clone())
                            .y(|d| d.desktop)
                            .label(|d| d.desktop.to_string()),
                        false,
                        cx,
                    )),
            )
            .child(
                section("Switch Tooltip").child(
                    Switch::new("switch4")
                        .checked(true)
                        .tooltip("This is a switch 4"),
                ),
            )
    }
}
