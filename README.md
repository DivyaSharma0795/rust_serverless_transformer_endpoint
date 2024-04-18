# Mini Project 10 - Rust Serverless Transformer Endpoint

-   Project Requirements

    -   Dockerize Hugging Face Rust transformer
    -   Deploy container to AWS Lambda
    -   Implement query endpoint
-   Grading Criteria
    -   Transformer packaging: 30%
    -   Serverless deployment: 30%
    -   Endpoint functionality: 30%
    -   Documentation: 10%
-   Deliverables
    -   Dockerfile and Rust code
    -   Screenshot of AWS Lambda
    -   cURL request against endpoint

### Step 1: Dockerize Hugging Face Rust transformer

-   Create a Dockerfile for the Rust application.

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

-   Create a new AWS Lambda function using the container image.
```
# Create a new ECR repository
aws ecr create-repository --repository-name myapp

# Authenticate Docker to your ECR repository
aws ecr get-login-password --region us-east-2 | docker login --username AWS --password-stdin your-account-id.dkr.ecr.region.amazonaws.com

# Tag your image
docker tag myapp:latest your-account-id.dkr.ecr.region.amazonaws.com/myapp:latest

# Push your image
docker push your-account-id.dkr.ecr.region.amazonaws.com/myapp:latest
```

Then, create a new Lambda function and specify the Docker image as the code:
    
    ```json
    {
        "containerImage": "your-account-id.dkr.ecr.region.amazonaws.com/myapp:latest"
    }
    ```

### Step 3: Implement query endpoint

-   Create a new Rust project using `cargo new myapp`
-   Use AWS API Gateway to create a new API endpoint that triggers the Lambda function.

```
# Create a new REST API
aws apigateway create-rest-api --name 'myapp'

# Create a new resource
aws apigateway put-rest-api --rest-api-id your-rest-api-id --parent-id your-parent-id --path-part myapp

# Create a new GET method
aws apigateway put-method --rest-api-id your-rest-api-id --resource-id your-resource-id --http-method GET --authorization-type "NONE"

# Set up integration with your Lambda function
aws apigateway put-integration --rest-api-id your-rest-api-id --resource-id your-resource-id --http-method GET --type AWS_PROXY --integration-http-method POST --uri arn:aws:apigateway:region:lambda:path/2015-03-31/functions/arn:aws:lambda:region:account-id:function:myapp/invocations

# Deploy your API
aws apigateway create-deployment --rest-api-id your-rest-api-id --stage-name prod
```

