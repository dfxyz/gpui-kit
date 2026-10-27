use std::sync::LazyLock;

use gpui::Hsla;

use crate::{
    highlighter::{FontStyle, FontWeightContent, HighlightThemeStyle, ThemeStyle},
    theme::color::{oklab, oklch},
};

#[cfg(debug_assertions)]
use crate::highlighter::HighlightTheme;
#[cfg(debug_assertions)]
use crate::theme::Theme;

#[cfg(debug_assertions)]
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[cfg(debug_assertions)]
use gpui::App;

macro_rules! custom_style {
    ($($field:ident: $default:literal,)*) => {
        #[derive(Clone, Copy, Debug)]
        pub struct CustomStyle {
            $($field: Hsla,)*
        }

        #[cfg(debug_assertions)]
        impl CustomStyle {
            #[allow(unused_mut, unused_variables)]
            fn from_json_str(json: &str) -> Result<Self, String> {
                let entries: serde_json::Map<String, serde_json::Value> =
                    serde_json::from_str(json).map_err(|err| err.to_string())?;

                let mut style = Self::default();
                for (key, value) in &entries {
                    #[allow(clippy::match_single_binding)]
                    match key.as_str() {
                        $(
                        stringify!($field) => match value.as_str().map(parse_oklch) {
                            Some(Ok(color)) => style.$field = color,
                            Some(Err(err)) => tracing::warn!(
                                "颜色配置 `{key}` 无效：{err}，回退到默认值"
                            ),
                            None => tracing::warn!(
                                "颜色配置 `{key}` 不是字符串，回退到默认值"
                            ),
                        },
                        )*
                        _ => tracing::warn!("未知的颜色配置 `{key}`，已忽略"),
                    }
                }

                Ok(style)
            }
        }

        impl Default for CustomStyle {
            fn default() -> Self {
                Self {
                    $($field: parse_oklch($default).unwrap_or_else(|err| {
                        panic!(
                            "颜色配置 `{}` 的默认值无效：{err}",
                            stringify!($field)
                        )
                    }),)*
                }
            }
        }

        $(
        #[inline(always)]
        pub fn $field() -> Hsla {
            current().$field
        }
        )*
    };
}

custom_style! {
    base: "oklch(0.95 0.01 120)",
    base_alternative: "oklch(0.94 0.01 120)",
    inlined: "oklch(0.95 0.025 150)",
    primary: "oklch(0.5 0.135 150)",
    input: "oklch(0.97 0.01 120)",
    input_selected: "oklch(0.5 0.135 150)",
    muted_bg: "oklch(0.92 0.01 120)",
    selection: "oklch(0.5 0.135 150 / 0.2)",
    editor_muted_text: "oklch(0.75 0 0)",
    text: "oklch(0.3 0 0)",
    muted_text: "oklch(0.6 0 0)",
    reversed_text: "oklch(0.97 0.01 120)",
    red_text: "oklch(0.5 0.2 30)",
    orange_text: "oklch(0.5 0.14 45)",
    yellow_text: "oklch(0.5 0.1 90)",
    olive_text: "oklch(0.5 0.1 120)",
    green_text: "oklch(0.5 0.135 150)",
    cyan_text: "oklch(0.5 0.08 210)",
    blue_text: "oklch(0.5 0.2 270)",
    purple_text: "oklch(0.5 0.2 300)",
    magenta_text: "oklch(0.5 0.2 330)",
    caret: "oklch(0.5 0.135 150)",
    border: "oklch(0.75 0 0)",
    ring: "oklch(0.5 0.135 150)",
    popover: "oklch(0.97 0.01 120)",
    overlay: "oklch(0 0 0 / 0.2)",
    common_hover_bg: "oklch(0.5 0.135 150 / 0.1)",
    common_active_bg: "oklch(0.5 0.135 150 / 0.2)",
    title_bar: "oklch(0.9 0.025 135)",
    title_bar_close_button_hover: "oklch(0.6 0.2 30)",
    title_bar_close_button_active: "oklch(0.5 0.2 30)",
    status_bar: "oklch(0.9 0.025 135)",
    scrollbar_thumb: "oklch(0.5 0.135 150 / 0.25)",
    scrollbar_thumb_hover: "oklch(0.5 0.135 150 / 0.5)",
    scrollbar_thumb_active: "oklch(0.5 0.135 150 / 0.25)",
    button_hover_bg: "oklch(0.97 0.05 120)",
    button_active_bg: "oklch(0.95 0.05 120)",
    primary_button_border: "oklch(0.75 0.1 150)",
    primary_button_hover_bg: "oklch(0.97 0.025 150)",
    primary_button_active_bg: "oklch(0.95 0.05 150)",
    danger_button_border: "oklch(0.75 0.1 30)",
    danger_button_hover_bg: "oklch(0.97 0.01 30)",
    danger_button_active_bg: "oklch(0.95 0.025 30)",
    chart_1: "oklch(0.7 0.135 150)",
    chart_2: "oklch(0.65 0.135 150)",
    chart_3: "oklch(0.6 0.135 150)",
    chart_4: "oklch(0.55 0.135 150)",
    chart_5: "oklch(0.5 0.135 150)",
    chart_bullish: "oklch(0.6 0.2 30)",
    chart_bearish: "oklch(0.6 0.135 150)",
    drag_border: "oklch(0.5 0.135 150)",
    drag_target: "oklch(0.5 0.135 150 / 0.1)",
    table_head: "oklch(0.92 0.025 135)",
    table_foot: "oklch(0.92 0.01 135)",
}

