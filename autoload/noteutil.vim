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

" Populate backlinks of current document in quickfix
function! noteutil#backlinks(...) abort
    let l:opt = extend(copy(get(a:000, 0, {})), {
                \ 'jump': v:false,
                \ }, 'keep')
    call s:quickfix_populate(noteutil#exec(
                \ 'note --link-to ' . expand('%:p:S')), l:opt)
endfunction

" Open the first file of the command
function! noteutil#open(cmd_args, ...) abort
    let l:cmd_args = [a:cmd_args]
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

function! s:complete_find_start()
    let l:lnum = line('.')
    let l:line = getline('.')
    let b:noteutil_compl_context = ''

    if l:line =~# '^.*\[[^\[\]]*$'
        let b:noteutil_compl_context = 'link'
        return matchend(l:line, '^.*\[') - 1
    endif

    return -1
endfunction

function! noteutil#complete(findstart, base)
    if a:findstart
        return s:complete_find_start()
    endif

    let results = []
    if b:noteutil_compl_context ==# 'link' && exists('b:noteutil_compl_cached_links')
        let results = b:noteutil_compl_cached_links
    endif

    return matchfuzzy(results, a:base)
endfunction

function! noteutil#update_markdown()
    let cmd = 'noteutil note'
            \ . ' --format "[%(title)](%(filepath))"'
            \ . ' --relative-to ' . expand('%:p')
    if !exists('b:noteutil_job_update_markdown_links')
        let b:noteutil_job_update_markdown_links = job_start(cmd, {'close_cb': 's:cb_update_markdown_links'})
    endif
endfunction

function! s:cb_update_markdown_links(channel)
    let lines = []
    while ch_status(a:channel, {'part': 'out'}) == 'buffered'
        let lines += [ch_read(a:channel)]
    endwhile
    let b:noteutil_compl_cached_links = lines
    unlet b:noteutil_job_update_markdown_links
endfunction
