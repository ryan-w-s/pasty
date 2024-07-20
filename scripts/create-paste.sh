#!/bin/bash

BASE_URL="http://localhost:8080"

# Function to create a new paste
create_paste() {
    local content="$1"
    echo "Creating a new paste with content: $content"
    curl -X POST "$BASE_URL/pastes" \
         -H "Content-Type: application/json" \
         -d "{\"content\":\"$content\"}"
    echo
}

# Function to get all pastes
get_all_pastes() {
    echo "Retrieving all pastes:"
    curl -X GET "$BASE_URL/pastes"
    echo
}

# Function to get a specific paste
get_paste() {
    local id="$1"
    echo "Retrieving paste with ID: $id"
    curl -X GET "$BASE_URL/pastes/$id"
    echo
}

# Main script
echo "Testing Pastebin API"

# Create a new paste
create_paste "This is a test paste"

# Get all pastes
get_all_pastes

# Get a specific paste (replace 1 with an actual ID from your database)
get_paste 1

echo "API test completed"
