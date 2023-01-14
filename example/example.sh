cargo build --release

git init --initial-branch main
echo "" >> ./.git/config
cat .gitconfig >> ./.git/config
echo ""

mkdir test
cp ./resource/git.png sample.png
git add sample.png
git commit -m "commit1: git.png to sample"
echo ""

git switch --create rust
cp ./resource/rust.png sample.png
git add sample.png
git commit -m "commit2: rust.png to sample"
echo ""

git switch main
git switch --create ferris
cp ./resource/ferris.png sample.png
git add sample.png
git commit -m "commit3: ferris.png to sample"
echo ""

git switch main
git merge rust
git merge ferris
