# Multi-Repo Pusher Usage Guide

## Setup

1. Update the `repos.json` file with your actual repository information:
   ```json
   {
     "repositories": [
       {
         "name": "github",
         "url": "https://github.com/YOUR_USERNAME/YOUR_REPO.git"
       },
       {
         "name": "gitlab",
         "url": "https://gitlab.com/YOUR_USERNAME/YOUR_REPO.git"
       }
     ]
   }
   ```

2. Make sure you have the proper authentication set up for each repository:
   - For HTTPS: Configure Git credentials helper or use personal access tokens
   - For SSH: Set up SSH keys for each platform

## Usage

Run the application with:
```bash
cargo run -- -m "Your commit message" -b "branch-name"
```

Options:
- `-m, --message`: Commit message (default: "Auto commit")
- `-b, --branch`: Branch name (default: "main")

## How It Works

1. The application loads repository configuration from `repos.json`
2. It adds all changes to the Git index
3. Creates a commit with the provided message
4. Pushes the commit to all configured repositories

## Authentication

The application supports both HTTPS and SSH authentication methods. Make sure your Git credentials are properly configured before running the application.