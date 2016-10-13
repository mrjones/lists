# npm install -g watchify
# npm install --save babelify babel-preset-react
watchify -v -t [ babelify --presets [ react ] ] js/app.js -o static/app.js 
