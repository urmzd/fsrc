#!/bin/bash

# Check if VERSION is set
if [ -z "$VERSION" ]; then
    echo "VERSION environment variable is not set."
    exit 1
fi

FILES_TO_UPDATE=(
    "action.yml"
    "examples/example.yml"
)

# Iterate over all files that need to be updated

for file in "${FILES_TO_UPDATE[@]}"; do
    # Check if file is a regular file and not a directory
    if [ -f "$file" ]; then
        # Replace urmzd/embed-md@v(.) with the $VERSION
        sed -i "s/urmzd\/embed-md@v[^ ]*/urmzd\/embed-md@v$VERSION/g" "$file"

        # Replace urmzd/embed-md:(.*) with $VERSION
        sed -i "s/urmzd\/embed-md:[^ ]*/urmzd\/embed-md:$VERSION/g" "$file"
    fi
done

echo "Version update completed."
