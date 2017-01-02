# hermes

# Getting Started

0. Download and install docker for mac: https://docs.docker.com/docker-for-mac/

1. Clone the repo
`git clone https://github.com/bwillis/hermes.git /Users/ben/Google\ Drive/Hermes`

2. Build the docker image
`docker build /Users/ben/Google\ Drive//Hermes/docker -t hermes`

3. Run the image mounting the local git repo as the working directory
`docker run -t -i -P --name dev -v /Users/ben/Google\ Drive/Hermes:/hermes hermes`

4. Start the webserver
`cd rust && cargo run`

5. Figure out what port locally is passed through-while the image is running, in the parent shell run:
`docker port XXXXX` (XXXXX is the hash of the running image)

6. Go to http://localhost:YYYY to view the webapp

7. Make a change in node/*, the rebuild the node app:
`npm run build`
