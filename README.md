# Deploy Rust as an Azure Function
Continuous deployment to Azure Functions using Rust and GitHub Actions. This repository contains a sample Rust application that can be deployed to Azure Functions using GitHub Actions. Although not a requirement it is easier to use [Visual Studio Code](https://code.visualstudio.com/?WT.mc_id=academic-0000-alfredodeza) for development. 

This repository is part of a [Coursera MLOps course offered by Duke University](https://www.coursera.org/learn/mlops-aws-azure-duke)

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

## Creating the Azure Function in the Azure Portal
Use a unique name for the function app. This name will be used in the GitHub Actions workflow to deploy the application to Azure Functions. The name must be unique across all Azure Functions.

For the _Runtime stack_ select _Custom Handler_ and for the _version_ select _custom_ as well.

Select _Linux_ as the _Operating System_ and any region like _East US_.

Once the function app is created, click on _Get Publish Profile_ and add it as a secret to the GitHub repository. The name of the secret should be `AZURE_FUNCTIONAPP_PUBLISH_PROFILE` and the value should be the content of the downloaded file.

## Deploying with VSCode
Although you can deploy directlty from VSCode, you might encounter the following error when you try to re-deploy using a different method:

```
Error:   When request Azure resource at PublishContent, zipDepoy : WEBSITE_RUN_FROM_PACKAGE in your function app is set to an URL. Please remove WEBSITE_RUN_FROM_PACKAGE app setting from your function app.
```

This setting was created by the VSCode extension, so you must go to the portal, then to the function, next to Configuration, and then Application Settings, and delete `WEBSITE_RUN_FROM_PACKAGE`

## Deploying with GitHub Actions
The workflow is defined in the `.github/workflows/main.yml` file. The workflow is set to be triggered manually via the `workflow_dispatch` setting. You can change this to trigger when a new commit is pushed to the `main` branch. The workflow will build the application and deploy it to Azure Functions.

The only requirement is to add the `AZURE_CREDENTIALS` secret to the repository. The value of the secret should be the output of the following command:

```bash
az ad sp create-for-rbac --name "rust-azure-function" --sdk-auth --role contributor --scopes /subscriptions/<SUBSCRIPTION_ID>/resourceGroups/<RESOURCE_GROUP_NAME>
```

The resulting output should look like:

```
  {
    "clientId": "<GUID>",
    "clientSecret": "<GUID>",
    "subscriptionId": "<GUID>",
    "tenantId": "<GUID>",
    (...)
  }
```

Copy those contents into the actions secrets as `AZURE_CREDENTIALS`. This secret is used in the workflow to authenticate with Azure.

### Caveats
Currently, Rust is not a _first class citizen_ in Azure Functions. This means that the deployment process is a bit more involved than other languages. If creating the function scaffolding in Visual Studio Code, you must select _"custom"_ for the runtime, whereas other languages have their own category.

Deploying using the GitHub Actions for Functions _does not work_ at the moment. The workflow will succeed but the function will timeout when invoked with an HTTP 500. The GitHub workflow uses instead the Azure Core Tools to deploy the function with the `func` command. This is a workaround until the GitHub Actions for Functions [is fixed](https://github.com/Azure/functions-action/issues/169).
