#! /bin/bash

url="http://127.0.0.1:8000/api/v1"

# Verify if the file exists
if [[ ! -f "$1" ]]; then
    echo "File $1 does not exist"
    exit 1
fi

# Read the file line by line
while IFS= read -r ip; do
    # Send the IP to the URL
    response=$(curl -s "${url}/${ip}")
    # Print the response
    echo "Response for $ip: $response"
    #sleep 2 # Sleep 2 seconds, to avoid being blocked with 429 too many requests
done < "$1"
