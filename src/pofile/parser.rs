use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::pofile::*;

impl POFile {
    pub fn parse(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = std::fs::File::open(path)?;
        let mut messages: Vec<Message> = Vec::new();
        let mut dirty = false;
        let mut cur_comments = String::new();
        let mut cur_source = String::new();
        let mut cur_flags = String::new();
        let mut cur_msgctxt: Option<String> = None;
        let mut cur_msgid = String::new();
        let mut cur_msgid_plural = String::new();
        let mut cur_msgstr = String::new();
        let mut cur_msgstr_plural = vec![String::new(); 10];
        let mut idle_buf = String::new();
        let mut cur_str_buf = &mut cur_msgid;

        for line in BufReader::new(file).lines() {
            let line = line?;
            if line.is_empty() {
                cur_str_buf = &mut idle_buf;
                if dirty {
                    save_msg(
                        &mut messages,
                        &mut cur_comments,
                        &mut cur_source,
                        &mut cur_flags,
                        &mut cur_msgctxt,
                        &mut cur_msgid,
                        &mut cur_msgid_plural,
                        &mut cur_msgstr,
                        &mut cur_msgstr_plural,
                    );
                    dirty = false;
                }
            } else if line.starts_with("#.") {
                cur_str_buf = &mut cur_comments;
                append_new_line_str(cur_str_buf, &line.as_str()[3..]);
                dirty = true;
            } else if line.starts_with("#:") {
                cur_str_buf = &mut cur_source;
                append_new_line_str(cur_str_buf, &line.as_str()[3..]);
                dirty = true;
            } else if line.starts_with("#,") {
                cur_str_buf = &mut cur_flags;
                append_new_line_str(cur_str_buf, &line.as_str()[3..]);
                dirty = true;
            } else if line.starts_with("msgid ") {
                cur_str_buf = &mut cur_msgid;
                append_str(
                    cur_str_buf,
                    &line.trim_start_matches("msgid ").trim_matches('"'),
                );
                dirty = true;
            } else if line.starts_with("msgid_plural ") {
                cur_str_buf = &mut cur_msgid_plural;
                append_str(
                    cur_str_buf,
                    &line.trim_start_matches("msgid_plural ").trim_matches('"'),
                );
                dirty = true;
            } else if line.starts_with("msgstr ") {
                cur_str_buf = &mut cur_msgstr;
                append_str(
                    cur_str_buf,
                    &line.trim_start_matches("msgstr ").trim_matches('"'),
                );
                dirty = true;
            } else if line.starts_with("msgstr[") {
                let index = line.chars().nth(7).unwrap().to_digit(10).unwrap() as usize;
                cur_str_buf = &mut cur_msgstr_plural[index];
                append_str(cur_str_buf, &line.as_str()[10..].trim_matches('"'));
                dirty = true;
            } else if line.starts_with("\"") {
                append_str(cur_str_buf, &line.trim_matches('"'));
                dirty = true;
            }
        }

        if dirty {
            save_msg(
                &mut messages,
                &mut cur_comments,
                &mut cur_source,
                &mut cur_flags,
                &mut cur_msgctxt,
                &mut cur_msgid,
                &mut cur_msgid_plural,
                &mut cur_msgstr,
                &mut cur_msgstr_plural,
            );
        }

        Ok(POFile {
            num_plural_forms: 1,
            plural_eval: |_| 0,
            messages: messages,
        })
    }
}

fn append_str(buf: &mut String, content: &str) {
    buf.push_str(content);
}

fn append_new_line_str(buf: &mut String, content: &str) {
    if !buf.is_empty() && !buf.ends_with('\n') {
        buf.push('\n');
    }
    buf.push_str(content);
}

fn save_msg(
    result: &mut Vec<Message>,
    cur_comments: &mut String,
    cur_source: &mut String,
    cur_flags: &mut String,
    cur_msgctxt: &mut Option<String>,
    cur_msgid: &mut String,
    cur_msgid_plural: &mut String,
    cur_msgstr: &mut String,
    cur_msgstr_plural: &mut Vec<String>,
) {
    let is_plural = !cur_msgid_plural.is_empty();
    result.push(Message {
        comments: std::mem::take(cur_comments),
        source: std::mem::take(cur_source),
        flags: std::mem::take(cur_flags),
        msgctxt: std::mem::take(cur_msgctxt),
        body: match is_plural {
            false => PluralizableMessage::Singular(SingularMessage {
                msgid: std::mem::take(cur_msgid),
                msgstr: std::mem::take(cur_msgstr),
            }),
            true => PluralizableMessage::Plural(PluralMessage {
                msgid: std::mem::take(cur_msgid),
                msgid_plural: std::mem::take(cur_msgid_plural),
                msgstr_plural: std::mem::take(cur_msgstr_plural),
            }),
        },
    });
    if is_plural {
        *cur_msgstr_plural = vec![String::new(); 10];
    }
}
