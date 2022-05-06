variable "fn_role" {}

resource "aws_lambda_function" "fn" {
  function_name = "tonteki"

  role = var.fn_role

  handler = "bootstrap"

  filename = "../target/lambda/tonteki-roulette/bootstrap.zip"
  source_code_hash = base64sha256(filesha256("../target/lambda/tonteki-roulette/bootstrap.zip"))
  runtime = "provided.al2"
  publish = false
  memory_size = 128
  timeout = 600
}

resource "aws_lambda_function_url" "fn" {
  function_name      = aws_lambda_function.fn.function_name
  authorization_type = "NONE"
}
