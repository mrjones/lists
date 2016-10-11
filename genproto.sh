# Get protc from: https://github.com/google/protobuf/releases
# sudo apt-get install protobuf-compiler
# cargo install protobuf
# ~/src/bin/protoc --proto_path proto --plugin ~/.cargo/bin/protoc-gen-rust --rust_out src/ --js_out=import_style=commonjs,binary:static proto/api.proto
~/src/bin/protoc --proto_path proto --plugin ~/.cargo/bin/protoc-gen-rust --rust_out ./src proto/storage_format.proto


