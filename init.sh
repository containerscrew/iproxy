#! /bin/bash

echo -e "Generating a random password for the database...\n"
openssl rand -base64 48

cp .env-example .env
echo -e "\nPlease update the compose.yml file with the database password generated above"
echo -e "Please update config.toml with the same username/password\n"
