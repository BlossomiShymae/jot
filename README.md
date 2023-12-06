# jot

jot is an experimental CLI tool to quickly jot notes by name. Nothing more. :>

## Environment variables

```bash
JOT_PATH    # Path for jot files. Will default to current directory if not set.
EDITOR      # The text editor to use for creating and editing notes.
```

## Commands

```bash
note      <NAME>   Read a stored note.
list      
create    <NAME>   Create a note and opens the editor.
edit      <NAME>   Edit a note via the editor.
remove    <NAME>   Remove a note.
help               Print this message or the help of the given subcommand(s)
```