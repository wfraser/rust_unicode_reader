#!/bin/bash

set -o errexit -o nounset

if [ "$TRAVIS_BRANCH" != "master" ]; then
    echo "This commit was made against the $TRAVIS_BRANCH branch an not master. Not deploying."
    exit 0
fi

rev=$(git rev-parse --short HEAD)

cd target/doc

git init
git config user.name "Bill Fraser"
git config user.email "wfraser@codewise.org"

git remote add upstream "https://$GH_TOKEN@github.com/wfraser/rust_unicode_reader.git"
git fetch upstream
git reset upstream/gh-pages

touch .

git add -A .
git commit -m "rebuild of pages at ${rev}"
git push -q upstream HEAD:gh-pages

