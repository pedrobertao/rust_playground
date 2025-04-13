.PHONY: clean-git

clean-git:
	@echo "🔍 Removendo .git de subpastas (exceto da raiz)..."
	@find . -type d -name ".git" \
		-not -path "./.git" \
		-exec rm -rf {} +
	@echo "✅ Limpeza concluída (raiz preservada)."