# Ping Server
Poorly named, Ping Server, is a server and client bundled into one.
Instance can be a server and/or client(s).
The server then exposes Prometheus metrics allowing for alarming.

Pre-compiled docker image available `theempty/ping-server:latest` or with matching version tags in the Cargo.toml.

## Example Server Configuration

```json
{
    "server": {
        "host": "0.0.0.0",
        "port": 9999,
        "key": "some_random_generated_secret_key",
        "read_timeout": 30,
        "wait_seconds": 10
    }
}
```

## Example Client Configuration

Connecting to a server running on ping1.myserver.com:9999 with "key" setting of `some_random_generated_secret_key`.
I then want the server to identify my as "My Remote Site 1".

```json
{
    "peers": [
        {
            "host": "ping1.myserver.com",
            "port": 9999,
            "key": "some_random_generated_secret_key",
            "name": "My Remote Site 1"
        }
    ]
}
```

## Example Dual Configuration

I am a second ping server that is also monitored by the first ping server.

```json
{
    "server": {
        "host": "0.0.0.0",
        "port": 9999,
        "key": "different_random_key",
        "read_timeout": 30,
        "wait_seconds": 10
    },
    "peers": [
        {
            "host": "ping1.myserver.com",
            "port": 9999,
            "key": "some_random_generated_secret_key",
            "name": "ping2.myserver.com"
        }
    ]
}
```

## Example Dual Client Configuration

I connect to ping1 and ping2 servers for higher availability.

```json
{
    "peers": [
        {
            "host": "ping1.myserver.com",
            "port": 9999,
            "key": "some_random_generated_secret_key",
            "name": "My Remote Site 1"
        },
        {
            "host": "ping2.myserver.com",
            "port": 9999,
            "key": "different_random_key",
            "name": "My Remote Site 1"
        },
    ]
}
```

## Example Metrics Output

```
# HELP demo_server_last_seen_seconds seconds since demo_server sent a ping
# TYPE demo_server_last_seen_seconds gauge
demo_server_last_seen_seconds 2
```
