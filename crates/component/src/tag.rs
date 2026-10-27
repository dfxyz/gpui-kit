use crate::{ColorName, Sizable, Size, StyledExt, theme::ActiveTheme as _};
use gpui::{
    AbsoluteLength, AnyElement, App, Hsla, IntoElement, ParentElement, RenderOnce, StyleRefinement,
    Styled, Window, div, prelude::FluentBuilder as _, relative, rems, transparent_white,
};

/// The variant of the Tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagVariant {
    Primary,
    #[default]
    Secondary,
    Danger,
    Success,
    Warning,
    Info,
    Color(ColorName),
    Custom {
        color: Hsla,
        foreground: Hsla,
        border: Hsla,
    },
}

impl TagVariant {
    fn bg(&self, cx: &App) -> Hsla {
        match self {
            Self::Primary => crate::custom_style::primary(),
            Self::Secondary => crate::custom_style::base_alternative(),
            Self::Danger => {
                crate::custom_style::red_text().opacity(crate::custom_style::text_to_bg_alpha())
            }
            Self::Success => {
                crate::custom_style::green_text().opacity(crate::custom_style::text_to_bg_alpha())
            }
            Self::Warning => {
                crate::custom_style::yellow_text().opacity(crate::custom_style::text_to_bg_alpha())
            }
            Self::Info => {
                crate::custom_style::cyan_text().opacity(crate::custom_style::text_to_bg_alpha())
            }
            Self::Color(color) => {
                if cx.theme().is_dark() {
                    color.scale(950).opacity(0.5)
                } else {
                    color.scale(50)
                }
            }
            Self::Custom { color, .. } => *color,
        }
    }

    fn border(&self, cx: &App) -> Hsla {
        match self {
            Self::Primary => crate::custom_style::primary(),
            Self::Secondary => crate::custom_style::border(),
            Self::Danger => {
                crate::custom_style::red_text().opacity(crate::custom_style::text_to_border_alpha())
            }
            Self::Success => crate::custom_style::green_text()
                .opacity(crate::custom_style::text_to_border_alpha()),
            Self::Warning => crate::custom_style::yellow_text()
                .opacity(crate::custom_style::text_to_border_alpha()),
            Self::Info => crate::custom_style::cyan_text()
                .opacity(crate::custom_style::text_to_border_alpha()),
            Self::Color(color) => {
                if cx.theme().is_dark() {
                    color.scale(800).opacity(0.5)
                } else {
                    color.scale(200)
                }
            }
            Self::Custom { border, .. } => *border,
        }
    }

    fn fg(&self, outline: bool, cx: &App) -> Hsla {
        match self {
            Self::Primary => {
                if outline {
                    crate::custom_style::primary()
                } else {
                    crate::custom_style::reversed_text()
                }
            }
            Self::Secondary => crate::custom_style::text(),
            Self::Danger => crate::custom_style::red_text(),
            Self::Success => crate::custom_style::green_text(),
            Self::Warning => crate::custom_style::yellow_text(),
            Self::Info => crate::custom_style::cyan_text(),
            Self::Color(color) => {
                if cx.theme().is_dark() {
                    color.scale(300)
                } else {
                    color.scale(600)
                }
            }
            Self::Custom { foreground, .. } => *foreground,
        }
    }
}

/// Tag is a small status indicator.
///
/// Only support: Medium, Small
#[derive(IntoElement)]
pub struct Tag {
    style: StyleRefinement,
    variant: TagVariant,
    outline: bool,
    size: Size,
    rounded: Option<AbsoluteLength>,
    children: Vec<AnyElement>,
}
impl Tag {
    /// Create a new Tag.
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            variant: TagVariant::default(),
            outline: false,
            size: Size::default(),
            rounded: None,
            children: Vec::new(),
        }
    }

    /// Create a new tag with default variant ([`TagVariant::Primary`]).
    pub fn primary() -> Self {
        Self::new().with_variant(TagVariant::Primary)
    }

    /// Create a new tag with default variant ([`TagVariant::Secondary`]).
    pub fn secondary() -> Self {
        Self::new().with_variant(TagVariant::Secondary)
    }

    /// Create a new tag with default variant ([`TagVariant::Danger`]).
    pub fn danger() -> Self {
        Self::new().with_variant(TagVariant::Danger)
    }

    /// Create a new tag with default variant ([`TagVariant::Success`]).
    pub fn success() -> Self {
        Self::new().with_variant(TagVariant::Success)
    }

    /// Create a new tag with default variant ([`TagVariant::Warning`]).
    pub fn warning() -> Self {
        Self::new().with_variant(TagVariant::Warning)
    }

    /// Create a new tag with default variant ([`TagVariant::Info`]).
    pub fn info() -> Self {
        Self::new().with_variant(TagVariant::Info)
    }

    /// Create a new tag with default variant ([`TagVariant::Custom`]).
    pub fn custom(color: Hsla, foreground: Hsla, border: Hsla) -> Self {
        Self::new().with_variant(TagVariant::Custom {
            color,
            foreground,
            border,
        })
    }

    /// Create a new tag with default variant ([`TagVariant::Color`]).
    pub fn color(color: impl Into<ColorName>) -> Self {
        Self::new().with_variant(TagVariant::Color(color.into()))
    }

    /// Set the variant of the Tag.
    pub fn with_variant(mut self, variant: TagVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Use outline style
    pub fn outline(mut self) -> Self {
        self.outline = true;
        self
    }

    /// Set rounded corners.
    pub fn rounded(mut self, radius: impl Into<AbsoluteLength>) -> Self {
        self.rounded = Some(radius.into());
        self
    }

    /// Set rounded full
    pub fn rounded_full(mut self) -> Self {
        self.rounded = Some(rems(1.).into());
        self
    }
}

impl Sizable for Tag {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl ParentElement for Tag {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Tag {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Tag {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let bg = if self.outline {
            transparent_white()
        } else {
            self.variant.bg(cx)
        };
        let fg = self.variant.fg(self.outline, cx);
        let border = self.variant.border(cx);
        let rounded = self.rounded.unwrap_or(
            match self.size {
                Size::XSmall | Size::Small => cx.theme().radius / 2.,
                _ => cx.theme().radius,
            }
            .into(),
        );

        div()
            .flex()
            .items_center()
            .border_1()
            .line_height(relative(1.25))
            .map(|this| match self.size {
                Size::XSmall | Size::Small => this.text_size(rems(0.625)),
                Size::Large => this.text_sm(),
                _ => this.text_xs(),
            })
            .map(|this| match self.size {
                Size::XSmall | Size::Small => this.px_1p5().py_0p5(),
                _ => this.px_2p5().py_1(),
            })
            .bg(bg)
            .text_color(fg)
            .border_color(border)
            .rounded(rounded)
            .refine_style(&self.style)
            .children(self.children)
    }
}
