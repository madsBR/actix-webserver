#!/bin/bash
docker login -u madsbr1990
docker pull madsbr1990/homepage:latest
docker run \
--name container \
--mount type=bind,source=/home/website-host/log,target=/app/log \
-p 80:8080 \
 madsbr1990/homepage:latest