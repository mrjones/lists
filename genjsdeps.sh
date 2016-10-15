# npm install -g browserify
browserify \
    --verbose \
    --require react \
    --require react-dom \
    --require react-router \
    --require jquery \
    --require google-protobuf \
    --outfile static/deps.js \
    --global-transform uglifyify
