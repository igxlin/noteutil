# Noteutil

A tool to help filter notes. So far I only use it personally. Thus the
API is unstable and there will be refactor from time to time. Use it at
your own risk.

## Usage

List periodic journals:

```bash
# List weekly journal of today
noteutil ls --journal-only --date today --period weekly
```

### As Vim Plugin

To use with vim plugin, simply clone this repo under your
`.vim/pack/*/start`. You can also use your favorite plugin manager. For
example,

```vim
# https://github.com/k-takata/minpac
call minpac#add(
    \ 'https://github.com/igxlin/noteutil', 
    \ {'do': { -> noteutil#install() })
```

It is very easy to create your own commands to put the filtered result
into quickfix window.

```vim
let g:noteutil_note_dir = 'path/to/notes'
command! NoteutilToday call noteutil#quickfix(
                \ 'ls --journal-only --date today')
```
