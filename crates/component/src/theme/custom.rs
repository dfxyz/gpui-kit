use std::str::FromStr;

use gpui::Hsla;

fn parse_oklch_expr(expr: &str) -> Hsla {
    let parts = expr.split('_').collect::<Vec<&str>>();
    assert!(parts.len() >= 4);

    let l = f32::from_str(parts[1]).unwrap() / 100.;
    let c = if parts[2] == "0" {
        0.
    } else {
        f32::from_str(&format!("0.{}", parts[2])).unwrap()
    };
    let h = f32::from_str(parts[3]).unwrap();

    let color = super::oklch(l, c, h);
    if parts.len() == 4 {
        color
    } else {
        color.alpha(f32::from_str(parts[4]).unwrap() / 100.)
    }
}

macro_rules! custom_colors {
    ($($expr:ident, [$($field:ident),* $(,)?], [$($getter:ident),* $(,)?] $(,)?),*) => {
        #[derive(Debug, Clone)]
        pub struct CustomColors {
            $(
                $expr: Hsla,
            )*
        }

        impl Default for CustomColors {
            fn default() -> Self {
                Self {
                    $(
                        $expr: parse_oklch_expr(stringify!($expr)),
                    )*
                }
            }
        }

        impl super::Theme {
            pub(crate) fn apply_custom_colors(&mut self) {
                $(
                    $(
                        self.colors.$field = self.custom_colors.$expr;
                    )*
                )*
                self.tokens = super::ThemeTokens::from(self.colors);
            }

            $(
                $(
                    #[inline]
                    pub fn $getter(&self) -> Hsla {
                        self.custom_colors.$expr
                    }
                )*
            )*
        }
    };
}

custom_colors! {
    _30_0_0, [
        foreground,
        accent_foreground,
        button_foreground,
        button_secondary_foreground,
        description_list_label_foreground,
        group_box_foreground,
        secondary_foreground,
        sidebar_foreground,
        sidebar_accent_foreground,
        table_head_foreground,
        table_foot_foreground,
        tab_foreground,
        tab_active_foreground,
    ], [],
    _60_0_0, [
        muted_foreground,
    ], [],

    _97_0_0, [
        primary_foreground,
        button_primary_foreground,
    ], [
        input_component,
    ],
    _97_005_120, [
        button,
        popover,
        slider_thumb,
        switch_thumb,
    ], [],
    _97_05_120, [
        button_hover,
    ], [],
    _95_005_120, [
        secondary,
        button_secondary,
        list_even,
        sidebar,
        table_even,
        tab_bar,
        tab_bar_segmented,
    ], [],
    _95_01_120, [
        accordion,
        background,
        list,
        table,
        tab_active,
    ], [],
    _95_025_120, [
        button_secondary_hover,
    ], [],
    _95_05_135, [
        button_active,
    ], [],
    _92_01_120, [
        button_secondary_active,
        group_box,
        muted,
        skeleton,
        table_foot,
    ], [],
    _92_05_120, [
        list_hover,
        secondary_hover,
        table_hover,
    ], [
        button_ghost_hover,
    ],
    _92_025_135, [
        list_head,
        table_head,
        description_list_label,
    ], [],
    _90_025_135, [
        title_bar,
        status_bar,
    ], [],
    _90_1_135, [
        accent,
        list_active,
        list_active_border,
        secondary_active,
        selection,
        sidebar_accent,
        table_active,
        table_active_border,
    ], [
        button_ghost_active,
    ],

    _40_135_150, [], [
        border_button_primary,
    ],
    _45_135_150, [
        button_primary_active,
    ], [],
    _50_135_150, [
        caret,
        chart_5,
        drag_border,
        primary,
        button_primary,
        slider_bar,
        progress_bar,
    ], [
        button_text_active,
    ],
    _50_135_150_25, [
        drop_target,
        scrollbar_thumb,
    ], [
        scrollbar_thumb_active,
    ],
    _50_135_150_50, [
        scrollbar_thumb_hover,
    ], [],
    _55_135_150, [
        button_primary_hover,
        chart_4,
    ], [],
    _60_135_150, [
        chart_3,
    ], [],
    _65_135_150, [
        chart_2,
    ], [],
    _70_135_150, [
        chart_1,
    ], [],
    _75_135_150, [
        ring,
    ], [],

    _85_0_0, [
        border,
        input,
        table_row_border,
        title_bar_border,
        sidebar_border,
        status_bar_border,
        switch,
    ], [],

    _50_2_270, [
        link,
    ], [],
    _60_2_270, [
        link_hover,
    ], [],
    _60_2_300, [
        link_active,
    ], [],

    _50_1_30, [
    ], [],
    _50_2_30, [
        button_danger_foreground,
        danger_foreground,
    ], [
        button_close_active,
    ],
    _60_2_30, [], [
        button_close_hover,
    ],
    _85_1_30, [], [
        border_button_danger,
        border_danger,
    ],
    _92_1_30, [
        button_danger_active,
    ], [],
    _95_1_30, [
        button_danger,
    ], [],
    _95_05_30, [
        button_danger_hover,
    ], [],
    _95_025_30, [
        danger,
    ], [],
    _97_025_30, [], [
        notification_danger,
    ],

    _50_1_90, [
        button_warning_foreground,
        warning_foreground,
    ], [],
    _85_05_90, [], [
        border_button_warning,
        border_warning,
    ],
    _92_05_90, [
        button_warning_active,
    ], [],
    _95_05_90, [
        button_warning,
    ], [],
    _95_025_90, [
        button_warning_hover,
        warning,
    ], [],
    _97_01_90, [], [
        notification_warning,
    ],

    _50_1_150, [
        button_success_foreground,
        success_foreground,
    ], [],
    _85_05_150, [], [
        border_button_success,
        border_success,
    ],
    _92_05_150, [
        button_success_active,
    ], [],
    _95_05_150, [
        button_success,
    ], [],
    _95_025_150, [
        button_success_hover,
        success,
    ], [
        inline_code,
    ],
    _95_01_150, [], [
        code_block,
    ],
    _97_01_150, [], [
        notification_success,
    ],

    _50_08_210, [
        button_info_foreground,
        info_foreground,
    ], [],
    _85_04_210, [], [
        border_button_info,
        border_info,
    ],
    _92_04_210, [
        button_info_active,
    ], [],
    _95_04_210, [
        button_info,
    ], [],
    _95_02_210, [
        button_info_hover,
        info,
    ], [],
    _97_01_210, [], [
        notification_info,
    ],
}
