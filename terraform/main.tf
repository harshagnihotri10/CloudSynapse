provider "aws" {
  region = "us-east-1"
}

resource "aws_s3_bucket" "orchestration-platform" {
  bucket = "orchestration-platform-bucket"
  acl    = "private"
}

resource "aws_eks_cluster" "orchestration_platform" {
  name     = "orchestration-platform"
  role_arn = aws_iam_role.eks_cluster.arn

  vpc_config {
    subnet_ids = aws_subnet.eks[*].id
  }
}
