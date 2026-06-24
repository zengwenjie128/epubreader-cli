use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
};

use crate::app::App;

pub fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let total_lines = app.current_lines().len();

    if app.show_toc {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(f.area());

        let items: Vec<ListItem> = app
            .chapters
            .iter()
            .enumerate()
            .map(|(i, title)| {
                let style = if i == app.chapter_index {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{:>3}. {}", i + 1, title)).style(style)
            })
            .collect();

        let toc = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" Contents "))
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(toc, chunks[0], &mut app.list_state);
        render_content(f, app, chunks[1], total_lines);
    } else {
        render_content(f, app, f.area(), total_lines);
    }
}

const CONTENT_WIDTH: u16 = 80;

fn render_content(f: &mut ratatui::Frame, app: &App, area: ratatui::layout::Rect, total_lines: usize) {
    // Center the content: pad both sides so the text column stays at CONTENT_WIDTH.
    let (left_pad, center_w) = if area.width > CONTENT_WIDTH + 1 {
        let pad = (area.width - CONTENT_WIDTH - 1) / 2; // -1 for scrollbar
        (pad, CONTENT_WIDTH)
    } else {
        (0, area.width.saturating_sub(1))
    };
    let right_pad = area.width.saturating_sub(left_pad + center_w + 1);

    let content_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(left_pad),
            Constraint::Length(center_w),
            Constraint::Length(1),
            Constraint::Length(right_pad),
        ])
        .split(area);

    let title = app
        .chapters
        .get(app.chapter_index)
        .map(|s| format!(" {} ({}/{}) ", s, app.chapter_index + 1, app.chapters.len()))
        .unwrap_or_default();

    let help = " j/k:scroll  n/p:chapter  t:contents  q:quit ";

    let para = Paragraph::new(app.current_lines())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .title_bottom(help),
        )
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0));

    f.render_widget(para, content_area[1]);

    let mut scrollbar_state = ScrollbarState::new(total_lines).position(app.scroll as usize);
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    f.render_stateful_widget(scrollbar, content_area[2], &mut scrollbar_state);
}
