# Rust Serverless Transformer Endpoint

This project demonstrates how to dockerize a Hugging Face Rust transformer, deploy the container to AWS Lambda, and implement a query endpoint.

## Project Requirements

- Dockerize Hugging Face Rust transformer
- Deploy container to AWS Lambda
- Implement query endpoint

## Grading Criteria

- Transformer packaging: 30%
- Serverless deployment: 30%
- Endpoint functionality: 30%
- Documentation: 10%

## Deliverables

- Dockerfile and Rust code
- Screenshot of AWS Lambda
- cURL request against endpoint

## Demo

### Step 1: Dockerize Hugging Face Rust transformer

We created a Dockerfile for the Rust application. Here's the Dockerfile:

```Dockerfile
FROM rust:1.55.0 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
CMD ["myapp"]
```

### Step 2: Deploy container to AWS Lambda

We deployed the Docker container to AWS Lambda. Here's the command we used to push the Docker image to the ECR repository:

``` bash
docker push your-account-id.dkr.ecr.region.amazonaws.com/myapp:latest
```
Then, we created a new Lambda function and specified the Docker image as the code.

### Step 3: Implement query endpoint

We created a new Rust project and used AWS API Gateway to create a new API endpoint that triggers the Lambda function.
Here's the command we used to deploy the API:

```
aws apigateway create-deployment --rest-api-id your-rest-api-id --stage-name prod
```

#### Screenshots
Here's a screenshot of the AWS Lambda function:

![lambda](images/lambda.png)

#### Testing
You can test the endpoint using the following cURL command:

```
curl -X GET 'https://your-api-gateway-url/prod/myapp'
```
