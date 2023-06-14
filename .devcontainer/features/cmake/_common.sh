#!/bin/bash

clear_local_apt_index() (
	set -e

	rm -rf /var/lib/apt/lists/*
	echo '🟩 Cleared local apt index'
)

ensure_apt_packages() (
	set -e

	export DEBIAN_FRONTEND=noninteractive
	if dpkg -s "$@" &>/dev/null; then
		echo "🟦 $@ is already installed"
	else
		if [[ $(find /var/lib/apt/lists/* | wc -l) == 0 ]]; then
			echo '🟪 Updating local apt index...'
			apt-get update -y
			echo '🟩 Updated local apt index'
		fi
		echo "🟪 Installing $@..."
		apt-get install -y --no-install-recommends "$@"
		echo "🟩 Installed $@"
	fi
)
