docker build --cache-from homepage:static_env -t homepage:static_env --target static_env . && \
docker build --cache-from homepage:static -t homepage:static --target static_builder . && \
docker build --cache-from homepage:rust_builder -t homepage:rust_builder --target rust_builder . && \
docker run -it homepage:rust_builder cargo test -p vcg -p homepage -p app_plugin --release --lib && \
docker build --cache-from homepage:rust_builder  -t homepage:local .