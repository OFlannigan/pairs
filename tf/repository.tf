resource "github_repository" "pairs" {
  name        = "pairs"
  description = "Command line tool to simplify pair programming, written in Rust"

  visibility = "public"

  has_discussions = true
  has_issues      = true
  has_projects    = false
  has_wiki        = true
}

resource "github_workflow_repository_permissions" "pairs_permissions" {
  repository = github_repository.pairs.name

  can_approve_pull_request_reviews = true
  default_workflow_permissions     = "read"
}

resource "github_branch" "development" {
  branch     = "dev"
  repository = github_repository.pairs.name
}

resource "github_branch" "main" {
  branch     = "main"
  repository = github_repository.pairs.name
}

resource "github_branch_default" "default" {
  branch     = github_branch.main.branch
  repository = github_repository.pairs.name
}

resource "github_branch_protection" "main_protection" {
  repository_id = github_repository.pairs.node_id

  pattern          = "main"
  allows_deletions = false

  allows_force_pushes = false

  require_conversation_resolution = true

  required_status_checks {
    strict   = true
    contexts = ["CI"]
  }
}

resource "github_issue_labels" "pairs_labels" {
  repository = github_repository.pairs.name

  label {
    color = "FF0000"
    name  = "Bug"
  }

  label {
    color = "FFFF00"
    name  = "Needs Triage"
  }

  label {
    color = "00FF00"
    name  = "Feature"
  }

  label {
    color = "FF0000"
    name  = "Urgent"
  }

  label {
    color       = "000000"
    name        = "pre_commit"
    description = "Pull requests that update pre_commit code"
  }

  label {
    color       = "0366D6"
    name        = "dependecies"
    description = "Pull requests that update a dependency file"
  }
}
