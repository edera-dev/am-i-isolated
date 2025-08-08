FROM docker.io/library/rust:1.89-alpine@sha256:a2b5309bf382aaa88d283f0ced3998717f15d8c88b4cc1391ce1c95a61b28359 AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/am-i-isolated /bin/am-i-isolated

FROM scratch AS final
ENTRYPOINT ["/bin/am-i-isolated"]
COPY --from=build /bin/am-i-isolated /bin/am-i-isolated
