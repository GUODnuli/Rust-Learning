#! /bin/bash

for dir in */; do
    if [ -d "${dir}/.git" ]; then
        rm -rf "${dir}/.git"
    fi

    if [ -f "${dir}/.gitignore" ]; then
        rm "${dir}/.gitignore"
    fi
done
