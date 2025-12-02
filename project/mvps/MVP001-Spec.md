## Components and Their Relationships
The LightRAG system is composed of multiple interconnected components that work together to enable intelligent, context-aware queries across various domains. The core architecture follows a layered design pattern, with clear boundaries between components and services.
### Central RAG Server
- Acts as the orchestration layer that manages communication between multiple RAG microservices.
- Maintains a configuration registry of available microservices with their port numbers and roles.
- Validates incoming queries and routes them to appropriate microservices based on intent and context.
### RAG Microservices
- Specialized services that handle specific domains (e.g., Software Engineering RAG, History RAG, Product Catalog RAG).
- Each microservice runs on a dedicated port (e.g., 8080, 8081) and exposes a REST API for query processing.
- Implements a consistent interface for retrieving context from its domain-specific vector database.
### Web Server Gateway
- Serves as a secure entry point accessible at `http://[::1]:{PORT}`.
- Accepts HTTP POST requests to the `/central/query` endpoint and validates request structure before forwarding to the RAG system.
- Acts as a middleware layer that enforces security and input validation.
### Vector Database
- Stores and retrieves high-dimensional vector embeddings of documents.
- Provides semantic similarity search capabilities to retrieve relevant context for queries.
- Serves as the foundational data source for all RAG microservices.
### Workflow Engine
- Manages the execution of RAG workflows, including health checks and query routing.
- Coordinates between the central server and microservices to ensure proper sequence and consistency of operations.
### Authentication Layer
- Implements API token and base URL validation for all requests.
- Uses environment variables (RAG_API_TOKEN and RAG_BASE_URL) to authenticate access to the RAG service.
- Ensures secure access to sensitive data and prevents unauthorized usage.
## Behavior and Inter-Module Interactions
The system operates through a series of well-defined interactions that ensure secure, reliable, and context-aware responses. All communication follows a standardized protocol with clear error handling and fallback mechanisms.
### Query Routing Mechanism
When a user submits a query, the central RAG server evaluates the intent and routes it to the appropriate microservice through the following steps:
1. The query is parsed to extract microservice name and port number (for Use Case 1 and Use Case 5).
2. The server validates the microservice name against its configuration registry.
3. The port number is validated to ensure it falls within the acceptable range (8000–9999).
4. If the query requires multiple microservices, the system analyzes the context to determine which services should respond.
5. The query is routed to the relevant microservice(s) via HTTP on their designated ports.
### Health Check Flow
The health check process is a critical pre-condition for any RAG workflow:
1. The system initiates an HTTP GET request to the `/health` endpoint.
2. The server responds with a 200 OK status if operational.
3. If the server is unavailable or returns an error, the workflow is halted immediately.
4. In cases of malformed responses or network timeouts, the system logs warnings and triggers fallback mechanisms (e.g., checking alternative endpoints or restarting the server process).
### Dynamic Routing Logic
For queries that require data from multiple domains, the system employs dynamic routing:
- The query is analyzed to identify key domains (e.g., user history, product catalog, real-time pricing).
- The system evaluates the context to determine which microservices should respond.
- Each microservice is invoked sequentially or in parallel based on the query structure.
- Retrieved data is aggregated and validated for consistency before being returned to the user.
### Error Handling and Fallback Strategies
All components implement comprehensive error handling:
- Invalid inputs trigger immediate error responses with clear messages.
- Failed microservice calls result in partial responses with annotations about missing data.
- Malformed queries are detected early and returned with correction guidance.
- Network failures trigger timeouts and fallback mechanisms to ensure workflow resilience.
## Data Flow and Integration Points
The data flow within the LightRAG system follows a clear path from user input to final response, with well-defined integration points at each stage.
### Request Flow
1. User sends an HTTP request (POST to `/central/query` or GET to `/health`) to the web server at `http://[::1]:{PORT}`.
2. The web server validates the request structure and performs input sanitization.
3. For queries, the request is forwarded to the RAG system's `/query` API.
4. The RAG system retrieves context from its vector database using the query.
5. The response is structured with answer content and source document metadata.
6. The web server returns the final response to the user.
### Integration Points
- **Web Server to RAG System**: Secure HTTP POST to `/query` endpoint with query parameter.
- **Central Server to Microservices**: HTTP POST to specific microservice ports with query data.
- **RAG System to Vector Database**: Internal vector search operations using semantic similarity.
- **Health Check Endpoint**: HTTP GET to `/health` to verify server operational status.
### Data Attributes
All responses include:
- The final answer to the user's query
- Clear attribution of data sources to ensure transparency and traceability.
## Interface Definitions
### API Endpoints and Methods