#[inline(always)]
pub const fn transparent() -> Hsla {
    Hsla::transparent_black()
}

#[inline(always)]
pub const fn text_to_bg_alpha() -> f32 {
    0.05
}

#[inline(always)]
pub const fn text_to_border_alpha() -> f32 {
    0.5
}

#[cfg(debug_assertions)]
static CUSTOM_STYLE: LazyLock<arc_swap::ArcSwap<CustomStyle>> =
    LazyLock::new(|| arc_swap::ArcSwap::from_pointee(CustomStyle::default()));

#[cfg(not(debug_assertions))]
static CUSTOM_STYLE: LazyLock<CustomStyle> = LazyLock::new(CustomStyle::default);

#[cfg(debug_assertions)]
#[inline(always)]
#[allow(dead_code)]
fn current() -> arc_swap::Guard<Arc<CustomStyle>> {
    CUSTOM_STYLE.load()
}

#[cfg(not(debug_assertions))]
#[inline(always)]
#[allow(dead_code)]
fn current() -> &'static CustomStyle {
    &CUSTOM_STYLE
}

/// 解析 `oklch(L C H)` 或 `oklch(L C H / A)` 形式的颜色表达式
#[allow(dead_code)]
fn parse_oklch(expr: &str) -> Result<Hsla, String> {
    let expr = expr.trim();
    if expr.is_empty() {
        return Ok(Hsla::default());
    }

    let inner = expr
        .strip_prefix("oklch(")
        .and_then(|rest| rest.strip_suffix(')'))
        .ok_or_else(|| format!("不是有效的 oklch 表达式：`{expr}`"))?;
    let (channels, alpha) = match inner.split_once('/') {
        Some((channels, alpha)) => (channels, Some(alpha)),
        None => (inner, None),
    };

    let channels = channels.split_whitespace().collect::<Vec<_>>();
    if channels.len() != 3 {
        return Err(format!(
            "需要 L、C、H 等 3 个通道，实际只有 {} 个",
            channels.len()
        ));
    }

    let lightness = parse_number(channels[0], expr)?;
    let chroma = parse_number(channels[1], expr)?;
    let hue = parse_number(channels[2], expr)?;
    let alpha = match alpha {
        Some(alpha) => parse_number(alpha, expr)?,
        None => 1.0,
    };

    let radians = hue.to_radians();
    let rgb =
        oklab::oklab_to_rgb_unclamped(lightness, chroma * radians.cos(), chroma * radians.sin());
    if [rgb.r, rgb.g, rgb.b]
        .iter()
        .any(|channel| !(0.0..=1.0).contains(channel))
    {
        tracing::warn!("`{expr}` 超出 sRGB 色域，颜色会被裁剪");
    }

    let mut color = oklch(lightness, chroma, hue);
    color.a = alpha;

    Ok(color)
}

fn parse_number(value: &str, expr: &str) -> Result<f32, String> {
    value
        .trim()
        .parse::<f32>()
        .map_err(|_| format!("`{value}` 不是有效的数字（`{expr}`）"))
}

