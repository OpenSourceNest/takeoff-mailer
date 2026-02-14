
🚀 Takeoff Mailer
A high-performance, resilient email consumer built in Rust to handle transactional email delivery for the TAKEOFF event.
This service implements a decoupled, event-driven architecture using RabbitMQ for reliable message delivery and Protobuf for strict schema enforcement.
🏗 System Architecture
The mailer is designed with a "Reliability First" philosophy to ensure no attendee ever misses a QR code or confirmation.
 * Asynchronous Processing: Offloads email sending from the main API to a dedicated Rust worker.
 * Protocol Buffers: Uses email.proto to ensure data integrity between the Node.js producer and Rust consumer.
 * DLX (Dead Letter Exchange): Features an automated "Hospital" queue (email_failed_queue) for messages that fail after maximum retries or fail decoding.
🛠 Tech Stack
 * Language: Rust (Tokio runtime)
 * Broker: RabbitMQ (via lapin)
 * Serialization: Google Protocol Buffers (prost)
 * Monitoring: Datadog (Metrics/Traces) & Dozzle (Logs)
 * Deployment: Docker / VPS
🚦 Getting Started
Prerequisites
 * Rust 1.75+
 * protoc (Protobuf Compiler)
 * Docker & Docker Compose
Environment Variables
Create a .env file in the root directory:
AMQP_ADDR=amqp://guest:guest@localhost:5672/%2f
SMTP_RELAY=smtp.example.com
SMTP_USER=user@example.com
SMTP_PASS=password
RUST_LOG=info

Running Locally
 * Start RabbitMQ:
   docker compose up -d rabbitmq

 * Run the Worker:
   cargo run

🏥 Fault Tolerance & Recovery
This repo includes a built-in recovery "stitch" for handling failures:
The Dead Letter Pattern
If a message fails delivery 3 times, it is automatically routed to the email_failed_queue. This prevents "Poison Pill" messages from clogging the main pipeline.

📊 Observability
This service is fully instrumented for production visibility:
 * Health Checks: Integrated with Docker health checks.
 * Unified Tagging: Tags every log and metric with env:takeoff-prod and service:email-worker.
 * Dashboard: Real-time throughput and error rates are monitored via Datadog.
📜 License
Distributed under the MIT License. See LICENSE for more information.
