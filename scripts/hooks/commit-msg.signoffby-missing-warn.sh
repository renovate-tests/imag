#!/usr/bin/env bash

#
# The following snippet can be used to _WARN_ if a Signed-off-by line is missing
# in the commit message
#

RED='\e[0;31m' # Red
NORMAL='\e[0m' # Text Reset

if [ "1" != "$(grep -c '^Signed-off-by: ' "$1")" ]; then
    echo -e >&2 "${RED}Missing Signed-off-by line.${NORMAL}\n"

    # To not only warn, but abort the commit, uncomment the next line
    # exit 1
fi

