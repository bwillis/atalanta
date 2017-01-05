FROM scorpil/rust

MAINTAINER Ben Willis

EXPOSE 3456

RUN apt-get update && \
  apt-get install -y git && \
  apt-get install -y  nodejs && \
  apt-get install -y npm

RUN ln -s /usr/bin/nodejs /usr/bin/node

RUN mkdir /atalanta
WORKDIR /atalanta

RUN git clone -b master https://github.com/bwillis/atalanta.git /atalanta

# build the rust application
RUN cd rust && cargo build

# build the react application
RUN cd node && npm install
