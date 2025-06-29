use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Widget},
};

use super::{palette::PALETTE, theme::THEME, util};

pub fn centered_clamp(max_width: u16, outer: Rect) -> Rect {
    let width = max_width.min(outer.width);
    Rect {
        x: outer.x + (outer.width - width) / 2,
        y: outer.y,
        width,
        height: outer.height,
    }
}

pub fn border_style(is_focused: bool) -> Style {
    let s_accent = Style::new().fg(PALETTE.primary_accent);
    let s_base = Style::new().fg(PALETTE.secondary);
    if is_focused {
        s_accent
    } else {
        s_base
    }
}

pub fn tab_style(selected: bool) -> Style {
    let s = Style::new().fg(PALETTE.primary_accent);
    if selected {
        s.add_modifier(Modifier::BOLD)
    } else {
        s.add_modifier(Modifier::DIM)
    }
}

pub fn render_divider(area: Rect, buf: &mut Buffer) {
    let div = Span::styled("─".repeat(area.width as usize), THEME.divider).into_centered_line();
    Widget::render(div, area, buf);
}

pub fn container_border(
    area: Rect,
    buf: &mut Buffer,
    title: &'static str,
    footer: Option<&'static str>,
    is_focused: bool,
) -> Rect {
    let title_top = format!("─ {title} ");
    let footer = footer
        .map(|f| format!(" {f} ─"))
        .unwrap_or_else(|| "─".to_owned());

    let border_style = util::border_style(is_focused);
    let title_line = Span::styled(title_top, border_style).into_left_aligned_line();
    let footer_line = Span::styled(footer, border_style).into_right_aligned_line();

    Widget::render(
        Block::bordered()
            .title_top(title_line)
            .title_bottom(footer_line)
            .border_style(border_style)
            .border_type(BorderType::Rounded),
        area,
        buf,
    );

    // calculate the inner area of the container
    let inner = area.inner(Margin::new(2, 1));
    let [inner_area] = Layout::vertical([Constraint::Fill(1)]).areas(inner);
    inner_area
}