| Endpoint                | Method | Description                                         | Request Parameters                        | Response Format                                  |
| ----------------------- | ------ | --------------------------------------------------- | ----------------------------------------- | ------------------------------------------------ |
| `/health`               | GET    | Health check endpoint to verify server availability | None                                      | 200 OK (healthy) or 503/404 (unavailable)        |
| `/central/query`        | POST   | Main query endpoint for user input                  | `query` (string)                          | Structured JSON with answer and source documents |
| `/query` (microservice) | POST   | Query endpoint for specific microservices           | `query` (string), `service_name` (string) | Structured JSON with answer and source documents |

### Data Models and Schemas
**RAG `QueryRequest` Schema**
```json
{
  "query": "string",
  "mode": enum: [0=local, 1=global, 2=hybrid, 3=naive, 4=mix (default), 5=bypass],
  "only_need_context": boolean,
  "only_need_prompt": boolean,
  "response_type": string,
  "top_k": integer,
  "chunk_top_k": integer,
  "max_entity_tokens": integer,
  "max_relation_tokens": integer,
  "max_total_tokens": integer,
  "conversation_history": array<object> (Format: [{'role': 'user/assistant', 'content': 'message'}]),
  "user_prompt": string,
  "enable_rerank": boolean,
  "include_references": boolean,
  "stream": boolean
}
```

**RAG `QueryResponse` Schema**
```json
{
  "response": string,
  "references": [
	  {
		  "reference_id": string,
		  "file_path": string
	  }
  ]
}
```

**Central Request Schema**
 ```json
 {
	 "agent_flow": [
		 {
			 "rag_name": string,
			 "rag_query": QueryRequest
		 }
	 ]
 }
 ```

**Central Response Schema**
```json
{
  "answer": "string",
}
```
### Protocol Specifications
- **Communication Protocol**: RESTful HTTP with JSON payloads.
- **Authentication**: API token and base URL via environment variables.
- **Error Handling**: Standardized HTTP status codes with descriptive error messages.
- **Response Format**: Structured JSON with clear separation between answer content and source data.
### Contract Definitions #ask Model-as-Code kullanılabilir mi?
All components adhere to a shared contract:
- The central server maintains a registry of all available microservices with their ports and roles.
- All microservices expose a consistent API interface for query processing.
- The response format is standardized to include answer content and source attribution.
- Error responses are consistent and include diagnostic information for debugging.
## Technical Implementation Details
### Environment Variables
- `RAG_API_TOKEN`: Required for authentication to the RAG service.
- `RAG_BASE_NAME`
- `RAG_BASE_URL`: Optional base URL for the RAG service (default: `https://your-rag-service.com/api-path`).
### Asynchronous Operations
- The system uses asynchronous HTTP client/server operations.
- All HTTP requests and responses are handled asynchronously to improve performance and scalability.
- The `async`/`await` pattern is used to manage non-blocking operations efficiently.
### Security Considerations
- All API requests require valid authentication via API token.
- The system logs all access attempts for security auditing.
- Unauthorized access attempts trigger immediate 401 responses with clear error messages.
### Performance Optimization
- The system implements caching mechanisms to reduce redundant operations.
- Load balancing is applied to distribute workloads across multiple microservices.
- Performance bottlenecks are identified and optimized in retrieval, embedding, and LLM inference components.
### Deployment and Operationalization
- The system is deployed using containerization (Kubernetes) for scalability and fault tolerance.
- CI/CD pipelines ensure automated deployment and testing of new versions.
- Monitoring and logging tools provide real-time visibility into system performance and errors.

## Validation and Error Handling
All user interactions are validated with clear error messages and recovery mechanisms:

- **Input Validation**: All required parameters are validated before processing.
- **Error Messages**: Clear, user-friendly messages are returned for all error conditions.
- **Fallback Mechanisms**: When a microservice fails, the system continues to process remaining services and returns a partial response with annotations.
- **Security Auditing**: All failed authentication attempts are logged for monitoring and analysis.

This design ensures a robust, scalable, and user-friendly RAG system that can handle complex, multi-domain queries with high reliability and accuracy.