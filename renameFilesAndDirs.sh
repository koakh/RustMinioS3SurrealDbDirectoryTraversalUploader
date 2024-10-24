#!/bin/bash

# https://claude.ai/chat/30ccf17f-8173-4917-a479-175656f796a0

# use find Arakén|Araken|Arakèn|Arakem
# cd /mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/AcervosNewCombinedFoldersFinal
# find . -name *Arakén* -type d
# find . -name "Arakén|Araken|Arakèn|Arakem" -type f

# Check if correct number of arguments provided
if [ "$#" -ne 3 ]; then
	echo "Usage: $0 <search_pattern> <replacement_pattern> <type>"
	echo "Example: $0 Arakén Arakém d"
	echo "Types:"
	echo "  d - directories only"
	echo "  f - files only"
	echo "  a - both files and directories"
	exit 1
fi

# Store command line arguments in variables
SEARCH_PATTERN="$1"
REPLACE_PATTERN="$2"
TYPE="$3"
LOG_FILE_PATH="./renameFilesAndDirs.log"

# Validate type argument
case "$TYPE" in
d)
	FIND_TYPE="-type d"
	ITEM_TYPE="directories"
	;;
f)
	FIND_TYPE="-type f"
	ITEM_TYPE="files"
	;;
a)
	FIND_TYPE=""
	ITEM_TYPE="files and directories"
	;;
*)
	echo "Error: Invalid type. Use 'd' for directories, 'f' for files, or 'a' for both."
	exit 1
	;;
esac

# Create temporary file for storing find results
TEMPFILE=$(mktemp)

# Store find results in temporary file
if [ -z "$FIND_TYPE" ]; then
	find . -name "*${SEARCH_PATTERN}*" >"${TEMPFILE}"
else
	find . -name "*${SEARCH_PATTERN}*" $FIND_TYPE >"${TEMPFILE}"
fi

# Check if any matches were found
if [ ! -s "${TEMPFILE}" ]; then
	echo "No matching ${ITEM_TYPE} found for pattern '*${SEARCH_PATTERN}*'"
	rm "${TEMPFILE}"
	exit 0
fi

# Display found items
echo "Found ${ITEM_TYPE} that will be renamed:"
cat "${TEMPFILE}"

# Prompt for confirmation
read -p "Do you want to proceed with renaming these ${ITEM_TYPE}? (y/N): " confirm
if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
	echo "Operation cancelled"
	rm "${TEMPFILE}"
	exit 1
fi

# Perform the renaming
while read item; do
	# Create new name by replacing pattern
	newitem=$(echo "$item" | sed "s/${SEARCH_PATTERN}/${REPLACE_PATTERN}/g")

	# Check if source exists and target doesn't
	if [ -e "$item" ] && [ ! -e "$newitem" ]; then
		echo "Renaming: $item -> $newitem" 2>&1 | tee -a ${LOG_FILE_PATH}
		mv "$item" "$newitem" 2>&1 | tee -a ${LOG_FILE_PATH}
	else
		echo "Skipping: $item (target may already exist)" 2>&1 | tee -a ${LOG_FILE_PATH}
	fi
done <"${TEMPFILE}"

# Clean up
rm "${TEMPFILE}"
echo "Rename operation completed"
