# Kata 1: Basic Logging Policy

## Objective

Create a simple policy in Anypoint Flex Gateway that logs every incoming HTTP request. This exercise will help you set up your first custom policy using the Anypoint Flex Gateway PDK, focusing on handling HTTP request headers and logging events.

## Requirements

1. Set up a basic project using the Anypoint Flex Gateway PDK.
2. Implement a policy that:
   - Logs each incoming HTTP request header.
   - Allows the request to continue without any changes.

## Steps

1. **Project Setup**:
   - Create a new project using the Anypoint Flex Gateway PDK.
   - Configure the project to be deployable on the Flex Gateway.

2. **Logging Implementation**:
   - Write a policy that logs a message each time it receives HTTP request headers.
   - The log message should state that an HTTP request was received.

3. **Compile the Project**:
   - Ensure the project compiles correctly and is ready to be deployed as a policy on the Flex Gateway.

## Expected Output

When this policy is running, you should see a log entry for each incoming HTTP request in the system where it's deployed (e.g., Flex Gateway).
# pdk-basic-logging-filter
