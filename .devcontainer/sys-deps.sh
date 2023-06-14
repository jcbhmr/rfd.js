#!/bin/bash
set -e

export DEBIAN_FRONTEND=noninteractive
sudo apt-get update -y

sudo apt-get install -y --no-install-recommends \
	libglib2.0-dev libatk1.0-dev libgtk-3-dev

# Clear cache files
sudo rm -rf /var/lib/apt/lists/*
