#!/bin/bash

set -ex

platform=$1
target=$2

rm -rf tmp
mkdir tmp
mkdir -p dist

tag=dev
if [[ $GITHUB_REF == refs/tags/TOS-Builder-* ]]; then
  tag=${GITHUB_REF:21}
fi

bin_pkgname=tosbuilder

mkdir tmp/$bin_pkgname
cp LICENSE README.md README_zh-CN.md tmp/$bin_pkgname

fmt=tar
if [ "$platform" = "x86_64-windows" ]; then
  cp target/release/tosbuilder.exe tmp/$bin_pkgname
  fmt=zip
elif [ "$target" = "" ]; then
  cp target/release/tosbuilder tmp/$bin_pkgname
else
  cp target/$target/release/tosbuilder tmp/$bin_pkgname
fi


mktarball() {
  dir=$1
  if [ "$fmt" = "tar" ]; then
    tar czvf dist/$dir.tar.gz -C tmp $dir
  else
    # Note that this runs on Windows, and it looks like GitHub Actions doesn't
    # have a `zip` tool there, so we use something else
    (cd tmp && 7z a ../dist/$dir.zip $dir/)
  fi
}

mktarball $bin_pkgname