use super::InlineTokenContext;
use crate::{ActiveTheme as _, Icon, StyledExt as _};
use gpui::{
    App, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce, StyleRefinement,
    Styled, Window, div, prelude::FluentBuilder as _,
};

/// The element an [`InlineToken`](super::InlineToken) renders as by default. Editing
/// and activation belong to the input.
#[derive(IntoElement)]
pub struct InputToken {
    context: InlineTokenContext,
    icon: Option<Icon>,
    style: StyleRefinement,
}
impl InputToken {
    pub fn new(context: &InlineTokenContext) -> Self {
        Self {
            context: context.clone(),
            icon: None,
            style: Default::default(),
        }
    }
    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}
impl Styled for InputToken {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for InputToken {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id("input-token")
            .flex()
            .items_center()
            .gap_1()
            .px_1()
            .h(self.context.line_height())
            .max_w(self.context.available_width())
            .rounded(cx.theme().radius)
            .border_1()
            .map(|this| {
                // The selection color is translucent as a fill; its opaque form
                // is the matching border.
                if self.context.is_selected() {
                    this.bg(crate::custom_style::selection())
                        .border_color(crate::custom_style::selection())
                } else {
                    this.bg(crate::custom_style::muted_bg())
                        .border_color(crate::custom_style::border())
                }
            })
            .text_color(crate::custom_style::text())
            .when(self.context.is_disabled(), |this| this.opacity(0.5))
            .when_some(self.icon, |this, icon| {
                this.child(icon.size_3().flex_shrink_0())
            })
            .child(
                div()
                    .min_w_0()
                    .text_ellipsis()
                    .child(self.context.token().label().clone()),
            )
            .refine_style(&self.style)
    }
}
