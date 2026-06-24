mod app;
mod ui;

use app::App;
use ui::ui;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{env, io};

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if app.show_toc {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                    KeyCode::Char('t') => app.show_toc = false,
                    KeyCode::Enter => {
                        if let Some(i) = app.list_state.selected() {
                            app.go_to_chapter(i);
                            app.show_toc = false;
                        }
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        let next = app.list_state.selected()
                            .map(|i| (i + 1).min(app.chapters.len() - 1))
                            .unwrap_or(0);
                        app.list_state.select(Some(next));
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        let prev = app.list_state.selected()
                            .map(|i| i.saturating_sub(1))
                            .unwrap_or(0);
                        app.list_state.select(Some(prev));
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),

                    KeyCode::Char('j') | KeyCode::Down => app.scroll_down(1),
                    KeyCode::Char('k') | KeyCode::Up => app.scroll_up(1),
                    KeyCode::PageDown | KeyCode::Char(' ') => app.scroll_down(20),
                    KeyCode::PageUp => app.scroll_up(20),
                    KeyCode::Home => app.scroll = 0,
                    KeyCode::End => {
                        let max = app.current_lines().len().saturating_sub(1).min(u16::MAX as usize) as u16;
                        app.scroll = max;
                    }

                    KeyCode::Char('n') | KeyCode::Right => {
                        let next = (app.chapter_index + 1).min(app.chapters.len().saturating_sub(1));
                        app.go_to_chapter(next);
                    }
                    KeyCode::Char('p') | KeyCode::Left => {
                        let prev = app.chapter_index.saturating_sub(1);
                        app.go_to_chapter(prev);
                    }

                    KeyCode::Char('t') => app.show_toc = true,

                    _ => {}
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "-h" || args[1] == "--help" {
        println!("Usage: epub-reader <file.epub>");
        println!();
        println!("Keybindings:");
        println!("  q / Ctrl+C     quit");
        println!("  j/k / ↑↓       scroll up/down");
        println!("  Space/PgDn     scroll down 20 lines");
        println!("  PgUp           scroll up 20 lines");
        println!("  n/p / ←→       next/previous chapter");
        println!("  t              toggle table of contents");
        std::process::exit(0);
    }

    let path = &args[1];
    if !std::path::Path::new(path).exists() {
        eprintln!("error: file not found: {}", path);
        std::process::exit(1);
    }

    let mut app = App::load(path)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result?;
    Ok(())
}
