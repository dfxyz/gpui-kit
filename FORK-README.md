# FORK README

本文件面向后续升级此 fork 的 AI Agent。先读完本文再动代码。

核心原则只有一句：**保留上游的新结构和行为，只把它的颜色重新套成本 fork 的固定浅色样式。**

执行摘要：按第 2 节 cherry-pick；按第 5 节替换冲突文件里的颜色；按第 6 节审计新组件；按第 9 节验证。

## 1. 基线与提交

- 上游基线：`v0.9.0`，父提交 `7591a35e`。
- fork 提交：`fork/v0.9.0` 分支 tip，提交信息 `FORK：自定义组件样式`。
- 当前分支：`fork/v0.9.0`。
- 规模：183 个文件，`+2242/-2083`，主要集中在 `crates/component`。
- 范围：
  - `crates/component/src/custom_style.rs`：新增的颜色中心。
  - `crates/component/**`：大部分组件的颜色替换，以及少量排版/交互调整。
  - `crates/base/src/calendar.rs`：周一作为一周第一天、区间起止状态（供组件层画圆角）。
  - `crates/story/**`：演示程序的适配和 `custom_style` 热更新入口。
- 不变量：`radius`、`motion`、`font`、`spacing`、`shadow` 仍由上游 `Theme` 控制，不要搬进 `custom_style`。

## 2. 升级流程

```bash
git fetch upstream --tags

# 从上游标签创建新分支
git switch -c fork/<version> <upstream-tag>

# cherry-pick 上一版 fork 分支的 tip
git cherry-pick fork/<previous-version>
```

例如从 `v0.9.0` 升级到下一版：

```bash
git switch -c fork/v0.10.0 v0.10.0
git cherry-pick fork/v0.9.0
```

把命令中的版本换成实际目标；可用 `git tag --sort=-v:refname` 查看最新 tag。

如果找不到上一版分支：

```bash
git log --all --oneline --grep='FORK：自定义组件样式'
```

冲突处理完后：

```bash
git add <resolved-files>
git cherry-pick --continue
```

规则：

- **每个版本只保留一个 `FORK：...` 提交**，不要把冲突修复拆成多个提交。
- 下一次升级时，cherry-pick 上一版 `fork/<version>` 分支的 tip，而不是永远 cherry-pick 首个 fork 提交。这样 fork 补丁会随上游一起更新，冲突更少。
- 如果新分支已存在但还没有 fork 提交，且没有本地改动，可以用 `git reset --hard <upstream-tag>` 重置后再 cherry-pick。
- 不要直接跨多个大版本合并；按上游 tag 逐个升级。

## 3. 冲突处理

| 冲突类型 | 处理方式 |
| --- | --- |
| 纯颜色替换 | 先取上游代码，再按第 5 节重新替换。不要保留旧 fork 的整段颜色代码。 |
| 上游重构/重命名 | 以上游结构为准，只重放 fork 的视觉意图；不要恢复上游已删除的旧实现。 |
| fork 行为改动 | 见第 7 节。若上游也改了同一行为，先保留上游；只有用户明确要求时才恢复 fork 行为。 |
| `cx` 改成 `_cx` | 这是消 unused 警告的噪音。优先保留上游签名，编译器提示时再加 `_`。 |
| `Cargo.lock` | 不要手工合并。清掉冲突标记后运行 `cargo check` 重新生成。 |
| 测试断言颜色 | 改成 `custom_style` 期望，或删除过时断言；不要为了测试通过把颜色改回 `Theme`。 |
| 新文件/新组件 | 不会出现在 cherry-pick 冲突里，但必须按第 6 节单独审计。 |

注意：cherry-pick 时 **`--ours` 是目标分支（上游新版），`--theirs` 是 fork 提交**。不要搞反。纯颜色文件可以 `git checkout --ours -- <file>` 取上游，再重新替换。

`crates/story` 的改色不影响库行为。story 冲突优先采用上游版本，只保留 `custom_style::watch`、必要的 `custom_style` 调用和用户明确要求的演示差异；不要让 story 的冲突阻塞 `crates/component` 的升级。

