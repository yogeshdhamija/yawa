#!/bin/bash

is_breaking=$(git show --no-patch --no-decorate | tail -n +4 | grep -c '^\s*break:')
is_feat=$(git show --no-patch --no-decorate | tail -n +4 | grep -c '^\s*feat:')
is_fix=$(git show --no-patch --no-decorate | tail -n +4 | grep -c '^\s*fix:')

old_latest_tag=$(git tag -l v\* --sort=-v:refname | head -n1)
diff_since_old_latest_tag=$(git log "${old_latest_tag}"..HEAD --oneline --no-decorate)

new_feats=$(echo "${diff_since_old_latest_tag}" | grep '^[a-z0-9]*\sfeat:' | cut -d\  -f3-)
new_fixes=$(echo "${diff_since_old_latest_tag}" | grep '^[a-z0-9]*\sfix' | cut -d\  -f3-)

if [[ is_breaking -ge 1 ]]
  then
    echo "This is a breaking change commit. Releasing major version.";
    echo "Latest tag: ${old_latest_tag}"
    new_latest_tag=$(echo "${old_latest_tag}" | awk -F. '{ print $1+1".0.0" }')
elif [[ is_feat -ge 1 ]]
  then
    echo "This is a feature commit. Releasing minor version.";
    echo "Latest tag: ${old_latest_tag}"
    new_latest_tag=$(echo "${old_latest_tag}" | awk -F. '{ print $1"."$2+1".0" }')
elif [[ is_fix -ge 1 ]]
  then
    echo "This is a fix commit. Releasing patch version.";
    echo "Latest tag: ${old_latest_tag}"
    new_latest_tag=$(echo "${old_latest_tag}" | awk -F. '{ print $1"."$2"."$3+1 }')
else
  echo "Not a feature or a fix commit."
  exit 0
fi

echo "Tagging: ${new_latest_tag}"

git config user.email "ydhamija96@gmail.com"
git config user.name "CD Github Action, on behalf of Yogesh Dhamija"

echo "Committing version bump Cargo.toml..."
sed -i "s/^version = \".*\"$/version = \"${new_latest_tag:1}\"/" ./Cargo.toml
cargo generate-lockfile
git add ./Cargo.toml ./Cargo.lock
git diff --staged
git commit -m "chore: version bump"

echo "Creating new tag..."
release_notes=$(printf '%s\n\nNew features:\n%s\n\nFixes:\n%s\n\n' "${new_latest_tag}" "${new_feats}" "${new_fixes}")
git tag -a "${new_latest_tag}" -m "${release_notes}"
git push
git push --tags