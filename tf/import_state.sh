#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

echo "Importing OpenTofu state for all resources..."

tofu import github_repository.pairs pairs
tofu import github_workflow_repository_permissions.pairs_permissions pairs
tofu import github_branch.development pairs:dev
tofu import github_branch.main pairs:main
tofu import github_branch_default.default pairs
tofu import github_branch_protection.main_protection pairs:main
tofu import github_issue_labels.pairs_labels pairs

echo "All resources imported successfully!"
