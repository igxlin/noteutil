let s:note_dir = get(g:, 'noteutil_note_dir', getcwd())

let s:prog = 'noteutil'
let s:base_dir = expand('<sfile>:h:h')

function! noteutil#install()
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
    let l:cmd = join([s:prog, a:args])
    let l:output = systemlist(l:cmd)
    if v:shell_error
        call s:warn(join(l:output, "\n"))
        throw 'Failed to execute command ' . l:cmd
    endif

    return l:output
endfunction

function! noteutil#quickfix(args) abort
    call s:quickfix_populate(noteutil#exec(a:args))
    call s:quickfix_toggle()
endfunction

" Open the first file of the command
function! noteutil#open(cmd_args, ...) abort
    let l:cmd_args = [a:cmd_args]
    if s:note_dir !=# ''
        let l:cmd_args += ['--root-dir', s:note_dir]
    endif
    let l:output = noteutil#exec(join(l:cmd_args))

    call s:quickfix_populate(l:output, {'jump': v:true})
endfunction

function! s:warn(msg) abort
    echohl WarningMsg
    for line in a:msg->split("\n")
        echom line
    endfor
    echohl None
endfunction

function! s:quickfix_populate(data, ...) abort
    let l:opt = extend(copy(get(a:000, 0, {})), {
                \ 'jump': v:false,
                \ }, 'keep')

    let l:efm = &errorformat
    set errorformat=%f
    execute (l:opt.jump ? 'cexpr' : 'cgetexpr') 'a:data'
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
