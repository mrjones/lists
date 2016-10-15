# npm install -g watchify
# npm install --save babelify babel-preset-react
# npm install --save google-protobuf
watchify \
    --verbose \
    --transform [ babelify --presets [ react ] ] \
    --external react \
    --external react-router \
    --external react-dom \
    --external google-protobuf \
    --external jquery \
    --outfile static/app.js \
    js/*.js
