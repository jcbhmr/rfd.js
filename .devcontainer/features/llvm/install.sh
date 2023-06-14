#!/bin/bash
set -e
source _common.sh

ensure_apt_packages lsb-release wget software-properties-common gnupg

wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh

if [[ -z $VERSION || $VERSION == latest ]]; then
	echo "🟪 Installing LLVM latest"
	./llvm.sh
	echo "🟩 Installed LLVM latest"
else
	echo "🟪 Installing LLVM v$VERSION"
	./llvm.sh "$VERSION"
	echo "🟩 Installed LLVM v$VERSION"
fi

clear_local_apt_index
