#!/bin/bash
set -e
source "config.conf"

RSYNC_OPTIONS="-azv --ignore-existing --delete --exclude "/_*""

if [[ "$SOURCE_DIR" != */ ]]; then
    SOURCE_DIR="${SOURCE_DIR}/"
fi

TARGET_DIR="${REMOTE_USER}@${REMOTE_HOST}:${TARGET_DIR}"

echo "Backup started at $(date)"
rsync $RSYNC_OPTIONS "$SOURCE_DIR" "$TARGET_DIR"
echo "Backup completed at $(date)"
