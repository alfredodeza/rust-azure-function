# Deploy Rust as an Azure Function
Continuous deployment to Azure Functions using Rust and GitHub Actions. This repository contains a sample Rust application that can be deployed to Azure Functions using GitHub Actions. Although not a requirement it is easier to use [Visual Studio Code](https://code.visualstudio.com/?WT.mc_id=academic-0000-alfredodeza) for development.

## Prerequisites
- [Azure account](https://azure.microsoft.com/free/?WT.mc_id=academic-0000-alfredodeza)
- [Rust](https://www.rust-lang.org/tools/install)
- [Azure Functions Core Tools](https://docs.microsoft.com/azure/azure-functions/functions-run-local?WT.mc_id=academic-0000-alfredodeza)
- [Visual Studio Code](https://code.visualstudio.com/?WT.mc_id=academic-0000-alfredodeza)
- [Azure Functions extension for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=ms-azuretools.vscode-azurefunctions&WT.mc_id=academic-0000-alfredodeza)

This repository is also Codespaces ready, so you can use it directly from GitHub. The only requirement left would be to ensure you have an Azure account.

Use the following [link](https://learn.microsoft.com/azure/azure-functions/create-first-function-vs-code-other?tabs=go%2Cmacos&WT.mc_id=academic-0000-alfredodeza) as a reference to the Azure documentation 

## Running the application locally with Azure Functions Core Tools
1. Install the dependencies with `cargo build`
2. Start the Azure Functions Core Tools with `func start` (this feature is already installed with Codespaces)
3. Send a `curl` request to generate a JSON response

```bash
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"text":"example string"}' \
  http://localhost:7071/api/token
```
