FROM docker.io/library/rust:1.91-alpine@sha256:8efbfb788786eeb127adc581394349c5fb567712156e0f8c2e499acadbc23756 AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/am-i-isolated /bin/am-i-isolated

FROM scratch AS final
ENTRYPOINT ["/bin/am-i-isolated"]
COPY --from=build /bin/am-i-isolated /bin/am-i-isolated
