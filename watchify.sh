# npm install -g watchify
# npm install --save babelify babel-preset-react
# npm install --save google-protobuf
watchify -v -t [ babelify --presets [ react ] ] js/*.js -o static/app.js 
