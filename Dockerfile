FROM ubuntu:20.04
ARG DEBIAN_FRONTEND=noninteractive
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN apt-get update \
    && apt-get install -y tzdata \
    && apt-get -y upgrade && apt-get install -y build-essential curl wget git vim libboost-all-dev
RUN curl -sL https://deb.nodesource.com/setup_14.x |  bash \
    && apt-get install -y nodejs \
    && npm install -y -g ssvmup --unsafe-perm \
    && npm install -y ssvm \
    && npm install express express-fileupload
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

###########################################################################################

RUN mkdir -p /root/workspace
WORKDIR /root/workspace
RUN rm -f ssvm-nodejs-starter
RUN git clone https://github.com/kosinix/raster.git
RUN git clone https://github.com/fanofanagabriel/ssvm-nodejs-starter.git
WORKDIR /root/workspace/ssvm-nodejs-starter
RUN ssvmup build
WORKDIR /root/workspace/ssvm-nodejs-starter/
RUN npm install
WORKDIR /root/workspace/ssvm-nodejs-starter/node/
CMD node index.js

