
# CloudSynapse
## Cloud-Based Intelligent Makefile Dependency Visualizer
A cloud-deployed intelligent Makefile dependency visualizer, leveraging AI to suggest optimization recommendations. This project enables developers to visualize complex Makefile dependencies and improve them with automated suggestions for efficiency and maintainability.

---

![Coverage](https://codecov.io/gh/harshagnihotri10/CloudSynapse/branch/main/graph/badge.svg)

---

## Table of Contents
- [Features](#features)
- [Tech Stack](#tech-stack)
- [Installation](#installation)
- [Usage](#usage)
- [API Reference](#api-reference)
- [Code Structure](#code-structure)
- [Contributing](#contributing)
- [License](#license)

---

## Features
- **Makefile Dependency Parsing**: Visualize and analyze Makefile dependencies.
- **AI-Powered Recommendations**: Automatically suggest optimizations for complex Makefiles.
- **Interactive Visualization**: Utilize D3.js for dynamic and user-friendly dependency graphs.
- **Cloud-Ready**: Designed to be deployed and scaled on AWS infrastructure.

---

## Tech Stack
- **Backend**: Rust (core logic and AI model integration)
- **Frontend**: React.js, D3.js for visualization
- **AI Models**: Python-based for optimization recommendations
- **Cloud Infrastructure**: AWS (EC2, S3, Lambda), Terraform for deployment
- **Database**: PostgreSQL for dependency data management
- **DevOps**: Docker, GitHub Actions for CI/CD

---

## Installation

### Clone the Repository
```bash
git clone https://github.com/harshagnihotri10/CloudSynapse.git
cd CloudSynapse
```

### Backend Setup
1. Install dependencies:
    ```bash
    cargo build --release
    ```
2. Start the backend server:
    ```bash
    ./target/release/cloudsynapse
    ```

### Frontend Setup
1. Navigate to the frontend directory:
    ```bash
    cd web
    ```
2. Install dependencies:
    ```bash
    npm install
    ```
3. Start the frontend server:
    ```bash
    npm start
    ```

### Deploy to AWS
1. Deploy using the provided deployment script:
    ```bash
    ./scripts/deploy.sh
    ```

---

## Usage
1. Access the frontend via the browser after starting the backend and frontend servers.
2. Upload a Makefile to the interface.
3. Visualize its dependencies.
4. Get AI-generated suggestions for optimizing your Makefile.

---

## API Reference

### Makefile Upload
#### POST /api/makefile/upload
- **Description**: Upload a Makefile to generate a dependency graph.
- **Request Body**:
    ```json
    {
      "file": "upload a Makefile"
    }
    ```
- Response:
    ```json
    {
      "graph_url": "url to the visualized dependency graph"
    }
    ```

### AI Recommendations
#### GET /api/optimization/recommend
- Description: Retrieve AI-generated optimization recommendations for a Makefile.
- Response:
    ```json
    {
      "recommendations": ["Optimization 1", "Optimization 2"]
    }
    ```

---

## Code Structure

- **`.github/workflows/`**: CI/CD configuration files
  - **`ci.yml`**: CI/CD pipeline configuration using GitHub Actions

- **`k8s/`**: Kubernetes deployment and service configuration
  - **`deployment.yaml`**: Kubernetes deployment manifest
  - **`service.yaml`**: Service definition for exposing backend
  - **`ingress.yaml`**: Ingress configuration for routing traffic

- **`models/`**: AI/ML models for Makefile optimization
  - **`optimization_model.py`**: Python-based AI model for optimization
  - **`__init__.py`**: Package initializer

- **`scripts/`**: Deployment and utility scripts
  - **`deploy.sh`**: Automates project deployment to AWS
  - **`setup_env.sh`**: Environment setup script
  - **`cleanup.sh`**: Cleans up the deployment environment

- **`src/`**: Rust source code for backend services
  - **`controllers/`**: API endpoints
    - **`makefile_controller.rs`**: Controller for handling Makefile-related requests
  - **`services/`**: Business logic and service layer
    - **`makefile_service.rs`**: Service for Makefile parsing and graph generation
  - **`utils/`**: Utility functions
    - **`graph_generator.rs`**: Logic for generating visual dependency graphs
  - **`models/`**: Rust models for AI integration and data processing
    - **`optimization_model.rs`**: AI interaction logic in Rust
  - **`main.rs`**: Entry point for the Rust backend

- **`terraform/`**: Infrastructure as Code using Terraform
  - **`main.tf`**: Main configuration for setting up cloud infrastructure
  - **`variables.tf`**: Customizable variables for the infrastructure
  - **`outputs.tf`**: Outputs from the infrastructure setup
  - **`provider.tf`**: Cloud provider configurations (e.g., AWS, GCP)

- **`web/`**: React.js frontend application
  - **`public/`**: Static assets and HTML templates
    - **`index.html`**: Main HTML file for the React app
  - **`src/`**: React source code
    - **`components/`**: Reusable UI components
      - **`GraphVisualizer.js`**: Component for visualizing Makefile dependencies
    - **`services/`**: API interaction logic
      - **`apiService.js`**: Service for backend API calls
    - **`App.js`**: Main React application component
  - **`package.json`**: Project metadata and Node.js dependencies
  - **`webpack.config.js`**: Webpack configuration for bundling the frontend

- **`Cargo.toml`**: Rust project configuration (dependencies and metadata)

- **`README.md`**: Project documentation

- **`LICENSE`**: License for the project (e.g., MIT)

- **`.gitignore`**: Ignored files and directories for Git

---

## Best Practices
- **Code Quality**: Enforced through ESLint and Rust clippy.
- **Testing**: Unit tests for both backend and frontend; continuous testing through GitHub Actions.
- **Security**: JWT-based authentication for API endpoints.

---

## Contributing
Contributions are welcome! Please ensure to:
- Follow the code standards.
- Write tests for new features.
- Submit PRs with meaningful commit messages.

---

## License
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

