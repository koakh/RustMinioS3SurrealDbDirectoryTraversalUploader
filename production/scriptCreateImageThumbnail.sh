#!/bin/bash

SOURCE_PATH="/home/mario/Downloads/WallPapers"

# find /mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/Acervos -type f -name '*.*' | sed 's|.*\.||' | sort -u > extensions.txt
# aac
# avi
# bmp
# cdr
# doc
# docx
# dwg
# epub
# gif
# jpeg
# jpg
# JPG       for f in *.JPG; do mv "$f" "${f//.JPG/.jpg}"; done
# mp3
# mp4
# mpeg
# ogg
# pdf
# png
# PNG
# ppt
# rtf
# tif
# webm
# wma
# WMA
# wmv

find "${SOURCE_PATH}" -type f \( -name '*.jpg' -o -name '*.JPG' \) \
    -exec sh -c 'for d; do dirname "$d"; done' sh {} + | sort -u -o jpeg_dirs.txt