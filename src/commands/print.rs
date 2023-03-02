use std::collections::HashSet;

use crate::{
    config::ExternalCommands,
    database::{Database, SqliteAsyncHandle},
    print::format_two_tokens,
};

use colored::Colorize;

pub(crate) async fn exec(
    db: SqliteAsyncHandle,
    external_commands: ExternalCommands,
    name: Option<String>,
) -> Result<String, anyhow::Error> {
    let note = {
        if let Some(name) = name {
            let note = db.lock().await.get(&name).await?;
            note
        } else {
            let list = db.lock().await.list().await?;
            let multi = false;
            crate::skim::open::Iteration::new(list, db.clone(), multi, external_commands.clone())
                .run()?
        }
    };

    let (tree, _) = note.construct_term_tree(HashSet::new(), db).await?;

    println!("{}", tree);

    eprintln!(
        "{}",
        format_two_tokens("printed", &format!("{}", note.name()))
    );
    Ok("success".cyan().to_string())
}
