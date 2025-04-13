.PHONY: clean-git

clean-git:
	@echo "🔍 Removing .git from subfolders"
	@find . -type d -name ".git" \
		-not -path "./.git" \
		-exec rm -rf {} +
	@echo "✅ Done."