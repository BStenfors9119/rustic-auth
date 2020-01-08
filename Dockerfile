FROM rust:1.39.0-stretch

RUN apt-get update
RUN apt-get install file git gzip sudo -y

RUN PROTOC_ZIP=protoc-3.7.1-linux-x86_64.zip
RUN curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v3.11.1/protoc-3.11.1-linux-x86_64.zip
RUN sudo unzip -o protoc-3.11.1-linux-x86_64.zip -d /usr/local bin/protoc
RUN sudo unzip -o protoc-3.11.1-linux-x86_64.zip -d /usr/local 'include/*'
RUN rm -f protoc-3.11.1-linux-x86_64.zip

RUN USER=root cargo new --bin rustic-auth
WORKDIR /rustic-auth

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./.env.stage ./.env

RUN pwd
RUN ls -al src
RUN ls target/release
RUN rm target/release/deps/rustic_auth*
RUN cargo build --release

EXPOSE 4114

CMD target/release/rustic_auth
