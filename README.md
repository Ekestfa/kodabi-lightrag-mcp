# kodabi-lightrag-mcp
The LightRAG MCP Server is a Model Context Protocol (MCP) server that provides seamless integration with LightRAG's HTTP APIs, enabling AI assistants and language models to interact with Retrieval-Augmented Generation capabilities through a standardized interface.

Build and test locally (commands)
Build the Docker image:
```dockerfile
docker build -t kodabi_lightrag_mcp:latest .
```

Run the container (port 8080 mapping):
```dockerfile
docker run --rm -p 8080:8080 --name kodabi_test kodabi_lightrag_mcp:latest
```
Test the health endpoint:
```dockerfile
curl -v http://127.0.0.1:8080/health
```
If the build fails due to missing runtime library version mismatch
- Inspect the binary's dynamic dependencies in the builder image:

```dockerfile
# inside builder container or locally (if you copy binary)
ldd target/release/kodabi_lightrag_mcp
```
