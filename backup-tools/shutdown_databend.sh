#!/bin/bash
set -e

killall ${DATABEND_QUERY_BIN} || true
killall ${DATABEND_META_BIN} || true