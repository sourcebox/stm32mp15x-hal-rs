set confirm off
set remotetimeout 10
set pagination off

target extended-remote :3333
set mem inaccessible-by-default off

monitor reset
monitor halt
load
continue
