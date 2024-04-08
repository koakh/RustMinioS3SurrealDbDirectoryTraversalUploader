#!/bin/bash

# chatgpt
# https://chat.openai.com/c/0d9c1c71-f697-404b-9bbb-1cf18e754659

# Define the directory to search
directory="/mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/Acervos"

# Navigate to the directory
cd "$directory" || exit

# Find all files recursively and rename their extensions to lowercase
find . -type f -exec sh -c '
	for file; do
		newname="${file%.*}"   # Get the file name without extension
		ext="${file##*.}"      # Get the extension
		newext=$(echo "$ext" | tr "[:upper:]" "[:lower:]")  # Convert extension to lowercase
		if [ "$ext" != "$newext" ]; then
			# use echo here first to view rename commands and target files		
			echo mv "$file" "$newname.$newext"   # Rename the file
		fi
	done
' sh {} +