### 冲突判断顺序

1. 先看 `git show fork/<previous-version> -- <file>`，确认原 fork 在这个文件里到底改了什么。下文用“上一版 fork 提交”指代该分支的 tip。
2. 如果只是 `theme()` 到 `custom_style()` 的替换，直接取上游版本，按映射表重做。
3. 如果包含第 7 节的行为改动，逐段合并，不要整文件取舍。
4. 如果上游已经把同一段代码改得面目全非，以上游为准，只保留 fork 的语义：
   - 背景、文字、边框、状态色来自 `custom_style`。
   - hover/active 使用统一的 `common_hover_bg` / `common_active_bg`。
   - 状态色使用 `*_text()` + `text_to_bg_alpha()` / `text_to_border_alpha()`。

## 4. 颜色系统

唯一入口：`crates/component/src/custom_style.rs`。

- 所有颜色 token 都是无参数函数：`custom_style::text()`、`custom_style::border()`。
- 默认值写在 `custom_style!` 宏里，必须是字符串字面量，格式为 `oklch(L C H)` 或 `oklch(L C H / A)`；默认值无效会在首次访问时 panic。
- 宏会自动生成 `CustomStyle`、`Default`、JSON 解析和 getter。不要手写这些代码。
- 新增 token 只需在 `custom_style!` 列表里加一行：

```rust
custom_style! {
    base: "oklch(0.95 0.01 120)",
    // ...
    my_new_surface: "oklch(0.93 0.01 120)",
}
```

- 调试构建支持热更新：`custom_style::watch(path, cx)` 会读取 JSON，文件变化时刷新窗口。
  - JSON 只接受字符串值，键名就是字段名。
  - 未知键会警告并忽略；非法值会回退到默认值。
  - release 构建忽略 JSON；永久修改必须改 `custom_style.rs`。
  - story 已经调用 watch；应用直接使用 component 时需要自己调用，并照 story 加 `#[cfg(all(debug_assertions, not(target_family = "wasm")))]`。
  - story 在仓库根目录运行时读取根目录的 `custom_style.json`，未写的字段用默认值：

  ```json
  {
    "primary": "oklch(0.5 0.135 150)",
    "base": "oklch(0.95 0.01 120)"
  }
  ```

- 固定浅色配色，不区分明暗模式。只用于选择颜色的 `is_dark()` 分支可以删掉；用于 `ColorName` 调色板或深色资源的 `is_dark()` 要保留。
- 在组件 crate 内优先写全限定路径 `crate::custom_style::text()`，减少 `use` 改动和后续冲突。
- 不要替换：
  - `cx.theme().radius*`、`motion_tokens()`、`font_family`、`font_size`、`spacing_tokens()`、`shadow`
  - `ColorName` 的 `red/green/blue/...` 调色板
  - 用户显式传入的颜色（`ButtonVariant::Custom`、`Tag::Color`、图表 series color 等）
- 新组件的颜色不要加进 `ThemeColor`、`default-theme.json` 或 `.theme-schema.json`；组件层的颜色源是 `custom_style`。只有确实需要新的语义角色时，才扩展 `custom_style!`。
- 新组件选色顺序：
  1. 页面/内容面 → `base()`
  2. 交替面/次级面 → `base_alternative()`
  3. 弱化块 → `muted_bg()`
  4. 输入/控件填充 → `input()`
  5. 浮层 → `popover()`
  6. 文字 → `text()` / `muted_text()`
  7. 边框/分隔/网格 → `border()`
  8. 强调/选中 → `primary()` / `input_selected()`
  9. hover/active → `common_hover_bg()` / `common_active_bg()`
  10. 状态 → `red_text()` / `yellow_text()` / `green_text()` / `cyan_text()` + tint alpha
  11. 深色实心上的文字 → `reversed_text()`
- 状态色 tint 统一用：

```rust
color.opacity(custom_style::text_to_bg_alpha())      // 背景
color.opacity(custom_style::text_to_border_alpha())  // 边框
```

