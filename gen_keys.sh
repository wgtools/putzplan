#!/bin/bash
key=$(openssl rand -base64 32)
sed -i 's/^secret_key = ".*"//' Rocket.toml
echo "secret_key = \"$key\"">>Rocket.toml
