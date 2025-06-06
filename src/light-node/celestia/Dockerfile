ARG GO_VERSION
FROM golang:${GO_VERSION}

ARG REPO
ARG PROJECT_FOLDER
ARG CHAIN_ID
ARG DAEMON_NAME
ARG DAEMON_HOME
ARG DAEMON_VERSION
ARG DEFAULT_KEY_NAME
ARG DEFAULT_RPC_PORT
ARG NODE_TYPE
ARG RPC_ENDPOINT
ARG TRUSTED_BLOCK_HASH
ARG TRUSTED_BLOCK_HEIGHT

WORKDIR /root

RUN apt update && \
    apt install -y curl jq git build-essential tar wget clang pkg-config libssl-dev bsdmainutils make ncdu gcc chrony liblz4-tool htop net-tools lsof aria2 && \
    rm -rf /var/lib/apt/lists/*

ENV REPO=${REPO}
ENV PROJECT_FOLDER=${PROJECT_FOLDER}
ENV CHAIN_ID=${CHAIN_ID}
ENV DAEMON_NAME=${DAEMON_NAME}
ENV DAEMON_HOME=${DAEMON_HOME}
ENV DAEMON_VERSION=${DAEMON_VERSION}
ENV DEFAULT_KEY_NAME=${DEFAULT_KEY_NAME}
ENV DEFAULT_RPC_PORT=${DEFAULT_RPC_PORT}
ENV NODE_TYPE=${NODE_TYPE}
ENV RPC_ENDPOINT=${RPC_ENDPOINT}
ENV TRUSTED_BLOCK_HASH=${TRUSTED_BLOCK_HASH}
ENV TRUSTED_BLOCK_HEIGHT=${TRUSTED_BLOCK_HEIGHT}

RUN echo "export REPO=${REPO}" >> $HOME/.bash_profile && \
    echo "export PROJECT_FOLDER=${PROJECT_FOLDER}" >> $HOME/.bash_profile && \
    echo "export CHAIN_ID=${CHAIN_ID}" >> $HOME/.bash_profile && \
    echo "export DAEMON_NAME=${DAEMON_NAME}" >> $HOME/.bash_profile && \
    echo "export DAEMON_HOME=${DAEMON_HOME}" >> $HOME/.bash_profile && \
    echo "export DAEMON_VERSION=${DAEMON_VERSION}" >> $HOME/.bash_profile && \
    echo "export DEFAULT_KEY_NAME=${DEFAULT_KEY_NAME}" >> $HOME/.bash_profile && \
    echo "export DEFAULT_RPC_PORT=${DEFAULT_RPC_PORT}" >> $HOME/.bash_profile && \
    echo "export NODE_TYPE=${NODE_TYPE}" >> $HOME/.bash_profile && \
    echo "export RPC_ENDPOINT=${RPC_ENDPOINT}" >> $HOME/.bash_profile && \
    echo "export TRUSTED_BLOCK_HASH=${TRUSTED_BLOCK_HASH}" >> $HOME/.bash_profile && \
    echo "export TRUSTED_BLOCK_HEIGHT=${TRUSTED_BLOCK_HEIGHT}" >> $HOME/.bash_profile

RUN git clone ${REPO} ${PROJECT_FOLDER}

WORKDIR /root/${PROJECT_FOLDER}

RUN git checkout ${DAEMON_VERSION}
RUN make build

RUN mv build/${DAEMON_NAME} /usr/local/bin/
RUN rm -rf build

RUN make cel-key
RUN mv cel-key /usr/local/bin/

WORKDIR /root

RUN celestia ${NODE_TYPE} init --p2p.network ${CHAIN_ID}

RUN sed -i "s|SampleFrom = .*|SampleFrom = ${TRUSTED_BLOCK_HEIGHT}|" ${DAEMON_HOME}/config.toml

CMD celestia light start \
    --core.ip ${RPC_ENDPOINT} \
    --headers.trusted-hash ${TRUSTED_BLOCK_HASH} \
    --keyring.keyname ${DEFAULT_KEY_NAME} \
    --rpc.addr http://0.0.0.0 \
    --rpc.port ${DEFAULT_RPC_PORT} \
    --p2p.network ${CHAIN_ID} \
    --rpc.skip-auth
