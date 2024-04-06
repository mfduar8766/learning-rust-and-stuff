#! /bin/bash

function addcommitpush () {

echo "Adding and commiting files..."  &

REGEX='^(feature|bugfix|refactor)\/[A-Za-z].*$'

message=\'"$@"\'

current=$(git branch | grep "*" | cut -b 3-)

echo $current

if [[ $current =~ $REGEX ]]; then
  git add --all :/ && git commit -a -m "$message"
else
  echo "Branch name must be prefixed with feature/, refactor/, bugfix/..."
  exit 1
fi

echo "You sure you wanna push? (y/n)"
read yn

if [ $yn = y ]; then
  git push origin $current
else
  echo "Could not push to branch: $current ..."
  exit 1
fi
}
addcommitpush $1