当前两个常量分别是 `0.05` 和 `0.5`。不要在新组件里散落 `0.1/0.2/0.04` 这类魔法数。`popover` 类表面（例如 Notification）用 `custom_style::popover().blend(status_color.opacity(...))`。

## 5. 颜色替换映射

完整字段以 `custom_style.rs` 的 `custom_style!` 为准，下面是升级时最常用的映射。表中的 `tokens.*` 泛指 `theme().tokens.*` 或 `semantic_tokens().colors.*`，按实际代码路径替换。

### 5.1 表面、文字、边框

| 上游写法 | Fork 写法 |
| --- | --- |
| `theme().background`、`tokens.background`、`tokens.accordion`、`tokens.table` | `custom_style::base()` |
| `tokens.secondary`、`tokens.description_list_label`、`tokens.table_even` | `custom_style::base_alternative()` |
| `theme().muted`、`tokens.muted`、`tokens.skeleton`、`tokens.group_box` | `custom_style::muted_bg()` |
| `theme().foreground`、`theme().popover_foreground`、`theme().accent_foreground`、`theme().secondary_foreground`、`description_list_label_foreground`、`table_head_foreground`、`table_foot_foreground` | `custom_style::text()` |
| `theme().muted_foreground` | `custom_style::muted_text()` |
| `theme().border`、`theme().input`（作为边框时）、`theme().table_row_border`、`theme().chart_grid`、`theme().sidebar_border`、`theme().status_bar_border`、`theme().title_bar_border` | `custom_style::border()` |
| `theme().input_background()` | `custom_style::input()` |
| `theme().popover`、`tokens.popover` | `custom_style::popover()` |
| `theme().overlay` | `custom_style::overlay()` |
| `theme().selection` | `custom_style::selection()` |
| `theme().ring` | `custom_style::ring()` |
| `theme().caret` | `custom_style::caret()` |
| `theme().transparent` | `custom_style::transparent()` |
| `theme().title_bar`、`tokens.title_bar` | `custom_style::title_bar()` |
| `theme().status_bar`、`tokens.status_bar` | `custom_style::status_bar()` |
| `theme().table_head`、`tokens.table_head` | `custom_style::table_head()` |
| `theme().table_foot`、`tokens.table_foot` | `custom_style::table_foot()` |
| `tokens.scrollbar_thumb` | `custom_style::scrollbar_thumb()` |
| `tokens.scrollbar_thumb_hover` | `custom_style::scrollbar_thumb_hover()` |
| `tokens.scrollbar_thumb_active` | `custom_style::scrollbar_thumb_active()` |
| `theme().drag_border` | `custom_style::drag_border()` |
| `tokens.drop_target` | `custom_style::drag_target()` |

补充：

- `theme().input` 作为**边框**时映射 `border()`；作为**控件填充**时映射 `input()`。
- `tokens.secondary` 作为普通交替表面时用 `base_alternative()`；作为弱化块/控件底色时用 `muted_bg()`。
- `tokens.tab_active` 用 `base()`；`tokens.tab_bar`、`tokens.tab_bar_segmented` 用 `muted_bg()`。
- `tokens.sidebar` 用 `muted_bg()`；`tokens.sidebar_accent` 的 hover 用 `common_hover_bg()`，选中用 `common_active_bg()`。
- 文本/编辑器相关：inline code 用 `inlined()`；code block background 用 `base_alternative()`；diff gutter/行号用 `editor_muted_text()`。

### 5.2 强调与状态

| 上游写法 | Fork 写法 |
| --- | --- |
| `theme().primary`、`tokens.primary`、`tokens.progress_bar`、`tokens.slider_bar` | `custom_style::primary()` |
| `theme().primary_foreground` | `custom_style::reversed_text()` |
| `theme().danger`、`tokens.colors.destructive` | `custom_style::red_text()` |
| `theme().success` | `custom_style::green_text()` |
| `theme().warning` | `custom_style::yellow_text()` |
| `theme().info` | `custom_style::cyan_text()` |
| `theme().link` | `custom_style::blue_text()` |
| `theme().link_active` | `custom_style::purple_text()` |
| `theme().accent`、`tokens.accent`（hover） | `custom_style::common_hover_bg()` |
| `theme().accent`、`tokens.accent`（按下/选中） | `custom_style::common_active_bg()` |
| `tokens.secondary_hover`、`tokens.list_hover`、`tokens.table_hover` | `custom_style::common_hover_bg()` |
| `tokens.secondary_active`、`tokens.list_active`、`tokens.table_active` | `custom_style::common_active_bg()` |
| `tokens.switch` | `custom_style::border()` |
| `tokens.switch_thumb`、`tokens.slider_thumb` | `custom_style::input()` |
| `tokens.skeleton` | `custom_style::muted_bg()` |

