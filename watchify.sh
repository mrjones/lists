# npm install -g watchify
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
