#!/usr/bin/env bash
sh ./_scripts/build.sh
docker run -d -p 4114:4114 --name rustic-auth-deployed rustic-auth:latest
