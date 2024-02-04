#!/bin/bash

# Load environment variables from .env file
if [ -f .env ]; then
  export $(cat .env | sed 's/#.*//g' | xargs)
  echo "Environment variables set from .env file."
else
  echo ".env file not found."
fi