Checkbox/Radio 的选中填充不要用 `primary()`，用 `input_selected()`；选中图标用 `reversed_text()`。Switch 的选中轨道才用 `primary()`。

状态色的 `*_foreground` 也不一定是 `reversed_text()`：Tag/Alert 这类浅色状态块用对应的 `*_text()`；只有深色实心填充上的文字才用 `reversed_text()`。

### 5.3 按钮

按钮在 fork 中被重做，不要直接恢复上游的 `outline` 配色逻辑。

| 位置 | Fork 规则 |
| --- | --- |
| 所有非 Ghost/Link/Text/Custom 的填充 | `button.bg = custom_style::input()` |
| 默认 hover | `custom_style::button_hover_bg()` |
| 默认 active | `custom_style::button_active_bg()` |
| Primary 边框 | `custom_style::primary_button_border()` |
| Primary hover/active | `primary_button_hover_bg()` / `primary_button_active_bg()` |
| Danger 边框 | `custom_style::danger_button_border()` |
| Danger hover/active | `danger_button_hover_bg()` / `danger_button_active_bg()` |
| Ghost hover/active | `common_hover_bg()` / `common_active_bg()` |
| Link 文字 | `blue_text()`；按下时 `purple_text()` |
| Text 按下文字 | `green_text()` |
| 禁用态 | 未禁用颜色的 `opacity(0.5)` |
| 选中态 | 等同于 active；选中且禁用时用 active 背景 `opacity(0.5)` |
| `.outline()` | 公开 API 保留，但 fork 中不再改变配色；不要重新引入 outline 调色板 |
| 边框 | 除 Ghost/Link/Text 外都有边框；底边是 `border_b_2()`，其余边 1px |

### 5.4 图表、表格与导航

| 上游写法 | Fork 写法 |
| --- | --- |
| `theme().chart_1` 到 `chart_5` | `custom_style::chart_1()` 到 `chart_5()` |
| `theme().chart_bullish` / `chart_bearish` | `custom_style::chart_bullish()` / `chart_bearish()` |
| 图表轴、网格、分隔线 | `custom_style::border()` |
| 图表默认 stroke/fill | `custom_style::chart_2()` |
| 图表圆点描边 | `custom_style::base()` |
| 图表标签 | `custom_style::text()` / `muted_text()` |
| `tokens.table_head` | `custom_style::table_head()` |
| `tokens.table_foot` | `custom_style::table_foot()` |
| `tokens.table_even` | `custom_style::base_alternative()` |
| `tokens.table_hover` | `custom_style::common_hover_bg()` |
| `tokens.table_active` | `custom_style::common_active_bg()` |
| `table_active_border` | `custom_style::green_text().opacity(0.5)` |
| `tokens.tab_bar` / `tab_bar_segmented` | `custom_style::muted_bg()` |
| `tokens.tab_active` | `custom_style::base()` |
| `tokens.list_hover` | `custom_style::common_hover_bg()` |
| `tokens.list_active` | `custom_style::common_active_bg()` |
| `tokens.sidebar_accent`（hover） | `custom_style::common_hover_bg()` |
| `tokens.sidebar_accent`（选中） | `custom_style::common_active_bg()` |

### 5.5 一个替换示例

上游：

```rust
let tokens = cx.theme().semantic_tokens();
div()
    .bg(tokens.colors.background)
    .border_color(tokens.colors.border)
    .text_color(tokens.colors.foreground)
    .child(div().bg(tokens.colors.muted))
```

Fork：

