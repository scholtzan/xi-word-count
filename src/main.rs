//! A plugin to count words in xi-editor.
extern crate xi_core_lib as xi_core;
extern crate xi_plugin_lib;
extern crate xi_rope;

use regex::Regex;
use std::path::Path;

use crate::xi_core::ConfigTable;
use xi_plugin_lib::{mainloop, ChunkCache, Plugin, View};
use xi_rope::interval::Interval;
use xi_rope::rope::RopeDelta;

struct WordCountPlugin {
    /// Number of words
    words: usize,

    /// Number of characters
    characters: usize,

    /// Number of lines
    lines: usize,

    /// Regex for capturing and counting words
    word_count_regex: Regex,
}

impl Plugin for WordCountPlugin {
    type Cache = ChunkCache;

    fn new_view(&mut self, view: &mut View<Self::Cache>) {
        self.words = self.count_words(view);
        self.lines = self.count_lines(view) + 1;
        self.characters = self.count_characters(view);
        view.add_status_item("line_count", &format!("lines: {}", self.lines), "left");
        view.add_status_item("word_count", &format!("words: {}", self.words), "left");
        view.add_status_item(
            "character_count",
            &format!("characters: {}", self.characters),
            "left",
        );
    }

    fn did_close(&mut self, _view: &View<Self::Cache>) {}

    fn did_save(&mut self, _view: &mut View<Self::Cache>, _old: Option<&Path>) {}

    fn config_changed(&mut self, _view: &mut View<Self::Cache>, _changes: &ConfigTable) {}

    fn update(
        &mut self,
        view: &mut View<Self::Cache>,
        _delta: Option<&RopeDelta>,
        _edit_type: String,
        _author: String,
    ) {
        self.words = self.count_words(view);
        self.lines = self.count_lines(view) + 1;
        self.characters = self.count_characters(view);
        view.update_status_item("line_count", &format!("lines: {}", self.lines));
        view.update_status_item("word_count", &format!("words: {}", self.words));
        view.update_status_item(
            "character_count",
            &format!("characters: {}", self.characters),
        );
    }
}

impl WordCountPlugin {
    fn new() -> WordCountPlugin {
        WordCountPlugin {
            words: 0,
            characters: 0,
            lines: 0,
            word_count_regex: Regex::new(r"(\w)+").unwrap(),
        }
    }

    fn count_words(&mut self, view: &mut View<ChunkCache>) -> usize {
        let end = view.get_buf_size();
        if let Ok(text) = view.get_region(Interval::new(0, end)) {
            let word_count = self.word_count_regex.captures_iter(text).map(|_c| 1).count();
            return word_count;
        }

        0
    }

    fn count_lines(&mut self, view: &mut View<ChunkCache>) -> usize {
        let end = view.get_buf_size();
        view.line_of_offset(end).unwrap_or_else(|_e| 0)
    }

    fn count_characters(&mut self, view: &mut View<ChunkCache>) -> usize {
        view.get_buf_size()
    }
}

fn main() {
    let mut plugin = WordCountPlugin::new();
    mainloop(&mut plugin).unwrap();
}
