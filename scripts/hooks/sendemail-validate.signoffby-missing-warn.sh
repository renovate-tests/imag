#!/usr/bin/env bash

#
# The following snippet can be used to _WARN_ if a Signed-off-by line is missing
# in the commit message of the patch
#
# Use
#
#   git config sendemail.validate true
#
# and link this script to your git hooks folder to enable.
#

GREEN='\e[0;32m'        # Green
RED='\e[0;31m' # Red
NORMAL='\e[0m' # Text Reset

GREPLINE="^Signed-off-by: $(git config user.name) <$(git config user.email)>"

if [ "$(grep -c "$GREPLINE" "$1")" -lt 1 ]; then
    echo -e >&2 "${RED}Missing Signed-off-by line.${NORMAL}\n"

    # To not only warn, but abort the patch sending, uncomment the next line
    # exit 1
else
    echo -e >&2 "${GREEN}Signed-off-by line found.${NORMAL}\n"
fi

