#!/bin/bash
set -e
source "config.conf"

./backup.sh
./validate.sh


