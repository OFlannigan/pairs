variable "github_token" {
  description = "GitHub token with permissions to manage repositories"
  type        = string
  sensitive   = true
  default     = ""
}
