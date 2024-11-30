#!/bin/bash

# set vite aws env
echo "VITE_AWS_ACCESS_KEY_ID=$(aws configure get aws_access_key_id)" >>.env
echo "VITE_AWS_SECRET_ACCESS_KEY=$(aws configure get aws_secret_access_key)" >>.env
