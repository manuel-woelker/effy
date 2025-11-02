#!/usr/bin/env bash

set -euo pipefail

./git-hooks/pre-commit-check.sh

jj desc
jj new
git push origin HEAD:refs/heads/master
git checkout master
git pull