```rust
div()
    .bg(crate::custom_style::base())
    .border_color(crate::custom_style::border())
    .text_color(crate::custom_style::text())
    .child(div().bg(crate::custom_style::muted_bg()))
```

hover/active 不要复用 `muted_bg()`，分别用 `common_hover_bg()` / `common_active_bg()`。

如果组件先把 `semantic_tokens()` 存进局部变量，不要只改这一行；把后面用到的 `tokens.colors.*` 逐个换成 `custom_style`，`radius/spacing/typography` 继续留在本地变量里使用。

### 5.6 Diff 组件：有意保留主题颜色

`crates/component/src/diff/mod.rs` 是唯一大量保留 `cx.theme()` 颜色读取的组件，不要按 5.1/5.2 的规则全量替换。用户已经确认当前的主题色效果更好。

保留主题读取的部分：

| 位置 | 保留写法 |
| --- | --- |
| 冲突颜色 | `cx.theme().success` / `cx.theme().info` / `cx.theme().muted_foreground` |
| 增删行颜色 | `cx.theme().danger` / `cx.theme().success` |
| `DiffDecoration` 默认背景 | `cx.theme().warning.opacity(0.2)` |
| 语法高亮 | `cx.theme().highlight_theme` |

这些是 Diff 的语义色，应跟随用户主题。只有当用户明确要求统一到 fork 调色板时，才替换为 `green_text()` / `cyan_text()` / `red_text()` / `yellow_text()` + tint alpha。

Diff 的其余颜色仍按 fork 规则使用 `custom_style`：表面、文字、边框、hover/active、选中背景等继续用 `base` / `text` / `muted_text` / `border` / `common_hover_bg` / `common_active_bg` / `selection`。

## 6. 上游新增组件的审计

新组件不会出现在 cherry-pick 冲突里，这是最容易漏掉的部分。

```bash
# 以上一次基线和本次目标标签为例
git diff --name-status <previous-tag> <target-tag> -- crates/component/src

# 在新/改文件中找直接使用主题颜色的地方
rg -n 'cx\.theme\(\)|semantic_tokens\(\)' crates/component/src
```

处理步骤：

1. 先读新组件源码，判断它是否真的绘制 UI。
2. 找出所有 `cx.theme()`、`tokens.colors.*`、`theme().*` 的颜色用法。
3. 按第 5 节替换；保留 `radius/motion/font/spacing`。Diff 组件是例外，见 5.6。
4. 如果新组件使用 `ThemeStyled`、`input_style`、`popover_style`、`focus_style` 等共享 helper，先确认 helper 已经走 `custom_style`；通常不需要重复改。新增 helper 如果仍取 `theme` 颜色，优先改 helper 本身，不要在每个组件里重复替换。
5. 只用于颜色选择的 `is_dark()` 分支删掉；用于 `ColorName`、图片资源、平台差异的保留。
6. 用户显式传入的颜色不要改。
7. 如果现有 token 不能表达新语义：
   - 优先复用 `base`、`base_alternative`、`muted_bg`、`input`、`popover`、`text`、`muted_text`、`border`、`primary`、`common_hover_bg`、`common_active_bg`。
   - 确实需要新角色时，在 `custom_style!` 中新增字段，名字用语义而不是组件名。
   - 只有 Base 层也绘制它时，才考虑同步修改 `Theme::base_theme()`。
8. 新增 token 后，同步更新本文第 5 节。

不要给每个新组件都新增一组颜色字段。这个 fork 的维护成本取决于 token 数量，而不是组件数量。

## 7. 必须保留的 fork 行为

这些不是纯颜色替换，升级时容易被上游覆盖。

