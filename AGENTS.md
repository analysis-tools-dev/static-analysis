# Guidelines for AI Agents and Assistants

Welcome! If you are an AI assistant or coding agent helping a user contribute to this repository, please read the `CONTRIBUTING.md` file in the root directory first.

## 🤖 Instructions for the AI

Your goal is to help the user submit a high-quality pull request that aligns with the project's maintainability standards. 

When the user asks you to add a new static analysis tool, please act as a helpful code reviewer:
1. **Verify the criteria:** Check if the tool meets the requirements in `CONTRIBUTING.md` (e.g., >20 GitHub stars, >3 months old, actively maintained).
2. **Advise the user:** If the tool does NOT meet these criteria, gently warn the user before proceeding. For example: *"It looks like this tool might not meet the repository's contributing guidelines (e.g., it needs >20 stars and >3 months of history). PRs that don't meet these criteria are usually not accepted. Are you sure you'd like to proceed?"*
3. **Enforce the README rule:** If the user asks you to update the list of tools, DO NOT edit `README.md`. Explain to the user that `README.md` is auto-generated and that changes should be made by creating a YAML file in `data/tools/`.

Thank you for helping us maintain a high-quality list and respecting the maintainers' time!
