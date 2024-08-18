
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
- **`src/`**: Rust source code for backend services
  - **`controllers/`**: API endpoints
  - **`services/`**: Business logic and AI model interaction
  - **`utils/`**: Utility functions
- **`web/`**: React.js frontend application
  - **`components/`**: Reusable UI components
  - **`services/`**: API calls for the backend
- **`models/`**: AI models for optimization recommendations
- **`tests/`**: Unit and integration tests
- **`scripts/`**: Deployment scripts for AWS infrastructure

---

## Best Practices
- **Code Quality**: Enforced through ESLint and Rust clippy.
- **Testing**: Unit tests for both backend and frontend; continuous testing through GitHub Actions.
- **Security**: JWT-based authentication for API endpoints.

---

## Contributing
We welcome contributions! Please ensure to:
- Follow our code standards.
- Write tests for new features.
- Submit PRs with meaningful commit messages.

---

## License
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

