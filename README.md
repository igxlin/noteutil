# Noteutil

A tool to help filter notes. So far I only use it personally. Thus the
API is unstable and there will be refactor from time to time. Use it at
your own risk.

## Usage

Get expected paths of journals. The path can be used to generated files
or to find related journals.

```bash
noteutil journal --period daily --date today
```

List periodic journals:

```bash
# List weekly journal of today in directory
noteutil ls --journal-only --date today --period weekly path/to/notes
```

### As Vim Plugin

To use with vim plugin, simply clone this repo under your
`.vim/pack/*/start`. You can also use your favorite plugin manager and
then run `:call noteutil#install()` in vim.

It is very easy to create your own commands to put the filtered result
into quickfix window.

```vim
let g:noteutil_note_dir = 'path/to/notes'
command! NoteutilToday call noteutil#quickfix(
                \ 'ls --journal-only --date today')
```
