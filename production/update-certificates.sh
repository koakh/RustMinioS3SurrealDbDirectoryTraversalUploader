#!/bin/bash

[[ EUID -ne 0 ]] && echo "This script must be run as root." && exit 1

cp /etc/letsencrypt/live/koakh.com/fullchain.pem /etc/letsencrypt/live/koakh.com/privkey.pem ./certs

# checki inside container certificates with
# docker exec -it rust-sveltekit-auth-starter-nginx ls -la /etc/letsencrypt/live/koakh.com/
