-module(germinal_rs).
-export([read/0, poll/1, enable_raw_mode/0, disable_raw_mode/0]).
-nifs([read/0, poll/1, enable_raw_mode/0, disable_raw_mode/0]).
-on_load(init/0).

% Change 'priv/libgerminal_rs' to your own path of .so file
init() ->
    ok = erlang:load_nif("priv/libgerminal_rs", 0).

read() ->
    exit(nif_library_not_loaded).

poll(time_ms) ->
    exit(nif_library_not_loaded).

enable_raw_mode() ->
    exit(nif_library_not_loaded).

disable_raw_mode() ->
    exit(nif_library_not_loaded).