#!/bin/bash
set -e
source "config.conf"

echo "Update databend-query-node.toml"
sed -i "s|data_path = \".*\"|data_path = \"${TARGET_DIR}\"|" databend-query-node.toml
echo "Copy databend-query-node.toml to remote host"
scp databend-query-node.toml ${REMOTE_USER}@${REMOTE_HOST}:/tmp/databend-query-node.toml

echo "Update databend-meta-node.toml"
sed -i "s|raft_dir = \".*\"|raft_dir = \"${META_DIR}\"|" databend-meta-node.toml
echo "Copy databend-meta-node.toml to remote host"
scp databend-meta-node.toml ${REMOTE_USER}@${REMOTE_HOST}:/tmp/databend-meta-node.toml

echo "Start databend on remote host"
ssh ${REMOTE_USER}@${REMOTE_HOST} 'bash -s' < <(cat ./config.conf; cat ./startup_databend.sh;)

echo "Validate data on remote host"
ssh ${REMOTE_USER}@${REMOTE_HOST} 'bash -s' < <(cat ./config.conf; cat ./run_sql.sh)

echo "Shutdown databend on remote host"
ssh ${REMOTE_USER}@${REMOTE_HOST} 'bash -s' < <(cat ./config.conf; cat ./shutdown_databend.sh)