# Backup Tools Documentation

This document describes how to use the backup tools for Databend.

## Overview

The backup tools provide functionality to:
- Back up Databend data (not include data of databend-meta) to a remote server
- Validate the backed up data


## Configuration

Edit `config.conf` to set the following parameters:

The configuration file `config.conf` contains the following parameters:

- `SOURCE_DIR`: The local directory path containing Databend data that needs to be backed up
- `REMOTE_USER`: Username for SSH connection to the remote backup server 
- `REMOTE_HOST`: IP address of the remote backup server
- `TARGET_DIR`: Directory on the remote server where backup data will be stored
- `DATABEND_META_BIN`: Path to the databend-meta binary on the remote server
- `DATABEND_QUERY_BIN`: Path to the databend-query binary on the remote server  
- `META_DIR`: Directory on the remote server for storing databend-meta data
## Usage
### Backup Script (backup.sh)

The `backup.sh` script performs the backup of Databend data to the remote server using rsync. It:
- Uses rsync to copy data from SOURCE_DIR to TARGET_DIR on the remote server
- Preserves file attributes and permissions
- Only copies new files
- Deletes files on target that no longer exist in source
- Excludes temporary files (starting with _)

### Validation Script (validate.sh)

The `validate.sh` script validates the backed up data by:
- Starting up a Databend instance using the backed up data and meta
- Scanning all tables to verify data accessibility
- Logging any tables that fail validation
- Providing a summary of successful and failed table scans
- Shutting down the Databend instance

Note that if a small number of tables fail validation, it does not mean the entire backup is unusable. The validation script will report which specific tables failed, allowing you to investigate those tables individually while still being able to use the successfully validated tables.

### Combined Script (backup_and_validate.sh)

The `backup_and_validate.sh` script provides an end-to-end backup solution by:
1. Running backup.sh to copy data to remote server
2. Running validate.sh to validate the backed up data
3. Failing if either the backup or validation step fails
