#!/bin/bash
pwd

newline() {
    echo -e "\n"
}

declare -r path=$(dirname $(realpath "$0"))

declare -i err=0

for file in "$path"/examples/example{0..20}.txt
do
    echo "$file"
    cat "$file"
    declare expected=$(cat "${file%.*}.expected")
    declare output=$(./run.sh "$file")
    echo "$output"
    echo "$expected"

    if [[ "$output" == "$expected" ]]
    then
        echo -e "\n[OK]"
    else
        echo -e "\n[FAIL]"
        ((err++))
    fi
done

exit $err