| 文件/区域 | Fork 行为 |
| --- | --- |
| `crates/component/src/lib.rs` | `pub mod custom_style;`；`init()` 中 `set_locale("zh-CN")` |
| `crates/component/Cargo.toml` | `arc-swap = "1.9.2"`，用于调试构建的颜色热更新 |
| `crates/component/src/theme/default-theme.json` | Default Light 字体改为 `Fira Noto SC`，字号 16/14 |
| `crates/component/src/theme/mod.rs` | `pub(crate) mod color`；`Theme::base_theme()` 用 `custom_style` 覆盖 `background` / `selection` / scrollbar thumb |
| `crates/component/src/theme/color.rs` | `oklab` 对 crate 可见；新增 `oklab_to_rgb_unclamped`；`oklab_to_rgb` 仍做 clamp |
| `crates/component/src/theme/schema.rs` | 应用主题配置时，highlight 必须经过 `custom_highlight_theme_style` |
| `crates/component/src/styled.rs` | popover 背景/文字、focus ring 使用 `custom_style` |
| `crates/component/src/highlighter/registry.rs`、`highlighter/wasm_stub.rs` | `ThemeStyle` 派生 `Default`，字段公开；状态背景用 `base()` |
| `crates/component/src/root.rs` | Root 表面背景/文字使用 `base()` / `text()` |
| `crates/component/src/button/button.rs` | 按钮扁平化重做；`.outline()` 不再影响配色；底边 2px；选中=active；禁用半透明 |
| `crates/base/src/calendar.rs` | 周一为第一天；`CalendarItemState` 增加 `range_start/range_end`；月份/年份标签改为年在上、月在下 |
| `crates/component/src/time/calendar.rs` | 周一为第一天；区间首尾圆角；weekday 头不再单独用 muted 色 |
| `crates/component/src/time/date_picker.rs` | 周一为第一天；日期格式改为 `%Y-%m-%d` |
| `crates/component/src/time/time_field.rs` | invalid 边框用 `red_text()` |
| `crates/component/src/checkbox.rs` | 指示器改为 `size_full()`；Checkbox 内容 `items_center()` |
| `crates/component/src/radio.rs` | 自绘圆点并按设备像素对齐；内容 `items_center()` |
| `crates/component/src/switch.rs` | 内容 `items_center()`；未禁用时 `on_mouse_down` 调 `prevent_default()` |
| `crates/component/src/input/group.rs` | 输入组背景改用 `input_style(disabled)`；invalid ring 用 `red_text()`；禁用/普通态不再走暗色分支 |
| `crates/component/src/dock/tab_panel.rs` | 标签页控制按钮从自定义 `ButtonCustomVariant` 改为 `ghost()`，依赖新的按钮样式 |
| `crates/component/src/tag.rs` | 去掉 hover `opacity(0.9)`；状态色用 tint alpha |
| `crates/component/src/title_bar.rs` | 去掉渐变背景，改用纯 `title_bar()`；删除旧渐变测试 |
| `crates/component/src/dialog/dialog.rs` | `centered` 默认 `true`；`margin_top()` 会关闭 centered |
| `crates/component/src/dialog/entrance.rs` | 默认 entrance 改为 `Fade` |
| `crates/component/src/list/list_item.rs`、`menu/menu_item.rs`、`searchable_list/item.rs` | 移除/调整默认字号（ListItem 用 `text_sm()`，MenuItem/SearchableList 去掉 `text_base()`） |
| `crates/story/src/lib.rs` | 启动时 watch `custom_style.json` |

注意：

- `checkbox_check_icon` 已被 `checkbox::indicator_size` + `checked_indicator` 取代；不要恢复旧实现。
- `default_title_bar_background` 已删除；不要恢复渐变。
- `ButtonVariant` 的 `outline_background`、`ButtonStyleState` 已删除；冲突时不要把它们搬回来。
- `crates/component/src/input/input.rs` 的 `input_style` 签名是 `(disabled)`；上游旧代码若仍调用 `(disabled, cx)`，改成新签名。
- 语法高亮颜色集中在 `custom_highlight_theme_style`；上游新增 syntax 字段时，在那里补 `ThemeStyle { color: Some(...), ..Default::default() }`，不要散落到组件里。
- 这些行为是否继续保留，最终以用户明确要求为准；没有要求时，按本表保留 fork 现状。

## 8. 已知例外与坑

