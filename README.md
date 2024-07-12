# SkillForge

**SkillForge** is an interactive platform designed to help users practice and enhance their skills through solving various questions and problems. This project is developed in Rust, focusing on high performance and reliability.

## Description

SkillForge allows users to:

- Solve practice questions
- Track their progress
- Improve their skills over time

This repository contains the codebase for the SkillForge application. It leverages environment variables for configuration, ensuring flexibility and security.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust installation)

### Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/your-username/skillforge.git
    cd skillforge
    ```

2. Create a `.env` file in the project root by copying the provided `.env.test` file and updating the values as necessary:

    ```sh
    cp .env.test .env
    ```

### Running the Application

To run the application, use the following command:

    ```sh
    cargo run
    ```

### Testing

To run the tests, use the following command:

    ```sh
    cargo test
    ```

## Configuration

The application uses environment variables for configuration. Ensure you have a `.env` file in the project root with the required variables. Refer to the `.env.test` file for the necessary keys and their sample values.
