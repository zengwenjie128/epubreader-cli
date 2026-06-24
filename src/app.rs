use epub::doc::EpubDoc;
use ratatui::{text::Line, widgets::ListState};

pub struct App {
    pub chapters: Vec<String>,
    pub chapter_index: usize,
    pub scroll: u16,
    pub list_state: ListState,
    pub show_toc: bool,
    contents: Vec<String>,
}

impl App {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut doc = EpubDoc::new(path)?;
        let num_items = doc.get_num_chapters();

        let mut chapters = Vec::new();
        let mut contents = Vec::new();

        for i in 0..num_items {
            doc.set_current_chapter(i);
            let title = doc
                .get_current_id()
                .unwrap_or_else(|| format!("Chapter {}", i + 1));

            let html = doc.get_current_str().map(|(s, _)| s).unwrap_or_default();
            let text = html2text::from_read(html.as_bytes(), 80);

            chapters.push(title);
            contents.push(text);
        }

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Ok(App {
            chapters,
            contents,
            chapter_index: 0,
            scroll: 0,
            list_state,
            show_toc: false,
        })
    }

    pub fn current_lines(&self) -> Vec<Line<'_>> {
        self.contents
            .get(self.chapter_index)
            .map(|text| text.lines().map(|l| Line::from(l.to_string())).collect())
            .unwrap_or_default()
    }

    pub fn go_to_chapter(&mut self, idx: usize) {
        if idx < self.chapters.len() {
            self.chapter_index = idx;
            self.scroll = 0;
            self.list_state.select(Some(idx));
        }
    }

    pub fn scroll_down(&mut self, amount: u16) {
        let max_scroll = self.current_lines().len().saturating_sub(1).min(u16::MAX as usize) as u16;
        self.scroll = (self.scroll + amount).min(max_scroll);
    }

    pub fn scroll_up(&mut self, amount: u16) {
        self.scroll = self.scroll.saturating_sub(amount);
    }
}
