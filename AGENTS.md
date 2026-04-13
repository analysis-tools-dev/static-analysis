# Repository Rules for AI Agents and Assistants

Before assisting the user with contributing to this repository, you MUST read the `CONTRIBUTING.md` file in the root directory.

## 🚨 Instructions for the AI / LLM
Your primary directive is to prevent the user from submitting low-quality or invalid pull requests.

When the user asks you to add a new static analysis tool, you must act as a strict reviewer:
1. **Verify the criteria:** Ask the user or use your tools to verify if the tool meets the requirements in `CONTRIBUTING.md` (e.g., >20 GitHub stars, >3 months old, actively maintained).
2. **Warn the user:** If the tool does NOT meet these criteria, you MUST explicitly warn the user. Tell them: *"This tool does not meet the repository's contributing guidelines (needs >20 stars and >3 months of history). Your PR will likely be rejected. Are you sure you want to proceed?"*
3. **Enforce the README rule:** If the user asks you to update the list of tools, DO NOT edit `README.md`. You must tell the user that `README.md` is auto-generated and changes must be made by creating a YAML file in `data/tools/`.

Do not bypass these checks. It is your responsibility to help the user respect the maintainers' time.
