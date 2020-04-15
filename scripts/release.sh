#!/usr/bin/env sh
[ -z "$1" ] && echo "Full release: $0 major | minor | patch \nPre-release: $0 premajor | preminor | prepatch --preid=alpha|beta|rc" && exit 1

OLD_VERSION=$(node -p -e "require('./package.json').version")
npm version --git-tag-version=false $@ || exit
VERSION=$(node -p -e "require('./package.json').version")

cd native || exit
sed -i "s/$OLD_VERSION/$VERSION/" Cargo.toml || exit
cargo update || exit
cd ..

npm test || exit

git commit -am "$VERSION" && git tag "v$VERSION" || exit
git push && git push --tags

echo "CI will now create a GitHub release, upload binaries, and publish to npm"
echo "Check https://github.com/covid19risk/tcn-node/actions for progress"