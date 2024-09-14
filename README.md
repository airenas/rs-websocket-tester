[![rust](https://github.com/airenas/rs-websocket-tester/actions/workflows/rust.yml/badge.svg)](https://github.com/airenas/rs-websocket-tester/actions/workflows/rust.yml)[![docker](https://github.com/airenas/rs-websocket-tester/actions/workflows/docker.yml/badge.svg)](https://github.com/airenas/rs-websocket-tester/actions/workflows/docker.yml)

# rs-websocket-tester
Simple client and server for Websocket testing packed in Docker.

## About
I run into the case when I did microservice deployments, and communication using websockets was unstable. I did not find a tool that simple initiates connection and communicates for a longer period of time. So I quickly created one.

## Samples

Test if client can access server `<SERVER>` using websocket protocol on the port `<PORT>`. **The only dependency required on both machines: DOCKER!**

### Run on server machine
Start server listening for websocket connections on `<PORT>`:

`docker run -it --rm -p <PORT>:8000 airenas/rs-ws-server:0.1`

Output of server:

```log
2024-09-14T10:10:22.255521Z  INFO rs_ws_server: Starting websocket server    
2024-09-14T10:10:22.255580Z  INFO rs_ws_server: Version      : v0.1.0-bb4aa77    
2024-09-14T10:10:22.255590Z  INFO rs_ws_server: Port         : 8000    
2024-09-14T10:10:22.279152Z  INFO rs_ws_server: Public IP    : xx.xx.xx.xx    
2024-09-14T10:10:22.279247Z  INFO rs_ws_server: Listening on : 0.0.0.0:8000 

```
### Run on client machine

Run on client machine to start connection and sending sample messages every 200ms:

`docker run -it --rm airenas/rs-ws-client:0.1 --url=ws://<SERVER>:<PORT>`

Output of the client:
```log
2024-09-14T10:13:35.751485Z  INFO rs_ws_client: Starting websocket client    
2024-09-14T10:13:35.751504Z  INFO rs_ws_client: Version      : v0.1.0-bb4aa77    
2024-09-14T10:13:35.751510Z  INFO rs_ws_client: URL          : ws://xx.xx.xx.xx:9000    
2024-09-14T10:13:35.751534Z  INFO rs_ws_client: Connecting to WebSocket: ws://xx.xx.xx.xx:9000/    
2024-09-14T10:13:35.764364Z  INFO rs_ws_client: Successfully connected to WebSocket! ws://xx.xx.xx.xx:9000/    
2024-09-14T10:13:35.764382Z  INFO rs_ws_client: WebSocket connection established    
2024-09-14T10:13:35.765549Z  INFO rs_ws_client: Sent: 0    
2024-09-14T10:13:35.770408Z  INFO rs_ws_client: Received: 5.98ms. Response to Message ID: 0    
2024-09-14T10:13:35.965079Z  INFO rs_ws_client: Sent: 1    
2024-09-14T10:13:35.970793Z  INFO rs_ws_client: Received: 200.38ms. Response to Message ID: 1    
2024-09-14T10:13:36.165814Z  INFO rs_ws_client: Sent: 2    
2024-09-14T10:13:36.175130Z  INFO rs_ws_client: Received: 204.34ms. Response to Message ID: 2    
2024-09-14T10:13:36.365503Z  INFO rs_ws_client: Sent: 3    
2024-09-14T10:13:36.370500Z  INFO rs_ws_client: Received: 195.37ms. Response to Message ID: 3
```

*Note: localhost for the client running in docker is not the same as machine's localhost. So simple testing `--url=ws://localhost:<PORT>` - won't succeed*


