if exists('g:loaded_noteutil')
    finish
endif
let g:loaded_noteutil = 1

let s:note_dir = get(g:, 'noteutil_note_dir', getcwd())

let s:prog = 'noteutil'
let s:base_dir = expand('<sfile>:h:h')

function! noteutil#install()
    if executable(s:prog)
        return
    endif

    if !executable('cargo')
        throw 'cargo not found'
    endif

    call s:warn('Running noteutil installer ...')
    echom system('cargo install --path ' . s:base_dir)
    if v:shell_error
        throw 'Failed to install ' . s:prog
    endif
endfunction

function! noteutil#exec(args) abort
    return systemlist(join([s:prog, a:args, s:note_dir]))
endfunction

function! noteutil#quickfix(args) abort
    call s:quickfix_populate(noteutil#exec(a:args))
    call s:quickfix_toggle()
endfunction

function! s:warn(msg)
    echohl WarningMsg
    echom a:msg
    echohl None
endfunction

function! s:quickfix_populate(data) abort
    let l:efm = &errorformat
    set errorformat=%f
    execute 'cgetexpr' 'a:data'
    let &errorformat = l:efm
endfunction

function! s:quickfix_toggle() abort
    let l:open = (len(getqflist()) > 0)

    if l:open
        execute 'copen'
        return
    endif

    execute 'cclose'
endfunction

