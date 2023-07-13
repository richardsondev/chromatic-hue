# Rust Application Template

This is a template project to build a Rust application using GitHub Actions and automatically publish release artifacts for Windows, Linux, and macOS.

## How to Use

1. **Create a new repository from this template.**
    
    On GitHub, click the "Use this template" button to create a new repository based on this template.

2. **Clone your new repository.**
    
    Clone your newly created repository to your local machine and navigate to the root directory of the project.

3. **Run the project locally.**

    You can run the project locally by using the following command in your terminal:
    
    ```bash
    cargo run
    ```

    This command will compile your code and run the resulting executable.

4. **Commit and push changes.**
    
    After making changes, commit and push to your repository. GitHub Actions will automatically start the build process.

## GitHub Actions

This project uses GitHub Actions for continuous integration. Upon each push to your repository or release creation, the workflow will:

- Check out the latest code.
- Set up the necessary Rust environment.
- Build and test the project.
- Create release artifacts for Windows, Linux, and macOS. (Only on release creation events)

The artifacts will then be available for download from the "Actions" tab on your repository and also from the "Releases" tab for release events.

## Docker image

This project also contains `buildpush.sh` which will build a distroless Docker image with the application as the entry point. By default, the 
script will push the built multi-arch images to DockerHub. [Here](https://hub.docker.com/r/richardsondev/hello_world/tags) is an example of 
what it would look like.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the [MIT License](LICENSE).
