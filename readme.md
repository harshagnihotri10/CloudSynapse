# Cloud-Based Intelligent Makefile Dependency Visualizer

## Overview
This project visualizes Makefile dependencies and provides AI-powered optimization recommendations, deployed on AWS.

## Features
- Parse Makefiles and generate dependency graphs.
- Interactive visualization with D3.js.
- AI recommendations for Makefile optimizations.
- Cloud deployment on AWS.

## Installation
1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/cloud-makefile-visualizer.git
    cd cloud-makefile-visualizer
    ```

2. Build and run the backend:
    ```sh
    cargo build --release
    ./target/release/cloud-makefile-visualizer
    ```

3. Deploy the frontend:
    ```sh
    cd web
    npm install
    npm run build
    ```

4. Deploy to AWS:
    ```sh
    ./scripts/deploy.sh
    ```


