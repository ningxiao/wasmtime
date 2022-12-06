#!/bin/bash
cur_dir=$(pwd)
function add_sum() {
    local x=$(($1))
    local y=$(($2))
    local result=$(wasmer run $cur_dir/target/wasm32-wasi/release/hello_world.wasm --invoke add_sum $x $y 2>/dev/null)
    echo "$result"
}
# main
echo $cur_dir
for num in "27 6" "6 27" "42 12" "33 44"; do
    set -- $num
    echo "add_sum($1, $2) = $(add_sum "$1" "$2")"
done