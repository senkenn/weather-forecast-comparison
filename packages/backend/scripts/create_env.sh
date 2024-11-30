#!/bin/bash

echo "AWS_ACCESS_KEY_ID=$(aws configure get aws_access_key_id)" >>.env
echo "AWS_SECRET_ACCESS_KEY=$(aws configure get aws_secret_access_key)" >>.env
