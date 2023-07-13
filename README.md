# Chromatic Hue

Chromatic Hue is a Rust application that smoothly transitions the color of Philips Hue lights through a color spectrum. It supports configuring multiple lights to sync at the same time and communicates with the Hue bridge on the local network. This utilizes the [hueclient](https://lib.rs/crates/hueclient) library for the communication with the Philips Hue bridge behind the scenes.

## Getting Started

To use Chromatic Hue, you have three options:

### 1. Using the Prebuilt Docker Image

You can use the prebuilt Docker image available on Docker Hub to quickly deploy Chromatic Hue without building from source.

```bash
$ docker run -d \
    -e HUE_BRIDGE_IP=<bridge_ip> \
    -e HUE_BRIDGE_USERNAME=<bridge_username> \
    -e HUE_LIGHT_IDS=<light_ids> \
    richardsondev/chromatic-hue
```

Replace `<bridge_ip>`, `<bridge_username>`, and `<light_ids>` with your own values.

### 2. Downloading the Prebuilt Release Binaries

You can also download prebuilt release binaries from the Chromatic Hue GitHub repository. Visit the [Releases](https://github.com/richardsondev/chromatic-hue/releases) page and download the appropriate binary for your operating system and architecture. Then, set the required environment variables mentioned in the previous section and run the downloaded binary.

### 3. Compiling from Source

To compile and run Chromatic Hue from source, follow these steps:

1. Install Rust and Cargo by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. Clone the Chromatic Hue repository:

```bash
$ git clone https://github.com/richardsondev/chromatic-hue.git
```

3. Navigate to the project directory:

```bash
$ cd chromatic-hue
```

4. Set the required environment variables:

- `HUE_BRIDGE_IP`: The IP address of your Hue bridge.
- `HUE_BRIDGE_USERNAME`: The username for accessing the Hue bridge API. Follow the Philips Hue documentation to create a new username.
- `HUE_LIGHT_IDS`: Comma-separated IDs of the lights you want to control.

5. Build and run the application:

```bash
$ cargo run
```

The application will start changing the color of the specified lights smoothly through a color spectrum. It will continue running indefinitely, periodically syncing with the current time.

## Configuration

The behavior of Chromatic Hue can be customized using the following environment variables:

- `HUE_BRIDGE_IP` (required): The IP address of the Hue bridge.
- `HUE_BRIDGE_USERNAME` (required): The username for accessing the Hue bridge API.
- `HUE_LIGHT_IDS` (required): Comma-separated IDs of the lights to control. Limit to a maximum of 100 light IDs.
- `FRAME_LIMIT` (optional): The maximum number of frames to run the animation (only for testing purposes).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
