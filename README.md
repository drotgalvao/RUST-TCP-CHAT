# Rust TCP Chat Application

This project demonstrates a simple client-server application written in Rust. It showcases how to establish a TCP connection between a client and a server, allowing for bidirectional communication.

## Project Structure

The project is divided into two main components:

- **Client**: Located in the `client` directory, this component is responsible for connecting to the server and sending/receiving messages.
- **Server**: Located in the `server` directory, this component listens for incoming connections from clients and facilitates message exchange between them.

## Getting Started

### Prerequisites

- Rust: Ensure you have Rust installed on your system. You can download it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
- Cargo: Rust's package manager, which comes with the Rust installation.

### Useful Rust Commands

- **Install Rust 1.63.0**: `rustup toolchain install 1.63.0`
- **Set Default Rust Version**: `rustup default 1.63.0`
- **Check Rust Version**: `rustc --version`
- **Set Rust Version for Directory**: `rustup override set 1.63.0`

### Installation

1. Clone the repository:

   ```sh
   [git clone https://github.com/...](https://github.com/drotgalvao/RUST-TCP-CHAT.git)
   ```

2. Change into the project directory:

   ```sh
   cd directory
   ```

3. Build the client and server Docker images using Docker Compose:

   ```sh
   docker-compose build
   ```


### Usage by docker-compose

1. **Start the server**:

   ```sh
   docker-compose up -d server
   ```

2. **Start the client-1**:

   ```sh
   docker-compose up -d client-1
   ```

3. **Access the client-1 container with terminal**:

   You can access the client-1 Docker container using the terminal by running:

   ```sh
   docker exec -it rust-client-1 bash
   ```

   or access it through the Docker interface terminal. Once inside the container, navigate to the app directory and execute the client application using `./client`.

4. **Start the client-2**:

   ```sh
   docker-compose up -d client-2
   ```

5. **Access the client-2 container with terminal**:

   Similar to client-1, you can access the client-2 Docker container using the terminal by running:

   ```sh
   docker exec -it rust-client-2 bash
   ```

### Building and Running Releases

To build and run the release versions of the client and server projects in Rust and execute them via the `.exe` files on Windows, follow these steps:

### 1. Set global environment variable

To set a global environment variable persistently on Windows using `setx`, follow these steps:

1. Open a command prompt.

2. Use the `ipconfig` command to get the IPv4 address you want to set as `SERVER_ADDRESS`.

3. Set the `SERVER_ADDRESS` environment variable using `setx`. For example, if the IP address is `192.168.1.3:8080`, you can use the following command:

   ```sh
   setx SERVER_ADDRESS "192.168.1.3:8080"
   ```

   This will set the `SERVER_ADDRESS` variable with the value `192.168.1.3:8080` persistently.

**Verifying the SERVER_ADDRESS variable:**

To verify that the `SERVER_ADDRESS` variable is set correctly, you can use the following command:

```sh
echo $env:SERVER_ADDRESS
```

This command should display the value of the `SERVER_ADDRESS` variable. If it doesn't display anything, there may be an issue with the variable's configuration.

### 2. Build Releases

1. Open a terminal.

2. For the client:
   ```sh
   cd path/to/client
   cargo build --release
   ```

3. For the server:
   ```sh
   cd path/to/server
   cargo build --release
   ```

### 3. Running Executables

1. For the server:
   ```sh
   cd path/to/server/target/release
   ./server
   ```

2. For client-1:
   ```sh
   cd path/to/client/target/release
   ./client
   ```

3. For client-2, repeat step 2 for client-1. Open a new terminal to run each additional client as needed. Repeat this step for each client you want to start.

### Notes:

- Ensure that necessary environment variables, such as `SERVER_ADDRESS`, are correctly set for the client executables.
- The above steps assume you are in a Windows development environment. If you are in a Linux or MacOS environment, the commands may vary slightly.

This should help you build and run the release versions of your Rust projects on Windows.


### Environment Variables

- **SERVER_ADDRESS**: The address of the server that the client will connect to. Default is `server:8080`.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Diego Galvão
