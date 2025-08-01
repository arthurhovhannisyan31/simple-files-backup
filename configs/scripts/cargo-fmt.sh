#!/bin/bash

. "./.husky/configs/colors"

cargo fmt -- --check

latest_exit_status=$?

if [[ ${latest_exit_status} -ne 0 ]]; then
  printf "⛔ ${RED} There are some code style issues, run 'cargo fmt' first \n\n${NC}"
  exit 1
fi

exit 0
