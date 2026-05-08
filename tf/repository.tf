resource "github_repository" "pairs" {
  name        = "pairs"
  description = "Command line tool to simplify pair programming, written in Rust"

  visibility = "private"

  has_discussions = true
  has_issues      = true
  has_projects    = false
  has_wiki        = true

  ignore_vulnerability_alerts_during_read = false
}