- `crates/component/src/diff/mod.rs` 是唯一大量保留主题颜色的组件，见 5.6。不要按普通组件全量替换它的冲突色、增删行色和 `DiffDecoration` 默认背景。
- `crates/component/src/input/input.rs` 的 code editor 背景仍用 `cx.theme().editor_background()`。这是已知例外。
- Base 层源码除日历外没有跟随 fork 改色；Base 只通过 `Theme::base_theme()` 的 `background` / `selection` / scrollbar thumb 覆盖间接获得一部分 `custom_style`。Base 新组件默认沿用上游实现，可能拿到这些覆盖色，但不会自动获得组件层的 `custom_style`；除非用户明确要求，不要改 Base。
- List/Table 的选中背景不再读取 `theme.list.active_highlight`，统一 `common_active_bg()`；这是 fork 的有意简化，不要为了恢复主题配置而加回分支。
- `Theme` 的 radius、motion、font、spacing、shadow 继续来自上游；不要因为组件颜色改了就把它们也迁到 `custom_style`。
- `custom_style` 是固定浅色方案，没有暗色版本；不要在升级时假设暗色模式会自动匹配。
- 旧测试里可能仍有断言 `Theme` 颜色的代码，尤其是 `button/button.rs` 的测试。它们不代表 fork 的预期；以 `custom_style` 和实际视觉为准，必要时更新测试。
- `custom_style.json` 只是本地调试文件，默认不提交；提交前检查 `git status`。
- `.outline()` 在 fork 中仍可调用，但不再改变按钮配色；不要把它当作上游的 outline 配色行为保留。
- `ThemeStyle` 的公开字段和 `Default` 是 `custom_highlight_theme_style` 的依赖；不要为了少改代码恢复私有字段。

## 9. 验证清单

```bash
cargo fmt --check
cargo check -p gpui-component
cargo clippy -p gpui-component --lib -- --deny warnings
```

按仓库约定，纯视觉调整不要求跑测试；优先 `check` / `clippy` 和 story 目视检查。如果跑测试，注意旧颜色断言可能已经过时。

重点检查：

- 所有新组件的背景、文字、边框、hover/active 是否都来自 `custom_style`。
- 状态色背景/边框是否使用 tint alpha，而不是硬编码透明度。
- 按钮各 variant、禁用态、选中态。
- Checkbox、Radio、Switch 的对齐和选中颜色。
- Tab、Table、List、Menu 的 hover/active。
- Calendar、DatePicker 的周一开头、区间圆角和日期格式。
- Dialog、Popover、Tooltip、Toast 的表面和遮罩。
- TitleBar、StatusBar、Scrollbar。
- 图表默认色、涨跌色、轴/网格色。

残留颜色审计：

```bash
rg -n 'cx\.theme\(\)\.(foreground|background|border|muted|primary|secondary|accent|danger|success|warning|info|link|popover|selection|ring|input|title_bar|status_bar|sidebar|table_|chart_|drag_|scrollbar|tab_|caret|overlay)' crates/component/src
rg -n 'tokens\.colors\.|semantic_tokens\(\)' crates/component/src
```

同时检查 `let theme = cx.theme();` 之后通过 `theme.foreground` 这类局部变量取色的写法。预期仍有少量例外（见第 8 节），但新组件里不应该出现。

Diff 组件命中的状态色、冲突色和 `DiffDecoration` 默认背景属于预期例外（见 5.6），不要为了清空审计结果而替换。

可选：用 `range-diff` 对比两次 fork 补丁，确认没有丢 hunk：

```bash
git range-diff <previous-tag>..fork/<previous-version> <target-tag>..fork/<target-version>
```

## 10. 提交约定

- 提交信息保持 `FORK：<简述>`，方便下次 `git log --grep='FORK：'`。
- 一个版本一个提交，冲突修复 amend 进同一个提交。
- 升级完成后把 `fork/<version>` push 到 origin，下一版从 origin 取；否则下次找不到上一版补丁。
- 只提交 fork 需要的改动，不要夹带上游已有的格式化、重命名或无关重构。
- 每次升级完成后更新本文：
  - 第 1 节的基线版本和父提交。
  - 第 7 节新增/变化的行为。
  - 第 5 节新增的 `custom_style` token。
