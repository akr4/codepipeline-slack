# codepipeline-slack
Notify CodePipeline status changes such as STARTED, SUCCEEDED, FAILED, and so on to a Slack channel.

The repository includes a lambda function and a Terraform module, so it's easy to set up in your environment.

<img src="https://raw.githubusercontent.com/akr4/codepipeline-slack/master/docs/screenshot.png" alt="Example Slack messages" width="393">

## Overview

<img src="https://raw.githubusercontent.com/akr4/codepipeline-slack/master/docs/overview.png" alt="Overview" width="600">

## Prerequisite
- Terraform 0.12
- Docker

## Set up
1. Build the Lambda function
2. Set up a Slack webhook URL to SSM parameter store
3. Update your AWS environment with Terraform

### Build the Lambda function
The following command builds the Lambda function in a Docker environment.
```sh
docker run --rm -v ${PWD}:/code -v \
  ${HOME}/.cargo/registry:/root/.cargo/registry -v \
  ${HOME}/.cargo/git:/root/.cargo/git softprops/lambda-rust
```

The zipped Lambda function will be at `./target/lambda/release/codepipeline-slack.zip`.

### Set up a Slack webhook URL to SSM parameter store
The Lambda function looks for a Slack's webhook URL from SSM parameter store. The parameter name can be configured by a Terraform variable.

<dl>
  <dt>name</dt>
  <dd>/codepipeline-slack/webhook-url-ssm-param</dd>
  <dt>value  (SecureString is recommended)</dt>
  <dd>https:....</dd>
</dl>

### Update your AWS environment with Terraform
First, copy `terraform` directory to your Terraform directory. And then add a module block to your Terraform configuration file:
```HCL
module "codepipeline-slack" {
  source                          = "../modules/codepipeline-slack"
  slack_webhook_url_ssm_parameter = "/codepipeline-slack/webhook-url-ssm-param"
  lambda_zip_file                 = "lambda/codepipeline-slack.zip"
}
```

<dl>
  <dt>slack_webhook_url_ssm_parameter</dt>
  <dd>The SSM parameter name which is configured in the previous section</dd>
  <dt>lambda_zip_file</dt>
  <dd>Path to the Lambda zip file</dd>
</dl>

Now you can apply the configuration by `terraform apply`.