#[cfg(debug_assertions)]
fn reload_from_json_str(json: &str) -> Result<(), String> {
    CUSTOM_STYLE.store(Arc::new(CustomStyle::from_json_str(json)?));
    Ok(())
}

#[cfg(debug_assertions)]
fn reload_from_json_file(path: &Path) -> Result<(), String> {
    let json = std::fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
    reload_from_json_str(&json)
}

pub fn custom_highlight_theme_style(mut style: HighlightThemeStyle) -> HighlightThemeStyle {
    style.editor_background = Some(base());
    style.editor_foreground = Some(text());
    style.editor_active_line = Some(common_hover_bg());
    style.editor_line_number = Some(editor_muted_text());
    style.editor_active_line_number = Some(text());
    style.editor_invisible = Some(editor_muted_text());
    style.editor_gutter_background = None;
    style.syntax.attribute = Some(ThemeStyle {
        color: Some(purple_text()),
        ..Default::default()
    });
    style.syntax.boolean = Some(ThemeStyle {
        color: Some(orange_text()),
        ..Default::default()
    });
    style.syntax.comment = Some(ThemeStyle {
        color: Some(olive_text()),
        ..Default::default()
    });
    style.syntax.comment_doc = Some(ThemeStyle {
        color: Some(olive_text()),
        ..Default::default()
    });
    style.syntax.constant = Some(ThemeStyle {
        color: Some(text()),
        ..Default::default()
    });
    style.syntax.constructor = Some(ThemeStyle {
        color: Some(blue_text()),
        ..Default::default()
    });
    style.syntax.embedded = Some(ThemeStyle {
        color: Some(text()),
        ..Default::default()
    });
    style.syntax.emphasis = Some(ThemeStyle {
        font_style: Some(FontStyle::Italic),
        ..Default::default()
    });
    style.syntax.emphasis_strong = Some(ThemeStyle {
        font_weight: Some(FontWeightContent::Bold),
        ..Default::default()
    });
    style.syntax.enum_ = Some(ThemeStyle {
        color: Some(blue_text()),
        ..Default::default()
    });
    style.syntax.function = Some(ThemeStyle {
        color: Some(text()),
        ..Default::default()
    });
    style.syntax.hint = Some(ThemeStyle {
        color: Some(muted_text()),
        ..Default::default()
    });
    style.syntax.keyword = Some(ThemeStyle {
        color: Some(green_text()),
        ..Default::default()
    });
    style.syntax.label = Some(ThemeStyle {
        color: Some(yellow_text()),
        ..Default::default()
    });
    style.syntax.link_text = Some(ThemeStyle {
        color: Some(blue_text()),
        ..Default::default()
    });
    style.syntax.link_uri = Some(ThemeStyle {
        color: Some(blue_text()),
        ..Default::default()
    });
    style.syntax.number = Some(ThemeStyle {
        color: Some(orange_text()),
        ..Default::default()
    });
    style.syntax.operator = Some(ThemeStyle {
        color: Some(text()),
        ..Default::default()
    });
    style.syntax.predictive = Some(ThemeStyle {
        color: Some(muted_text()),
        font_style: Some(FontStyle::Italic),
        ..Default::default()
    });
    style.syntax.preproc = Some(ThemeStyle {
        color: Some(yellow_text()),
        ..Default::default()
    });
    style.syntax.primary = Some(ThemeStyle {
        color: Some(text()),
        ..Default::default()
    });
    style.syntax.property = Some(ThemeStyle {
        color: Some(purple_text()),
        ..Default::default()
    });
    style.syntax.punctuation = Some(ThemeStyle {
        color: Some(text()),
        ..Default::default()
    });
    style.syntax.punctuation_bracket = Some(ThemeStyle {
        color: Some(text()),
        ..Default::default()
    });
    style.syntax.punctuation_delimiter = Some(ThemeStyle {
        color: Some(text()),
        ..Default::default()
    });
    style.syntax.punctuation_list_marker = Some(ThemeStyle {
        color: Some(green_text()),
        ..Default::default()
    });
    style.syntax.punctuation_special = Some(ThemeStyle {
        color: Some(orange_text()),
        ..Default::default()
    });
    style.syntax.string = Some(ThemeStyle {
        color: Some(cyan_text()),
        ..Default::default()
    });
    style.syntax.string_escape = Some(ThemeStyle {
        color: Some(orange_text()),
        ..Default::default()
    });
    style.syntax.string_regex = Some(ThemeStyle {
        color: Some(cyan_text()),
        ..Default::default()
    });
    style.syntax.string_special = Some(ThemeStyle {
        color: Some(orange_text()),
        ..Default::default()
    });
    style.syntax.string_special_symbol = Some(ThemeStyle {
        color: Some(orange_text()),
        ..Default::default()
    });
    style.syntax.tag = Some(ThemeStyle {
        color: Some(green_text()),
        ..Default::default()
    });
    style.syntax.tag_doctype = Some(ThemeStyle {
        color: Some(yellow_text()),
        ..Default::default()
    });
    style.syntax.text_code_span = Some(ThemeStyle {
        color: Some(cyan_text()),
        ..Default::default()
    });
    style.syntax.text_literal = Some(ThemeStyle {
        color: Some(cyan_text()),
        ..Default::default()
    });
    style.syntax.title = Some(ThemeStyle {
        color: Some(green_text()),
        ..Default::default()
    });
    style.syntax.type_ = Some(ThemeStyle {
        color: Some(blue_text()),
        ..Default::default()
    });
    style.syntax.variable = Some(ThemeStyle {
        color: Some(text()),
        ..Default::default()
    });
    style.syntax.variable_special = Some(ThemeStyle {
        color: Some(green_text()),
        ..Default::default()
    });
    style.syntax.variant = Some(ThemeStyle {
        color: Some(magenta_text()),
        ..Default::default()
    });
    style
}

