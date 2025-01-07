#! /bin/bash

echo -e "Creating mongodb root password\n"
openssl rand -base64 48 | tr -dc 'a-zA-Z0-9' | head -c 48

echo -e "Now change the password in config.toml and compose.yml\n"
