#!/bin/bash

# Set the path to your SQLite database file
DB_PATH="./sqlite:pasty.db"

# Function to clean the database
clean_database() {
    echo "Cleaning the database..."
    sqlite3 "$DB_PATH" <<EOF
DELETE FROM pastes;
VACUUM;
EOF
    echo "Database cleaned successfully."
}

# Function to display current record count
show_record_count() {
    echo "Current record count:"
    sqlite3 "$DB_PATH" "SELECT COUNT(*) FROM pastes;"
}

# Main script
echo "SQLite Database Cleaner"
echo "----------------------"

# Show record count before cleaning
show_record_count

# Prompt user for confirmation
read -p "Do you want to clean the database? (y/n): " confirm

if [[ $confirm == [Yy]* ]]; then
    clean_database
    # Show record count after cleaning
    show_record_count
else
    echo "Operation cancelled."
fi

echo "Script completed."
