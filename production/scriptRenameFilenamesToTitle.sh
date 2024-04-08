#!/bin/bash

# Function to convert uppercase to PascalCase
to_pascal_case() {
    local input="$1"
    local output=""
    local prev_char=""
    local current_char=""

    for ((i=0; i<${#input}; i++)); do
        current_char="${input:i:1}"
        if [[ "$prev_char" =~ [[:alnum:]] && "$current_char" =~ [[:upper:]] ]]; then
            output+=" $current_char"
        elif [[ "$prev_char" =~ [[:alnum:]] && "$current_char" =~ [[:digit:]] ]]; then
            output+="$current_char"
        else
            output+="$current_char"
        fi
        prev_char="$current_char"
    done

    echo "$output" | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) tolower(substr($i,2))}1'
}

# Function to recursively process directory
process_directory() {
    local dir="$1"
    local files=("$dir"/*)
    for file in "${files[@]}"; do
        if [ -d "$file" ]; then
            process_directory "$file"
        elif [ -f "$file" ]; then
            filename=$(basename "$file")
            if [[ "$filename" == *[[:upper:]]* ]]; then
                new_filename=$(to_pascal_case "$filename")
                if [ "$filename" != "$new_filename" ]; then
                    # echo "Renaming '$filename' to '$new_filename'"
                    # mv "$file" "$(dirname "$file")/$new_filename"
                    printf "mv "
                    printf "'$file' \ \n   "
                    printf "'$(dirname "$file")/$new_filename'"
                    printf "\n\n"
                fi
            fi
        fi
    done
}

# Start processing from the current directory
# process_directory "$(pwd)"
process_directory "/mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/Acervos"
