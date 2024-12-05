#!/bin/bash

# Function to log messages
log_message() {
    local message="$1"
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $message"
}

# Check if the database query command is available
command -v bendsql > /dev/null 2>&1 || { log_message "Error: 'bendsql' command not found. Exiting."; exit 1; }

# Fetch all tables from the system (excluding the 'system' and 'information_schema' databases)
tables=$(echo "SELECT catalog, database, name FROM system.tables WHERE database != 'system' and database != 'information_schema';" | bendsql)
if [ $? -ne 0 ]; then
    log_message "Error: Failed to fetch table information."
    exit 1
fi

# Initialize counters for success and failure
success_count=0
failed_count=0

# Iterate over each table and check its accessibility
while read -r line; do
    # Skip header line or empty lines
    if [[ -n "$line" && "$line" != "catalog	database name" ]]; then
        # Parse catalog, database, and table name
        IFS=$'\t' read -r catalog database table <<< "$line"
        
        # Log the start of the scanning process
        log_message "Scanning table: $catalog.$database.$table"
        
        # Try to scan the table, capture output and error
        output=$(echo "SELECT * FROM \`$catalog\`.\`$database\`.\`$table\` IGNORE_RESULT;" | bendsql 2>&1)
        
        # Check if the scan was successful
        if [ $? -ne 0 ]; then
            # Log the failure and increment failure counter
            log_message "Failed to scan table: $catalog.$database.$table. Reason: $output"
            ((failed_count++))
        else
            # Increment success counter if no error occurred
            ((success_count++))
        fi
    fi
done <<< "$tables"

# Final summary
log_message "Validate complete. Successfully scanned $success_count tables, failed to scan $failed_count tables."

# Exit with appropriate status code
if [ $failed_count -gt 0 ]; then
    exit 1
else
    exit 0
fi
