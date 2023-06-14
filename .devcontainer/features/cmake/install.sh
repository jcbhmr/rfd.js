#!/bin/bash
set -e
source _common.sh

ensure_apt_packages curl ca-certificates jq

if [[ -z $VERSION || $VERSION == latest ]]; then
	echo "🟪 Fetching latest CMake release..."
	curl -fsSLo latest-release.json https://api.github.com/repos/Kitware/CMake/releases/latest
	version=$(jq -r .tag_name latest-release.json | sed 's/^v//')
	echo "🟦 Using latest CMake release: v$version"
else
	version="$VERSION"
	echo "🟦 Using CMake release: v$version"
fi

echo "🟪 Installing CMake v$version..."
curl -fsSLo cmake.sh "https://github.com/Kitware/CMake/releases/download/v$version/cmake-$version-linux-x86_64.sh"
chmod +x cmake.sh
./cmake.sh --skip-license --prefix=/usr/local
echo "🟩 Installed CMake v$version"

if [[ $INSTALLBUILDESSENTIAL == true ]]; then
	echo "🟪 Installing build-essential..."
	ensure_apt_packages build-essential
	echo "🟩 Installed build-essential"
fi

if [[ $INSTALLNINJA == true ]]; then
	echo "🟪 Installing Ninja..."
	ensure_apt_packages ninja-build
	echo "🟩 Installed Ninja"
fi

clear_local_apt_index
