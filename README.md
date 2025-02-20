# **Computer Communication Assignment3 Tester**

Automated integration tests to evaluate students' Assignment 3 submissions in Computer Communication.

## Features
- Automated Testing: Streamlines the evaluation process by automating test execution.
- Comprehensive Coverage: Includes a variety of tests to ensure all aspects of the assignment are assessed.
- Detailed Logging: Generates logs for each test, aiding in debugging and result analysis.

## Prerequisites
- Before using the tester, ensure you have the following installed:
- Rust programing language
- Cargo package manager

### Test Descriptions
The tester includes the following tests:

1. **Usage** 
- Validates the handling of command-line arguments.
2. **Validate Input**
- Ensures robust request validation by the server, Server returns a **400 Bad Request** status when the HTTP method, path, or version is absent or malformed.
3. **Only GET Method**
- Confirms the server's adherence to supported HTTP methods. Checks that the Server responds with a **501 Not Implemented** status for methods other than **GET**.
4. **Path Does Not Exist**
- Verifies server behavior for non-existent paths. Server should return a **404 Not Found** status when the requested path is unavailable.
5. **Temporary Redirect**
- ssesses the server's handling of directory requests without trailing slashes.\
  The Server should issue a **302 Found** status, redirecting to the same path with a trailing slash when a directory is requested without it
6. **Forbidden**
- Ensures proper permission enforcement by the server.
- The Server should return a **403 Forbidden** status code under the following conditions:
    - The requested file lacks read permissions.
    - An ancestor directory lacks execute permissions, preventing traversal.
    - The requested resource is not a regular file.
7. **Search For Index Html**
- Verifies default file serving within directories When a directory path is requested, the server serves the index.html file within that directory, if present.
8. **Return Dir Content**
- Assesses directory listing functionality, if an index.html is absent in a requested directory, the server responds with a generated listing of the directory's contents.
9. **File Size Exceeds OS Buffer**
- Evaluates server performance with large files:
    - Server efficiently handles read/write operations for files exceeding the operating system's buffer size.
    - Ensures the **Content-Type** header is omitted when the MIME type is unidentifiable.
    - Verifies complete transmission of the file's bytes.
10. **Deadlock**
- Tests server resilience under concurrent load. Subjects the server's thread pool to high concurrency to detect potential deadlocks or resource contention issues.
11. **Valgrind**
- Detect memory leaks and errors using Valgrind.

### Tester Usage
1. **Prepare the Testee Directory**
- Place the student's assignment files inside the testee directory located in the working space root (parent directory).
2. **Execute the Tester** 
- Execute the following command ***from the working space root (parent directory)*** \
``` cargo run -p assignment3-tester``` \
This command will execute all the integration tests against the provided assignment files.
3. **Review Log Files**
- After execution, log files will be generated inside the testee directory.