#[cfg(debug_assertions)]
fn sync_highlight_theme(cx: &mut App) {
    let config = {
        let theme = Theme::global(cx);
        if theme.mode.is_dark() {
            theme.dark_theme.clone()
        } else {
            theme.light_theme.clone()
        }
    };
    if let Some(style) = config.highlight.clone() {
        let highlight_theme = Arc::new(HighlightTheme {
            name: config.name.to_string(),
            appearance: config.mode,
            style: custom_highlight_theme_style(style),
        });
        Theme::global_mut(cx).highlight_theme = highlight_theme;
    }
}

/// 加载 JSON 配置文件并监听它的变更；变更时重新加载并刷新窗口
#[cfg(all(debug_assertions, not(target_family = "wasm")))]
pub fn watch(path: impl Into<PathBuf>, cx: &mut App) {
    use notify::Watcher as _;

    let path = path.into();
    match reload_from_json_file(&path) {
        Ok(()) => tracing::info!("已加载自定义颜色配置"),
        Err(err) => tracing::warn!("加载自定义颜色配置时发生错误：{err}"),
    }
    sync_highlight_theme(cx);
    Theme::sync_base(cx);
    cx.refresh_windows();

    let dir = match path.parent() {
        Some(dir) if !dir.as_os_str().is_empty() => dir.to_path_buf(),
        _ => match std::env::current_dir() {
            Ok(dir) => dir,
            Err(err) => {
                tracing::error!("无法开启自定义颜色配置的热更：{err}");
                return;
            }
        },
    };

    let (tx, rx) = smol::channel::bounded(1);
    let watched = path.clone();
    let mut watcher = match notify::RecommendedWatcher::new(
        move |res: notify::Result<notify::Event>| match res {
            Ok(event) => {
                if event.need_rescan() || event.paths.iter().any(|touched| touched == &watched) {
                    let _ = tx.try_send(());
                }
            }
            Err(err) => tracing::error!("接收自定义颜色配置变更事件失败：{err}"),
        },
        notify::Config::default().with_follow_symlinks(false),
    ) {
        Ok(watcher) => watcher,
        Err(err) => {
            tracing::error!("无法开启自定义颜色配置的热更：{err}");
            return;
        }
    };

    cx.spawn(async move |cx| {
        if let Err(err) = watcher.watch(&dir, notify::RecursiveMode::NonRecursive) {
            tracing::error!("监听 {} 失败：{err}", dir.display());
        }

        while rx.recv().await.is_ok() {
            match reload_from_json_file(&path) {
                Ok(()) => tracing::info!("已热更自定义颜色配置"),
                Err(err) => tracing::error!("热更自定义颜色配置时发生错误：{err}"),
            }
            _ = cx.update(|cx| {
                sync_highlight_theme(cx);
                Theme::sync_base(cx);
                cx.refresh_windows();
            });
        }
    })
    .detach();
}
