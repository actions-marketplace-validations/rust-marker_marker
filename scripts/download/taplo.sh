#!/usr/bin/env bash

set -euo pipefail

script_dir=$(readlink -f $(dirname ${BASH_SOURCE[0]}))

. $script_dir/lib.sh

version=0.8.1

base_url=https://github.com/tamasfe/taplo/releases/download/$version

file_stem=taplo-$os-$arch_rust

download_and_decompress $base_url/$file_stem.gz

mv $file_stem taplo
with_log chmod +x taplo
move_to_path taplo
