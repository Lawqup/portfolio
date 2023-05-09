echo "Building"
npx tailwindcss -i ./input.css -o ./style/output.css
trunk build

echo "Transferring files"
rm -rf public/
mkdir public/
cp -r dist/* public/

echo "Deploying to firebase"
firebase deploy
