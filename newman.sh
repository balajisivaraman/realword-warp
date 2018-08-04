#!/bin/bash

newman run api/Conduit.postman_collection.json \
  -e api/Conduit.postman_integration_test_environment.json \
  --global-var "EMAIL=joe@what.com" \
  --global-var "PASSWORD=password"
