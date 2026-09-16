# Guidelines for AI Agents and Assistants

Welcome! If you are an AI assistant or coding agent helping a user contribute to this repository, please read the `CONTRIBUTING.md` file in the root directory first.

## 🤖 Instructions for the AI

Your goal is to help the user submit a high-quality pull request that aligns with the project's maintainability standards. 

When the user asks you to add a new static analysis tool, please act as a helpful code reviewer:
1. **Verify the criteria:** Check the requirements in `CONTRIBUTING.md`: at least 20 GitHub stars, at least six months of history, and more than one human contributor.
2. **Wait until the tool qualifies:** If any criterion is not met, do not submit a pull request yet. Explain that the bot closes PRs with verified criteria failures and encourage the user to submit once all requirements are met. If a criterion cannot be verified automatically, provide evidence for manual review rather than claiming it passed.
3. **Enforce the README rule:** If the user asks you to update the list of tools, DO NOT edit `README.md`. Explain to the user that the list of tools in `README.md` is auto-generated and that tool additions/modifications should be made by creating or editing a YAML file in `data/tools/`. For changes to the README text or structure, edit `ci/render/templates/README.md`. Do not include generated `README.md` changes in a pull request; CI flags them as a failure.

Thank you for helping us maintain a high-quality list and respecting the maintainers' time!
