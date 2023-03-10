use colored::Colorize;
use std::sync::Arc;

use skim::{
    prelude::{unbounded, Key, SkimOptionsBuilder},
    Skim, SkimItemReceiver, SkimItemSender,
};

use crate::{
    config::{ExternalCommands, SurfParsing},
    database::SqliteAsyncHandle,
    note::{AsyncQeuryResources, Note, PreviewType},
};

pub(crate) struct Iteration {
    db: SqliteAsyncHandle,
    items: Option<Vec<Note>>,
    external_commands: ExternalCommands,
    surf_parsing: SurfParsing,
    preview_type: PreviewType,
}

pub enum Action {
    Open(Note),
    Noop,
    TogglePreview,
}

pub struct Out {
    pub action: Action,
    pub next_items: Vec<Note>,
}

impl Iteration {
    pub(crate) fn new(
        items: Vec<Note>,
        db: SqliteAsyncHandle,
        external_commands: ExternalCommands,
        surf_parsing: SurfParsing,
        preview_type: PreviewType,
    ) -> Self {
        Self {
            items: Some(items),
            db,
            external_commands,
            surf_parsing,
            preview_type,
        }
    }

    pub(crate) async fn run(mut self) -> anyhow::Result<Out> {
        let items = self.items.take().unwrap();

        let options = SkimOptionsBuilder::default()
            .height(Some("100%"))
            .preview(Some(""))
            .multi(false)
            .bind(vec![
                "ctrl-c:abort",
                "Enter:accept",
                "ESC:abort",
                "ctrl-h:accept",
                "ctrl-l:accept",
                "ctrl-t:accept",
            ])
            .build()?;

        let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

        let db = self.db;
        let cloned = items.clone();
        let _jh = std::thread::spawn(move || {
            for mut note in cloned {
                note.set_resources(AsyncQeuryResources {
                    db: db.clone(),
                    external_commands: self.external_commands.clone(),
                    surf_parsing: self.surf_parsing.clone(),
                    preview_type: self.preview_type,
                });

                let result = tx.send(Arc::new(note));
                if result.is_err() {
                    eprintln!("{}", format!("{:?}", result).red());
                }
            }
        });

        if let Some(out) = Skim::run_with(&options, Some(rx)) {
            let selected_items = out
                .selected_items
                .iter()
                .map(|selected_item| {
                    (**selected_item)
                        .as_any()
                        .downcast_ref::<Note>()
                        .unwrap()
                        .to_owned()
                })
                .collect::<Vec<Note>>();

            match out.final_key {
                Key::Enter => {
                    if let Some(item) = selected_items.first() {
                        return Ok(Out {
                            action: Action::Open(item.clone()),
                            next_items: items,
                        });
                    } else {
                        return Err(anyhow::anyhow!("no item selected"));
                    }
                }
                Key::Ctrl('c') | Key::ESC => {
                    return Err(anyhow::anyhow!(
                        "user chose to abort current iteration of explore cycle"
                    ))
                }

                Key::Ctrl('h') => {
                    if let Some(item) = selected_items.first() {
                        let mut next = item.fetch_backlinks().await.unwrap()?;
                        if next.is_empty() {
                            next = items;
                        }
                        return Ok(Out {
                            action: Action::Noop,
                            next_items: next,
                        });
                    } else {
                        return Err(anyhow::anyhow!("no item selected"));
                    }
                }

                Key::Ctrl('l') => {
                    if let Some(item) = selected_items.first() {
                        let mut next = item.fetch_forward_links().await.unwrap()?;
                        if next.is_empty() {
                            next = items;
                        }
                        return Ok(Out {
                            action: Action::Noop,
                            next_items: next,
                        });
                    } else {
                        return Err(anyhow::anyhow!("no item selected"));
                    }
                }

                Key::Ctrl('t') => {
                    return Ok(Out {
                        action: Action::TogglePreview,
                        next_items: items,
                    });
                }
                _ => {
                    unreachable!();
                }
            };
        } else {
            return Err(anyhow::anyhow!("skim internal errors"));
        }
    }
}
