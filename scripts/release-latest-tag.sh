#!/bin/bash

latest_tag=$(git tag -l v\* --sort=-v:refname | head -n1)
is_already_released=$(gh release list | grep -c "${latest_tag}")

if [[ is_already_released -ge 1 ]]
  then
    echo "Latest tag already released. Exiting with error..."
    exit 1
fi

notes=$(git tag -l --format="%(contents:body)" "${latest_tag}")

echo "Releasing: ${latest_tag}";
gh release create "${latest_tag}" --generate-notes --notes "${notes}"