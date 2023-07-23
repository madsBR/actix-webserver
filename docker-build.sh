docker build --cache-from homepage:static -t homepage:static --target static_builder . && \
docker build --cache-from homepage:rust_builder -t homepage:rust_builder --target rust_builder . && \
docker build --cache-from homepage:rust_builder  -t homepage:local .