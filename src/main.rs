// Copyright 2016 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A plugin to count words in xi-editor.
extern crate xi_core_lib as xi_core;
extern crate xi_plugin_lib;
extern crate xi_rope;

use std::ops::Range;
use std::path::Path;
use regex::Regex;

use crate::xi_core::ConfigTable;
use xi_plugin_lib::{mainloop, ChunkCache, Error, Plugin, View};
use xi_rope::delta::Builder as EditBuilder;
use xi_rope::interval::Interval;
use xi_rope::rope::RopeDelta;

struct WordCountPlugin {
    /// Number of words
    words: usize,

    /// Number of characters
    characters: usize,

    word_count_regex: Regex,
}

impl Plugin for WordCountPlugin {
    type Cache = ChunkCache;

    fn new_view(&mut self, view: &mut View<Self::Cache>) {
        let end = view.get_buf_size();
        if let Ok(word_count) = self.count_words(view, Range { start: 0, end: end }) {
            eprintln!("{:?}", word_count);
            self.words = word_count
        }
    }

    fn did_close(&mut self, view: &View<Self::Cache>) { }

    fn did_save(&mut self, view: &mut View<Self::Cache>, _old: Option<&Path>) { }

    fn config_changed(&mut self, _view: &mut View<Self::Cache>, _changes: &ConfigTable) {}

    fn update(
        &mut self,
        view: &mut View<Self::Cache>,
        delta: Option<&RopeDelta>,
        _edit_type: String,
        _author: String,
    ) {
        if let Some(delta) = delta {
            let (iv, _) = delta.summary();
            let text: String = delta.as_simple_insert().map(String::from).unwrap_or_default();
            if text == "!" {
                let _ = self.capitalize_word(view, iv.end());
            }
        }
    }
}

impl WordCountPlugin {
    fn new() -> WordCountPlugin {
        WordCountPlugin {
            words: 0,
            characters: 0,
            word_count_regex: Regex::new(r"(\w)+").unwrap(),
        }
    }

    fn count_words(&mut self, view: &mut View<ChunkCache>, region: Range<usize>) -> Result<usize, Error> {
        let start_line = view.line_of_offset(region.start)?;
        let end_line = view.line_of_offset(region.end)?;

        let word_count: usize = (start_line..=end_line).map(|line_nr| {
            match view.get_line(line_nr) {
                Ok(line) => {
                    self.word_count_regex.captures_iter(line).map(|c| 1).count()
                }
                _ => 0
            }
        }).sum();

        Ok(word_count)
    }


    /// Uppercases the word preceding `end_offset`.
    fn capitalize_word(&self, view: &mut View<ChunkCache>, end_offset: usize) -> Result<(), Error> {
        //NOTE: this makes it clear to me that we need a better API for edits
        let line_nb = view.line_of_offset(end_offset)?;
        let line_start = view.offset_of_line(line_nb)?;

        let mut cur_utf8_ix = 0;
        let mut word_start = 0;
        for c in view.get_line(line_nb)?.chars() {
            if c.is_whitespace() {
                word_start = cur_utf8_ix;
            }

            cur_utf8_ix += c.len_utf8();

            if line_start + cur_utf8_ix == end_offset {
                break;
            }
        }

        let new_text = view.get_line(line_nb)?[word_start..end_offset - line_start].to_uppercase();
        let buf_size = view.get_buf_size();
        let mut builder = EditBuilder::new(buf_size);
        let iv = Interval::new(line_start + word_start, end_offset);
        builder.replace(iv, new_text.into());
        view.edit(builder.build(), 0, false, true, "sample".into());
        Ok(())
    }
}

fn main() {
    let mut plugin = WordCountPlugin::new();
    mainloop(&mut plugin).unwrap();
}
