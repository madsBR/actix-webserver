#!/bin/bash

docker run \
--name container \
--mount type=bind,source=./actix-engine/engine/log,target=/app/log \
-p 80:8080 \
 $1