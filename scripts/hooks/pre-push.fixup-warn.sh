#!/usr/bin/env bash

#
# The following snippet can be used to WARN about "!fixup" / "WIP" / "TMP"
# commits when pushing
#
# Aborting the push is possible
#

remote="$1"
url="$2"

z40=0000000000000000000000000000000000000000

while read local_ref local_sha remote_ref remote_sha
do
    if [ "$local_sha" != $z40 ]
    then
        if [ "$remote_sha" = $z40 ]
        then
            # New branch, examine all commits
            range="$local_sha"
        else
            # Update to existing branch, examine new commits
            range="$remote_sha..$local_sha"
        fi

        # Check for WIP commit
        commit=$(git rev-list -n 1 --grep '^WIP|^TMP|!fixup' "$range")
        if [ -n "$commit" ]
        then
            echo >&2 "Found WIP commit in $local_ref, not pushing"

            # TO NOT ONLY WARN BUT ABORT UNCOMMENT THE NEXT LINE
            # exit 1
        fi
    fi
done

exit 0
