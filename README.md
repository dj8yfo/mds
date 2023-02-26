
# Get focused

![Alt](./logo.gif "Getting focused in your own special way")

# Flow

1. Install the beast itself.

```
cargo install --locked mds 
```

2. Install external commands used in default config, besides `firefox`, which is the default browser for opening links, 
by running [install_dependencies.sh](./install_dependencies.sh)
```
wget -O - https://raw.githubusercontent.com/dj8yfo/mds/master/install_dependencies.sh  | bash
```
3. Create config at `$HOME/.config/mds/config.kdl` with [content](./config.kdl).
  1. Edit the folder, where you'd like to put notes on your system. (Replace `/home/user/notes` default value)
4. Check your config got correctly fetched up.
  ```
  mds debug-cfg
  ```
5. Initialize .sqlite database in your notes folder with
  ```
  mds init  
  ```

6. Enjoy:
  ```
  mds -h
  ```

  ```
  Usage: mds <COMMAND>

  Commands:
    debug-cfg  print `Debug` representtion of `config`
    init       `initialize` .sqlite database in notes dir, specified by config
    n          create a `note`
    t          create a `tag` (note without file body)
    l          `link` 2 notes A -> B, selected twice in skim interface
    o          start an infinite skim selection loop to `open` notes
    e          `explore` notes/tags by <c-h> (backlinks) , <c-l> (links forward)
    s          `surf` (fuzzy find) through all `[markdown reference](links)`, 
                               found in all notes, 
                               reachable by forward links from note/tag S, 
                               selected interactively by skim
    unlink     `unlink` 2 notes A -> B, selected twice in skim interface
    help       Print this message or the help of the given subcommand(s)

  Options:
    -h, --help     Print help
    -V, --version  Print version
  ```
