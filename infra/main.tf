terraform {
  cloud {
      organization = "nspinosa"

      workspaces {
        name = "mushie_bot"
      }
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.16"
    }

    docker = {
      source  = "kreuzwerker/docker"
      version = "3.0.2"
    }
  }

  required_version = ">= 1.2.0"
}

provider "aws" {
  region = "us-east-1"
}

provider "docker" {
}

resource "aws_ecr_repository" "mushie_bot" {
  name = "mushie_bot"
}

resource "aws_ecs_cluster" "mushie_bot" {
  name = "mushie_bot"
}

resource "aws_ecs_task_definition" "mushie_bot_task" {
  family = "mushie_bot_task"
  container_definitions = jsonencode([{
      "name" : "mushie_bot_task",
      "image" : "${aws_ecr_repository.mushie_bot.repository_url}",
      "essential" : true,
      "portMappings" : [
        {
          "containerPort" : 3000,
          "hostPort" : 3000
        }
      ],
      "memory" : 512,
      "cpu" : 256
  }])

  requires_compatibilities = ["EC2"]
  network_mode             = "bridge"
  memory                   = 512
  cpu                      = 256
  execution_role_arn       = aws_iam_role.ecsTaskExecutionRole.arn
}

resource "aws_iam_role" "ecsTaskExecutionRole" {
  name               = "ecsTaskExecutionRole"
  assume_role_policy = data.aws_iam_policy_document.assume_role_policy.json
}

data "aws_iam_policy_document" "assume_role_policy" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["ecs-tasks.amazonaws.com"]
    }
  }
}

resource "aws_ecs_service" "mushie_bot" {
  name            = "mushie_bot_service"
  cluster         = "${aws_ecs_cluster.mushie_bot.id}"
  task_definition = "${aws_ecs_task_definition.mushie_bot_task.arn}"
  launch_type     = "EC2"
  desired_count   = 1
}
