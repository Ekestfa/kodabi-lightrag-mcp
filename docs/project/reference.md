# How MCP Server Works with SSE: Step-by-Step Guide

Server-Sent Events (SSE) is a critical transport mechanism that enables the MCP Server to communicate with clients in real-time, allowing for continuous data streaming from the server to the client. This integration is foundational to the Model Context Protocol (MCP), providing a reliable, unidirectional channel for real-time updates.

Below is a detailed, step-by-step guide on how MCP Server works with SSE, based on the technical specifications and implementation patterns described in the provided context.

---

## Step 1: Establish the Connection Using HTTP with SSE

The MCP Server uses **Streamable HTTP** (a modern replacement for HTTP+SSE) to establish a secure, real-time connection with clients. This transport is preferred over the older `stdio` method due to its scalability and remote accessibility.

- The client initiates a connection by sending an **HTTP GET request** to the MCP server endpoint, such as `https://your-mcp-server.com/mcp`.
- The request must include the header `Accept: text/event-stream` to signal the client's intent to use SSE.
- The server responds with a `200 OK` status and sets the `Content-Type: text/event-stream` header, indicating that it will begin streaming events.

> This step ensures that the client is properly configured to receive a continuous stream of messages from the server, as defined in the MCP transport specification.

---

## Step 2: Initialize the Protocol and Negotiate Capabilities

Before any data is exchanged, the client and server perform an initialization handshake to establish compatibility and define the session parameters.

- The client sends an **`initialize`** request to the server, which includes:
  - A `protocolVersion` field (e.g., `2025-06-18`) to ensure version compatibility.
  - A `capabilities` object that lists supported features (e.g., tools, prompts).
  - A `clientInfo` object containing the client name and version.

- The server responds with an `initializeResult` that includes:
  - A `serverInfo` object with the server name and version.
  - A `sessionID` (a globally unique, cryptographically secure identifier, such as a UUID) that will be used for all subsequent requests.

> This step is critical for ensuring that both the client and server agree on the protocol version and capabilities, and it establishes a secure session context.

---

## Step 3: The Server Streams Data via SSE Events

Once the connection is initialized, the MCP Server begins streaming real-time updates to the client using SSE. This is where the core functionality of real-time interaction occurs.

- The server maintains an open TCP connection to the client and begins sending data in the form of **SSE events**.
- Each event is formatted as a line in the response stream, starting with `data: ` followed by valid JSON content.
- The events may include:
  - Tool execution results (e.g., weather data, search results).
  - Notifications (e.g., "server status: OK").
  - Related requests or notifications that precede the final response.

> Example of an SSE event:
> ```
> data: {"result": {"city": "Paris", "temperature": 22, "condition": "Sunny"}}
> ```

> The client parses these events line-by-line, extracting valid JSON messages and processing them in real time.

---

## Step 4: Handle Tool Calls and Return Results

When a user or AI agent requests a tool (e.g., "get weather for Paris"), the MCP Server executes the corresponding function and returns the result via the SSE stream.

- The client sends a **tool call** using a JSON-RPC request via HTTP POST to the server.
- The server processes the request, executes the tool (e.g., calls a weather API), and returns the result in a structured format.
- The result is then sent back to the client as a valid JSON message in the SSE stream.

> This allows AI agents to perform actions like querying external data sources, without requiring the client to maintain a persistent connection or poll for updates.

---

## Step 5: Manage Session Lifecycle and Handle Errors

The MCP Server ensures reliable communication through proper session management and error handling.

- **Session ID**: The server assigns a session ID during initialization. All subsequent requests must include this ID in the `Mcp-Session-Id` header.
- **Error Handling**: If the server encounters an error (e.g., invalid input, API failure), it sends an error message in the SSE stream with a `result` field containing the error details.
- **Disconnection**: If the client disconnects, the server does not interpret this as cancellation. Instead, it may send a `CancelledNotification` to explicitly terminate the session.
- **Resumability**: If the client reconnects after a disconnection, it can send a `Last-Event-ID` header to request the server to resume from that point, ensuring no message loss.

> This mechanism ensures that the communication remains resilient and reliable, even in the face of network instability.

---

## Step 6: Support for Multiple Clients and Concurrent Streams

The MCP Server can support multiple clients simultaneously, each with its own dedicated SSE stream.

- The server ensures that each message is sent to only one stream, preventing message mixing.
- Clients can maintain multiple streams for different tools or services.
- This allows for rich, parallel interactions—such as one client receiving weather updates while another receives news alerts—without interference.

> This scalability makes the MCP Server ideal for use in complex, real-time applications like AI assistants, chatbots, and real-time dashboards.

---

## Step 7: Secure Communication with HTTPS

All MCP Server endpoints must use **HTTPS** to ensure data confidentiality and integrity.

- The server binds to port 443 (the standard HTTPS port) and uses TLS to encrypt all data in transit.
- This protects sensitive information such as authentication tokens, user inputs, and tool results from eavesdropping and man-in-the-middle attacks.

> This is a mandatory requirement for production deployments and aligns with security best practices in modern web applications.

---

## Practical Example: Building an MCP Server with SSE

To implement this in practice:

1. **Use the `mcp-framework` CLI** to generate a basic server:
   ```bash
   npm i -g mcp-framework
   mcp create weather-http-server --http --port 1337 --cors
   cd weather-http-server
   mcp add tool weather
   ```

2. **Define a tool** in `src/tools/WeatherTool.ts` that returns weather data.

3. **Start the server**:
   ```bash
   npm run build && npm start
   ```
   The server is now accessible at `http://localhost:1337/mcp`.

4. **Connect a client** (e.g., Cursor or a custom Python client) to this endpoint using SSE.

5. **Send a tool call** like `@weather({\"city\": \"Paris\"})` and receive the result in real time.

---

## Summary

MCP Server leverages Server-Sent Events (SSE) to deliver real-time, unidirectional data streaming from the server to the client. This integration enables AI agents to receive live updates, execute tools, and interact with external systems in a seamless, real-time manner.

Key advantages include:
- Real-time responsiveness.
- Scalability through HTTP-based communication.
- Secure, encrypted transport via HTTPS.
- Resilience through session management and resumability.
- Compatibility with a wide range of clients and tools.

This architecture forms the backbone of modern AI agent systems that require real-time interaction with external data sources.

---

### References

- [1] https://dev.to/docteurrs/build-an-mcp-client-because-copy-pasting-json-isnt-engineering-3d9k
- [2] https://modelcontextprotocol.io/specification/2025-06-18/basic/transports
- [3] https://composio.dev/blog/mcp-server-step-by-step-guide-to-building-from-scrtch
- [4] https://medium.com/@matteo28/how-to-build-a-rag-mcp-server-3c514a265207