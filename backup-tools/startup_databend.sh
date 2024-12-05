#!/bin/bash
set -e

killall ${DATABEND_QUERY_BIN} || true
killall ${DATABEND_META_BIN} || true
sleep 1

for bin in ${DATABEND_QUERY_BIN} ${DATABEND_META_BIN}; do
	if test -n "$(pgrep $bin)"; then
		echo "The $bin is not killed. force killing."
		killall -9 $bin || true
	fi
done

echo 'Start databend-meta...'
nohup ${DATABEND_META_BIN} -c /tmp/databend-meta-node.toml --single --log-level=ERROR > /dev/null 2>&1 &
echo "Waiting on databend-meta 10 seconds..."
sleep 10

echo 'Start databend-query...'
nohup ${DATABEND_QUERY_BIN} -c /tmp/databend-query-node.toml --internal-enable-sandbox-tenant > /dev/null 2>&1 &
echo "Waiting on databend-query 10 seconds..."
sleep 10
